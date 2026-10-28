# Changelog

## [0.2.0] - 2026-10-28

### Rendering

- Replace the CPU-updated disk mesh and screen-space star displacement with an embedded GPU curved-ray renderer.
- Add a Schwarzschild-inspired light-path integrator, horizon absorption, multiple disk intersections, and a directional procedural starfield.
- Add animated disk turbulence, approximate Doppler brightening and gravitational frequency shifts, optional polar jets, and HDR post-processing.
- Add low, medium, and high quality presets, exposure, bloom, and disk opacity controls.

### Interaction

- Add smooth orbiting, exponential zoom, automatic orbit, and four launch presets.
- Add a dark observatory panel, a fully hideable HUD, and keyboard visual toggles.
- Add F12 screenshots and validated CLI options for deterministic PNG capture.

### Delivery

- Add an English README with actual application captures, controls, architecture, and physical limitations.
- Refine ignore rules while retaining the application lockfile and curated screenshots.
- Add formatting, strict Clippy, unit tests, WebAssembly linting, and actual OpenGL render validation in CI.
- Add Linux, Windows, and both macOS architecture archives, with release publication gated by successful checks and SHA-256 checksums.

## [0.1.0] - 2026-10-28

- Initial Rust/Macroquad scene with an orbit camera, animated disk mesh, particles, background stars, optional jets, and simulation controls.
