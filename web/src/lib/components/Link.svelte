<script lang="ts">
	import { cva, cx } from "$lib/utils"
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes } from "svelte/elements"

	const link = cva({
		base: "inline-flex shrink-0 items-center justify-center rounded-full text-sm whitespace-nowrap transition-all outline-none select-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			intent: {
				primary: "bg-white font-medium text-black hover:bg-white/80",
				secondary: "hover:bg-neutral-900",
				outline: "border hover:bg-neutral-900"
			},
			size: {
				default: "h-7 px-3"
			}
		}
	})

	let {
		intent = "primary",
		size = "default",
		class: className,
		children,
		...others
	}: HTMLAnchorAttributes & {
		intent?: VariantProps<typeof link>["intent"]
		size?: VariantProps<typeof link>["size"]
	} = $props()
</script>

<a class={cx(link({ intent, size }), className)} {...others}>
	{@render children?.()}
</a>
