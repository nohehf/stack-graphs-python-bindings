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
from stack_graphs_python import index, query_definition, Position

# ...
```

You can refer to the example in [test/test.py](https://github.com/nohehf/stack-graphs-python-bindings/blob/main/tests/test.py) for a concrete usage example.

## Development

### Ressources

https://pyo3.rs/v0.21.2/getting-started

### Setup

```bash
pipx install maturin # or pip, ...
```

### Testing

```bash
maturin develop
python tests/test.py
```
