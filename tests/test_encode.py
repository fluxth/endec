import pytest

import endec
from endec.exceptions import EncodeError


def test_encode_utf8():
    assert endec.encode("utf8_string") == b"utf8_string"
    assert endec.encode("utf8_string", "utf-8") == b"utf8_string"
    assert endec.encode("utf8_string", encoding="utf-8") == b"utf8_string"


def test_encode_errors_strict():
    assert (
        endec.encode("こんにちは", "utf-8", "strict")
        == b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf"
    )
    assert (
        endec.encode("こんにちは", "utf-8", errors="strict")
        == b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf"
    )


def test_encode_errors_strict_failure():
    with pytest.raises(EncodeError):
        endec.encode("こんにちは", "ascii", "strict")
    with pytest.raises(EncodeError):
        endec.encode("こんにちは", "ascii", errors="strict")


def test_encode_errors_xmlcharrefreplace():
    assert (
        endec.encode("こんにちは", "ascii", "xmlcharrefreplace")
        == b"&#12371;&#12435;&#12395;&#12385;&#12399;"
    )
    assert (
        endec.encode("こんにちは", "ascii", errors="xmlcharrefreplace")
        == b"&#12371;&#12435;&#12395;&#12385;&#12399;"
    )


def test_encode_errors_unknown():
    # python stdlib does not raise LookupError unless we have an error
    endec.encode(
        "unknown_errors_param", "ascii", "unknown"  # type: ignore [reportArgumentType]
    )
    endec.encode(
        "unknown_errors_param", "ascii", errors="unknown"  # type: ignore [reportArgumentType]
    )


def test_encode_errors_unknown_failure():
    with pytest.raises(LookupError):
        endec.encode(
            "これはasciiではありません", "ascii", "unknown"  # type: ignore [reportArgumentType]
        )
    with pytest.raises(LookupError):
        endec.encode(
            "これはasciiではありません", "ascii", errors="unknown"  # type: ignore [reportArgumentType]
        )
