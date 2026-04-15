<script lang="ts">
	import { cx } from "$lib/utils"
	import { DropdownMenu } from "bits-ui"

	let { class: className, ...others }: DropdownMenu.ItemProps = $props()
</script>

<DropdownMenu.Item
	class={cx(
		"relative flex cursor-default items-center gap-2 rounded-xl px-2 py-1.5 text-sm outline-hidden select-none focus:bg-neutral-900 [&_svg]:pointer-events-none [&_svg]:shrink-0",
		className
	)}
	{...others}
/>
