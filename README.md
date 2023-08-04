# HarcBench

## Synopsis

A CPU Benchmark programmed in Rust using a modified offline ray tracing renderer from https://github.com/RayTracing/raytracing.github.io.

## Project status

We plan to use tokio as runtime and diplay intermediate image in real time similiar to Cinebench.

This package is intend to use as many CPU resources as possible.

This package is currently in development and has a initially runnable version.

First release is ready.

## Usage

```bash
cargo run [resolution_width] [aspect_ratio] [samples_per_pixel] [max_depth] [vfov]
```