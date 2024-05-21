from helpers.virtual_files import string_to_virtual_files
from stack_graphs_python import Indexer, Querier, Language
import os
import pytest

code = """
;---index.py---
from module import definition
                   ^{ref1}

print(definition)
      ^{query}

;---module.py---
definition = "definition"
^{ref2}

"""


@pytest.mark.skip(
    "Stack-graphs python module resolution is currently broken. See issue: https://github.com/github/stack-graphs/issues/430"
)
def test_py_ok():
    with string_to_virtual_files(code) as (dir, positions):
        db_path = os.path.join(dir, "db.sqlite")
        indexer = Indexer(db_path, [Language.Python])
        indexer.index_all([dir])
        querier = Querier(db_path)
        source_reference = positions["query"]
        results = querier.definitions(source_reference)
        assert len(results) == 2
        expected = [positions["ref1"], positions["ref2"]]
        for i, result in enumerate(results):
            assert result == expected[i]
