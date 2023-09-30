import pytest

import endec


def test_decode_utf8():
    assert endec.decode(b"utf8_bytes") == "utf8_bytes"
    assert endec.decode(b"utf8_bytes", "utf-8") == "utf8_bytes"


def test_encode_errors_strict():
    with pytest.raises(ValueError):  # FIXME: UnicodeDecodeError
        endec.decode(b"\x00\x42\x69\xff", "utf-8", "strict")

    assert (
        endec.decode(
            b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf",
            "utf-8",
            "strict",
        )
        == "こんにちは"
    )
