<script lang="ts">
	import { cva, cx } from "$lib/utils"
	import type { VariantProps } from "cva"
	import type { HTMLAnchorAttributes } from "svelte/elements"

	const link = cva({
		base: "inline-flex h-7 shrink-0 cursor-pointer items-center justify-center rounded-full px-3 text-sm whitespace-nowrap transition-all outline-none select-none [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			intent: {
				primary: "bg-white text-black hover:bg-white/80",
				secondary: "hover:bg-neutral-900",
				outline: "border hover:bg-neutral-900"
			}
		}
	})

	let {
		class: className,
		children,
		intent = "primary",
		...others
	}: HTMLAnchorAttributes & {
		intent?: VariantProps<typeof link>["intent"]
	} = $props()
</script>

<a class={cx(link({ intent }), className)} {...others}>
	{@render children?.()}
</a>
