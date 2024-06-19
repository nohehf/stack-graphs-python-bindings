import os
import time

from stack_graphs_python import index, Querier, Position, Language

# index ./js_sample directory

# convert ./js_sample directory to absolute path
dir = os.path.abspath("./tests/js_sample")
db_path = os.path.abspath("./db.sqlite")

print("Indexing directory: ", dir)
print("Database path: ", db_path)

index([dir], db_path, language=Language.TypeScript)

source_reference = Position(path=dir + "/index.js", line=2, column=12)

print("Querying definition for: ", source_reference.path)

querier = Querier(db_path)


def run_n_queries(n):
    start = time.time()
    for i in range(n):
        querier.definitions(source_reference)
    end = time.time()
    # delta in ms
    delta = (end - start) * 1000
    print(
        f"{n} queries took: {delta}ms",
    )


run_n_queries(100)
run_n_queries(1000)
run_n_queries(10000)
