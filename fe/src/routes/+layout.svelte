<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import { ClerkProvider, SignedIn, SignedOut, UserButton } from 'svelte-clerk';
	import { colors } from '$lib/colors';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<style>
	:global(body) {
		background: #1C1C1E;
		margin: 0;
		padding: 0;
		color: white;
	}
</style>

<ClerkProvider>
	<header style="
		background: {colors.primaryCharcoal};
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 1rem 2rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	">
		<h1 style="margin: 0; font-size: 1.5rem; color: white;">Cynnycty</h1>
		<nav style="display: flex; gap: 1rem; align-items: center;">
			<SignedOut>
				<a href="/sign-in" style="text-decoration: none; color: white; font-weight: 500;">Sign In</a>
				<a href="/sign-up" style="background: {colors.primaryCyan}; color: white; padding: 0.5rem 1rem; border-radius: 6px; text-decoration: none; font-weight: 500;">Sign Up</a>
			</SignedOut>
			<SignedIn>
				<a href="/dashboard" style="text-decoration: none; color: white; margin-right: 1rem;">Dashboard</a>
				<UserButton />
			</SignedIn>
		</nav>
	</header>
	{@render children?.()}
</ClerkProvider>
