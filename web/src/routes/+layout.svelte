<script lang="ts">
	import "./layout.css"

	let { children } = $props()
</script>

<div class="container mx-auto px-6 lg:px-20 xl:max-w-7xl">
	{@render children()}
</div>
