from dataclasses import dataclass, field
from pathlib import Path

from mashumaro.mixins.toml import DataClassTOMLMixin

RC_FILE_PATH = Path.home() / ".ghclrc"


@dataclass
class Config(DataClassTOMLMixin):
    mirror_url: str = field(default="https://github.com/")
    destiny: Path = field(default=Path())
    no_owner: bool = field(default=False)
    git_config: list[str] = field(default_factory=list)

    def __post_init__(self):
        self.destiny = self.destiny.expanduser()


def load_config():
    if not RC_FILE_PATH.exists():
        print(f"NO config file found.\nCreating a new one at {RC_FILE_PATH}.")
        config = Config()
        RC_FILE_PATH.write_text(config.to_toml())
        return config

    return Config.from_toml(RC_FILE_PATH.read_text())
