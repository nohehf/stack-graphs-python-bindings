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
