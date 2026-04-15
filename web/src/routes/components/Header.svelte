<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Link from "$lib/components/Link.svelte"

	const nav = [
		{ title: "dynadot", href: "https://www.dynadot.com/domain/search?rscreg=shorter" },
		{ title: "repo", href: "https://github.com/aanesn/shorter" },
		{ title: "contact", href: "mailto:contact@shorter.dev" }
	]
</script>

<header class="flex h-16 items-center justify-between">
	<nav class="flex items-center gap-x-2">
		<a href="/" class="mr-2.5">
			{@html Logomark}
		</a>
		{#each nav as { title, href }}
			<Link {href} intent="secondary" class="hidden lg:flex">
				{title}
			</Link>
		{/each}
	</nav>
	<Link class="duration-300">start now</Link>
</header>
