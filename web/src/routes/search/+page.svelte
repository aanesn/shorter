<script lang="ts">
	import { createQueries, createQuery } from "@tanstack/svelte-query"
	import type { LookupRes, SearchRes } from "$lib/bindings"
	import Button from "$lib/components/Button.svelte"
	import Input from "$lib/components/Input.svelte"
	import { apiUrl, buildSearchParams } from "$lib/utils"
	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/state"

	let value = $state(browser ? (page.url.searchParams.get("q") ?? "") : "")
	let searchParams = $derived(buildSearchParams({ q: value }))

	$effect(() => {
		goto("/search" + searchParams, { replaceState: true })
	})

	const searchQuery = createQuery<SearchRes>(() => ({
		queryKey: ["search", value],
		queryFn: async () => await fetch(`${apiUrl}/search${searchParams}`).then((r) => r.json())
	}))

	const lookupQueries = createQueries(() => ({
		queries: (searchQuery.data?.domains ?? []).map((domain) => ({
			queryKey: ["lookup", domain],
			queryFn: async (): Promise<LookupRes> =>
				await fetch(`${apiUrl}/lookup?domain=${domain}`).then((r) => r.json())
		}))
	}))
</script>

<svelte:head>
	<title>Search | Shorter</title>
	<meta name="description" content="Search for shorter versions of your domain" />
</svelte:head>

<Input placeholder="Type a domain..." class="fixed right-0 bottom-10 left-0" autofocus bind:value />
