<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
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

	const lookupQuery = createQuery<LookupRes>(() => ({
		queryKey: ["lookup", searchQuery.data?.domains],
		queryFn: async () =>
			await fetch(`${apiUrl}/lookup`, {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({ domains: searchQuery.data?.domains ?? [] })
			}).then((r) => r.json()),
		enabled: !!searchQuery.data?.domains?.length
	}))
</script>

<svelte:head>
	<title>Search | Shorter</title>
	<meta name="description" content="Search for shorter versions of your domain" />
</svelte:head>

<div class="py-6">
	<Input placeholder="Type a domain..." autofocus bind:value />
	<div class="flex flex-col gap-y-3 pt-3">
		{#if searchQuery.isSuccess}
			{#each searchQuery.data.domains as domain, i}
				<div
					class="flex h-20 items-center justify-between rounded-2xl border bg-neutral-950 px-5"
				>
					<span class="truncate">{domain}</span>
					{#if lookupQuery.isSuccess}
						{@const isAvailable = lookupQuery.data.available[i]}
						<Button
							href={`https://www.dynadot.com/domain/search?rscreg=shorter&domain=${domain}`}
							target="_blank"
							rel="noreferrer"
							class={`w-24 ${isAvailable ? "text-green-500" : "text-red-500"}`}
						>
							{isAvailable ? "Continue" : "Lookup"}
						</Button>
					{:else}
						<div class="h-10 w-24 animate-pulse rounded-full bg-neutral-900"></div>
					{/if}
				</div>
			{/each}
			<p class="text-sm text-neutral-500">* These are affiliate links</p>
		{/if}
	</div>
</div>
