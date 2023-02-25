# noinspection PyUnresolvedReferences,PyProtectedMember
from vimania_todos import _vimania_todos


def test_rs_sum_as_string():
    print("xxxxx")
    print(_vimania_todos.sum_as_string(1, 2))
    assert _vimania_todos.sum_as_string(1, 2) == "103"
