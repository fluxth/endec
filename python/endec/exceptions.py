from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from typing import Optional


class DecodeError(UnicodeError):
    encoding: str
    target: bytes
    reason: "Optional[str]"

    def __init__(
        self, encoding: str, target: bytes, reason: "Optional[str]" = None, /
    ) -> None:
        self.encoding = encoding
        self.target = target
        self.reason = reason

    def __str__(self) -> str:
        reason = ""
        if self.reason:
            reason = self.reason + "\n"

        return reason + f"decoding with '{self.encoding}' codec failed"


class EncodeError(UnicodeError):
    encoding: str
    target: str
    reason: "Optional[str]"

    def __init__(
        self, encoding: str, target: str, reason: "Optional[str]" = None, /
    ) -> None:
        self.encoding = encoding
        self.target = target
        self.reason = reason

    def __str__(self) -> str:
        reason = ""
        if self.reason:
            reason = self.reason + "\n"

        return reason + f"encoding with '{self.encoding}' codec failed"
