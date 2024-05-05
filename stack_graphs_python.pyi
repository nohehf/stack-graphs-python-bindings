def index(paths: list[str], db_path: str) -> None: ...

class Position:
    path: str
    line: int
    column: int

    def __init__(self, path: str, line: int, column: int) -> None: ...

def query_definition(reference: Position, db_path: str) -> list[Position]: ...
