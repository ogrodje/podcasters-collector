# Spotify Podcasters Collector 🦀

A tiny tool that collects stats from [Spotify Podcasters Platform](https://podcasters.spotify.com/).

It's written in Rust, and it was great fun to build.

We use it ourselves to check stats for our [Podcast Ogrodje](https://anchor.fm/ogrodje).

## Usage

```bash
Usage: podcasters-collector [OPTIONS] --email <EMAIL> --password <PASSWORD>

Options:
  -e, --email <EMAIL>
  -p, --password <PASSWORD>
  -f, --format <FORMAT>      [default: string] [possible values: string, json, csv]
  -h, --help                 Print help information
  -V, --version              Print version information
```

Or with Docker Image available on [GitHub Container Registry - `ghcr.io`][ghcr-podcasters-collector].

```bash
docker run --rm ghcr.io/ogrodje/podcasters-collector:latest \
  podcasters-collector --email <EMAIL> --password <PASSWORD>
```

## Development

Get the latest Rust and then use cargo to build and run this thing.

```bash
$ cargo build --release
$ ./target/release/podcasters-collector --help
```

Build a Docker image

```bash
docker build . -t ogrodje/podcasters-collector -f Dockerfile
```

## Author

- [Oto Brglez](https://github.com/otobrglez)

![Twitter Follow](https://img.shields.io/twitter/follow/otobrglez?style=social)


[ghcr-podcasters-collector]: https://github.com/ogrodje/podcasters-collector/pkgs/container/podcasters-collector
