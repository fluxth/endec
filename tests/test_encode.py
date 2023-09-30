import pytest

import endec


def test_encode_utf8():
    assert endec.encode("utf8_string") == b"utf8_string"
    assert endec.encode("utf8_string", "utf-8") == b"utf8_string"


def test_encode_errors_strict():
    with pytest.raises(ValueError):  # FIXME: UnicodeEncodeError
        endec.encode("こんにちは", "ascii", "strict")

    assert (
        endec.encode("こんにちは", "utf-8", "strict")
        == b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf"
    )


def test_encode_errors_xmlcharrefreplace():
    assert (
        endec.encode("こんにちは", "ascii", "xmlcharrefreplace")
        == b"&#12371;&#12435;&#12395;&#12385;&#12399;"
    )


def test_encode_errors_unknown():
    with pytest.raises(LookupError):
        endec.encode("これはasciiではありません", "ascii", "unknown")

    # python stdlib does not raise LookupError unless we have an error
    endec.encode("unknown_errors_param", "ascii", "unknown")
