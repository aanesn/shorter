<script>
	import Input from "$lib/components/Input.svelte"
	import Header from "./components/Header.svelte"
</script>

<svelte:head>
	<title>shorter</title>
	<meta name="description" content="a domain shortener tool" />
</svelte:head>

<Header />

<div class="py-40">
	<div class="flex flex-col items-center gap-y-6">
		<h1 class="text-center text-3xl font-medium">find shorter versions of your domain.</h1>
		<Input class="max-w-xl" placeholder="type a domain..." autocomplete="off" autofocus />
	</div>
</div>
