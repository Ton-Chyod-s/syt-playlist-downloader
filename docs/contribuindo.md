# Contribuindo

Obrigado pelo interesse em contribuir com o SYT Playlist Downloader!

## Como contribuir

### 1. Fork e clone

```bash
# Faça um fork no GitHub e clone seu fork
git clone https://github.com/SEU_USUARIO/syt-playlist-downloader.git
cd syt-playlist-downloader
```

### 2. Crie uma branch

```bash
git checkout -b feat/minha-feature
# ou
git checkout -b fix/nome-do-bug
```

### 3. Faça as alterações

Consulte o [Guia de Desenvolvimento](desenvolvimento.md) para configurar o ambiente local.

### 4. Commit seguindo Conventional Commits

```bash
git commit -m "feat: adiciona suporte a playlists privadas"
git commit -m "fix: corrige erro ao cancelar download"
git commit -m "refactor: melhora tratamento de erros no Spotify"
git commit -m "docs: atualiza guia de instalação"
```

| Prefixo | Quando usar |
|---|---|
| `feat` | Nova funcionalidade |
| `fix` | Correção de bug |
| `refactor` | Refatoração sem mudança de comportamento |
| `docs` | Documentação |
| `chore` | Tarefas de manutenção (CI, dependências) |

### 5. Abra um Pull Request

Descreva no PR:
- O que foi alterado e por quê
- Como testar a mudança
- Screenshots ou vídeos (se relevante para UI)

## Reportando bugs

Abra uma [Issue](https://github.com/Ton-Chyod-s/syt-playlist-downloader/issues) com:

- **O que aconteceu** — comportamento observado
- **O que era esperado** — comportamento esperado
- **Passos para reproduzir** — detalhado, passo a passo
- **Versão do app** — visível no título da janela ou na página de Releases
- **Sistema operacional** — versão do Windows
- **Log de erros** — copie o conteúdo do console de log do app

## Sugerindo melhorias

Abra uma Issue com o label `enhancement` descrevendo:

- Qual problema a melhoria resolve
- Como você imagina a solução
- Exemplos ou referências (se houver)

## Diretrizes de código

- **Frontend (TypeScript/React):** prefira componentes funcionais e hooks; evite estado global desnecessário
- **Backend (Rust):** siga as convenções do Rust (`cargo clippy` sem warnings); valide todos os inputs antes de usar
- **Mensagens de erro:** devem ser claras e orientadas ao usuário (em português)
- **Sem dependências desnecessárias:** avalie o impacto no tamanho do binário antes de adicionar crates

## Estrutura de contribuição

```
syt-playlist-downloader/
├── src/           # Contribuições de frontend vão aqui
├── src-tauri/     # Contribuições de backend (Rust) vão aqui
└── docs/          # Contribuições de documentação vão aqui
```
