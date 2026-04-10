import { openUrl } from "@tauri-apps/plugin-opener";

const GITHUB_URL = "https://github.com/Ton-Chyod-s/syt-playlist-downloader";

interface Props {
  onClose: () => void;
}

async function openLink(url: string) {
  await openUrl(url);
}

export function HelpModal({ onClose }: Props) {
  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>ℹ️ Sobre & Ajuda</h2>
          <button className="btn-close" onClick={onClose}>✕</button>
        </div>

        <div className="modal-body">
          <section>
            <h3>🚀 Como usar</h3>
            <ol>
              <li>Escolha o modo: <strong>📋 Playlist</strong>, <strong>⚡ Original</strong> ou cole um link do Spotify</li>
              <li>Cole a URL do YouTube ou Spotify</li>
              <li>Escolha a pasta onde os arquivos serão salvos</li>
              <li>Clique em <strong>Baixar</strong></li>
            </ol>
          </section>

          <section>
            <h3>📋 Modos de download</h3>
            <ul>
              <li>
                <strong>📋 Playlist</strong> — baixa a playlist em MP4 1080p compatível (H.264 + MP3). Ideal para reprodução em qualquer dispositivo.
              </li>
              <li>
                <strong>⚡ Original</strong> — baixa em melhor qualidade disponível (4K, VP9, AV1…), sem restrição de codec ou resolução. Funciona para vídeos individuais ou playlists inteiras.
              </li>
              <li>
                <strong>🎵 Spotify</strong> — busca as músicas da playlist no YouTube e baixa como MP3 de alta qualidade. Cole um link <code>open.spotify.com/playlist/...</code> e informe o cookie <code>sp_dc</code>.
              </li>
            </ul>
          </section>

          <section>
            <h3>🛠️ Ferramentas necessárias</h3>
            <ul>
              <li>
                <span className="tool-name">yt-dlp</span> — baixa vídeos do YouTube
                <button className="btn-link" onClick={() => openLink("https://github.com/yt-dlp/yt-dlp")}>
                  GitHub ↗
                </button>
              </li>
              <li>
                <span className="tool-name">ffmpeg</span> — converte e mescla áudio/vídeo
                <button className="btn-link" onClick={() => openLink("https://ffmpeg.org/download.html")}>
                  Download ↗
                </button>
              </li>
              <li>
                <span className="tool-name">Node.js</span> — necessário para yt-dlp resolver o JS do YouTube
                <button className="btn-link" onClick={() => openLink("https://nodejs.org")}>
                  Download ↗
                </button>
              </li>
            </ul>
          </section>

          <section>
            <h3>🍪 Como exportar cookies (YouTube privado)</h3>
            <p>Instale a extensão <strong>Get cookies.txt LOCALLY</strong> no Chrome/Firefox, acesse o YouTube logado e exporte o arquivo.</p>
            <button className="btn-link" onClick={() => openLink("https://chromewebstore.google.com/detail/get-cookiestxt-locally/cclelndahbckbenkjhflpdbgdldlbecc")}>
              Extensão Chrome ↗
            </button>
          </section>

          <section>
            <h3>🎵 Cookie sp_dc (Spotify)</h3>
            <p>Abra <strong>open.spotify.com</strong> logado, pressione <strong>F12</strong> → Application → Cookies → <code>open.spotify.com</code>, copie o valor do cookie <code>sp_dc</code> e cole no campo correspondente.</p>
          </section>

          <div className="modal-footer">
            <button className="btn-github" onClick={() => openLink(GITHUB_URL)}>
              ⭐ Ver no GitHub
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
