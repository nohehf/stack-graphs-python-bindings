import os
from stack_graphs_python import index

# index ./js_sample directory

# convert ./js_sample directory to absolute path
dir = os.path.abspath("./js_sample")

index([dir], "./js_sample.db")
