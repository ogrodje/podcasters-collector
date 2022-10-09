# anchor-collector

A tiny tool that collects stats from [Anchor](https://anchor.fm/).

It's written in Rust, and it was fun to build.

## Usage & Development

Get the latest Rust and then use cargo to build and run.

```bash
$ cargo build --relase
```

The tool needs `ANCHOR_EMAIL` and `ANCHOR_PASSWORD` environment variables to be set.

```bash
$ export ANCHOR_EMAIL="<email used for login>"
$ export ANCHOR_PASSWORD="<password used for login>"
$ ./target/release/anchor-collector
```

## Author

- [Oto Brglez](https://github.com/otobrglez)

![Twitter Follow](https://img.shields.io/twitter/follow/otobrglez?style=social)
