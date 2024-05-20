from stack_graphs_python import Position


def test_position_eq():
    assert Position("a", 1, 2) == Position("a", 1, 2)
    assert Position("a", 1, 2) != Position("a", 1, 3)
    assert Position("a", 1, 2) != Position("a", 2, 2)
    assert Position("a", 1, 2) != Position("b", 1, 2)
    assert Position("a", 1, 2) != "Position(path='a', line=1, column=2)"
    assert Position("a", 1, 2) is not None
    assert Position("a", 1, 2) != 1


def test_position_repr():
    assert repr(Position("a", 1, 2)) == 'Position(path="a", line=1, column=2)'
