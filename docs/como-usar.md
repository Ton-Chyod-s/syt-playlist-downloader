# Como Usar

## Visão geral da interface

A janela principal é dividida em três áreas:

- **Cabeçalho** — nome do app, subtítulo e botão de ajuda
- **Formulário** — campos de URL, pasta, cookies e configurações
- **Console de log** — exibe o progresso do download em tempo real

## Baixando do YouTube

### 1. Escolha o modo de download

Use os botões **Playlist** ou **Original** no topo do formulário:

- **Playlist** — recomendado para playlists; gera MP4 1080p compatível com qualquer dispositivo
- **Original** — melhor qualidade disponível (4K, VP9, AV1); funciona para vídeos individuais também

### 2. Cole a URL

Cole a URL da playlist ou vídeo do YouTube no campo **URL**:

```
https://youtube.com/playlist?list=PL...
https://youtube.com/watch?v=...
```

### 3. Escolha a pasta de destino

O campo **Pasta de destino** é preenchido automaticamente com a pasta de downloads do sistema. Clique em **📁** para escolher outra pasta.

### 4. (Opcional) Defina um limite

No campo **Limite**, informe um número para baixar apenas os primeiros N itens da playlist. Deixe vazio para baixar tudo.

### 5. (Opcional) Cookies para playlists privadas

Se a playlist for privada ou precisar de login, forneça o arquivo `cookies.txt`. Veja como exportar em [Dependências → Cookies do YouTube](dependencias.md#cookies-do-youtube).

### 6. Clique em Baixar

O botão muda para **Baixando...** e o console exibe o progresso. A barra de progresso mostra o andamento geral da playlist e o progresso individual de cada arquivo.

---

## Baixando do Spotify

Ao colar uma URL do Spotify, o formulário muda automaticamente para o modo Spotify.

### URLs suportadas

Apenas playlists públicas ou privadas (com cookie):

```
https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M
```

> URLs de músicas, álbuns ou artistas não são suportadas.

### Cookie sp_dc

O campo **Cookie `sp_dc`** é exibido quando uma URL do Spotify é detectada. Para obter o cookie:

1. Acesse [open.spotify.com](https://open.spotify.com) e faça login
2. Pressione `F12` para abrir o DevTools
3. Vá em **Application → Cookies → open.spotify.com**
4. Localize o cookie `sp_dc` e copie seu valor
5. Cole no campo correspondente

O valor é salvo na sessão e não precisa ser inserido novamente enquanto o app estiver aberto.

### Como o Spotify é baixado

O app:
1. Autentica na API do Spotify usando o cookie `sp_dc`
2. Busca a lista de músicas da playlist
3. Para cada música, pesquisa no YouTube Music (`ytsearch1: artista - título`)
4. Baixa o áudio em melhor qualidade e converte para **MP3**

---

## Cancelando o download

Clique no botão **⛔ Cancelar** durante o download para interromper o processo imediatamente. O processo `yt-dlp` em execução é encerrado forçadamente.

---

## Console de log

O console exibe mensagens simplificadas:

| Emoji | Significado |
|---|---|
| 🔍 | Buscando informações da playlist/vídeo |
| 🌐 | Carregando página do YouTube |
| ⚙️ | Processando (JS challenge, etc.) |
| 📥 | Preparando download |
| 📦 | Baixando item X de Y |
| 🔄 | Convertendo/mesclando com ffmpeg |
| ✅ | Download concluído com sucesso |
| ❌ | Erro encontrado |
| ⛔ | Download cancelado |

Clique em **🗑️ Limpar** para apagar o histórico do console.
