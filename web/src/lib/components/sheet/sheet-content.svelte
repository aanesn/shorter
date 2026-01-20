<script lang="ts">
	import { Dialog } from "bits-ui"
	import type { VariantProps } from "cva"
	import type { ComponentProps, Snippet } from "svelte"
	import { cva, cx } from "$lib/utils"
	import SheetOverlay from "./sheet-overlay.svelte"

	const sheetVariants = cva({
		base: "bg-black data-[state=open]:animate-in data-[state=closed]:animate-out fixed z-50 flex flex-col gap-4 shadow-lg transition ease-in-out data-[state=closed]:duration-300 data-[state=open]:duration-500",
		variants: {
			side: {
				left: "data-[state=closed]:slide-out-to-start data-[state=open]:slide-in-from-start inset-y-0 start-0 h-full w-3/4 border-e sm:max-w-sm",
				right: "data-[state=closed]:slide-out-to-end data-[state=open]:slide-in-from-end inset-y-0 end-0 h-full w-3/4 border-s sm:max-w-sm"
			}
		}
	})
	let {
		ref = $bindable(null),
		class: className,
		side = "right",
		portalProps,
		children,
		...restProps
	}: Dialog.ContentProps & {
		portalProps?: ComponentProps<typeof Dialog.Portal>
		side?: VariantProps<typeof sheetVariants>["side"]
		children: Snippet
	} = $props()
</script>

<Dialog.Portal {...portalProps}>
	<SheetOverlay />
	<Dialog.Content class={cx(sheetVariants({ side }), className)} {...restProps}>
		{@render children?.()}
	</Dialog.Content>
</Dialog.Portal>
