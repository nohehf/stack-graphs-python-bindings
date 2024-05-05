import os
from stack_graphs_python import index

# index ./js_sample directory

# convert ./js_sample directory to absolute path
dir = os.path.abspath("./tests/js_sample")
db = os.path.abspath("./js_sample.db")

print("Indexing directory: ", dir)
print("Database path: ", db)

index([dir], db)
