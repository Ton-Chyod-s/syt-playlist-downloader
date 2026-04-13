# Dependências

O SYT Playlist Downloader depende de três ferramentas externas que precisam estar instaladas e disponíveis no **PATH do sistema**.

---

## yt-dlp

**Função:** Realiza o download de vídeos e áudios do YouTube (e centenas de outros sites).

### Instalação

**Opção 1 — winget (recomendado)**

```bash
winget install yt-dlp.yt-dlp
```

**Opção 2 — Download direto**

Baixe o executável em [github.com/yt-dlp/yt-dlp/releases](https://github.com/yt-dlp/yt-dlp/releases) e coloque em uma pasta que esteja no PATH (ex: `C:\Windows\System32` ou `C:\tools\`).

### Verificar instalação

```bash
yt-dlp --version
```

### Manter atualizado

```bash
yt-dlp -U
```

> O yt-dlp é atualizado frequentemente para contornar mudanças do YouTube. Mantenha-o atualizado se tiver problemas de download.

---

## ffmpeg

**Função:** Mescla streams de vídeo e áudio e realiza conversões de formato.

### Instalação

**Opção 1 — winget**

```bash
winget install Gyan.FFmpeg
```

**Opção 2 — Download manual**

1. Acesse [ffmpeg.org/download.html](https://ffmpeg.org/download.html) → Windows builds
2. Baixe a versão **essentials** ou **full**
3. Extraia e adicione a pasta `bin/` ao PATH do sistema

### Verificar instalação

```bash
ffmpeg -version
```

---

## Node.js

**Função:** O yt-dlp usa Node.js para resolver os desafios JavaScript do YouTube (necessário para evitar erros de extração).

### Instalação

Baixe o instalador LTS em [nodejs.org](https://nodejs.org) e siga as instruções. O instalador adiciona o Node.js ao PATH automaticamente.

**Ou via winget:**

```bash
winget install OpenJS.NodeJS.LTS
```

### Verificar instalação

```bash
node --version
```

> Versão mínima recomendada: **v18**.

---

## Adicionando ao PATH

Se uma das ferramentas não for reconhecida no terminal, adicione ao PATH manualmente:

1. Pressione `Win + R` → digite `sysdm.cpl` → Enter
2. Vá em **Avançado → Variáveis de Ambiente**
3. Em **Variáveis do sistema**, selecione `Path` → **Editar**
4. Clique em **Novo** e adicione o caminho da pasta que contém o executável
5. Clique em OK e reinicie o terminal

---

## Cookies do YouTube

Para baixar playlists **privadas** ou conteúdo que requer login, você precisará exportar seus cookies do navegador.

### Exportar com a extensão

1. Instale a extensão **Get cookies.txt LOCALLY** no Chrome ou Firefox:
   - [Chrome Web Store](https://chromewebstore.google.com/detail/get-cookiestxt-locally/cclelndahbckbenkjhflpdbgdldlbecc)
2. Acesse [youtube.com](https://youtube.com) com sua conta logada
3. Clique no ícone da extensão → **Export** → salve como `cookies.txt`
4. No SYT Downloader, clique em **📄** no campo `cookies.txt` e selecione o arquivo exportado

> O arquivo `cookies.txt` deve ser mantido seguro — ele contém suas credenciais de sessão.
