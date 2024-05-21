from helpers.virtual_files import string_to_virtual_repo
from stack_graphs_python import index, Querier, Language
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
    with string_to_virtual_repo(code) as (dir, positions):
        db_path = os.path.abspath("./db.sqlite")
        dir = os.path.abspath(dir)
        index([dir], db_path, language=Language.TypeScript)
        querier = Querier(db_path)
        source_reference = positions["query"]
        results = querier.definitions(source_reference)
        assert len(results) == 2
        expected = [positions["ref1"], positions["ref2"]]
        for i, result in enumerate(results):
            assert result == expected[i]
