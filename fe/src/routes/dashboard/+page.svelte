<script lang="ts">
	import { user, session } from 'svelte-clerk';
	import { onMount } from 'svelte';
	import { colors } from '$lib/colors';

	let backendProfile: any = $state(null);
	let loading = $state(true);
	let error = $state(null);

	onMount(async () => {
		if ($session) {
			try {
				// Get the JWT token from Clerk
				const token = await $session.getToken();

				// Call the backend API
				const response = await fetch('http://localhost:3000/api/v1/profiles/me', {
					headers: {
						'Authorization': `Bearer ${token}`
					}
				});

				if (response.ok) {
					backendProfile = await response.json();
				} else {
					error = `Backend error: ${response.status}`;
				}
			} catch (e) {
				error = `Failed to fetch profile: ${e}`;
			} finally {
				loading = false;
			}
		} else {
			loading = false;
		}
	});
</script>

<div style="max-width: 800px; margin: 2rem auto; padding: 2rem;">
	<h1>Dashboard (Test Page)</h1>
	<p style="color: #666; margin-bottom: 2rem;">This is a temporary test page to verify Clerk authentication.</p>

	{#if $user}
		<div style="background: #f5f5f5; padding: 1.5rem; border-radius: 8px; margin-bottom: 2rem;">
			<h2 style="margin-top: 0;">Welcome, {$user.firstName || 'User'}!</h2>

			<div style="margin-top: 1rem;">
				<h3 style="margin-bottom: 0.5rem;">Clerk User Info (Frontend):</h3>
				<ul style="list-style: none; padding: 0;">
					<li><strong>Clerk ID:</strong> {$user.id}</li>
					<li><strong>Email:</strong> {$user.primaryEmailAddress?.emailAddress || 'N/A'}</li>
					<li><strong>Name:</strong> {$user.firstName} {$user.lastName}</li>
					<li><strong>Username:</strong> {$user.username || 'N/A'}</li>
				</ul>
			</div>
		</div>

		{#if loading}
			<div style="background: #e3f2fd; padding: 1.5rem; border-radius: 8px; margin-bottom: 2rem;">
				<p>Loading profile from backend...</p>
			</div>
		{:else if error}
			<div style="background: #ffebee; padding: 1.5rem; border-radius: 8px; border-left: 4px solid #f44336; margin-bottom: 2rem;">
				<h3 style="margin-top: 0;">Backend Error</h3>
				<p>{error}</p>
			</div>
		{:else if backendProfile}
			<div style="background: #e8f5e9; padding: 1.5rem; border-radius: 8px; border-left: 4px solid #4caf50; margin-bottom: 2rem;">
				<h3 style="margin-top: 0;">✅ Backend Profile (from Rust + ArcadeDB):</h3>
				<ul style="list-style: none; padding: 0;">
					<li><strong>User ID (Internal):</strong> {backendProfile.user_id}</li>
					<li><strong>Clerk ID:</strong> {backendProfile.clerk_id}</li>
					<li><strong>Display Name:</strong> {backendProfile.display_name || 'N/A'}</li>
					<li><strong>Email:</strong> {backendProfile.email || 'N/A'}</li>
				</ul>
				<p style="margin-top: 1rem; color: #2e7d32; font-weight: 500;">
					🎉 Full-stack authentication working! Profile created in ArcadeDB.
				</p>
			</div>
		{/if}

		<div style="background: #fff3cd; padding: 1.5rem; border-radius: 8px; border-left: 4px solid #ffc107;">
			<h3 style="margin-top: 0;">Next Steps:</h3>
			<p>Now that Clerk auth is working, we need to:</p>
			<ol>
				<li>Send Clerk JWT to Rust backend</li>
				<li>Backend verifies JWT and extracts clerkId</li>
				<li>Look up or create Profile (userId + clerkId)</li>
				<li>Return userId for all operations</li>
			</ol>
		</div>

		<div style="margin-top: 2rem; padding: 1.5rem; background: #e3f2fd; border-radius: 8px;">
			<h3 style="margin-top: 0;">Raw User Object (for debugging):</h3>
			<pre style="background: #263238; color: #aed581; padding: 1rem; border-radius: 4px; overflow-x: auto; font-size: 0.9rem;">{JSON.stringify($user, null, 2)}</pre>
		</div>
	{:else}
		<div style="background: #ffebee; padding: 1.5rem; border-radius: 8px; border-left: 4px solid #f44336;">
			<h3 style="margin-top: 0;">Not Authenticated</h3>
			<p>You should not see this page. The route is protected and should redirect to sign-in.</p>
		</div>
	{/if}
</div>
