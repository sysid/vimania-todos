import pytest

# noinspection PyUnresolvedReferences,PyProtectedMember
from vimania_todos import _vimania_todos


def test_rs_sum_as_string():
    print("xxxxx")
    print(_vimania_todos.sum_as_string(1, 2))
    assert _vimania_todos.sum_as_string(1, 2) == "103"


@pytest.mark.parametrize(
    ("text", "result"),
    (
        ("- [ ] bla bub ()", "-%13% [ ] bla bub ()"),
        # Bug: trailing single quote
        ("- [ ] bla bub '()'", "-%13% [ ] bla bub '()'"),
        ("'- [ ] invalid single quote'", "'- [ ] invalid single quote'"),
        ("- [b] xxxx: invalid", "- [b] xxxx: invalid"),
        ("[ ] xxxx: invalid", "[ ] xxxx: invalid"),
        ("- [ ] todoa ends () hiere.", "-%13% [ ] todoa ends () hiere."),
        ("-%123% [d] should be deleted", ""),
        ("- [D] should be deleted", ""),
    ),
)
def test_handle_it(dal, text, result):
    lines = text.split("\n")
    new_lines = _vimania_todos.handle_it(lines, path="testpath", read=False)
    new_text = "\n".join(new_lines)
    print(new_text)
    assert new_text == result
