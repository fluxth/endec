from typing import Literal

EncodeErrorHandler = Literal["strict", "xmlcharrefreplace"]
DecodeErrorHandler = Literal["strict", "replace"]
DecodeBomHandler = Literal["evaluate", "evaluateall", "strip", "ignore"]

def encode(
    input_str: str,
    /,
    encoding: str = "utf-8",
    errors: EncodeErrorHandler = "strict",
) -> bytes: ...
def decode(
    input_bytes: bytes,
    /,
    encoding: str = "utf-8",
    errors: DecodeErrorHandler = "strict",
    *,
    bom: DecodeBomHandler = "evaluate",
) -> str: ...
