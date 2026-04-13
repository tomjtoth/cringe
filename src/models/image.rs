use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{de::Error as _, Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Image {
    UrlOnly(String),

    Uploaded {
        id: Option<i32>,
        user_id: Option<i32>,

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

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UrlOnly(url) => f.debug_tuple("UrlOnly").field(url).finish(),
            Self::Uploaded {
                id,
                user_id,
                position,
                prompt,
                url,
                bytes,
            } => {
                let bytes = bytes.as_ref().map(|bytes|
                        // keeping 2 digits from fractional part
                        (bytes.len() as f64 / 10.24).trunc() / 100.0);

                f.debug_struct("Uploaded")
                    .field("id", id)
                    .field("user_id", user_id)
                    .field("position", position)
                    .field("prompt", prompt)
                    .field("url", url)
                    .field("bytes [kiB]", &bytes)
                    .finish()
            }
        }
    }
}

impl Image {
    pub fn id(&self) -> &Option<i32> {
        match self {
            Self::Uploaded { id, .. } => id,
            _ => &None,
        }
    }

    pub fn user_id(&self) -> &Option<i32> {
        match self {
            Self::Uploaded { user_id, .. } => user_id,
            _ => &None,
        }
    }

    #[cfg(feature = "server")]
    pub fn convert(&mut self) -> anyhow::Result<()> {
        if let Self::Uploaded { bytes, .. } = self {
            if let Some(bytes) = bytes {
                let img = image::load_from_memory(bytes.as_slice())?;

                let mut converted = Vec::new();
                img.write_to(
                    &mut std::io::Cursor::new(&mut converted),
                    image::ImageFormat::Avif,
                )?;

                *bytes = converted;
            }
        }

        Ok(())
    }

    pub fn pos(&self) -> &Option<i16> {
        match self {
            Self::Uploaded { position, .. } => position,
            _ => &None,
        }
    }

    pub fn set_pos(&mut self, pos: Option<i16>) {
        if let Self::Uploaded { position, .. } = self {
            *position = pos;
        }
    }

    pub fn prompt(&self) -> &Option<String> {
        match self {
            Self::Uploaded { prompt, .. } => &prompt,
            _ => &None,
        }
    }

    pub fn set_prompt(&mut self, val: String) {
        if let Self::Uploaded { prompt, .. } = self {
            *prompt = if val == "" { None } else { Some(val) }
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
        if let Self::Uploaded { url: my_url, .. } = self {
            *my_url = url
        }
    }

    pub fn has_url(&self) -> bool {
        match self {
            Self::Uploaded { url, .. } => url.is_some(),
            _ => true,
        }
    }

    pub fn bytes(&self) -> &Option<Vec<u8>> {
        match self {
            Self::Uploaded { bytes, .. } => bytes,
            _ => &None,
        }
    }

    pub fn has_bytes(&self) -> bool {
        match self {
            Self::Uploaded { bytes, .. } => bytes.as_ref().map_or(false, |b| b.len() > 0),
            _ => false,
        }
    }

    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        if let Self::Uploaded {
            bytes: my_bytes, ..
        } = self
        {
            *my_bytes = Some(bytes)
        }
    }

    pub async fn set_bytes_resized(&mut self, _bytes: Vec<u8>) {
        #[cfg(target_arch = "wasm32")]
        {
            use futures::channel::oneshot;
            use js_sys::{Array, Uint8Array};
            use wasm_bindgen::{closure::Closure, JsCast, JsValue};
            use web_sys::{window, Blob, CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap};

            let uint8 = Uint8Array::from(_bytes.as_slice());

            let parts = Array::new();
            parts.push(&uint8.buffer());

            let blob = Blob::new_with_u8_array_sequence(&parts).unwrap();

            // Use ImageBitmap (no extra <img> element) and draw it to the canvas
            let window = window().unwrap();

            // create_image_bitmap returns a Promise, await it to get an ImageBitmap
            let bitmap_promise = window.create_image_bitmap_with_blob(&blob).unwrap();
            let bitmap_js = wasm_bindgen_futures::JsFuture::from(bitmap_promise)
                .await
                .unwrap();
            let img: ImageBitmap = bitmap_js.dyn_into().unwrap();

            let document = window.document().unwrap();

            let canvas = document
                .create_element("canvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();

            let (w, h) = {
                let max_size = 1920;

                let w = img.width();
                let h = img.height();

                let larger = w.max(h);

                if larger > max_size {
                    let ratio = max_size as f64 / larger as f64;

                    ((w as f64 * ratio) as u32, (h as f64 * ratio) as u32)
                } else {
                    (w, h)
                }
            };

            canvas.set_width(w);
            canvas.set_height(h);

            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            ctx.draw_image_with_image_bitmap_and_dw_and_dh(&img, 0.0, 0.0, w as f64, h as f64)
                .unwrap();

            // Convert to blob (JPEG compression) using callback -> oneshot channel

            let (tx, rx) = oneshot::channel();
            let mut tx = Some(tx);

            let cb = Closure::wrap(Box::new(move |blob: JsValue| {
                if let Some(tx) = tx.take() {
                    _ = tx.send(blob);
                }
            }) as Box<dyn FnMut(JsValue)>);

            // to_blob_with_type_and_quality takes (callback, mime_type, quality)
            canvas
                .to_blob_with_type_and_encoder_options(
                    cb.as_ref().unchecked_ref(),
                    "image/jpeg",
                    &JsValue::from_f64(0.95),
                )
                .unwrap();

            let blob_js = rx.await.unwrap();
            let blob: Blob = blob_js.dyn_into().unwrap();

            // Keep the callback alive until after the blob arrives, then release it.
            drop(cb);

            let array_buffer = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
                .await
                .unwrap();

            let u8_array = Uint8Array::new(&array_buffer);
            let res = u8_array.to_vec();
            dioxus::prelude::info!("res.len = {}", res.len());
            self.set_bytes(res);
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
