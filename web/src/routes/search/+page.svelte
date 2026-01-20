<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import type { SearchRes } from "$lib/bindings"
	import Input from "$lib/components/Input.svelte"
	import { apiUrl, buildSearchParams } from "$lib/utils"
	import { goto } from "$app/navigation"
	import { page } from "$app/state"

	let value = $state(page.url.searchParams.get("q") ?? "")
	let searchParams = $derived(buildSearchParams({ q: value }))

	$effect(() => {
		if (value) goto("/search" + searchParams, { replaceState: true })
	})

	const searchQuery = createQuery<SearchRes>(() => ({
		queryKey: ["search", value],
		queryFn: async () => await fetch(`${apiUrl}/search${searchParams}`).then((r) => r.json())
	}))
</script>

<svelte:head>
	<title>Search | Shorter</title>
	<meta name="description" content="Search for shorter versions of your domain" />
</svelte:head>

<Input placeholder="Type a domain..." autofocus bind:value />
{JSON.stringify(searchQuery.data)}
