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

class Querier:
    def __init__(self, db_path: str) -> None: ...
    def definitions(self, reference: Position) -> list[Position]: ...

def index(paths: list[str], db_path: str, language: Language) -> None: ...
