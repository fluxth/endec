import endec

# https://en.wikipedia.org/wiki/ISO/IEC_2022#Character_set_designations


def test_encode_gzdm4():
    assert endec.encode("㊤㊥㊦", "iso-2022-jp") == b"\x1b$B-e-f-g\x1b(B"


def test_decode_gzdm4():
    assert endec.decode(b"\x1b$B-e-f-g\x1b(B", "iso-2022-jp") == "㊤㊥㊦"
