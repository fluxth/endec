use std::borrow::Cow;

use encoding_rs::Encoding;

pub(crate) enum EncodeErrorHandler {
    Strict,
    XMLCharacterReferenceReplace,
    InvalidHandler,
}

pub(crate) enum EncodeError {
    EncodeFailed { used_encoding: &'static Encoding },
    InvalidErrorHandler,
}

pub(crate) fn encode<'s>(
    input_str: &'s str,
    encoding: &'static Encoding,
    error_handler: EncodeErrorHandler,
) -> Result<Cow<'s, [u8]>, EncodeError> {
    let (out, used_encoding, has_unmappable) = encoding.encode(input_str);

    if has_unmappable {
        match error_handler {
            EncodeErrorHandler::XMLCharacterReferenceReplace => Ok(out),
            EncodeErrorHandler::Strict => Err(EncodeError::EncodeFailed { used_encoding }),
            EncodeErrorHandler::InvalidHandler => Err(EncodeError::InvalidErrorHandler),
        }
    } else {
        Ok(out)
    }
}
