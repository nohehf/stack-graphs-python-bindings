import os
from virtual_files import (
    string_to_virtual_repo,
    _split_files,
    _remove_position_markers,
    _get_positions_in_file,
)
from stack_graphs_python import Position

test_string = """
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
"""


def test__split_files():
    expected_files = [
        (
            "main.py",
            """from module import module
     ^{pos1}
import lib
from lib.file import file

print(module)
print(lib.lib)
          ^{pos2}
print(file)
""",
        ),
        (
            "module.py",
            """module = "module.py"
          ^{pos3}
""",
        ),
        (
            "lib/__init__.py",
            """lib = "lib/__init__.py"
""",
        ),
        (
            "lib/file.py",
            """file = "lib/file.py"
""",
        ),
    ]

    files = _split_files(test_string)

    if len(files) < len(expected_files):
        assert False, "Not enough files"

    for i, (expected_file_path, expected_content) in enumerate(expected_files):
        assert files[i][0] == expected_file_path, "File path does not match"
        assert files[i][1] == expected_content, "Content does not match"


def test__remove_position_markers():
    expected_string = """
;---main.py---
from module import module
import lib
from lib.file import file

print(module)
print(lib.lib)
print(file)

;---module.py---
module = "module.py"

;---lib/__init__.py---
lib = "lib/__init__.py"

;---lib/file.py---
file = "lib/file.py"
"""

    assert (
        _remove_position_markers(test_string) == expected_string
    ), "Position markers should be removed"


def test__get_positions_in_file():
    file = """from module import module
     ^{pos1}
import lib

print(module)
      ^{pos2}
"""

    expected_positions = {
        "pos1": Position("main.py", 0, 5),
        "pos2": Position("main.py", 3, 6),
    }

    positions = _get_positions_in_file("main.py", file)

    for identifier, expected_position in expected_positions.items():
        assert identifier in positions, f"{identifier} not found in positions"
        # TODO(@nohehf): Implement __eq__ for Position
        assert (
            positions[identifier].path == expected_position.path
            and positions[identifier].line == expected_position.line
            and positions[identifier].column == expected_position.column
        ), f"Position of {identifier} does not match"


def test_string_to_virtual_repo():
    expected_files = {
        "main.py": (
            """from module import module
import lib
from lib.file import file

print(module)
print(lib.lib)
print(file)
""",
            {
                "pos1": Position("main.py", 0, 5),
                "pos2": Position("main.py", 5, 10),
            },
        ),
        "module.py": (
            """module = "module.py"
""",
            {"pos3": Position("module.py", 0, 10)},
        ),
        "lib/__init__.py": (
            """lib = "lib/__init__.py"
""",
            {},
        ),
        "lib/file.py": (
            """file = "lib/file.py"
""",
            {},
        ),
    }

    virtual_repo, positions = string_to_virtual_repo(test_string)

    with virtual_repo() as repo_path:
        for file_path, (expected_content, expected_positions) in expected_files.items():
            full_path = os.path.join(repo_path, file_path)
            assert os.path.exists(
                full_path
            ), f"{file_path} does not exist in the temporary repository"
            with open(full_path, "r") as f:
                content = f.read()
            assert content == expected_content, f"Content of {file_path} does not match"

            for identifier, expected_position in expected_positions.items():
                assert identifier in positions, f"{identifier} not found in positions"
                assert (
                    positions[identifier].path == expected_position.path
                    and positions[identifier].line == expected_position.line
                    and positions[identifier].column == expected_position.column
                ), f"Position of {identifier} does not match"


# TODO: The string should be validated
# def test_invalid_string():
#     invalid_string = """
# ;---main.py---
# from module import module
#      ^{pos1}
# ;---invalid
# """
#     with pytest.raises(ValueError):
#         string_to_virtual_repo(invalid_string)


def test_cleanup():
    test_string = """
;---main.py---
print("Hello, World!")
"""
    virtual_repo, _ = string_to_virtual_repo(test_string)
    with virtual_repo() as repo_path:
        assert os.path.exists(repo_path), "Temporary directory should exist"
    assert not os.path.exists(repo_path), "Temporary directory should be cleaned up"
