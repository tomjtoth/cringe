use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{de::Error as _, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Image {
    Url(String),

    Advanced {
        id: Option<i32>,
        url: String,
        prompt: Option<String>,
        #[serde(default)]
        position: Option<i16>,
    },

    Uploaded {
        id: Option<i32>,
        #[serde(serialize_with = "vec_to_base64", deserialize_with = "bytes_from_json")]
        bytes: Vec<u8>,
        prompt: Option<String>,
        #[serde(default)]
        position: Option<i16>,
    },
}

impl Image {
    pub fn id(&self) -> Option<i32> {
        match self {
            Self::Url(_) => None,
            Self::Advanced { id, .. } | Self::Uploaded { id, .. } => id.clone(),
        }
    }

    pub fn set_id(&mut self, id: Option<i32>) {
        let other_id = id;

        match self {
            Self::Url(_) => (),
            Self::Advanced { id, .. } | Self::Uploaded { id, .. } => *id = other_id,
        }
    }

    pub fn pos(&self) -> &Option<i16> {
        match self {
            Self::Url(_) => &None,
            Self::Advanced { position, .. } | Self::Uploaded { position, .. } => position,
        }
    }

    pub fn set_pos(&mut self, pos: Option<i16>) {
        match self {
            Self::Url(_) => (),
            Self::Advanced { position, .. } | Self::Uploaded { position, .. } => {
                *position = pos;
            }
        };
    }

    pub fn prompt(&self) -> Option<&str> {
        match self {
            Self::Url(_) => None,
            Self::Advanced { prompt, .. } | Self::Uploaded { prompt, .. } => prompt.as_deref(),
        }
    }

    pub fn set_prompt(&mut self, val: String) {
        match self {
            Self::Url(_) => (),
            Self::Advanced { prompt, .. } | Self::Uploaded { prompt, .. } => {
                *prompt = if val == "" { None } else { Some(val) }
            }
        }
    }

    pub fn src(&self) -> String {
        match self {
            Self::Url(src) => src.clone(),
            Self::Advanced { url, .. } => url.clone(),
            Self::Uploaded { bytes, .. } => {
                use base64::{engine::general_purpose::STANDARD, Engine as _};

                format!("data:image/avif;base64,{}", STANDARD.encode(bytes))
            }
        }
    }

    pub fn set_url(&mut self, url: Option<String>) {
        if let Some(new_url) = url {
            match self {
                Self::Advanced { url, .. } => *url = new_url,
                // idc about the others
                _ => (),
            }
        }
    }

    pub fn has_bytes(&self) -> bool {
        match self {
            Self::Uploaded { bytes, .. } => bytes.len() > 0,
            _ => false,
        }
    }

    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        match self {
            Self::Uploaded {
                bytes: my_bytes, ..
            } => *my_bytes = bytes,
            _ => (),
        }
    }
}

fn vec_to_base64<S>(bytes: &Vec<u8>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let b64 = STANDARD.encode(bytes);
    s.serialize_str(&b64)
}

fn bytes_from_json<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer).map_err(D::Error::custom)?;

    match v {
        serde_json::Value::String(s) => {
            // Try base64 first
            let b64 = base64::engine::general_purpose::STANDARD.decode(&s);
            if let Ok(bytes) = b64 {
                return Ok(bytes);
            }

            // Try PostgreSQL hex bytea: may begin with "\\x" or "x" or "0x"
            let mut hex = s.as_str();
            if hex.starts_with("\\x") {
                hex = &hex[2..];
            } else if hex.starts_with("0x") {
                hex = &hex[2..];
            } else if hex.starts_with('x') {
                hex = &hex[1..];
            }

            // decode hex manually
            if hex.len() % 2 == 0 {
                let mut out = Vec::with_capacity(hex.len() / 2);
                for i in (0..hex.len()).step_by(2) {
                    let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                        .map_err(|e| D::Error::custom(e.to_string()))?;
                    out.push(byte);
                }
                return Ok(out);
            }

            Err(D::Error::custom(
                "failed to decode image bytes string as base64 or hex",
            ))
        }

        serde_json::Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for v in arr {
                let n = v
                    .as_u64()
                    .ok_or_else(|| D::Error::custom("invalid byte array element"))?;
                out.push(n as u8);
            }
            Ok(out)
        }
        _ => Err(D::Error::custom("invalid bytes value")),
    }
}

#[cfg(feature = "server")]
pub type TImages = sqlx::types::Json<Vec<Image>>;
#[cfg(not(feature = "server"))]
pub type TImages = Vec<Image>;
