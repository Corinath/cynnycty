<script lang="ts">
	import { colors } from '$lib/colors';

	interface Props {
		variant?: 'primary' | 'secondary' | 'tertiary';
		href?: string;
		type?: 'button' | 'submit';
		onclick?: () => void;
		[key: string]: any;
	}

	let {
		variant = 'primary',
		href,
		type = 'button',
		onclick,
		children,
		...restProps
	}: Props = $props();

	// Define styles for each variant
	const baseStyles = `
		padding: 1rem 2.5rem;
		border-radius: 8px;
		text-decoration: none;
		font-size: 1.125rem;
		display: inline-block;
		cursor: pointer;
		border: none;
		transition: opacity 0.2s, border-color 0.2s;
		font-family: inherit;
	`;

	const variantStyles = {
		primary: `
			background: ${colors.primaryCyan};
			color: white;
			font-weight: 600;
			border: 2px solid ${colors.primaryCyan};
		`,
		secondary: `
			background: transparent;
			color: ${colors.primaryCyan};
			font-weight: 500;
			border: 2px solid ${colors.primaryCyan};
		`,
		tertiary: `
			background: transparent;
			color: ${colors.primaryCyan};
			font-weight: 500;
			border: 2px solid transparent;
		`
	};

	const combinedStyles = baseStyles + variantStyles[variant];
</script>

{#if href}
	<a {href} style={combinedStyles} {...restProps}>
		{@render children?.()}
	</a>
{:else}
	<button {type} {onclick} style={combinedStyles} {...restProps}>
		{@render children?.()}
	</button>
{/if}

<style>
	a:hover,
	button:hover {
		opacity: 0.9;
	}

	button:active,
	a:active {
		opacity: 0.8;
	}
</style>
