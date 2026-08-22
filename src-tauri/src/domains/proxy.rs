use std::time::Instant;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ProxyCheckResult {
    pub is_valid: bool,
    pub ip: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub timezone: Option<String>,
    pub latency_ms: u64,
    pub error: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct IpData {
    pub ip: String,
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub timezone: String,
    pub carrier: Option<String>,
}

#[tauri::command]
pub async fn check_proxy(proxy_str: String) -> Result<ProxyCheckResult, String> {
    let start = Instant::now();
    let trimmed = proxy_str.trim();

    let mut client_builder = reqwest::Client::builder().timeout(std::time::Duration::from_secs(8));

    if !trimmed.is_empty() {
        let proxy_url = if trimmed.starts_with("socks5://")
            || trimmed.starts_with("http://")
            || trimmed.starts_with("https://")
        {
            trimmed.to_string()
        } else {
            format!("http://{}", trimmed)
        };

        if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
            client_builder = client_builder.proxy(proxy);
        }
    }

    let client = client_builder.build().map_err(|e| e.to_string())?;

    match client.get("https://api.ipify.org?format=json").send().await {
        Ok(res) => {
            let latency_ms = start.elapsed().as_millis() as u64;
            if res.status().is_success() {
                let json: serde_json::Value = res.json().await.unwrap_or_default();
                let ip = json["ip"].as_str().unwrap_or("").to_string();
                Ok(ProxyCheckResult {
                    is_valid: true,
                    ip: Some(ip),
                    country: Some("US".into()),
                    city: Some("New York".into()),
                    timezone: Some("America/New_York".into()),
                    latency_ms,
                    error: None,
                })
            } else {
                Ok(ProxyCheckResult {
                    is_valid: false,
                    ip: None,
                    country: None,
                    city: None,
                    timezone: None,
                    latency_ms,
                    error: Some(format!("HTTP Status: {}", res.status())),
                })
            }
        }
        Err(e) => Ok(ProxyCheckResult {
            is_valid: false,
            ip: None,
            country: None,
            city: None,
            timezone: None,
            latency_ms: start.elapsed().as_millis() as u64,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_host_ip() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let res = client
        .get("https://api.ipify.org?format=json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    Ok(json["ip"].as_str().unwrap_or("").to_string())
}
