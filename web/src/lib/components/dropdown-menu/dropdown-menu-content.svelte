<script lang="ts">
	import { cx } from "$lib/utils"
	import { DropdownMenu } from "bits-ui"

	let { sideOffset = 8, class: className, ...others }: DropdownMenu.ContentProps = $props()
</script>

<DropdownMenu.Portal>
	<DropdownMenu.Content
		{sideOffset}
		class={cx(
			"z-50 w-(--bits-dropdown-menu-anchor-width) min-w-52 overflow-x-hidden overflow-y-auto rounded-2xl border bg-black p-1 shadow-md duration-100 outline-none data-[side=bottom]:slide-in-from-top-2 data-[side=inline-end]:slide-in-from-left-2 data-[side=inline-start]:slide-in-from-right-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 data-[state=closed]:animate-out data-[state=closed]:overflow-hidden data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95",
			className
		)}
		{...others}
	/>
</DropdownMenu.Portal>
