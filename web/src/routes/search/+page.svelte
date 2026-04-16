<script>
	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import Input from "$lib/components/Input.svelte"
	import { apiUrl } from "$lib/utils"
	import { createQuery } from "@tanstack/svelte-query"

	let value = $state(browser ? (page.url.searchParams.get("q") ?? "") : "")
	let searchPath = $derived(value ? "/search?q=" + encodeURIComponent(value) : "/search")

	$effect(() => {
		if (!browser) return
		goto(searchPath, { replaceState: true })
	})

	const searchQuery = createQuery(() => ({
		queryKey: ["search", value],
		queryFn: async () => await fetch(apiUrl + searchPath).then((res) => res.json()),
		enabled: !!value
	}))
</script>

<svelte:head>
	<title>search | shorter</title>
	<meta name="description" content="search for shorter versions of your domain" />
</svelte:head>

<Input placeholder="type a domain..." autocomplete="off" autofocus bind:value />
{JSON.stringify(searchQuery.data)}
