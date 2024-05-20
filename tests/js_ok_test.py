from helpers.virtual_files import string_to_virtual_repo
from stack_graphs_python import index, Querier, Language
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


def test_stack_graphs_python():
    with string_to_virtual_repo(code) as (dir, positions):
        db_path = os.path.abspath("./db.sqlite")
        dir = os.path.abspath(dir)
        index([dir], db_path, language=Language.JavaScript)
        querier = Querier(db_path)
        source_reference = positions["query"]
        results = querier.definitions(source_reference)
        assert len(results) == 2
        expected = [positions["ref1"], positions["ref2"]]
        for i, result in enumerate(results):
            assert result == expected[i]
