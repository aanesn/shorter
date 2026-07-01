<script lang="ts">
	import Footer from "./components/Footer.svelte"
	import Header from "./components/Header.svelte"

	let { children } = $props()
</script>

<div class="flex min-h-screen flex-col">
	<Header />
	<div class="flex-1">
		{@render children()}
	</div>
	<Footer />
</div>
