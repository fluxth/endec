import pytest

import endec
from endec.exceptions import DecodeError


def test_decode_utf8():
    assert endec.decode(b"utf8_bytes") == "utf8_bytes"
    assert endec.decode(b"utf8_bytes", "utf-8") == "utf8_bytes"
    assert endec.decode(b"utf8_bytes", encoding="utf-8") == "utf8_bytes"


def test_decode_errors_strict():
    with pytest.raises(DecodeError):
        endec.decode(b"\x00\x42\x69\xff", "utf-8", "strict")
        endec.decode(b"\x00\x42\x69\xff", "utf-8", errors="strict")

    assert (
        endec.decode(
            b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf",
            "utf-8",
            "strict",
        )
        == "こんにちは"
    )
    assert (
        endec.decode(
            b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf",
            "utf-8",
            errors="strict",
        )
        == "こんにちは"
    )


def test_decode_errors_unknown():
    with pytest.raises(LookupError):
        endec.decode(
            b"\x42\x42", "ascii", "unknown"  # type: ignore [reportArgumentType]
        )

    # FIXME make this pass
    # python stdlib does not raise LookupError unless we have an error
    # endec.decode(
    #    b"unknown_errors_param", "ascii", "unknown"  # type: ignore [reportArgumentType]
    # )


def test_decode_bom_evaluate():
    # bom should default to evaluate
    assert endec.decode(b"\xff\xfetest", "utf-16") == "整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16") == "瑥獴"

    # explicit bom args
    assert endec.decode(b"\xff\xfetest", "utf-16", bom="evaluate") == "整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16", bom="evaluate") == "瑥獴"
    assert endec.decode(b"\xff\xfeetts", "utf-16", bom="evaluate") == "瑥獴"

    # explicit bom kwargs
    assert endec.decode(b"\xff\xfetest", "utf-16", "strict", "evaluate") == "整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16", "strict", "evaluate") == "瑥獴"
    assert endec.decode(b"\xff\xfeetts", "utf-16", "strict", "evaluate") == "瑥獴"


def test_decode_bom_strip():
    # explicit bom args
    assert endec.decode(b"\xff\xfetest", "utf-16", bom="strip") == "整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16", bom="strip") == "\ufffe整瑳"
    assert endec.decode(b"\xff\xfeetts", "utf-16", bom="strip") == "瑥獴"

    # explicit bom kwargs
    assert endec.decode(b"\xff\xfetest", "utf-16", "strict", "strip") == "整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16", "strict", "strip") == "\ufffe整瑳"
    assert endec.decode(b"\xff\xfeetts", "utf-16", "strict", "strip") == "瑥獴"


# FIXME def test_decode_bom_ignore_error_strict()
# FIXME def test_decode_bom_ignore_error_replace()
