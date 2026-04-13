pub fn validate_output_dir(dir: &str) -> Result<String, String> {
    if dir.trim().is_empty() {
        return Err("Pasta de destino não pode ser vazia.".into());
    }
    if dir.contains("..") {
        return Err("Pasta de destino inválida: não é permitido usar '..'.".into());
    }
    let path = std::path::Path::new(dir);
    if !path.is_absolute() {
        return Err("Pasta de destino deve ser um caminho absoluto.".into());
    }
    if !path.exists() {
        return Err("Pasta de destino não encontrada. Verifique se o caminho existe.".into());
    }
    if !path.is_dir() {
        return Err("O caminho informado não é uma pasta.".into());
    }
    Ok(dir.to_string())
}

pub fn validate_spotify_id(id: &str) -> Result<String, String> {
    if id.len() != 22 || !id.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("ID de playlist Spotify inválido.".into());
    }
    Ok(id.to_string())
}

pub fn validate_cookies_path(path: &str) -> Result<(), String> {
    let p = std::path::Path::new(path);
    if !p.exists() {
        return Err("Arquivo de cookies não encontrado.".into());
    }
    if !p.is_file() {
        return Err("O caminho de cookies deve apontar para um arquivo.".into());
    }
    if p.extension().and_then(|e| e.to_str()) != Some("txt") {
        return Err("O arquivo de cookies deve ter extensão .txt.".into());
    }
    Ok(())
}
