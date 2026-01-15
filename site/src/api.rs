use api_types::resources::{CreateResource, Resource};
use api_types::responses::{ApiListResponse, ApiResponse};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

const API_BASE: &str = "http://localhost:3000";

pub async fn list_resources() -> Result<Vec<Resource>, String> {
    let response = fetch_json::<ApiListResponse<Resource>>(
        &format!("{API_BASE}/api/v1/resources"),
        "GET",
        None,
    )
    .await?;
    Ok(response.data)
}

pub async fn create_resource(
    name: String,
    description: Option<String>,
) -> Result<Resource, String> {
    let body = serde_json::to_string(&CreateResource { name, description })
        .map_err(|error| error.to_string())?;
    let response = fetch_json::<ApiResponse<Resource>>(
        &format!("{API_BASE}/api/v1/resources"),
        "POST",
        Some(body),
    )
    .await?;
    Ok(response.data)
}

pub async fn delete_resource(id: &str) -> Result<(), String> {
    fetch(&format!("{API_BASE}/api/v1/resources/{id}"), "DELETE", None).await?;
    Ok(())
}

async fn fetch_json<T: serde::de::DeserializeOwned>(
    url: &str,
    method: &str,
    body: Option<String>,
) -> Result<T, String> {
    let text = fetch(url, method, body).await?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

async fn fetch(url: &str, method: &str, body: Option<String>) -> Result<String, String> {
    let opts = RequestInit::new();
    opts.set_method(method);
    opts.set_mode(RequestMode::Cors);

    if let Some(body_str) = body {
        opts.set_body(&wasm_bindgen::JsValue::from_str(&body_str));
    }

    let request =
        Request::new_with_str_and_init(url, &opts).map_err(|error| format!("{error:?}"))?;

    request
        .headers()
        .set("Content-Type", "application/json")
        .map_err(|error| format!("{error:?}"))?;

    let window = web_sys::window().ok_or("No window")?;
    let response_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|error| format!("{error:?}"))?;

    let response: Response = response_value
        .dyn_into()
        .map_err(|error| format!("{error:?}"))?;

    if !response.ok() {
        return Err(format!("HTTP {}", response.status()));
    }

    let text = JsFuture::from(response.text().map_err(|error| format!("{error:?}"))?)
        .await
        .map_err(|error| format!("{error:?}"))?;

    text.as_string().ok_or("Response not a string".to_string())
}
