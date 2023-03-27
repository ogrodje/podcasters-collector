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
  -f, --format <FORMAT>      [default: string] [possible values: string, csv, influx-dbcsv, json]
  -h, --help                 Print help information
  -V, --version              Print version information
```

### Binaries

- [podcasters-collector for x86_64 with musl (.tar.gz)](https://github.com/ogrodje/podcasters-collector/releases/download/refs%2Fheads%2Fmaster/podcasters-collector-x86_64-unknown-linux-musl.tar.gz)

### Docker

With Docker Image available on [GitHub Container Registry - `ghcr.io`][ghcr-podcasters-collector].

```bash
docker run --rm ghcr.io/ogrodje/podcasters-collector:latest \
  podcasters-collector --email <EMAIL> --password <PASSWORD>
```

### Importing into InfluxDB

```bash
$ ./target/release/podcasters-collector ... --format influx-dbcsv > plays.csv
$ influx write -b experimenting -f plays.csv
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

