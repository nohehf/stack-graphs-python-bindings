from helpers.virtual_files import string_to_virtual_files
from stack_graphs_python import index, Language
import os
import pytest

ok_code = """
;---index.ts---
class A {
    @decorator()
    method() {
        // ...
    }
}
"""

# Tree sitter will fail on this: https://github.com/tree-sitter/tree-sitter-typescript/issues/283
ko_code = """
;---index.ts---
class A {
    @decorator<T>()
    method() {
        // ...
    }
}
"""


@pytest.mark.skip("WIP")
def test_ts_ok():
    with string_to_virtual_files(ok_code) as (dir, _):
        db_path = os.path.abspath("./db.sqlite")
        dir = os.path.abspath(dir)
        index([dir], db_path, language=Language.TypeScript)


@pytest.mark.skip("WIP")
def test_ts_ko():
    with string_to_virtual_files(ko_code) as (dir, _):
        print("here")
        db_path = os.path.abspath("./db.sqlite")
        dir = os.path.abspath(dir)
        index([dir], db_path, language=Language.TypeScript)
