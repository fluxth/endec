# endec

Web-friendly **en**coding and **dec**oding library.

**endec** uses [`encoding_rs`](https://github.com/hsivonen/encoding_rs) (which powers Firefox) under the hood.

## Installation

Requires Python 3.8+

```
$ pip install endec
```

**NOTE: This project is in a pre-release state, please do not use it in production workloads.**

## Examples

### Encode

```python
import endec

utf8_bytes = endec.encode("こんにちは")
assert utf8_bytes == b"\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf"

iso2022jp_bytes = endec.encode("こんにちは", "iso-2022-jp")
assert iso2022jp_bytes == b"\x1b$B$3$s$K$A$O\x1b(B"
```

### Decode

```python
import endec

utf8_str = endec.decode(b'\xe3\x81\x93\xe3\x82\x93\xe3\x81\xab\xe3\x81\xa1\xe3\x81\xaf')
assert utf8_str == "こんにちは"

iso2022jp_str = endec.decode(b"\x1b$B$3$s$K$A$O\x1b(B", "iso-2022-jp")
assert iso2022jp_str == "こんにちは"
```

### Codecs

Please refer to [WHATWG Web Encoding Standard](https://encoding.spec.whatwg.org/#concept-encoding-get) for avaliable codecs.

## License

This project is licensed under the terms of the MIT license.
