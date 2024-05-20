"""
This is a utility to create a virtual repository from a string
It allows to quickly write tests, as creating those files and getting the correct positions is cumbersome

It parses a string into a list of files and positions in the files
Files separators are defined as ';---{filepath}---'. It should be preceded by a newline and followed by a newline
Positions queries are defined as '^{identifier}' where identifier is a string, prefixed by any number of spaces
Positions queries are stripped from the file content in the temporary filesystem

For example this string:

string = \"""
;---main.py---
from module import module
     ^{pos1}
import lib
from lib.file import file

print(module)
print(lib.lib)
          ^{pos2}
print(file)

;---module.py---
module = "module.py"
          ^{pos3}

;---lib/__init__.py---
lib = "lib/__init__.py"

;---lib/file.py---
file = "lib/file.py"
\"""

When parsed via:

virtual_repo, positions = string_to_virtual_repo(string)
with virtual_repo() as repo_path:
    ...

Will create the following temporary filesystem:
/
├── main.py
├── module.py
└── lib
    ├── __init__.py
    └── file.py

And output the following positions:
{
    "pos1": Position("main.py", 0, 6),
    "pos2": Position("main.py", 5, 11),
    "pos3": Position("module.py", 0, 10),
}
"""

import os
import re
import tempfile
import shutil
from typing import Callable
from stack_graphs_python import Position
import contextlib


POSITION_MARKER_REGEX = r"( *)\^{([a-zA-Z0-9_]+)}\n"
FILE_SEPARATOR_REGEX = r"\n;---([a-zA-Z0-9\_./]+)---\n"


def _remove_position_markers(string: str) -> str:
    return re.subn(POSITION_MARKER_REGEX, "", string)[0]


def _split_files(string: str) -> list[tuple[str, str]]:
    """
    Split a string into a list of tuples where each tuple contains the file path and the file content
    """
    files = []
    matches = [m for m in re.finditer(FILE_SEPARATOR_REGEX, string)]
    # Get the file path from match group 0
    # Get the file content from the end of the previous match to the start of the current match
    for i in range(1, len(matches) + 1):
        end = len(string) if i == len(matches) else matches[i].start()
        file_path = matches[i - 1].group(1)
        file_content = string[matches[i - 1].end() : end]
        files.append((file_path, file_content))

    return files


def _get_positions_in_file(file_path: str, contents: str) -> dict[str, Position]:
    positions = {}
    while match := re.search(POSITION_MARKER_REGEX, contents):
        identifier = match.group(2)
        line = contents.count("\n", 0, match.start()) - 1
        # count the number of spaces
        column = len(match.group(1))
        positions[identifier] = Position(file_path, line, column)
        contents = contents[: match.start()] + contents[match.end() :]

    return positions


@contextlib.contextmanager
def _contextmanager(files: list[tuple[str, str]]):
    temp_dir = tempfile.mkdtemp()
    temp_dir = "./temp_dir"
    try:
        for file_path, file_content in files:
            file_content = _remove_position_markers(file_content)
            full_path = os.path.join(temp_dir, file_path)
            os.makedirs(os.path.dirname(full_path), exist_ok=True)
            with open(full_path, "w") as f:
                f.write(file_content)
        yield temp_dir
    finally:
        print("Removing temp dir")
        shutil.rmtree(temp_dir)


def string_to_virtual_repo(
    string: str,
) -> tuple[
    Callable[[], contextlib._GeneratorContextManager[str]],
    dict[str, Position],
]:
    files = _split_files(string)
    positions_map = {}
    for file_path, file_content in files:
        positions_map.update(_get_positions_in_file(file_path, file_content))
    return (
        lambda: _contextmanager(files),
        positions_map,
    )
