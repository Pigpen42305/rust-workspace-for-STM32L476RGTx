import sys
from os import name, environ as env, pathsep

if sys.version_info < (3, 9):
    raise ValueError("Python version not supported!")

if name == 'nt':
    raise OSError("Incorrect operating system!")

from pathlib import Path

PATH: str | None = env.get('Path', env.get('PATH'))
_path: list[str] = [] if PATH is None else PATH.split(pathsep)
path: list[Path] = [Path(x).expanduser() for x in _path]
cargo_path = (Path.home() / '.cargo' / 'bin').expanduser()

if (not cargo_path.exists()) or (cargo_path not in path):
    print(
        "'~/.cargo/bin' was not found on PATH or is not installed.",
        "Please install and run rustup from the rust foundation website.",
        sep='\n',
        file=sys.stderr,
    )
    exit(1)

exit(0)
