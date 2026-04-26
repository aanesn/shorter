## installation

make sure you've got [git](https://git-scm.com) installed, then clone this repo:

```sh
git clone https://github.com/aanesn/shorter.git
```

to run the server you need [rust](https://rust-lang.org) installed, then from inside the `shorter` directory run this command:

```sh
cargo run
```

to run the website you need a js package manager like [pnpm](https://pnpm.io), then from inside the `shorter/web` directory run these commands:

```sh
pnpm i
pnpm run dev
```

## architecture

- `server`: an http api written in [axum](https://github.com/tokio-rs/axum) and deployed to [aws lambda](https://aws.amazon.com/lambda)
- `web`: a static site written in [svelte](https://svelte.dev) and deployed to [cloudflare pages](https://pages.cloudflare.com)

## credits

inspired by [panelsdesu](https://panelsdesu.com) and [instantdomainsearch](https://instantdomainsearch.com)
