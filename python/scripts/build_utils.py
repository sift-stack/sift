#!/usr/bin/env python3

import argparse
import os
import subprocess
import venv
from itertools import combinations
from pathlib import Path
from typing import List, Optional
from zipfile import ZipFile

from wheel.pkginfo import read_pkg_info_bytes


def get_extras_from_wheel(wheel_path: str) -> List[str]:
    """Extract the list of extras from a wheel's metadata.

    Args:
        wheel_path: Path to the wheel file to inspect.

    Returns:
        List of extra names declared in the wheel metadata.
    """
    with ZipFile(wheel_path) as wheel:
        for name in wheel.namelist():
            if name.endswith(".dist-info/METADATA"):
                metadata = read_pkg_info_bytes(wheel.read(name))
                return [
                    line.split(":")[1].strip() for line in metadata.get_all("Provides-Extra") or []
                ]
    return []


def get_extra_combinations(extras: List[str]) -> List[str]:
    """Generate all possible combinations of extras.

    Args:
        extras: List of extra names to generate combinations from.

    Returns:
        List of comma-separated strings representing each combination of extras.
    """
    all_combinations = []
    for r in range(len(extras) + 1):
        all_combinations.extend(",".join(c) for c in combinations(extras, r))
    return all_combinations


def test_install(
    package_name: str, extras: Optional[str], dist_dir: str, venv_dir: str, is_windows: bool
) -> None:
    """Test package installation with given extras in a fresh venv.

    Args:
        package_name: Name of the package to install.
        extras: Optional comma-separated string of extras to install.
        dist_dir: Directory containing wheel and dependencies.
        venv_dir: Directory to create virtual environment in.
        is_windows: Whether running on Windows platform.
    """
    print(f"Testing installation with extras: {extras or 'none'}")

    # Create and activate venv
    venv.create(venv_dir, with_pip=True)

    # Build activation command based on OS
    if is_windows:
        activate_script = os.path.join(venv_dir, "Scripts", "activate")
    else:
        activate_script = os.path.join(venv_dir, "bin", "activate")

    # Build installation command
    if extras:
        install_cmd = f'pip install --no-index --find-links="{dist_dir}" "{package_name}[{extras}]"'
    else:
        install_cmd = f'pip install --no-index --find-links="{dist_dir}" {package_name}'

    # Run installation in the venv
    full_cmd = f'source "{activate_script}" && {install_cmd} && deactivate'
    subprocess.run(full_cmd, shell=True, check=True, executable="/bin/bash")


def main():
    parser = argparse.ArgumentParser(
        description="Test package installation with all extra combinations"
    )
    parser.add_argument(
        "--dist-dir", required=True, help="Directory containing wheel and dependencies"
    )
    parser.add_argument("--package-name", required=True, help="Name of the package to install")
    parser.add_argument("--is-windows", action="store_true", help="Whether running on Windows")
    args = parser.parse_args()

    dist_dir = Path(args.dist_dir)
    wheel_file = next(dist_dir.glob(f"{args.package_name.replace('-', '_')}*.whl"))

    # Get all extras from the wheel
    extras = get_extras_from_wheel(str(wheel_file))
    combinations = get_extra_combinations(extras)

    # Test base installation first
    test_install(
        package_name=args.package_name,
        extras=None,
        dist_dir=str(dist_dir),
        venv_dir="test_venv_base",
        is_windows=args.is_windows,
    )

    # Test each combination of extras
    for combo in combinations:
        if combo:  # Skip empty string from base combination
            test_install(
                package_name=args.package_name,
                extras=combo,
                dist_dir=str(dist_dir),
                venv_dir=f'test_venv_{combo.replace(",", "_")}',
                is_windows=args.is_windows,
            )


if __name__ == "__main__":
    main()
