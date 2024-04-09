import pytest

import endec
from endec.exceptions import DecodeError


def test_decode_utf8():
    assert endec.decode(b"utf8_bytes") == "utf8_bytes"
    assert endec.decode(b"utf8_bytes", "utf-8") == "utf8_bytes"


def test_decode_errors_strict():
    with pytest.raises(DecodeError):
        endec.decode(b"\x00\x42\x69\xff", "utf-8", "strict")

    assert (
        endec.decode(
            b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf",
            "utf-8",
            "strict",
        )
        == "こんにちは"
    )
