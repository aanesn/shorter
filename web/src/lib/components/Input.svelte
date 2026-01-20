<script lang="ts">
	import type { HTMLInputAttributes } from "svelte/elements"
	import { cx } from "$lib/utils"

	let { value = $bindable(), class: className, ...restProps }: HTMLInputAttributes = $props()
</script>

<input
	class={cx(
		"flex h-11 w-full min-w-0 rounded-full bg-neutral-900 px-4.5 py-1 text-base shadow-xs outline-none placeholder:text-neutral-500 md:text-sm",
		className
	)}
	bind:value
	{...restProps}
/>
