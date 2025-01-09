use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use std::io::Read;

use crate::Base64Format;

pub fn process_encode(reader: &mut impl Read, format: Base64Format) -> Result<String> {
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    Ok(encoded)
}

pub fn process_decode(reader: &mut impl Read, format: Base64Format) -> Result<Vec<u8>> {
    let mut buf = String::new();
    // reader.read_to_string(&mut buf)?;
    reader.read_to_string(&mut buf).unwrap();

    let buf = buf.trim();

    println!("input: {}", buf);

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_reader;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        let mut reader = get_reader(input).unwrap();
        assert!(process_encode(&mut reader, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::UrlSafe;
        let mut reader = get_reader(input).unwrap();
        assert!(process_decode(&mut reader, format).is_ok());
    }
}
