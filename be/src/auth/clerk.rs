use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClerkClaims {
    pub sub: String, // This is the clerkId
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub azp: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub email_verified: Option<bool>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<JwkKey>,
}

#[derive(Debug, Deserialize)]
struct JwkKey {
    #[serde(rename = "kid")]
    key_id: String,
    #[serde(rename = "n")]
    modulus: String,
    #[serde(rename = "e")]
    exponent: String,
}

pub struct ClerkJwks {
    keys: HashMap<String, DecodingKey>,
    publishable_key: String,
}

impl ClerkJwks {
    pub async fn new(publishable_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut jwks = Self {
            keys: HashMap::new(),
            publishable_key: publishable_key.clone(),
        };
        jwks.fetch_jwks().await?;
        Ok(jwks)
    }

    async fn fetch_jwks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Extract the instance ID from the publishable key
        // Format: pk_test_<base64>  or pk_live_<base64>
        // The base64 part encodes the Clerk frontend API domain
        let parts: Vec<&str> = self.publishable_key.split('_').collect();
        if parts.len() < 3 {
            return Err("Invalid Clerk publishable key format".into());
        }

        // Decode the base64 to get the domain
        // Note: Clerk's publishable key format is pk_test_<base64>$
        // The $ at the end is not base64, it's a delimiter
        // The decoded value also has $ at the end which needs stripping
        let encoded = parts[2].trim_end_matches('$');

        let decoded = String::from_utf8(
            general_purpose::STANDARD_NO_PAD.decode(encoded)
                .map_err(|e| format!("Failed to decode publishable key: {}", e))?,
        )?;

        // The decoded string also ends with $, strip it
        let domain = decoded.trim_end_matches('$');

        let jwks_url = format!("https://{}/.well-known/jwks.json", domain);

        tracing::info!("Fetching JWKS from: {}", jwks_url);

        let response = reqwest::get(&jwks_url).await?;
        let jwks_response: JwksResponse = response.json().await?;

        for key in jwks_response.keys {
            let decoding_key = DecodingKey::from_rsa_components(&key.modulus, &key.exponent)?;
            self.keys.insert(key.key_id, decoding_key);
        }

        tracing::info!("Loaded {} JWKS keys", self.keys.len());
        Ok(())
    }

    pub async fn verify_token(
        &self,
        token: &str,
    ) -> Result<ClerkClaims, Box<dyn std::error::Error>> {
        // Decode the header to get the key ID
        let header = decode_header(token).map_err(|e| {
            tracing::error!("Failed to decode JWT header: {}", e);
            e
        })?;

        let kid = header.kid.ok_or("Token missing 'kid' in header")?;

        // Get the corresponding decoding key
        let decoding_key = self
            .keys
            .get(&kid)
            .ok_or_else(|| {
                tracing::error!("Unknown key ID '{}' in token header. Available keys: {:?}", kid, self.keys.keys().collect::<Vec<_>>());
                "Unknown key ID in token header"
            })?;

        // Set up validation
        // Clerk tokens don't use audience validation in the standard way
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_aud = false; // Clerk doesn't use standard aud claim
        validation.validate_exp = true;

        // Decode and verify the token
        let token_data = decode::<ClerkClaims>(token, decoding_key, &validation).map_err(|e| {
            tracing::error!("JWT decode failed: {}", e);
            e
        })?;

        Ok(token_data.claims)
    }
}
