use alloc::{
    string::{String as RustString, ToString},
    vec::Vec,
};

use jvm::{Jvm, Result};

// Charsets shared by java.lang.String and java.io.InputStreamReader so both support the same set.
#[derive(Clone, Copy)]
pub enum Charset {
    Utf8,
    EucKr,
    Iso8859_1,
    UsAscii,
}

impl Charset {
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_uppercase().replace('_', "-").as_str() {
            "UTF-8" | "UTF8" => Some(Self::Utf8),
            "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949" => Some(Self::EucKr),
            "ISO-8859-1" | "LATIN1" => Some(Self::Iso8859_1),
            "US-ASCII" | "ASCII" => Some(Self::UsAscii),
            _ => None,
        }
    }

    // Canonical name stored in InputStreamReader's charset field and compared there.
    pub fn canonical_name(&self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::EucKr => "EUC-KR",
            Self::Iso8859_1 => "ISO-8859-1",
            Self::UsAscii => "US-ASCII",
        }
    }

    pub async fn resolve(jvm: &Jvm, name: &str) -> Result<Self> {
        match Self::from_name(name) {
            Some(x) => Ok(x),
            None => Err(jvm.exception("java/io/UnsupportedEncodingException", name).await),
        }
    }

    pub fn decode(&self, bytes: &[u8]) -> RustString {
        match self {
            Self::Utf8 => RustString::from_utf8_lossy(bytes).into_owned(),
            Self::EucKr => encoding_rs::EUC_KR.decode(bytes).0.to_string(),
            Self::Iso8859_1 | Self::UsAscii => bytes.iter().map(|&b| b as char).collect(),
        }
    }

    pub fn encode(&self, string: &str) -> Vec<u8> {
        match self {
            Self::Utf8 => string.as_bytes().to_vec(),
            Self::EucKr => encoding_rs::EUC_KR.encode(string).0.to_vec(),
            Self::Iso8859_1 => string.chars().map(|c| if (c as u32) <= 0xff { c as u8 } else { b'?' }).collect(),
            Self::UsAscii => string.chars().map(|c| if c.is_ascii() { c as u8 } else { b'?' }).collect(),
        }
    }

    // How many trailing bytes of `bytes` to withhold from a non-final decode because they open a character the buffer
    // does not yet complete. No wildcard arm on purpose: a new charset must decide this, or its characters split at
    // read boundaries vanish silently.
    pub fn bytes_to_hold_back(&self, bytes: &[u8]) -> usize {
        match self {
            Self::Utf8 => {
                if bytes.is_empty() {
                    return 0;
                }
                let mut lead_index = bytes.len() - 1;
                while lead_index > 0 && bytes[lead_index] & 0xc0 == 0x80 {
                    lead_index -= 1;
                }
                let expected_length = match bytes[lead_index] {
                    0xc0..=0xdf => 2,
                    0xe0..=0xef => 3,
                    0xf0..=0xf7 => 4,
                    0x00..=0xbf | 0xf8..=0xff => 1,
                };
                if bytes.len() - lead_index < expected_length {
                    bytes.len() - lead_index
                } else {
                    0
                }
            }
            Self::EucKr => {
                // Unlike UTF-8, EUC-KR trail bytes overlap the lead range (0x81..=0xfe), so the last
                // byte alone cannot say whether the final pair is complete — a whole pair ends in a
                // byte that looks exactly like a lead. Withholding it there strands the pair's own
                // lead byte, which the decoder then swallows into state this read is about to drop.
                // The buffer always begins on a character boundary, so walk it forward instead: a byte
                // >= 0x81 opens a pair, anything else stands alone. Only a lead byte that overruns
                // the buffer is held back.
                let mut index = 0;
                while index < bytes.len() {
                    index += if bytes[index] >= 0x81 { 2 } else { 1 };
                }
                if index > bytes.len() { 1 } else { 0 }
            }
            Self::Iso8859_1 | Self::UsAscii => 0,
        }
    }

    pub fn new_stream_decoder(&self) -> CharsetStreamDecoder {
        match self {
            Self::Utf8 => CharsetStreamDecoder::EncodingRs(encoding_rs::UTF_8.new_decoder_without_bom_handling()),
            Self::EucKr => CharsetStreamDecoder::EncodingRs(encoding_rs::EUC_KR.new_decoder_without_bom_handling()),
            Self::Iso8859_1 | Self::UsAscii => CharsetStreamDecoder::ByteToChar,
        }
    }
}

pub enum CharsetStreamDecoder {
    EncodingRs(encoding_rs::Decoder),
    // single-byte charsets where each byte maps to the same code point
    ByteToChar,
}

impl CharsetStreamDecoder {
    // Returns (bytes consumed, utf-16 code units written), like encoding_rs's decode_to_utf16.
    pub fn decode_to_utf16(&mut self, src: &[u8], dst: &mut [u16], last: bool) -> (usize, usize) {
        match self {
            Self::EncodingRs(decoder) => {
                let (_, read, written, _) = decoder.decode_to_utf16(src, dst, last);
                (read, written)
            }
            Self::ByteToChar => {
                let len = core::cmp::min(src.len(), dst.len());
                for (d, &s) in dst.iter_mut().zip(src) {
                    *d = s as u16;
                }
                (len, len)
            }
        }
    }
}
