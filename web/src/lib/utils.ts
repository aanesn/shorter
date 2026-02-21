import { defineConfig } from "cva"
import { twMerge } from "tailwind-merge"
import { dev } from "$app/environment"

export const { cva, cx } = defineConfig({
	hooks: {
		onComplete: (className) => twMerge(className)
	}
})

export const apiUrl = dev
	? "http://127.0.0.1:8080"
	: "https://vpcfklbh3hvfsauarqu2zjcw6i0hjrrw.lambda-url.eu-west-1.on.aws"

export function buildSearchParams(params: Record<string, string>): string {
	const searchParams = new URLSearchParams()
	for (const [key, value] of Object.entries(params)) {
		value ? searchParams.set(key, value) : searchParams.delete(key)
	}
	return searchParams.toString() ? `?${searchParams}` : ""
}
