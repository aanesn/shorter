<script lang="ts">
	import Logomark from "$lib/assets/logomark.svg?raw"
	import Menu from "$lib/assets/menu.svg?raw"
	import Link from "$lib/components/Link.svelte"
	import * as DropdownMenu from "$lib/components/dropdown-menu"

	const nav = [
		{ title: "dynadot", href: "https://www.dynadot.com/domain/search?rscreg=shorter" },
		{ title: "repo", href: "https://github.com/aanesn/shorter" },
		{ title: "contact", href: "mailto:contact@shorter.dev" }
	]

	const dropdownMenu = [{ title: "start now", href: "/" }, ...nav]
</script>

<header class="flex h-16 items-center justify-between">
	<nav class="flex items-center gap-x-3">
		<a href="/" class="mr-3">
			{@html Logomark}
		</a>
		{#each nav as { title, href }}
			<Link {href} intent="secondary" class="hidden lg:flex">
				{title}
			</Link>
		{/each}
	</nav>
	<Link class="hidden duration-300 lg:flex">start now</Link>
	<DropdownMenu.Root>
		<DropdownMenu.Trigger class="lg:hidden">
			{@html Menu}
		</DropdownMenu.Trigger>
		<DropdownMenu.Content class="lg:hidden">
			{#each dropdownMenu as { title, href }}
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a {...props} {href}>{title}</a>
					{/snippet}
				</DropdownMenu.Item>
			{/each}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
</header>
