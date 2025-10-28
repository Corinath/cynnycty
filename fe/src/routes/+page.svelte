<script lang="ts">
	import { colors } from '$lib/colors';
</script>

<div style="
	background: {colors.primaryCharcoal};
	min-height: calc(100vh - 5rem);
	display: flex;
	align-items: center;
	justify-content: center;
	padding: 2rem;
">
	<div style="
		text-align: center;
		max-width: 600px;
	">
		<h1 style="
			color: white;
			font-size: 3.5rem;
			margin: 0 0 1rem 0;
			font-weight: 700;
		">
			Welcome to Cynnycty
		</h1>

		<p style="
			color: rgba(255, 255, 255, 0.8);
			font-size: 1.5rem;
			margin: 0 0 3rem 0;
			font-weight: 300;
		">
			The totally coolest thing ever
		</p>

		<div style="
			display: flex;
			gap: 1rem;
			justify-content: center;
			flex-wrap: wrap;
		">
			<a href="/sign-up" style="
				background: {colors.primaryCyan};
				color: white;
				padding: 1rem 2.5rem;
				border-radius: 8px;
				text-decoration: none;
				font-weight: 600;
				font-size: 1.125rem;
				transition: opacity 0.2s;
				display: inline-block;
			">
				Sign Up
			</a>

			<a href="/sign-in" style="
				background: transparent;
				color: white;
				padding: 1rem 2.5rem;
				border-radius: 8px;
				text-decoration: none;
				font-weight: 500;
				font-size: 1.125rem;
				border: 2px solid rgba(255, 255, 255, 0.3);
				transition: border-color 0.2s;
				display: inline-block;
			">
				Sign In
			</a>
		</div>
	</div>
</div>
