<script lang="ts">
	import Header from "./components/Header.svelte"
	import "./layout.css"

	let { children } = $props()
</script>

<div class="container mx-auto px-6 lg:px-16 xl:max-w-7xl">
	<Header />
	{@render children()}
</div>
