# Arquitetura

## Estrutura do projeto

```
syt-playlist-downloader/
├── src/                        # Frontend — React + TypeScript
│   ├── App.tsx                 # Componente raiz, orquestra o layout
│   ├── App.css                 # Estilos globais
│   ├── main.tsx                # Entry point React
│   ├── components/
│   │   ├── DownloadForm.tsx    # Formulário principal de download
│   │   ├── LogConsole.tsx      # Console de logs em tempo real
│   │   ├── HelpModal.tsx       # Modal de ajuda e sobre
│   │   └── UpdateChecker.tsx   # Banner e lógica de auto-update
│   └── hooks/
│       └── useDownload.ts      # Hook de estado e lógica de download
│
├── src-tauri/                  # Backend — Rust + Tauri
│   ├── src/
│   │   ├── lib.rs              # Comandos Tauri, download, Spotify
│   │   └── main.rs             # Entry point Rust
│   ├── tauri.conf.json         # Configuração do app Tauri
│   ├── Cargo.toml              # Dependências Rust
│   └── capabilities/          # Permissões Tauri
│
├── docs/                       # Documentação (este site)
├── public/                     # Assets estáticos
├── vite.config.ts              # Configuração do Vite
└── package.json
```

## Fluxo de dados

```
Usuário preenche formulário (DownloadForm)
              │
              ▼
       useDownload.startDownload()
              │
              ▼
    invoke("download_playlist")  ──► Rust: lib.rs
              │                              │
              │                    valida inputs
              │                    spawna yt-dlp
              │                    captura stdout/stderr
              │                              │
              │◄──── emit("download-progress") ◄────┘
              │
    listen("download-progress")
              │
              ▼
       handleLog(msg)
       ├── parseia progresso ([download] X%)
       ├── parseia item da playlist (item N of M)
       ├── converte para mensagem amigável
       └── atualiza logs + barra de progresso
```

## Componentes principais

### `useDownload` (hook)

Centraliza todo o estado e lógica de download:

- **Estado:** `logs`, `loading`, `done`, `progress`, `playlistInfo`
- **`startDownload()`** — registra listener de eventos Tauri e invoca o comando Rust
- **`cancelDownload()`** — invoca `cancel_download` no Rust
- **`handleLog()`** — parseia cada linha do yt-dlp e atualiza estado

O progresso é calculado combinando o item atual da playlist com o progresso do arquivo individual:

```
progress = ((current - 1) + fileProgress / 100) / total * 100
```

### `lib.rs` (Rust)

Contém três funções principais:

#### `download_playlist` (comando Tauri)

Entry point chamado pelo frontend. Valida inputs, detecta o modo (Spotify, Original ou Playlist) e despacha para a função correta.

#### `download_spotify`

1. Valida o ID da playlist Spotify
2. Tenta buscar as faixas via **embed público** (sem auth)
3. Se falhar, autentica via `sp_dc` → token Bearer → API oficial
4. Gera um arquivo batch temporário com as queries `ytsearch1:`
5. Spawna o yt-dlp com o batch file

#### `spawn_download`

Abstração para spawnar qualquer processo externo (`yt-dlp`):

- Cria o processo sem janela no Windows (`CREATE_NO_WINDOW`)
- Spawna threads para ler `stdout` e `stderr` em paralelo
- Emite eventos `download-progress` para o frontend
- Limpa arquivos temporários após conclusão

### `UpdateChecker`

Usa `@tauri-apps/plugin-updater` para verificar novas versões na inicialização. O endpoint de verificação aponta para o `latest.json` do GitHub Releases. O progresso do download da atualização também é exibido em um banner.

## Segurança

- **Validação de inputs no Rust:** o caminho de destino não pode conter `..`, deve ser absoluto e a pasta deve existir
- **Validação do cookies.txt:** verifica se o arquivo existe, é um arquivo e tem extensão `.txt`
- **Validação do ID Spotify:** deve ter exatamente 22 caracteres alfanuméricos
- **Sem janela no Windows:** o processo yt-dlp é criado com `CREATE_NO_WINDOW` para não exibir janelas do console
- **CSP configurada:** `default-src 'self'` com exceções mínimas para IPC do Tauri

## Dependências Rust relevantes

| Crate | Uso |
|---|---|
| `tauri` | Framework desktop |
| `tauri-plugin-updater` | Auto-update |
| `tauri-plugin-dialog` | Seletor de arquivo/pasta |
| `tauri-plugin-opener` | Abrir URLs no navegador |
| `ureq` | Requisições HTTP (Spotify API) |
| `serde_json` | Parse de JSON da API Spotify |
| `base64` | Encode das credenciais Spotify |
