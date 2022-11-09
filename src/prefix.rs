use crate::{derivation::SelfAddressing, error::Error};

use core::{fmt, str::FromStr};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Clone)]
pub struct SelfAddressingPrefix {
    /// Hashing method
    pub derivation: SelfAddressing,
    /// Hashing result
    pub digest: Vec<u8>,
}

impl SelfAddressingPrefix {
    pub fn new(code: SelfAddressing, digest: Vec<u8>) -> Self {
        Self {
            derivation: code,
            digest,
        }
    }

    pub fn verify_binding(&self, sed: &[u8]) -> bool {
        self.derivation.digest(sed) == self.digest
    }

    pub fn to_str(&self) -> String {
        // empty data cannot be prefixed!
        match self.digest.len() {
            0 => "".to_string(),
            _ => {
                let dc = self.derivation_code();
                let lead_bytes = if dc.len() % 4 != 0 { dc.len() } else { 0 };
                // replace lead bytes with code
                let derivative_text =
                    from_bytes_to_text(&self.derivative())[lead_bytes..].to_string();
                [dc, derivative_text].join("")
            }
        }
    }

    pub fn derivative(&self) -> Vec<u8> {
        self.digest.to_owned()
    }
    pub fn derivation_code(&self) -> String {
        self.derivation.to_str()
    }
}

impl FromStr for SelfAddressingPrefix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = SelfAddressing::from_str(s)?;
        let prefix_b64_len = code.code_len() + code.derivative_b64_len();
        let c_len = code.code_len();
        if s.len() == prefix_b64_len {
            let decoded = from_text_to_bytes(&s[c_len..].as_bytes())?[c_len..].to_vec();

            Ok(Self::new(code.into(), decoded))
        } else {
            Err(Error::DeserializationError(format!(
                "Incorrect Prefix Length: {}",
                s
            )))
        }
    }
}

impl fmt::Display for SelfAddressingPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

/// Serde compatible Serialize
impl Serialize for SelfAddressingPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_str())
    }
}

/// Serde compatible Deserialize
impl<'de> Deserialize<'de> for SelfAddressingPrefix {
    fn deserialize<D>(deserializer: D) -> Result<SelfAddressingPrefix, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        SelfAddressingPrefix::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Default for SelfAddressingPrefix {
    fn default() -> Self {
        Self {
            derivation: SelfAddressing::Blake3_256,
            digest: vec![],
        }
    }
}

pub fn from_text_to_bytes(text: &[u8]) -> Result<Vec<u8>, Error> {
    let lead_size = (4 - (text.len() % 4)) % 4;
    let full_derivative = ["A".repeat(lead_size).as_bytes(), text].concat();

    Ok(base64::decode_config(full_derivative, base64::URL_SAFE)?.to_vec())
}

pub fn from_bytes_to_text(bytes: &[u8]) -> String {
    let lead_size = (3 - (bytes.len() % 3)) % 3;
    let full_derivative: Vec<_> = std::iter::repeat(0)
        .take(lead_size)
        .chain(bytes.to_vec().into_iter())
        .collect();

    base64::encode_config(full_derivative, base64::URL_SAFE)
}
