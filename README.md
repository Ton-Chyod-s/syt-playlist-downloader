# SYT Playlist Downloader

![License](https://img.shields.io/badge/license-MIT-green) ![Platform](https://img.shields.io/badge/platform-Windows%20x64-blue) ![Version](https://img.shields.io/github/v/release/Ton-Chyod-s/syt-playlist-downloader?color=orange)

Aplicativo desktop para baixar playlists e vídeos do YouTube e músicas do Spotify, construído com Tauri + React + TypeScript.

## Download

Baixe o instalador mais recente na página de [Releases](https://github.com/Ton-Chyod-s/syt-playlist-downloader/releases/latest):

| Arquivo | Tipo |
|---|---|
| `SYT.Playlist.Downloader_x.x.x_x64-setup.exe` | NSIS Installer ✅ Recomendado |
| `SYT.Playlist.Downloader_x.x.x_x64_en-US.msi` | MSI Installer |

> **Windows x64 only.**

## Atualização automática

O app verifica atualizações ao iniciar. Quando uma nova versão estiver disponível, um banner aparecerá com a opção de atualizar com um clique.

## Screenshots

<p align="center">
  <a href="docs/screenshot-main.png"><img src="docs/screenshot-main.png" alt="Tela principal" width="30%" /></a>
  <a href="docs/screenshot-download.png"><img src="docs/screenshot-download.png" alt="Download YouTube" width="30%" /></a>
  <a href="docs/screenshot-spotify.png"><img src="docs/screenshot-spotify.png" alt="Download Spotify" width="30%" /></a>
</p>

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

O build de release é feito automaticamente via GitHub Actions ao criar uma tag:

```bash
git tag v0.x.x
git push --tags
```

Para build local:

```bash
npm run tauri build
```

## Stack

- [Tauri v2](https://tauri.app)
- [React](https://react.dev)
- [TypeScript](https://www.typescriptlang.org)
- [Rust](https://www.rust-lang.org)
