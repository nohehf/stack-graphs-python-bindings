from enum import Enum

class Language(Enum):
    Python = 0
    JavaScript = 1
    TypeScript = 2
    Java = 3

class Position:
    """
    A position in a given file:
    - path: the path to the file
    - line: the line number (0-indexed)
    - column: the column number (0-indexed)
    """

    path: str
    line: int
    column: int

    def __init__(self, path: str, line: int, column: int) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __repr__(self) -> str: ...

class Querier:
    """
    A class to query the stack graphs database
    - db_path: the path to the database

    Usage: see Querier.definitions
    """
    def __init__(self, db_path: str) -> None: ...
    def definitions(self, reference: Position) -> list[Position]:
        """
        Get the definitions of a given reference
        - reference: the position of the reference
        - returns: a list of positions of the definitions
        """
        ...
    def __repr__(self) -> str: ...

class Indexer:
    """
    A class to build the stack graphs of a given set of files
    - db_path: the path to the database
    - languages: the list of languages to index
    """
    def __init__(self, db_path: str, languages: list[Language]) -> None: ...
    def index_all(self, paths: list[str]) -> None:
        """
        Index all the files in the given paths, recursively
        """
        ...
    def __repr__(self) -> str: ...

def index(paths: list[str], db_path: str, language: Language) -> None:
    """
    DeprecationWarning: The 'index' function is deprecated. Use 'Indexer' instead.
    """
    ...
