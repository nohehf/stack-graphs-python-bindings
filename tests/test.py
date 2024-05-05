import os
from stack_graphs_python import index, query_definition, Position

# index ./js_sample directory

# convert ./js_sample directory to absolute path
dir = os.path.abspath("./tests/js_sample")
db = os.path.abspath("./js_sample.db")

print("Indexing directory: ", dir)
print("Database path: ", db)

index([dir], db)

source_reference: Position = Position(path=dir + "/index.js", line=2, column=12)

print("Querying definition for: ", source_reference.path)

results = query_definition(source_reference, db)

print("Results: ", results)

for result in results:
    print("Path: ", result.path)
    print("Line: ", result.line)
    print("Column: ", result.column)
    print("\n")
