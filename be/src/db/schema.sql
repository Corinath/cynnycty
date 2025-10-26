-- Cynnycty Database Schema
-- ArcadeDB Multi-Model Database Schema Definition

-- ============================================================================
-- PROFILE TYPE
-- ============================================================================
-- Core user profile type that can function as both document and vertex
-- Uses internal userId as primary identifier for all relationships
-- clerkId is just for auth provider mapping (can be swapped later)

CREATE DOCUMENT TYPE Profile;

-- Core Identity Fields
-- userId: Our internal immutable identifier (UUID) - PRIMARY KEY
-- clerkId: Clerk's external auth ID (nullable for auth provider flexibility)
CREATE PROPERTY Profile.userId STRING;
CREATE PROPERTY Profile.clerkId STRING;

-- Profile Data Fields
CREATE PROPERTY Profile.displayName STRING;
CREATE PROPERTY Profile.aboutMe STRING;
CREATE PROPERTY Profile.avatarUrl STRING;

-- Timestamps
CREATE PROPERTY Profile.createdAt DATETIME;
CREATE PROPERTY Profile.updatedAt DATETIME;

-- ============================================================================
-- INDEXES
-- ============================================================================
-- Unique index on userId - our source of truth for all operations
CREATE INDEX Profile_userId_idx ON Profile (userId) UNIQUE;

-- Unique index on clerkId - for fast auth provider lookups
-- This is how we map from Clerk's ID to our internal userId
CREATE INDEX Profile_clerkId_idx ON Profile (clerkId) UNIQUE;

-- ============================================================================
-- FUTURE: Social Graph Edge Types (Commented for now)
-- ============================================================================
-- Uncomment these when ready to implement social features
-- All edges reference Profile via userId

-- CREATE EDGE TYPE Follows;
-- CREATE PROPERTY Follows.createdAt DATETIME;

-- CREATE EDGE TYPE Blocks;
-- CREATE PROPERTY Blocks.createdAt DATETIME;

-- ============================================================================
-- FUTURE: Content Vertex Types (Commented for now)
-- ============================================================================
-- CREATE VERTEX TYPE Post;
-- CREATE PROPERTY Post.postId STRING;
-- CREATE PROPERTY Post.authorId STRING;  -- References Profile.userId
-- CREATE PROPERTY Post.content STRING;
-- CREATE PROPERTY Post.createdAt DATETIME;
-- CREATE PROPERTY Post.updatedAt DATETIME;

-- CREATE EDGE TYPE Posted;  -- Profile -> Post
-- CREATE EDGE TYPE Liked;   -- Profile -> Post
-- CREATE EDGE TYPE Commented; -- Profile -> Post

-- ============================================================================
-- AUTH FLOW (Information Flow)
-- ============================================================================
-- Correct Flow: userId-first approach
--
-- 1. User signs up:
--    - Generate new userId (UUID)
--    - Get clerkId from Clerk after signup
--    - CREATE Profile with both userId and clerkId
--
-- 2. User signs in:
--    - Clerk provides clerkId
--    - Backend looks up: SELECT FROM Profile WHERE clerkId = ?
--    - Get userId from result
--    - Use userId for all subsequent operations
--
-- 3. Future auth provider migration:
--    - Keep userId unchanged (source of truth)
--    - Replace clerkId field with newAuthProviderId
--    - All relationships remain intact
-- ============================================================================
