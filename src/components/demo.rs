use base64::{engine::general_purpose, Engine};
use leptos::prelude::*;
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputFormat {
    Auto,
    Hex,
    Base64,
}

fn json_to_messagepack(json_str: &str) -> Result<String, String> {
    let json_value: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    let messagepack = rmp_serde::to_vec(&json_value)
        .map_err(|e| format!("Failed to serialize to MessagePack: {}", e))?;
    Ok(general_purpose::STANDARD.encode(&messagepack))
}

fn messagepack_to_json(encoded_str: &str, format: InputFormat) -> Result<String, String> {
    let messagepack = match format {
        InputFormat::Hex => {
            let normalized = normalize_hex_input(encoded_str);
            if normalized.is_empty() {
                return Err("Input is empty after normalization".to_string());
            }
            hex::decode(&normalized).map_err(|e| format!("Failed to decode Hex: {}", e))?
        }
        InputFormat::Base64 => general_purpose::STANDARD
            .decode(encoded_str.trim())
            .map_err(|e| format!("Failed to decode Base64: {}", e))?,
        InputFormat::Auto => {
            let normalized = normalize_hex_input(encoded_str);
            if is_hex(&normalized) {
                hex::decode(&normalized).map_err(|e| format!("Failed to decode Hex: {}", e))?
            } else {
                general_purpose::STANDARD
                    .decode(encoded_str.trim())
                    .map_err(|e| format!("Failed to decode Base64: {}", e))?
            }
        }
    };

    let json_value = msgpack_bytes_to_json_value(&messagepack)?;
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| format!("Failed to serialize to JSON: {}", e))
}

fn msgpack_bytes_to_json_value(bytes: &[u8]) -> Result<serde_json::Value, String> {
    let mut cursor = Cursor::new(bytes);
    let msgpack_value = rmpv::decode::read_value(&mut cursor)
        .map_err(|e| format!("Failed to deserialize MessagePack: {}", e))?;
    rmpv_to_json(msgpack_value)
}

fn rmpv_to_json(val: rmpv::Value) -> Result<serde_json::Value, String> {
    match val {
        rmpv::Value::Nil => Ok(serde_json::Value::Null),
        rmpv::Value::Boolean(b) => Ok(serde_json::Value::Bool(b)),
        rmpv::Value::Integer(i) => {
            if let Some(n) = i.as_i64() {
                Ok(serde_json::Value::Number(n.into()))
            } else if let Some(n) = i.as_u64() {
                Ok(serde_json::Value::Number(n.into()))
            } else {
                Err(format!("Integer value out of range: {}", i))
            }
        }
        rmpv::Value::F32(f) => serde_json::Number::from_f64(f as f64)
            .map(serde_json::Value::Number)
            .ok_or_else(|| format!("Float value not representable in JSON: {}", f)),
        rmpv::Value::F64(f) => serde_json::Number::from_f64(f)
            .map(serde_json::Value::Number)
            .ok_or_else(|| format!("Float value not representable in JSON: {}", f)),
        rmpv::Value::String(s) => match s.into_str() {
            Some(s) => Ok(serde_json::Value::String(s.to_owned())),
            None => Err("MessagePack string is not valid UTF-8".to_string()),
        },
        rmpv::Value::Binary(b) => Ok(serde_json::Value::String(hex::encode(b))),
        rmpv::Value::Array(arr) => {
            let items: Result<Vec<_>, _> = arr.into_iter().map(rmpv_to_json).collect();
            Ok(serde_json::Value::Array(items?))
        }
        rmpv::Value::Map(entries) => {
            let mut map = serde_json::Map::new();
            for (k, v) in entries {
                let key = rmpv_key_to_string(k)?;
                let value = rmpv_to_json(v)?;
                map.insert(key, value);
            }
            Ok(serde_json::Value::Object(map))
        }
        rmpv::Value::Ext(type_id, data) => {
            let mut map = serde_json::Map::new();
            map.insert(
                "type".to_string(),
                serde_json::Value::Number(type_id.into()),
            );
            map.insert(
                "data".to_string(),
                serde_json::Value::String(hex::encode(data)),
            );
            Ok(serde_json::Value::Object(map))
        }
    }
}

fn rmpv_key_to_string(key: rmpv::Value) -> Result<String, String> {
    match key {
        rmpv::Value::String(s) => s
            .into_str()
            .map(|s| s.to_owned())
            .ok_or_else(|| "Map key is not valid UTF-8".to_string()),
        rmpv::Value::Integer(i) => Ok(i.to_string()),
        rmpv::Value::Boolean(b) => Ok(b.to_string()),
        rmpv::Value::F32(f) => Ok(f.to_string()),
        rmpv::Value::F64(f) => Ok(f.to_string()),
        rmpv::Value::Binary(b) => Ok(hex::encode(b)),
        rmpv::Value::Nil => Ok("null".to_string()),
        other => Err(format!("Unsupported map key type: {:?}", other)),
    }
}

fn normalize_hex_input(s: &str) -> String {
    let trimmed = s.trim();
    let without_prefix = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);
    without_prefix
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.len() % 2 == 0 && s.chars().all(|c| c.is_ascii_hexdigit())
}

#[component]
pub fn Demo() -> impl IntoView {
    let (json_input, set_json_input) = signal(String::from("{\n  \"name\": \"shub\",\n  \"version\": 1,\n  \"entries\": [\n    {\"index\": 1, \"name\": \"ip0\", \"type\": \"u8\", \"value\": 192},\n    {\"index\": 5, \"name\": \"port\", \"type\": \"u16\", \"value\": 8883},\n    {\"index\": 11, \"name\": \"mhost\", \"type\": \"str\", \"value\": \"mqtt.local\"}\n  ]\n}"));
    let (msgpack_input, set_msgpack_input) = signal(String::new());
    let (json_output, set_json_output) = signal(String::new());
    let (msgpack_output, set_msgpack_output) = signal(String::new());
    let (error, set_error) = signal(String::new());
    let (format, set_format) = signal(InputFormat::Auto);

    let convert_to_msgpack = move |_| {
        match json_to_messagepack(&json_input.get()) {
            Ok(mp) => {
                set_msgpack_output.set(mp);
                set_error.set(String::new());
            }
            Err(e) => set_error.set(e),
        }
    };

    let convert_to_json = move |_| {
        match messagepack_to_json(&msgpack_input.get(), format.get()) {
            Ok(json) => {
                set_json_output.set(json);
                set_error.set(String::new());
            }
            Err(e) => set_error.set(e),
        }
    };

    view! {
        <section class="section demo">
            <div class="section-content">
                <div class="section-label">
                    <span class="pipe">"\u{2551}"</span>
                    <span class="label-text">"LIVE DEMO: MESSAGEPACK \u{2194} JSON"</span>
                    <span class="pipe">"\u{2551}"</span>
                </div>

                <div class="demo-panels">
                    // JSON -> MessagePack
                    <div class="demo-panel">
                        <div class="demo-panel-header">
                            <span class="pipe">"\u{2551}"</span>
                            <span class="panel-title">"JSON \u{2192} MessagePack"</span>
                            <span class="pipe">"\u{2551}"</span>
                        </div>
                        <div class="demo-panel-body">
                            <span class="pipe">"\u{2551}"</span>
                            <div class="panel-inner">
                                <div class="format-spacer"></div>
                                <textarea
                                    class="demo-textarea"
                                    placeholder="Paste JSON here..."
                                    prop:value=move || json_input.get()
                                    on:input=move |ev| {
                                        set_json_input.set(event_target_value(&ev));
                                    }
                                />
                                <button class="demo-btn" on:click=convert_to_msgpack>
                                    "[ CONVERT ]"
                                </button>
                                <textarea
                                    class="demo-textarea output"
                                    readonly
                                    placeholder="Base64 output..."
                                    prop:value=move || msgpack_output.get()
                                />
                            </div>
                            <span class="pipe">"\u{2551}"</span>
                        </div>
                    </div>

                    // MessagePack -> JSON
                    <div class="demo-panel">
                        <div class="demo-panel-header">
                            <span class="pipe">"\u{2551}"</span>
                            <span class="panel-title">"MessagePack \u{2192} JSON"</span>
                            <span class="pipe">"\u{2551}"</span>
                        </div>
                        <div class="demo-panel-body">
                            <span class="pipe">"\u{2551}"</span>
                            <div class="panel-inner">
                                <div class="format-select">
                                    <span class="format-label">"FORMAT:"</span>
                                    <button
                                        class=move || if format.get() == InputFormat::Auto { "format-btn active" } else { "format-btn" }
                                        on:click=move |_| set_format.set(InputFormat::Auto)
                                    >"Auto"</button>
                                    <button
                                        class=move || if format.get() == InputFormat::Hex { "format-btn active" } else { "format-btn" }
                                        on:click=move |_| set_format.set(InputFormat::Hex)
                                    >"Hex"</button>
                                    <button
                                        class=move || if format.get() == InputFormat::Base64 { "format-btn active" } else { "format-btn" }
                                        on:click=move |_| set_format.set(InputFormat::Base64)
                                    >"Base64"</button>
                                </div>
                                <textarea
                                    class="demo-textarea"
                                    placeholder="Paste MessagePack (hex or base64)..."
                                    prop:value=move || msgpack_input.get()
                                    on:input=move |ev| {
                                        set_msgpack_input.set(event_target_value(&ev));
                                    }
                                />
                                <button class="demo-btn" on:click=convert_to_json>
                                    "[ CONVERT ]"
                                </button>
                                <textarea
                                    class="demo-textarea output"
                                    readonly
                                    placeholder="JSON output..."
                                    prop:value=move || json_output.get()
                                />
                            </div>
                            <span class="pipe">"\u{2551}"</span>
                        </div>
                    </div>
                </div>

                // Error display
                {move || {
                    let err = error.get();
                    if err.is_empty() {
                        view! { <span /> }.into_any()
                    } else {
                        view! {
                            <div class="demo-error">
                                <span class="pipe">"\u{2551}"</span>
                                <span class="error-text">{format!("ERROR: {}", err)}</span>
                                <span class="pipe">"\u{2551}"</span>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
            <div class="section-border mid-border">
                <span class="corner tl">"\u{2560}"</span>
                <span class="line"></span>
                <span class="corner tr">"\u{2563}"</span>
            </div>
        </section>
    }
}
