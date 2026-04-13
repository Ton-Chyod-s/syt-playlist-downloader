# Modos de Download

O SYT Playlist Downloader oferece três modos distintos, cada um otimizado para um caso de uso específico.

---

## Playlist

**Formato de saída:** MP4 (H.264 + MP3, até 1080p)

Ideal para quem quer compatibilidade máxima — o vídeo gerado toca em qualquer dispositivo, TV, celular ou player de mídia sem precisar de codecs especiais.

### Comportamento

- Seleciona o melhor vídeo disponível com codec **H.264 (avc)** em até **1080p**, formato `.mp4`
- Mescla com o melhor áudio disponível usando **ffmpeg**
- O áudio é re-encodado para **MP3** com qualidade `-q:a 2` (alta qualidade, ~190 kbps VBR)
- Compatível com o padrão do Multimídia mp5 e similares

### Nomenclatura dos arquivos

```
001 - Nome do Vídeo.mp4
002 - Outro Vídeo.mp4
```

### Comando yt-dlp equivalente

```bash
yt-dlp \
  -f "bestvideo[vcodec^=avc][height<=1080][ext=mp4]+bestaudio/bestvideo[vcodec^=avc][height<=1080]+bestaudio/best[height<=1080]" \
  --merge-output-format mp4 \
  --postprocessor-args "ffmpeg:-c:v copy -c:a libmp3lame -q:a 2" \
  -N 4 --concurrent-fragments 4 \
  -o "%(playlist_index)s - %(title)s.%(ext)s" \
  "URL_DA_PLAYLIST"
```

---

## Original

**Formato de saída:** MP4 (melhor qualidade disponível — até 4K, VP9, AV1)

Ideal para arquivar vídeos com a máxima fidelidade. Não há restrição de codec ou resolução.

### Comportamento

- Seleciona o melhor vídeo + melhor áudio disponíveis (`bestvideo+bestaudio`)
- Mescla tudo em um container **MP4** via ffmpeg
- Suporta vídeos individuais (`watch?v=...`) além de playlists

### Nomenclatura dos arquivos

```
001. Nome do Vídeo.mp4
002. Outro Vídeo.mp4
```

> O separador muda quando a playlist possui índice.

### Comando yt-dlp equivalente

```bash
yt-dlp \
  -f "bestvideo+bestaudio/best" \
  --merge-output-format mp4 \
  -N 4 --concurrent-fragments 4 \
  -o "%(playlist_index)s%(playlist_index&. - |)s%(title)s.%(ext)s" \
  "URL"
```

---

## Spotify

**Formato de saída:** MP3 (alta qualidade, ~320 kbps)

Baixa as músicas de uma playlist do Spotify buscando cada faixa no YouTube Music.

### Fluxo completo

```
Spotify Playlist URL
       │
       ▼
  Autenticar via sp_dc ──► Listar faixas (artista + título)
       │
       ▼
  Para cada faixa: ytsearch1:{artista} - {título}
       │
       ▼
  yt-dlp baixa o melhor áudio ──► ffmpeg converte para MP3
       │
       ▼
  {título}.mp3 na pasta de destino
```

### Autenticação

A autenticação é feita via cookie `sp_dc`. O app tenta primeiro ler a playlist via **embed público** do Spotify (sem autenticação). Se falhar (ex: playlist privada), usa o `sp_dc` para obter um token Bearer e acessa a **API oficial do Spotify**.

### Nomenclatura dos arquivos

```
Nome da Música.mp3
```

### Limitações

- Apenas playlists (`/playlist/...`) são suportadas
- A correspondência com o YouTube é baseada em texto — pode haver variações para músicas menos populares
- O cookie `sp_dc` expira eventualmente e precisará ser renovado

### Comando yt-dlp equivalente

```bash
yt-dlp \
  --default-search ytsearch \
  -f "bestaudio[ext=webm]/bestaudio/bestaudio[ext=m4a]/bestaudio" \
  --extract-audio --audio-format mp3 --audio-quality 0 \
  --ignore-errors \
  -N 4 --newline \
  -o "%(title)s.%(ext)s" \
  --batch-file queries.txt   # contém "ytsearch1: artista - titulo" por linha
```

---

## Comparativo rápido

| | Playlist | Original | Spotify |
|---|:---:|:---:|:---:|
| Fonte | YouTube | YouTube | Spotify → YouTube |
| Formato | MP4 | MP4 | MP3 |
| Resolução máx. | 1080p | 4K+ | — |
| Codec vídeo | H.264 | Qualquer | — |
| Codec áudio | MP3 | Qualquer | MP3 |
| Vídeos individuais | Sim | Sim | Não |
| Playlists | Sim | Sim | Sim |
| Requer login | Não* | Não* | Sim (sp_dc) |

*Playlists privadas requerem `cookies.txt`.
