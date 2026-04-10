# Contribuindo com o SYT Playlist Downloader

Obrigado pelo interesse em contribuir! Abaixo estão as diretrizes para manter o projeto organizado.

## Pré-requisitos

- [Node.js](https://nodejs.org) (v18+)
- [Rust](https://www.rust-lang.org/tools/install)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)
- [ffmpeg](https://ffmpeg.org/download.html)

## Rodando em modo de desenvolvimento

```bash
# Instalar dependências
npm install

# Iniciar em modo dev
npm run tauri dev
```

## Gerando o build

```bash
npm run tauri build
```

Os instaladores ficam em `src-tauri/target/release/bundle/`.

## Como contribuir

1. Faça um **fork** do repositório
2. Crie uma branch para sua feature ou correção
   ```bash
   git checkout -b feat/minha-feature
   ```
3. Faça as alterações e commite seguindo o padrão [Conventional Commits](https://www.conventionalcommits.org/pt-br/v1.0.0/)
   ```
   feat: adiciona suporte a playlists privadas
   fix: corrige erro ao cancelar download
   refactor: melhora tratamento de erros no Spotify
   ```
4. Abra um **Pull Request** descrevendo o que foi alterado e o motivo

## Estrutura do projeto

```
syt-playlist-downloader/
├── src/                  # Frontend (React + TypeScript)
│   ├── components/       # Componentes da interface
│   └── hooks/            # Hooks personalizados
├── src-tauri/            # Backend (Rust + Tauri)
│   └── src/
│       └── lib.rs        # Lógica principal de download
└── public/               # Arquivos estáticos
```

## Reportando bugs

Abra uma [Issue](https://github.com/Ton-Chyod-s/syt-playlist-downloader/issues) descrevendo:

- O que aconteceu
- O que era esperado
- Passos para reproduzir
- Versão do sistema operacional
