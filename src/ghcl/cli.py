"""github clone tool"""

from pathlib import Path
from typing import Optional

from .config import load_config
from .core import clone


def main():
    import argparse

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("url", help="GitHub repository URL")
    parser.add_argument("-d", "--destiny", type=Path, help="Destination directory")
    parser.add_argument(
        "--no-owner", action="store_true", help="Clone repository without owner name"
    )

    args = parser.parse_args()
    # print(args)
    # return
    url: str = args.url
    destiny: Optional[Path] = args.destiny
    no_owner: bool = args.no_owner

    config = load_config()
    if destiny is not None:
        config.destiny = destiny
    config.no_owner = no_owner or config.no_owner

    try:
        clone(url, config)
    except Exception as e:
        print(f"Error: {e}")


if __name__ == "__main__":
    main()
