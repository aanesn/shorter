<script>
	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import Input from "$lib/components/Input.svelte"

	let value = $state(browser ? (page.url.searchParams.get("q") ?? "") : "")

	$effect(() => {
		if (!browser) return
		goto(value ? "/search?q=" + encodeURIComponent(value) : "/search", { replaceState: true })
	})
</script>

<Input placeholder="type a domain..." autocomplete="off" autofocus bind:value />
