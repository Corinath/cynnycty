<script lang="ts">
	import { colors } from '$lib/colors';
	import { Button } from '$lib/components/common';
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
			<Button variant="primary" href="/sign-up">Sign Up</Button>
			<Button variant="secondary" href="/sign-in">Sign In</Button>
		</div>
	</div>
</div>
