# Contributing

Thanks for helping improve Singularity. Small, focused changes are easiest to review.

## Setup

Use the current stable Rust toolchain and a working OpenGL desktop environment. See the README for runtime dependencies.

```bash
cargo run --release --locked
```

## Before opening a pull request

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --release --locked
```

For a shader or camera change, capture the cinematic, edge-on, top-down, and jets presets at the same time and quality. Inspect the central shadow, secondary disk image, disk seam, and both orbital poles. Include the relevant images and your GPU/driver information in the pull request. Avoid claiming a performance improvement without reporting resolution, quality, hardware, and measurements.

Tests cover input validation and camera geometry. CI also compiles the shaders on Mesa and checks that the captured frames contain a visible scene. The smoke test is deliberately tolerant of driver differences; it does not establish physical correctness or interactive performance on every supported GPU.

## Renderer conventions

- Keep shader assets embedded with `include_str!` so release archives remain self-contained.
- Preserve GLSL ES 1.00 compatibility and the desktop OpenGL backend.
- Treat radiance encoding in `blackhole.frag` and decoding in `post.frag` as one contract.
- Document new physical approximations in the README.
- Avoid CPU allocations in the draw loop unless a resource size actually changes.

## Documentation captures

```bash
bash scripts/capture.sh
```

Use actual application output. Do not replace screenshots with concept art. Personal F12 captures belong in the ignored `screenshots/` directory.

## Releases

1. Update the version in `Cargo.toml` and add a matching `## [VERSION]` section to `CHANGELOG.md`.
2. Run `cargo check` to refresh the root package version in `Cargo.lock`.
3. Complete the checks above and push the reviewed change to `main`.
4. The CI workflow builds all four archives and publishes `vVERSION` only after the validation jobs pass.

The workflow uses the repository's `GITHUB_TOKEN`; no personal token is required. Publication is restricted to `main`, and only the release job receives `contents: write`. Existing releases are skipped. If publication is interrupted after a draft is created, inspect that draft and its assets before publishing or deleting it; do not replace a published release silently.
