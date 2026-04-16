<script>
	import { goto } from "$app/navigation"
	import Input from "$lib/components/Input.svelte"
	import Link from "$lib/components/Link.svelte"
	import Header from "./components/Header.svelte"

	const domains = ["naval", "linktree", "google", "instagram", "matt"]
</script>

<svelte:head>
	<title>shorter</title>
	<meta name="description" content="a domain shortener tool" />
</svelte:head>

<Header />

<div class="py-40">
	<div class="flex flex-col items-center gap-y-6">
		<h1 class="text-center text-3xl font-medium">find shorter versions of your domain.</h1>
		<Input
			placeholder="type a domain..."
			autocomplete="off"
			autofocus
			class="max-w-xl"
			oninput={(e) => goto("/search?q=" + encodeURIComponent(e.currentTarget.value))}
		/>
		<div class="mx-auto flex w-full max-w-xs flex-wrap justify-center gap-3">
			{#each domains as domain}
				<Link href={"/search?q=" + domain} intent="outline" class="text-neutral-500">
					{domain}
				</Link>
			{/each}
		</div>
	</div>
</div>
