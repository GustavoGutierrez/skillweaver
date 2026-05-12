#!/usr/bin/env python3
"""Render the Homebrew formula template with version and checksum placeholders."""

import argparse
import sys


def main() -> None:
    parser = argparse.ArgumentParser(description="Render Homebrew formula template")
    parser.add_argument("--version", required=True, help="Release version (e.g. 0.1.0)")
    parser.add_argument("--checksum", required=True, help="SHA256 checksum of the tarball")
    parser.add_argument(
        "--template",
        default="packaging/homebrew/Formula/skillweaver.rb",
        help="Path to formula template",
    )
    parser.add_argument(
        "--output",
        default="Formula/skillweaver.rb",
        help="Output path for rendered formula",
    )
    args = parser.parse_args()

    try:
        with open(args.template, "r", encoding="utf-8") as f:
            template = f.read()
    except FileNotFoundError:
        print(f"Error: template not found: {args.template}", file=sys.stderr)
        sys.exit(1)

    rendered = (
        template.replace("{{VERSION}}", args.version)
        .replace("{{CHECKSUM}}", args.checksum)
    )

    with open(args.output, "w", encoding="utf-8") as f:
        f.write(rendered)

    print(f"Rendered formula -> {args.output} (version={args.version})")


if __name__ == "__main__":
    main()