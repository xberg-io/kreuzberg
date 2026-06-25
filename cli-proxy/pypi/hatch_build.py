"""Custom hatchling build hook that bundles the native xberg binary into a wheel.

When building a platform-specific wheel in CI, the target triple is supplied via
the ``XBERG_CLI_TARGET`` env var (the build host is always linux/amd64, so the
triple cannot be inferred from ``platform.*``). The matching
``xberg-cli-<target>.tar.gz`` / ``.zip`` is located (repo root or ``dist/``),
the binary is extracted into ``xberg_cli/bin/<target>/``, force-included in the
wheel, and the wheel is tagged for that platform so PyPI serves the right artifact.

If no target/binary is found (e.g. the sdist build, or an unknown platform), the
hook is a no-op and the package falls back to the runtime downloader in
``xberg_cli/downloader.py`` (see ``cli.py``).
"""

from __future__ import annotations

import os
import shutil
import tarfile
import zipfile
from pathlib import Path

from hatchling.builders.hooks.plugin.interface import BuildHookInterface

# Rust target triple -> wheel platform tag. PyPI uses the platform tag to serve the
# correct wheel per OS/arch/libc.
_TAG_MAP = {
    "x86_64-pc-windows-msvc": "win_amd64",
    "x86_64-unknown-linux-gnu": "manylinux_2_28_x86_64",
    "aarch64-unknown-linux-gnu": "manylinux_2_28_aarch64",
    "x86_64-unknown-linux-musl": "musllinux_1_2_x86_64",
    "aarch64-unknown-linux-musl": "musllinux_1_2_aarch64",
    "aarch64-apple-darwin": "macosx_11_0_arm64",
    "x86_64-apple-darwin": "macosx_11_0_x86_64",
}


class CustomBuildHook(BuildHookInterface):
    """Inject the matching native binary into a platform-tagged wheel."""

    PLUGIN_NAME = "custom"

    def initialize(self, version: str, build_data: dict) -> None:  # noqa: ARG002
        target = os.environ.get("XBERG_CLI_TARGET", "").strip()
        if not target:
            # sdist build or unbundled wheel: leave a pure, download-at-runtime package.
            return

        archive = self._find_archive(target)
        if archive is None:
            # Target requested but no binary staged — fail loudly rather than ship an
            # empty platform wheel that shadows the working sdist on PyPI.
            raise RuntimeError(
                f"XBERG_CLI_TARGET={target} but no xberg-cli-{target}.(tar.gz|zip) "
                f"found in repo root or dist/; refusing to build an empty platform wheel."
            )

        wheel_tag = _TAG_MAP.get(target)
        if wheel_tag is None:
            raise RuntimeError(f"no wheel platform tag mapped for target {target}")

        binary = self._extract_binary(archive, target)

        # force_include maps absolute source paths -> in-wheel relative paths.
        relative = f"xberg_cli/bin/{target}/{binary.name}"
        build_data.setdefault("force_include", {})[str(binary)] = relative

        # Make it a platform wheel (not pure-python, not py3-none-any).
        build_data["pure_python"] = False
        build_data["infer_tag"] = False
        build_data["tag"] = f"py3-none-{wheel_tag}"

    def _find_archive(self, target: str) -> Path | None:
        root = Path(self.root)  # the project dir hatchling is building (cli-proxy/pypi)
        repo_root = root.parent.parent
        for base in (repo_root, repo_root / "dist", root, root / "dist"):
            for ext in ("tar.gz", "zip"):
                candidate = base / f"xberg-cli-{target}.{ext}"
                if candidate.is_file():
                    return candidate
        return None

    def _extract_binary(self, archive: Path, target: str) -> Path:
        is_windows = target.endswith("windows-msvc")
        binary_name = "xberg.exe" if is_windows else "xberg"
        extract_dir = Path(self.root) / ".build-extract" / target
        if extract_dir.exists():
            shutil.rmtree(extract_dir, ignore_errors=True)
        extract_dir.mkdir(parents=True, exist_ok=True)

        if str(archive).lower().endswith(".zip"):
            with zipfile.ZipFile(archive) as zf:
                zf.extractall(extract_dir)
        else:
            with tarfile.open(archive, "r:gz") as tf:
                tf.extractall(extract_dir)

        for candidate in extract_dir.rglob(binary_name):
            if candidate.is_file():
                candidate.chmod(0o755)
                return candidate
        raise RuntimeError(f"binary {binary_name} not found inside {archive.name}")
