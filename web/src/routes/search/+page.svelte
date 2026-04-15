<script>
	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import Input from "$lib/components/Input.svelte"

	let value = $state(page.url.searchParams.get("q") ?? "")

	$effect(() => {
		goto(value ? "/search?q=" + encodeURIComponent(value) : "/search", { replaceState: true })
	})
</script>

<Input placeholder="type a domain..." autocomplete="off" autofocus bind:value />
