#!/usr/bin/env python3
"""Reject blank, truncated, or visually empty GPU captures in CI (Pillow required)."""
import argparse
from pathlib import Path
from PIL import Image, ImageStat

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("images", nargs="+", type=Path)
args = parser.parse_args()
for path in args.images:
    with Image.open(path) as image:
        image.load()
        assert image.format == "PNG", f"{path}: expected PNG"
        assert image.width >= 320 and image.height >= 240, f"{path}: invalid size"
        rgb = image.convert("RGB")
        stats = ImageStat.Stat(rgb)
        assert max(stats.stddev) > 12, f"{path}: blank or near-uniform frame"
        raw = rgb.tobytes()
        pixels = list(zip(raw[0::3], raw[1::3], raw[2::3]))
        bright = sum(max(p) > 90 for p in pixels)
        dark = sum(max(p) < 20 for p in pixels)
        assert bright > len(pixels) * 0.01, f"{path}: no visible accretion disk"
        assert dark > len(pixels) * 0.1, f"{path}: missing dark sky/shadow"
        print(f"OK {path}: {image.width}x{image.height}, {bright} bright pixels")
