# Stack-Graphs Python bindings

Opinionated Python bindings for the [tree-sitter-stack-graphs](https://github.com/github/stack-graphs) rust library.

It exposes very few, easy to use functions to index files and query references.

This is a proof of concept draft, to test scripting utilities using stack-graphs easily.

It uses pyo3 and maturin to generate the bindings.

## Installation & Usage

```bash
pip install stack-graphs-python-bindings # or poetry, ...
```

```python
import os
from stack_graphs_python import index, Querier, Position, Language

db_path = os.path.abspath("./db.sqlite")
dir = os.path.abspath("./tests/js_sample")

# Index the directory (creates stack-graphs database)
index([dir], db_path, language=Language.JavaScript)

# Instantiate a querier
querier = Querier(db_path)

# Query a reference at a given position (0-indexed line and column): 
# foo in: const baz = foo
source_reference = Position(path=dir + "/index.js", line=2, column=12)
results = querier.definitions(source_reference)

for r in results:
    print(f"{r.path}, l:{r.line}, c: {r.column}")
```

Will result in:

```bash
[...]/stack-graphs-python-bindings/tests/js_sample/index.js, l:0, c: 9
[...]/stack-graphs-python-bindings/tests/js_sample/module.js, l:0, c: 13
```

That translates to:

```javascript
// index.js
import { foo } from "./module"

// module.js
export const foo = "bar"
```

## Development

### Ressources

https://pyo3.rs/v0.21.2/getting-started

### Requirements

- Rust
- Python 3.11+

### Setup

```bash
# Setup venv and install maturin through pip
make setup
```

### Testing

```bash
make test
```
