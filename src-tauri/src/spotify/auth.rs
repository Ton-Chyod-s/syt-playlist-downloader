use base64::{Engine as _, engine::general_purpose::STANDARD as B64};

pub fn get_token_from_sp_dc(sp_dc: &str) -> Result<String, String> {
    let cookie = format!("sp_dc={}", sp_dc);

    let resp = ureq::get("https://open.spotify.com/get_access_token?reason=transport&productType=web_player")
        .set("Cookie", &cookie)
        .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36")
        .set("Referer", "https://open.spotify.com/")
        .call()
        .map_err(|e| match e {
            ureq::Error::Status(code, _) => format!("Erro ao obter token via sp_dc (HTTP {}).", code),
            _ => "Erro de rede ao obter token sp_dc.".into(),
        })?;

    let json: serde_json::Value = resp
        .into_json()
        .map_err(|e| format!("Erro ao ler resposta do token sp_dc: {}", e))?;

    if json["isAnonymous"].as_bool().unwrap_or(true) {
        return Err(
            "Cookie sp_dc inválido ou expirado. \
            Abra open.spotify.com, faça login, pressione F12 → Application → Cookies → open.spotify.com, \
            copie o valor do cookie 'sp_dc' e cole aqui."
            .into(),
        );
    }

    json["accessToken"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Campo 'accessToken' não encontrado: {}", json))
}

pub fn get_spotify_token(client_id: &str, client_secret: &str) -> Result<String, String> {
    let creds = B64.encode(format!("{}:{}", client_id, client_secret));

    let resp = ureq::post("https://accounts.spotify.com/api/token")
        .set("Authorization", &format!("Basic {}", creds))
        .send_form(&[("grant_type", "client_credentials")])
        .map_err(|e| format!("Erro ao autenticar no Spotify: {}", e))?;

    let json: serde_json::Value = resp
        .into_json()
        .map_err(|e| format!("Erro ao ler token: {}", e))?;

    json["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Token não encontrado: {}", json))
}
