# endec

[![PyPI - Version](https://img.shields.io/pypi/v/endec)](https://pypi.org/project/endec/)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/endec)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/fluxth/endec/build.yml)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/fluxth/endec/test.yml?label=tests)

Web-compatible **en**coding and **dec**oding library

**endec** uses [`encoding_rs`](https://github.com/hsivonen/encoding_rs) (which powers Firefox) under the hood.

## Installation

Requires Python 3.8+

```
$ pip install endec
```

## Examples

### Codecs

Please refer to [WHATWG Web Encoding Standard](https://encoding.spec.whatwg.org/#concept-encoding-get) for available codecs.

### Encode

```python
import endec

utf8_bytes = endec.encode("こんにちは")
assert utf8_bytes == b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf"

iso2022jp_bytes = endec.encode("㊤㊥㊦", "iso-2022-jp")
assert iso2022jp_bytes == b"\x1b$B-e-f-g\x1b(B"

"㊤㊥㊦".encode("iso-2022-jp")  # Standard Library `encode`
# UnicodeEncodeError: 'iso2022_jp' codec can't encode character '\u32a4' in position 0: illegal multibyte sequence
```

### Decode

```python
import endec

utf8_str = endec.decode(b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf")
assert utf8_str == "こんにちは"

iso2022jp_str = endec.decode(b"\x1b$B-e-f-g\x1b(B", "iso-2022-jp")
assert iso2022jp_str == "㊤㊥㊦"

b"\x1b$B-e-f-g\x1b(B".decode("iso-2022-jp")  # Standard Library `decode`
# UnicodeDecodeError: 'iso2022_jp' codec can't decode bytes in position 3-4: illegal multibyte sequence
```

### Error Handling

```python
import endec
from endec.exceptions import EncodeError, DecodeError

try:
    invalid_encode = endec.encode("漢字", "ascii")
except EncodeError as exc:
    # endec.exceptions.EncodeError: encoding with 'windows-1252' codec failed
    raise exc

try:
    invalid_decode = endec.decode(b"\x42\xff\x42", "iso-2022-jp")
except DecodeError as exc:
    # endec.exceptions.DecodeError: decoding with 'ISO-2022-JP' codec failed
    raise exc
```

## License

This project is licensed under the terms of the [MIT license](https://github.com/fluxth/endec/blob/main/LICENSE).
