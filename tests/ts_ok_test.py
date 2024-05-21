from helpers.virtual_files import string_to_virtual_files
from stack_graphs_python import Indexer, Querier, Language
import os
import pytest

code = """
;---index.ts---
import { foo } from "./module"
         ^{ref1}

const baz: string = foo
            ^{query}

console.log(baz)

;---module.ts---
export const foo = "bar"
             ^{ref2}

"""


@pytest.mark.skip("WIP")
def test_ts_ok() -> None:
    with string_to_virtual_files(code) as (dir, positions):
        db_path = os.path.join(dir, "db.sqlite")
        dir = os.path.abspath(dir)
        indexer = Indexer(db_path, [Language.TypeScript])
        indexer.index_all([dir])
        querier = Querier(db_path)
        source_reference = positions["query"]
        results = querier.definitions(source_reference)
        assert len(results) == 2
        expected = [positions["ref1"], positions["ref2"]]
        for i, result in enumerate(results):
            assert result == expected[i]
