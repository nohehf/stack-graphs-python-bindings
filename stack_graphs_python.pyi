from enum import Enum

class Language(Enum):
    Python = 0
    JavaScript = 1
    TypeScript = 2
    Java = 3

class Position:
    path: str
    line: int
    column: int

    def __init__(self, path: str, line: int, column: int) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __repr__(self) -> str: ...

class Querier:
    def __init__(self, db_path: str) -> None: ...
    def definitions(self, reference: Position) -> list[Position]: ...
    def __repr__(self) -> str: ...

class Indexer:
    def __init__(self, db_path: str, languages: list[Language]) -> None: ...
    def index_all(self, paths: list[str]) -> None: ...
    def __repr__(self) -> str: ...

def index(paths: list[str], db_path: str, language: Language) -> None:
    """
    DeprecationWarning: The 'index' function is deprecated. Use 'Indexer' instead.
    """
    ...
