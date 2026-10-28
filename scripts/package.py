#!/usr/bin/env python3
"""Package an embedded-assets desktop build with its license and documentation."""
import argparse
import shutil
import tarfile
import tempfile
import tomllib
import zipfile
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("target")
args = parser.parse_args()
root = Path(__file__).resolve().parent.parent
version = tomllib.loads((root / "Cargo.toml").read_text())["package"]["version"]
name = f"singularity-v{version}-{args.target}"
extension = ".exe" if "windows" in args.target else ""
binary = root / "target" / args.target / "release" / f"singularity-rust{extension}"
if not binary.is_file():
    parser.error(f"build does not exist: {binary}")
dist = root / "dist"
dist.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory() as temp:
    folder = Path(temp) / name
    folder.mkdir()
    for source in [binary, root / "LICENSE", root / "README.md", root / "CHANGELOG.md"]:
        shutil.copy2(source, folder / source.name)
    if extension:
        archive = dist / f"{name}.zip"
        with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as output:
            for path in sorted(folder.iterdir()):
                output.write(path, f"{name}/{path.name}")
    else:
        archive = dist / f"{name}.tar.gz"
        with tarfile.open(archive, "w:gz") as output:
            output.add(folder, arcname=name)
    print(archive)
