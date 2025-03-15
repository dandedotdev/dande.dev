---
date: 2025-03-15 15:12:47
description: "This is the first blog post I've written. I want to document some of my findings and thoughts here."
draft: false
last_modified: 2025-03-15 15:12:47
tags: ['essay', 'leptos', 'rust']
title: 'My First Blog Post'
---

## Foreword

Hi, I'm Dandelion! My friends usually call me Dande. Welcome to my personal website, built with the [Leptos](https://github.com/leptos-rs/leptos) web framework and [Axum](https://github.com/tokio-rs/axum).

## Why Using Leptos

I've been using [Next.js](https://github.com/vercel/next.js) and [React](https://github.com/facebook/react) since 2022, before diving into the world of [Rust](https://github.com/rust-lang/rust).

I genuinely enjoy Rust's programming philosophy and wanted to incorporate it as much as possible in my work. While I recognize that the JavaScript ecosystem is quite mature, I was determined to build my personal website with Rust.

I spent about a week experimenting with both Leptos and [Dioxus](https://github.com/DioxusLabs/dioxus), and found that Leptos' JSX-like syntax was easier for me to use. That's why I ultimately chose Leptos.

> I also recommended using Dioxus for the frontend. A lot of people are contributing to its ecosystem.

## Acknowledgements

This blog template is highly inspired by [itehax/rust-blog](https://github.com/itehax/rust-blog) and [timlrx/tailwind-nextjs-starter-blog](https://github.com/timlrx/tailwind-nextjs-starter-blog).

I also learned a lot from [leptos-rs/leptos/examples/static-routing](https://github.com/leptos-rs/leptos/tree/main/examples/static_routing) and [this discussion on GitHub](https://github.com/leptos-rs/leptos/discussions/3039).

Later in development, I realized I should compile static posts at build time. [NiklasEi/cinnog_example](https://github.com/NiklasEi/cinnog_example) provided significant inspiration for this approach.

In the early stages of this project, I referenced the layouts of [timlrx/timlrx.com](https://github.com/timlrx/timlrx.com) and [hta218/leohuynh.dev](https://github.com/hta218/leohuynh.dev).

Thanks to the following projects, which I've used to build this website:

- [akesson/cargo-leptos](https://github.com/akesson/cargo-leptos)
- [bram209/leptosfmt](https://github.com/bram209/leptosfmt)
- [carloskiki/leptos-icons](https://github.com/carloskiki/leptos-icons)
- [favicon.io](https://favicon.io/)
- [fontsource/font-files](https://github.com/fontsource/font-files)
- [gaucho-labs/tailwind-fuse](https://github.com/gaucho-labs/tailwind-fuse)
- [igorshubovych/markdownlint-cli](https://github.com/igorshubovych/markdownlint-cli)
- [leptos-rs/leptos](https://github.com/leptos-rs/leptos)
- [lint-staged/lint-staged](https://github.com/lint-staged/lint-staged) or [hyf0/rslint-staged](https://github.com/hyf0/rslint-staged)
- [markdownlint/markdownlint](https://github.com/markdownlint/markdownlint)
- [prettier/prettier](https://github.com/prettier/prettier)
- [rayon-rs/rayon](https://github.com/rayon-rs/rayon)
- [rust-lang/rust](https://github.com/rust-lang/rust)
- [Synphonyte/leptos-use](https://github.com/Synphonyte/leptos-use)
- [tailwindlabs/tailwindcss](https://github.com/tailwindlabs/tailwindcss)
- [tamasfe/taplo](https://github.com/tamasfe/taplo)
- [timlrx/rehype-prism-plus](https://github.com/timlrx/rehype-prism-plus)
- [tokio-rs/axum](https://github.com/tokio-rs/axum)
- [typicode/husky](https://github.com/typicode/husky) or [rhysd/cargo-husky](https://github.com/rhysd/cargo-husky)
- [twitter/twemoji](https://github.com/twitter/twemoji)

At last, and most importantly, I would like to express my sincere gratitude to my mentors and friends, [dacozai](https://github.com/dacozai), [leonzchang](https://github.com/leonzchang) and [vpochapuis](https://github.com/vpochapuis), who provided invaluable assistance and guidance throughout the development of this project.

## Future Plans

I hope to continuously improve and implement component libraries necessary for this blog, and take time to share some development-related experiences (after all, I spent some time improving the code highlighting functionality).

I also hope that perhaps in the near future, I can pick up my interest in photography again and share some pictures of my life. I wish to document various things about myself.
