<script lang="ts">
	import Input from "$lib/components/Input.svelte"
	import { buildSearchParams } from "$lib/utils"
	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/state"

	let value = $state(browser ? (page.url.searchParams.get("q") ?? "") : "")

	$effect(() => {
		if (value) goto("/search" + buildSearchParams({ q: value }))
	})
</script>

<svelte:head>
	<title>Shorter</title>
	<meta name="description" content="A domain shortener tool" />
</svelte:head>

<div class="py-20 lg:py-40">
	<div class="flex flex-col items-center gap-y-4">
		<h1 class="text-center font-manrope text-4xl font-semibold md:text-5xl lg:text-6xl">
			Search for shorter <br /> domains
		</h1>
		<h2 class="max-w-sm text-center text-neutral-400 max-md:text-balance lg:text-lg">
			Discover shorter versions of your domain like linktree.com -> linktr.ee
		</h2>
		<Input placeholder="Type a domain..." class="max-w-xs" autofocus bind:value />
	</div>
</div>

<video src="/demo.mp4" autoplay loop muted playsinline class="my-20 rounded-2xl shadow-xl"></video>
