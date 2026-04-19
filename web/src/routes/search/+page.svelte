<script lang="ts">
	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import type { SearchRes } from "$lib/bindings"
	import Input from "$lib/components/Input.svelte"
	import Link from "$lib/components/Link.svelte"
	import { apiUrl } from "$lib/utils"
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
</script>

<svelte:head>
	<title>search | shorter</title>
	<meta name="description" content="search for shorter versions of your domain" />
</svelte:head>

<div class="flex flex-col gap-y-3 py-6">
	<Input placeholder="type a domain..." autocomplete="off" autofocus bind:value />
	{#if searchQuery.isSuccess}
		{#each searchQuery.data.domains as domain}
			<div class="flex h-18 w-full items-center justify-between rounded-3xl border px-6">
				<span class="truncate">{domain}</span>
				<Link
					href={"https://www.dynadot.com/domain/search?rscreg=shorter&domain=" + domain}
					intent="secondary"
					class="text-red-500"
				>
					look up
				</Link>
			</div>
		{/each}
		<span class="text-sm text-neutral-500">* these are affiliate links</span>
	{/if}
</div>
