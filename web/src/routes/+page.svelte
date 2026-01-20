<script>
	import Input from "$lib/components/Input.svelte"
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
		<Input placeholder="Type a domain..." class="max-w-xs" autofocus />
	</div>
</div>
