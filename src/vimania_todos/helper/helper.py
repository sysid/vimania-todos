def sanitize(text_: str) -> str:
    return text_.replace("'", r"`")
