from helpers.virtual_files import string_to_virtual_files
from stack_graphs_python import Indexer, Language, FileStatus
import os

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


def test_ts_ok():
    with string_to_virtual_files(ok_code) as (dir, _):
        db_path = os.path.join(dir, "db.sqlite")
        dir = os.path.abspath(dir)
        indexer = Indexer(db_path, [Language.TypeScript])
        indexer.index_all([dir])
        status = indexer.status_all()
        assert len(status) == 1
        assert status[0].path == os.path.join(dir, "index.ts")
        assert status[0].status == FileStatus.Indexed


def test_ts_ko():
    with string_to_virtual_files(ko_code) as (dir, _):
        db_path = os.path.join(dir, "db.sqlite")
        dir = os.path.abspath(dir)
        indexer = Indexer(db_path, [Language.TypeScript])
        indexer.index_all([dir])
        status = indexer.status_all()
        assert len(status) == 1
        assert status[0].path == os.path.join(dir, "index.ts")
        assert status[0].status == FileStatus.Error
        assert status[0].error is not None
        # TODO(@nohehf): Add logs when we fail to index a file
        assert status[0].error != "Error parsing source"
