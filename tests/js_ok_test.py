from helpers.virtual_files import string_to_virtual_files
from stack_graphs_python import index, Indexer, Querier, Language, FileStatus
import os

code = """
;---index.js---
import { foo } from "./module"
         ^{ref1}

const baz = foo
            ^{query}

console.log(baz)

;---module.js---
export const foo = "bar"
             ^{ref2}

"""


def test_js_ok():
    with string_to_virtual_files(code) as (dir, positions):
        db_path = os.path.join(dir, "db.sqlite")
        indexer = Indexer(db_path, [Language.JavaScript])
        indexer.index_all([dir])
        status = indexer.status_all()
        assert len(status) == 2
        assert status[0].path == os.path.join(dir, "index.js")
        assert status[0].status == FileStatus.Indexed
        querier = Querier(db_path)
        source_reference = positions["query"]
        results = querier.definitions(source_reference)
        assert len(results) == 2
        expected = [positions["ref1"], positions["ref2"]]
        for i, result in enumerate(results):
            assert result == expected[i]


def test_js_ok_legacy_index():
    with string_to_virtual_files(code) as (dir, positions):
        db_path = os.path.join(dir, "db.sqlite")
        index([dir], db_path, language=Language.JavaScript)
        querier = Querier(db_path)
        source_reference = positions["query"]
        results = querier.definitions(source_reference)
        assert len(results) == 2
        expected = [positions["ref1"], positions["ref2"]]
        for i, result in enumerate(results):
            assert result == expected[i]
