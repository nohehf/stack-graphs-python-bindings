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

def index(paths: list[str], db_path: str, language: Language) -> None: ...
def query_definition(reference: Position, db_path: str) -> list[Position]: ...
