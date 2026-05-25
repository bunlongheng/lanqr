<div align="center">

# lanqr

**A QR code of your LAN address, right in the terminal.**

Scan it from your phone to open a local dev server on the same network. No more typing IPs.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white)
![Release](https://img.shields.io/github/v/release/bunlongheng/lanqr?sort=semver)

<img src="assets/hero.svg" alt="lanqr QR code" width="220">

</div>

## Why

Your dev server runs on `localhost`, but your phone needs the LAN IP. `lanqr` finds your
LAN address and prints a scannable QR, so you point your camera and go.

## Features

- One command, instant QR in the terminal
- Auto-detects your LAN IP (std only, no config)
- Pass a port, a full URL, or any text
- Single static binary, one dependency
- Light QR on dark terminals, scans as-is

## Install

```bash
cargo install --git https://github.com/bunlongheng/lanqr
```

Or build from source:

```bash
git clone https://github.com/bunlongheng/lanqr
cd lanqr && cargo build --release   # binary at ./target/release/lanqr
```

## Usage

```bash
lanqr                        # QR of http://<your-lan-ip>
lanqr 3000                   # QR of http://<your-lan-ip>:3000
lanqr https://example.com    # encode any URL or text
```

Example:

```text
$ lanqr 3000

http://192.168.1.42:3000

█████████████████████████████████
████ ▄▄▄▄▄ █▀ ▄▀█ ▄█ ▄▄▄▄▄ ████
████ █   █ █▀█▄▀ █  █ █   █ ████
████ █▄▄▄█ █ ▄▀█▄██ █ █▄▄▄█ ████
████▄▄▄▄▄▄▄█▄▀ █▄█ █▄█▄▄▄▄▄▄▄████
   ... (scannable in your terminal) ...
```

## How it works

`lanqr` opens a UDP socket toward a public address so the OS reveals which network
interface it would use, then reads that interface's local IP (no packet is sent). It
encodes the resulting URL with the `qrcode` crate and renders it as Unicode half-blocks,
inverted so the code stays scannable on a dark terminal.

## License

[MIT](LICENSE)
