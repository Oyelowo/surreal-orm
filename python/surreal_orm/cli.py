"""Minimal CLI for migrations (stub).

This provides a small command-line entrypoint similar to the Rust `migrator`
binary. It uses argparse to avoid adding runtime dependencies in the scaffold.
"""
import argparse
from .migrations import discover_migrations


def main(argv=None):
    parser = argparse.ArgumentParser(prog="surreal-migrator")
    parser.add_argument("path", nargs="?", default="migrations", help="migrations directory")
    args = parser.parse_args(argv)

    migrations = discover_migrations(args.path)
    print(f"Found {len(migrations)} migrations in {args.path}")


if __name__ == "__main__":
    main()
