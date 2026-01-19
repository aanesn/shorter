<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Button from "$lib/components/Button.svelte"
</script>

<header class="flex h-16 items-center justify-between lg:h-18">
	<a href="/">
		{@html Logomark}
	</a>
	<Button class="duration-300" intent="secondary">Search</Button>
</header>
