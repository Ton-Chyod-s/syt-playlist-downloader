# SYT Playlist Downloader

Aplicativo desktop para baixar playlists e vídeos do YouTube e músicas do Spotify, construído com Tauri + React + TypeScript.

## Funcionalidades

- **Playlist** — baixa playlist do YouTube em MP4 1080p (H.264 + MP3), compatível com qualquer dispositivo
- **Original** — baixa em melhor qualidade disponível (4K, VP9, AV1…), sem restrição de codec ou resolução. Funciona para vídeos individuais ou playlists inteiras
- **Spotify** — busca as músicas da playlist no YouTube Music e baixa como MP3 de alta qualidade

## Dependências

| Ferramenta | Função |
|---|---|
| [yt-dlp](https://github.com/yt-dlp/yt-dlp) | Download de vídeos do YouTube |
| [ffmpeg](https://ffmpeg.org/download.html) | Conversão e mesclagem de áudio/vídeo |
| [Node.js](https://nodejs.org) | Necessário para yt-dlp resolver o JS do YouTube |

Todos devem estar instalados e disponíveis no PATH.

## Como usar

1. Escolha o modo: **Playlist**, **Original** ou cole um link do Spotify
2. Cole a URL do YouTube ou Spotify
3. Escolha a pasta de destino
4. Clique em **Baixar**

### Spotify

Cole um link `open.spotify.com/playlist/...` e informe o cookie `sp_dc`:

1. Acesse [open.spotify.com](https://open.spotify.com) logado
2. Pressione `F12` → Application → Cookies → `open.spotify.com`
3. Copie o valor do cookie `sp_dc` e cole no campo correspondente

### YouTube privado (cookies.txt)

Instale a extensão **Get cookies.txt LOCALLY** no Chrome ou Firefox, acesse o YouTube logado e exporte o arquivo `cookies.txt`.

## Desenvolvimento

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

## Stack

- [Tauri v2](https://tauri.app)
- [React](https://react.dev)
- [TypeScript](https://www.typescriptlang.org)
- [Rust](https://www.rust-lang.org)
