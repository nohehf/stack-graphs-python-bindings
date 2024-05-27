from helpers.virtual_files import string_to_virtual_files
from stack_graphs_python import Indexer, Querier, Language
import os

code1 = """
;---index.ts---
import { foo } from "./module";
         ^{ref1}
const baz: number = Number(foo);
                           ^{query}
console.log(baz);

;---module.ts---
export const foo: string = "42";
             ^{ref2}

"""

code2 = """
;---index.ts---
import { foo } from "./module.js";
         ^{ref1}
const baz: number = Number(foo);
                           ^{query}
console.log(baz);

;---module.ts---
export const foo: string = "42";
             ^{ref2}

"""

code3 = """
;---index.ts---
import { foo } from "./module.ts";
         ^{ref1}
const baz: number = Number(foo);
                           ^{query}
console.log(baz);

;---module.ts---
export const foo: string = "42";
             ^{ref2}

"""


def test_ts_ok() -> None:
    for code in [code1, code2, code3]:
        with string_to_virtual_files(code1) as (dir, positions):
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
