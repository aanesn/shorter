<script lang="ts">
	import type { HTMLInputAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { value = $bindable(), class: className, ...others }: HTMLInputAttributes = $props()
</script>

<input
	class={cx(
		"h-12 w-full min-w-0 rounded-full bg-neutral-900 px-4 py-1 shadow-xs transition-[color,box-shadow] outline-none placeholder:text-neutral-500",
		className
	)}
	bind:value
	{...others}
/>
