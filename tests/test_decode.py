import pytest

import endec
from endec.exceptions import DecodeError


def test_decode_utf8():
    assert endec.decode(b"utf8_bytes") == "utf8_bytes"
    assert endec.decode(b"utf8_bytes", "utf-8") == "utf8_bytes"
    assert endec.decode(b"utf8_bytes", encoding="utf-8") == "utf8_bytes"


def test_decode_errors_strict():
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


def test_decode_errors_strict_failure():
    with pytest.raises(DecodeError):
        endec.decode(b"\x00\x42\x69\xff", "utf-8", "strict")
    with pytest.raises(DecodeError):
        endec.decode(b"\x00\x42\x69\xff", "utf-8", errors="strict")


def test_decode_errors_replace():
    assert endec.decode(b"\x42\x69\xff\xee", "utf-8", "replace") == "Bi\ufffd\ufffd"
    assert (
        endec.decode(b"\x42\x69\xff\xee", "utf-8", errors="replace") == "Bi\ufffd\ufffd"
    )


def test_decode_errors_unknown():
    # python stdlib does not raise LookupError unless we have an error
    endec.decode(
        b"unknown_errors_param", "utf-8", "unknown"  # type: ignore [reportArgumentType]
    )
    endec.decode(
        b"unknown_errors_param", "utf-8", errors="unknown"  # type: ignore [reportArgumentType]
    )


def test_decode_errors_unknown_failure():
    with pytest.raises(LookupError):
        endec.decode(
            b"\x00\x42\x69\xff", "utf-8", "unknown"  # type: ignore [reportArgumentType]
        )
    with pytest.raises(LookupError):
        endec.decode(
            b"\x00\x42\x69\xff", "utf-8", errors="unknown"  # type: ignore [reportArgumentType]
        )


def test_decode_bom_evaluate():
    # bom should default to evaluate
    assert endec.decode(b"\xff\xfetest", "utf-16") == "整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16") == "瑥獴"
    assert endec.decode(b"\xff\xfeetts", "utf-16") == "瑥獴"
    assert endec.decode(b"\xef\xbb\xbftest", "utf-8") == "test"

    # explicit bom kwargs
    assert endec.decode(b"\xff\xfetest", "utf-16", bom="evaluate") == "整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16", bom="evaluate") == "瑥獴"
    assert endec.decode(b"\xff\xfeetts", "utf-16", bom="evaluate") == "瑥獴"
    assert endec.decode(b"\xef\xbb\xbftest", "utf-8", bom="evaluate") == "test"


def test_decode_bom_evaluate_morph():
    # evaluate should not morph other codecs other than utf-16be, utf-16le

    # bom should default to evaluate
    assert endec.decode(b"\xff\xfetest", "latin1") == "ÿþtest"

    with pytest.raises(DecodeError):
        endec.decode(b"\xfe\xfftest", "utf-8")

    # explicit bom kwargs
    assert endec.decode(b"\xff\xfetest", "latin1", bom="evaluate") == "ÿþtest"

    with pytest.raises(DecodeError):
        endec.decode(b"\xfe\xfftest", "utf-8", bom="evaluate")


def test_decode_bom_evaluateall():
    # evaluateall should morph other codecs other than utf-16be, utf-16le

    # morph to utf-16le
    assert endec.decode(b"\xff\xfetest", "latin1", bom="evaluateall") == "整瑳"

    # morph to utf-16be
    assert endec.decode(b"\xfe\xfftest", "latin1", bom="evaluateall") == "瑥獴"

    # morph to utf-8
    assert endec.decode(b"\xef\xbb\xbftest", "latin1", bom="evaluateall") == "test"


def test_decode_bom_strip():
    assert endec.decode(b"\xff\xfetest", "utf-16", bom="strip") == "整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16", bom="strip") == "\ufffe整瑳"
    assert endec.decode(b"\xff\xfeetts", "utf-16", bom="strip") == "瑥獴"
    assert endec.decode(b"\xef\xbb\xbftest", "utf-8", bom="strip") == "test"


def test_decode_bom_ignore_error_strict():
    # errors should default to strict
    assert endec.decode(b"\xff\xfetest", "utf-16", bom="ignore") == "\ufeff整瑳"
    assert endec.decode(b"\xfe\xfftest", "utf-16", bom="ignore") == "\ufffe整瑳"
    assert endec.decode(b"\xff\xfeetts", "utf-16", bom="ignore") == "\ufeff瑥獴"
    assert endec.decode(b"\xef\xbb\xbftest", "utf-8", bom="ignore") == "\ufefftest"

    # explicit errors args
    assert (
        endec.decode(b"\xff\xfetest", "utf-16", "strict", bom="ignore") == "\ufeff整瑳"
    )
    assert (
        endec.decode(b"\xfe\xfftest", "utf-16", "strict", bom="ignore") == "\ufffe整瑳"
    )
    assert (
        endec.decode(b"\xff\xfeetts", "utf-16", "strict", bom="ignore") == "\ufeff瑥獴"
    )
    assert (
        endec.decode(b"\xef\xbb\xbftest", "utf-8", "strict", bom="ignore")
        == "\ufefftest"
    )

    # explicit errors kwargs
    assert (
        endec.decode(b"\xff\xfetest", "utf-16", errors="strict", bom="ignore")
        == "\ufeff整瑳"
    )
    assert (
        endec.decode(b"\xfe\xfftest", "utf-16", errors="strict", bom="ignore")
        == "\ufffe整瑳"
    )
    assert (
        endec.decode(b"\xff\xfeetts", "utf-16", errors="strict", bom="ignore")
        == "\ufeff瑥獴"
    )
    assert (
        endec.decode(b"\xef\xbb\xbftest", "utf-8", errors="strict", bom="ignore")
        == "\ufefftest"
    )


def test_decode_bom_ignore_error_strict_failure():
    with pytest.raises(DecodeError):
        endec.decode(b"\xff\xfetest", "utf-8", "strict", bom="ignore")

    with pytest.raises(DecodeError):
        endec.decode(b"\xff\xfetest", "utf-8", errors="strict", bom="ignore")


def test_decode_bom_ignore_error_replace():
    # explicit errors args
    assert (
        endec.decode(b"\xff\xfetest", "utf-8", "replace", bom="ignore")
        == "\ufffd\ufffdtest"
    )
    assert (
        endec.decode(b"\xfe\xfftest", "utf-8", "replace", bom="ignore")
        == "\ufffd\ufffdtest"
    )

    # explicit errors kwargs
    assert (
        endec.decode(b"\xff\xfetest", "utf-8", errors="replace", bom="ignore")
        == "\ufffd\ufffdtest"
    )
    assert (
        endec.decode(b"\xfe\xfftest", "utf-8", errors="replace", bom="ignore")
        == "\ufffd\ufffdtest"
    )
