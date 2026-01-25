<p align="center">
  <img width="150" height="150" src="https://pub-559bbe0eb4084860a418c2ff890aa138.r2.dev/shorter.png" alt="logomark">
  <h1 align="center">Shorter</h1>
  <p align="center">
  	A domain shortener tool
    <br />
    <a href="https://shorter.dev"><strong>shorter.dev</strong></a>
  </p>
</p>

Shorter is a domain shortener tool that helps you find shorter versions of your domain like linktree.com -> linktr.ee

## Installation

Make sure you have [git](https://git-scm.com) installed, then clone this repository:

```bash
git clone https://github.com/aanesn/shorter
```

## Getting started

To run the server you need [rust](https://rust-lang.org) installed, then run this command:

```bash
cargo run
```

To run the website you need a javascript package manager like [pnpm](https://pnpm.io) installed, then run these commands:

```bash
cd web
pnpm i
pnpm run dev
```

## Architecture

- `server`: A http api built with [axum](https://github.com/tokio-rs/axum) and deployed to [aws lambda](https://aws.amazon.com/lambda)
- `web`: A static site built with [svelte](https://svelte.dev) and deployed to [cloudflare](https://cloudflare.com)

## Credits

Inspired by [instant domain](https://instantdomainsearch.com) and [panelsdesu](https://panelsdesu.com)
