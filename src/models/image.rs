use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{de::Error as _, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Image {
    UrlOnly(String),

    Uploaded {
        id: Option<i32>,

        #[serde(default)]
        position: Option<i16>,
        prompt: Option<String>,

        url: Option<String>,
        #[serde(
            default,
            serialize_with = "option_vec_to_base64",
            deserialize_with = "option_bytes_from_json"
        )]
        bytes: Option<Vec<u8>>,
    },
}

impl Image {
    pub fn id(&self) -> Option<i32> {
        match self {
            Self::UrlOnly(_) => None,
            Self::Uploaded { id, .. } => id.clone(),
        }
    }

    pub fn set_id(&mut self, id: Option<i32>) {
        let other_id = id;

        match self {
            Self::UrlOnly(_) => (),
            Self::Uploaded { id, .. } => *id = other_id,
        }
    }

    pub fn pos(&self) -> &Option<i16> {
        match self {
            Self::UrlOnly(_) => &None,
            Self::Uploaded { position, .. } => position,
        }
    }

    pub fn set_pos(&mut self, pos: Option<i16>) {
        match self {
            Self::UrlOnly(_) => (),
            Self::Uploaded { position, .. } => {
                *position = pos;
            }
        };
    }

    pub fn prompt(&self) -> Option<&str> {
        match self {
            Self::UrlOnly(_) => None,
            Self::Uploaded { prompt, .. } => prompt.as_deref(),
        }
    }

    pub fn set_prompt(&mut self, val: String) {
        match self {
            Self::UrlOnly(_) => (),
            Self::Uploaded { prompt, .. } => *prompt = if val == "" { None } else { Some(val) },
        }
    }

    pub fn src(&self) -> String {
        match self {
            Self::UrlOnly(src) => src.clone(),
            Self::Uploaded { url, bytes, .. } => {
                if let Some(u) = url {
                    return u.clone();
                }

                let encoded = bytes
                    .as_ref()
                    .map(|b| STANDARD.encode(b))
                    .unwrap_or_default();

                format!("data:image/avif;base64,{}", encoded)
            }
        }
    }

    pub fn set_url(&mut self, url: Option<String>) {
        match self {
            Self::Uploaded { url: my_url, .. } => *my_url = url,
            // idc about the others
            _ => (),
        }
    }

    pub fn has_url(&self) -> bool {
        match self {
            Self::Uploaded { url, .. } => url.is_some(),
            _ => false,
        }
    }

    pub fn has_bytes(&self) -> bool {
        match self {
            Self::Uploaded { bytes, .. } => bytes.as_ref().map_or(false, |b| b.len() > 0),
            _ => false,
        }
    }

    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        match self {
            Self::Uploaded {
                bytes: my_bytes, ..
            } => *my_bytes = Some(bytes),
            _ => (),
        }
    }
}

fn option_vec_to_base64<S>(bytes: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match bytes {
        Some(b) => {
            let b64 = STANDARD.encode(b);
            s.serialize_str(&b64)
        }
        None => s.serialize_none(),
    }
}

fn option_bytes_from_json<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = serde_json::Value::deserialize(deserializer).map_err(D::Error::custom)?;

    match v {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(s) => {
            // Try base64 first
            let b64 = STANDARD.decode(&s);
            if let Ok(bytes) = b64 {
                return Ok(Some(bytes));
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
                return Ok(Some(out));
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
            Ok(Some(out))
        }
        _ => Err(D::Error::custom("invalid bytes value")),
    }
}

#[cfg(feature = "server")]
pub type TImages = sqlx::types::Json<Vec<Image>>;
#[cfg(not(feature = "server"))]
pub type TImages = Vec<Image>;
