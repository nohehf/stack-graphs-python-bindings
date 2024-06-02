# Stack-Graphs Python bindings

Opinionated Python bindings for the [tree-sitter-stack-graphs](https://github.com/github/stack-graphs) rust library.

(Note: the project curretly relies on [fork](https://github.com/nohehf/stack-graphs))

It exposes a minimal, opinionated API to leverage the stack-graphs library for reference resolution in source code.

The rust bindings are built using [PyO3](https://pyo3.rs) and [maturin](https://maturin.rs).

Note that this is a work in progress, and the API is subject to change. This project is not affiliated with GitHub.

## Installation & Usage

```bash
pip install stack-graphs-python-bindings
```

### Example

Given the following directory structure:

```bash
tests/js_sample
├── index.js
└── module.js
```

`index.js`:

```javascript
import { foo } from "./module"
const baz = foo
```

`module.js`:

```javascript
export const foo = "bar"
```

The following Python script:

```python
import os
from stack_graphs_python import Indexer, Querier, Position, Language

db_path = os.path.abspath("./db.sqlite")
dir = os.path.abspath("./tests/js_sample")

# Index the directory (creates stack-graphs database)
indexer = Indexer(db_path, [Language.JavaScript])
indexer.index_all([dir])

# Instantiate a querier
querier = Querier(db_path)

# Query a reference at a given position (0-indexed line and column):
# foo in: const baz = foo
source_reference = Position(path=dir + "/index.js", line=2, column=12)
results = querier.definitions(source_reference)

for r in results:
    print(r)
```

Will output:

```bash
Position(path="[...]/tests/js_sample/index.js", line=0, column=9)
Position(path="[...]/tests/js_sample/module.js", line=0, column=13)
```

That translates to:

```javascript
// index.js
import { foo } from "./module"
      // ^ line 0, column 9

// module.js
export const foo = "bar"
          // ^ line 0, column 13
```

> **Note**: All the paths are absolute, and line and column numbers are 0-indexed (first line is 0, first column is 0).

## Known stack-graphs / tree-sitter issues

- Typescript: tree-sitter-typescript fails when passing a generic type to a decorator: <https://github.com/tree-sitter/tree-sitter-typescript/issues/283>

## Development

### Ressources

https://pyo3.rs/v0.21.2/getting-started

### Requirements

- Rust
- Python 3.11+

### Setup

```bash
# Setup venv and install dev dependencies
make setup
```

### Testing

```bash
make test
```

### Manual testing

```bash
# build the package
make develop
# activate the venv
. venv/bin/activate
```

### Roadmap

Before releasing 0.1.0, which I expect to be a first stable API, the following needs to be done:

- [ ] Add more testing, especially:
  - [ ] Test all supported languages (Java, ~~Python~~, ~~TypeScript~~, ~~JavaScript~~)
  - [x] Test failing cases, eg. files that cannot be indexed
- [ ] Add options to the classes:
  - [ ] Verbosity
  - [ ] Force for the Indexer
  - [ ] Fail on error for the Indexer, or continue indexing
- [ ] Handle the storage (database) in a dedicated class, and pass it to the Indexer and Querier *-> this might not be necessary*
- [x] Add methods to query the indexing status (eg. which files have been indexed, which failed, etc.)
- [x] Rely on the main branch of stack-graphs, and update the bindings accordingly
- [ ] Better error handling, return clear errors, test them and add them to the `.pyi` interface
- [ ] Lint and format the rust code
- [ ] CI/CD for the rust code
- [ ] Lint and format the python code
- [ ] Propper changelog, starting in 0.1.0

I'd also like to add the following features, after 0.1.0:

- [ ] Expose the exact, lower-level API of stack-graphs, for more flexibility, in a separate module (eg. `stack_graphs_python.core`)
- [ ] Benchmark performance