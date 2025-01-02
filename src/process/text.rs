use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
// use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use std::{fs, io::Read};

use anyhow::Result;

trait TextSign {
    /// Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

// trait TextVerify {
//     /// Verify the data from the reader with the signature
//     fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
// }

struct Blake3 {
    key: [u8; 32],
}

// struct Ed25519Signer {
//     key: SigningKey,
// }

// struct Ed25519Verifier {
//     key: VerifyingKey,
// }

use crate::{get_reader, TextSignFormat};
pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let key = &key[..32];
            let key = key.try_into()?;
            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}", signed);
    Ok(())
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

// impl TextSign for Ed25519Signer {
//     fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
//         let mut buf = Vec::new();
//         reader.read_to_end(&mut buf)?;
//         let sig = self.key.sign(&buf);
//         Ok(sig.to_bytes().to_vec())
//     }
// }

// impl TextVerify for Blake3 {
//     fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
//         let mut buf = Vec::new();
//         reader.read_to_end(&mut buf)?;
//         let hash = blake3::hash(&buf);
//         let hash = hash.as_bytes();
//         Ok(hash == sig)
//     }
// }

// impl TextVerify for Ed25519Signer {
//     fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
//         let mut buf = Vec::new();
//         reader.read_to_end(&mut buf)?;
//         let sig = Signature::from_bytes(sig.try_into()?);
//         let ret = self.key.verify(&buf, &sig).is_ok();
//         Ok(ret)
//     }
// }
