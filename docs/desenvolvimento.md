# Ambiente de Desenvolvimento

## Pré-requisitos

| Ferramenta | Versão mínima | Link |
|---|---|---|
| Node.js | 18+ | [nodejs.org](https://nodejs.org) |
| Rust | stable | [rustup.rs](https://rustup.rs) |
| Tauri CLI | 2.x | via `cargo install tauri-cli` |
| yt-dlp | qualquer | [github.com/yt-dlp](https://github.com/yt-dlp/yt-dlp) |
| ffmpeg | qualquer | [ffmpeg.org](https://ffmpeg.org) |

## Configurando o ambiente

### 1. Clone o repositório

```bash
git clone https://github.com/Ton-Chyod-s/syt-playlist-downloader.git
cd syt-playlist-downloader
```

### 2. Instale as dependências Node

```bash
npm install
```

### 3. Instale o Rust e o target Windows

```bash
# Instalar Rust via rustup (se ainda não tiver)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Adicionar target para cross-compile (se necessário)
rustup target add x86_64-pc-windows-msvc
```

## Rodando em modo de desenvolvimento

```bash
npm run tauri dev
```

Isso inicia o servidor Vite (frontend) e compila o backend Rust em modo debug. O app abre automaticamente com hot-reload ativo para o frontend.

> Alterações no Rust (`src-tauri/src/`) reiniciam o processo automaticamente.

## Build de produção

### Build local

```bash
npm run tauri build
```

Os instaladores gerados ficam em:

```
src-tauri/target/release/bundle/
├── nsis/
│   └── SYT.Playlist.Downloader_x64-setup.exe
└── msi/
    └── SYT.Playlist.Downloader_x64_en-US.msi
```

### Build via GitHub Actions

O build de release é disparado automaticamente ao criar e enviar uma tag:

```bash
git tag v0.x.x
git push --tags
```

O workflow `.github/workflows/release.yml` compila o projeto no GitHub Actions, assina os instaladores com a chave de release e publica os artefatos na página de Releases.

## Scripts disponíveis

| Comando | Descrição |
|---|---|
| `npm run dev` | Inicia apenas o servidor Vite (frontend) |
| `npm run build` | Build do frontend para a pasta `dist/` |
| `npm run tauri dev` | Inicia app completo em modo dev |
| `npm run tauri build` | Gera instaladores de produção |

## Assinatura (updater)

O auto-updater usa assinatura com Minisign. A chave pública está em `tauri.conf.json` e a privada é usada apenas no CI. Para gerar um novo par de chaves:

```bash
npm run tauri signer generate -- -w ./tauri-signing-key.key
```

> Nunca commite a chave privada (`.key`) no repositório.
