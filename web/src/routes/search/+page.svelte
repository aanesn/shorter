<script lang="ts">
	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import type { LookupRes, SearchRes } from "$lib/bindings"
	import Input from "$lib/components/Input.svelte"
	import Link from "$lib/components/Link.svelte"
	import { apiUrl, dynadotUrl } from "$lib/utils"
	import { createQuery } from "@tanstack/svelte-query"

	let value = $state(browser ? (page.url.searchParams.get("q") ?? "") : "")
	let searchPath = $derived(value ? "/search?q=" + encodeURIComponent(value) : "/search")

	$effect(() => {
		if (!browser) return
		goto(searchPath, { replaceState: true })
	})

	const searchQuery = createQuery<SearchRes>(() => ({
		queryKey: ["search", value],
		queryFn: async () => await fetch(apiUrl + searchPath).then((res) => res.json()),
		enabled: !!value
	}))

	const lookupQueries = $derived(
		(searchQuery.data?.domains ?? []).map((domain) =>
			createQuery<LookupRes>(() => ({
				queryKey: ["lookup", domain],
				queryFn: async () =>
					await fetch(`${apiUrl}/lookup?domain=${domain}`).then((res) => res.json())
			}))
		)
	)
</script>

<svelte:head>
	<title>search | shorter</title>
	<meta name="description" content="search for shorter versions of your domain" />
</svelte:head>

<div class="flex flex-col gap-y-3 py-6">
	<Input placeholder="type a domain..." autocomplete="off" autofocus bind:value />
	{#if searchQuery.isSuccess}
		{#each searchQuery.data.domains as domain, i}
			{@const lookupQuery = lookupQueries[i]}
			<div class="flex h-18 w-full items-center justify-between rounded-3xl border px-6">
				<span class="truncate">{domain}</span>
				{#if lookupQuery.isSuccess}
					{@const available = lookupQuery.data.available}
					<Link
						href={`${dynadotUrl}&domain=${domain}`}
						intent="secondary"
						class={available ? "text-green-500" : "text-red-500"}
					>
						{available ? "continue" : "look up"}
					</Link>
				{:else if lookupQuery.isPending}
					<div class="h-7 w-20 animate-pulse rounded-full bg-neutral-900"></div>
				{/if}
			</div>
		{/each}
		<span class="text-sm text-neutral-500">* these are affiliate links</span>
	{/if}
</div>
