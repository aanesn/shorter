<script>
	import { goto } from "$app/navigation"
	import Input from "$lib/components/Input.svelte"
	import Link from "$lib/components/Link.svelte"

	const domains = ["naval", "linktree", "google", "instagram", "matt"]
</script>

<svelte:head>
	<title>shorter</title>
	<meta name="description" content="a domain shortener tool" />
</svelte:head>

<div class="py-40">
	<div class="flex flex-col items-center gap-y-6">
		<h1 class="text-center font-manrope text-3xl font-semibold tracking-tight lg:text-4xl">
			search for shorter domains.
		</h1>
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

<div class="pt-20 pb-40">
	<video class="rounded-4xl" autoplay muted loop playsinline>
		<source src="/demo.mp4" type="video/mp4" />
	</video>
</div>
