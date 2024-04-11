use std::borrow::Cow;

use encoding_rs::{Encoding, UTF_16BE, UTF_16LE};

pub(crate) enum DecodeErrorHandler {
    Strict,
    Replace,
    InvalidHandler,
}

pub(crate) enum DecodeBOMHandler {
    Evaluate,
    EvaluateAll,
    Strip,
    Ignore,
    InvalidHandler,
}

pub(crate) enum DecodeError {
    DecodeFailed { used_encoding: &'static Encoding },
    InvalidErrorHandler,
    InvalidBOMHandler,
}

pub(crate) fn decode<'b>(
    input_bytes: &'b [u8],
    encoding: &'static Encoding,
    error_handler: DecodeErrorHandler,
    bom_handler: DecodeBOMHandler,
) -> Result<Cow<'b, str>, DecodeError> {
    let decoder_output = match bom_handler {
        DecodeBOMHandler::Evaluate => Some(decode_bom_evaluate(input_bytes, encoding)),
        DecodeBOMHandler::EvaluateAll => Some(decode_bom_evaluate_all(input_bytes, encoding)),
        DecodeBOMHandler::Strip => Some(decode_bom_strip(input_bytes, encoding)),
        DecodeBOMHandler::Ignore => decode_bom_ignore(input_bytes, encoding, &error_handler),
        DecodeBOMHandler::InvalidHandler => return Err(DecodeError::InvalidBOMHandler),
    };

    match (decoder_output, error_handler) {
        (None, DecodeErrorHandler::InvalidHandler) => Err(DecodeError::InvalidErrorHandler),
        (None, DecodeErrorHandler::Strict | DecodeErrorHandler::Replace) => {
            Err(DecodeError::DecodeFailed {
                used_encoding: encoding,
            })
        }
        (
            Some(DecoderOutput {
                output,
                has_replaced,
                ..
            }),
            DecodeErrorHandler::InvalidHandler,
        ) => {
            if has_replaced {
                // invalid error handler check in strict mode
                Err(DecodeError::InvalidErrorHandler)
            } else {
                Ok(output)
            }
        }
        (Some(DecoderOutput { output, .. }), DecodeErrorHandler::Replace) => Ok(output),
        (
            Some(DecoderOutput {
                output,
                used_encoding,
                has_replaced,
            }),
            DecodeErrorHandler::Strict,
        ) => {
            if has_replaced {
                // disallow replacement for strict mode
                // TODO add reason in exception
                Err(DecodeError::DecodeFailed { used_encoding })
            } else {
                Ok(output)
            }
        }
    }
}

struct DecoderOutput<'a> {
    output: Cow<'a, str>,
    used_encoding: &'static Encoding,
    has_replaced: bool,
}

fn decode_bom_evaluate<'b>(
    input_bytes: &'b [u8],
    encoding: &'static Encoding,
) -> DecoderOutput<'b> {
    if encoding == UTF_16BE || encoding == UTF_16LE {
        // only allow morphing for utf-16 in default evaluate mode
        decode_bom_evaluate_all(input_bytes, encoding)
    } else {
        decode_bom_strip(input_bytes, encoding)
    }
}

fn decode_bom_evaluate_all<'b>(
    input_bytes: &'b [u8],
    encoding: &'static Encoding,
) -> DecoderOutput<'b> {
    let (output, used_encoding, has_replaced) = encoding.decode(input_bytes);
    DecoderOutput {
        output,
        used_encoding,
        has_replaced,
    }
}

fn decode_bom_strip<'b>(input_bytes: &'b [u8], encoding: &'static Encoding) -> DecoderOutput<'b> {
    let (output, has_replaced) = encoding.decode_with_bom_removal(input_bytes);
    DecoderOutput {
        output,
        used_encoding: encoding,
        has_replaced,
    }
}

fn decode_bom_ignore<'b>(
    input_bytes: &'b [u8],
    encoding: &'static Encoding,
    error_handler: &'_ DecodeErrorHandler,
) -> Option<DecoderOutput<'b>> {
    match *error_handler {
        DecodeErrorHandler::Replace => {
            let (output, has_replaced) = encoding.decode_without_bom_handling(input_bytes);
            Some(DecoderOutput {
                output,
                used_encoding: encoding,
                has_replaced,
            })
        }
        DecodeErrorHandler::Strict => {
            let output =
                encoding.decode_without_bom_handling_and_without_replacement(input_bytes)?;
            Some(DecoderOutput {
                output,
                used_encoding: encoding,
                has_replaced: false,
            })
        }
        DecodeErrorHandler::InvalidHandler => None,
    }
}
