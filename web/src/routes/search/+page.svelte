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
		goto("/search" + searchParams, { replaceState: true, keepFocus: true, noScroll: true })
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

<div class="flex min-h-screen flex-col">
	<div class="flex flex-1 flex-col gap-y-3 pt-3 pb-15">
		{#each searchQuery.data?.domains as domain, i}
			{@const lookup = lookupQueries[i]}
			{@const available = lookup.data?.available}
			<div
				class="flex h-20 items-center justify-between rounded-2xl border bg-neutral-950 px-5"
			>
				{domain}
				{#if lookup.isSuccess}
					<Button
						href={`https://www.dynadot.com/domain/search?rscreg=shorter&domain=${domain}`}
						target="_blank"
						rel="noreferrer"
						class={`w-24 ${available ? "text-green-500" : "text-red-500"}`}
					>
						{available ? "Continue" : "Lookup"}
					</Button>
				{:else}
					<div class="h-10 w-24 animate-pulse rounded-full bg-neutral-900"></div>
				{/if}
			</div>
		{/each}
	</div>
	<Input placeholder="Type a domain..." class="sticky bottom-12" autofocus bind:value />
</div>
