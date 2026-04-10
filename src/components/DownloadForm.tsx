import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

interface Props {
  onSubmit: (params: { url: string; outputDir: string; cookiesPath: string; playlistEnd: number | null; clientId: string; clientSecret: string; spDc: string; mode: string }) => void;
  onCancel: () => void;
  loading: boolean;
}

export function DownloadForm({ onSubmit, onCancel, loading }: Props) {
  const [url, setUrl] = useState("");
  const [outputDir, setOutputDir] = useState("C:\\Users\\NovoAdmin\\Downloads");
  const [cookiesPath, setCookiesPath] = useState("C:\\Users\\NovoAdmin\\Downloads\\cookies.txt");
  const [playlistEnd, setPlaylistEnd] = useState("");
  const [spDc, setSpDc] = useState(() => sessionStorage.getItem("spotify_sp_dc") || "");
  const [mode, setMode] = useState<"playlist" | "original">("playlist");

  const isSpotify = url.includes("spotify.com");
  const isSpotifyPlaylist = isSpotify && url.includes("/playlist/");
  const isSpotifyInvalid = isSpotify && !isSpotifyPlaylist;

  function handleSpDcChange(val: string) {
    setSpDc(val);
    sessionStorage.setItem("spotify_sp_dc", val);
  }

  function handleSubmit() {
    if (!url.trim()) return;
    if (isSpotify && !spDc.trim()) return;
    const limit = playlistEnd.trim() !== "" ? parseInt(playlistEnd) : null;
    onSubmit({ url, outputDir, cookiesPath, playlistEnd: limit && limit > 0 ? limit : null, clientId: "", clientSecret: "", spDc, mode: isSpotify ? "spotify" : mode });
  }

  async function browseFolderDir() {
    const selected = await open({ directory: true, multiple: false, defaultPath: outputDir });
    if (selected) setOutputDir(selected as string);
  }

  async function browseCookiesFile() {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Cookies", extensions: ["txt"] }],
    });
    if (selected) setCookiesPath(selected as string);
  }

  return (
    <div className="form">
      {!isSpotify && (
        <div className="mode-selector">
          <button
            className={`btn-mode ${mode === "playlist" ? "active" : ""}`}
            onClick={() => setMode("playlist")}
            disabled={loading}
            type="button"
          >
            Playlist
          </button>
          <button
            className={`btn-mode ${mode === "original" ? "active" : ""}`}
            onClick={() => setMode("original")}
            disabled={loading}
            type="button"
          >
            Original
          </button>
        </div>
      )}

      <div className="field">
        <label>URL</label>
        <input
          type="text"
          placeholder={
            isSpotify
              ? "https://open.spotify.com/playlist/..."
              : "https://youtube.com/playlist?list=... ou watch?v=..."
          }
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          disabled={loading}
        />
        {isSpotifyInvalid && (
          <small className="field-error">
            ❌ URL inválida. Use uma playlist do Spotify: <code>open.spotify.com/playlist/...</code>
          </small>
        )}
      </div>

      {isSpotify && (
        <div className="field">
          <label>Cookie <code>sp_dc</code></label>
          <input
            type="password"
            placeholder="Cole aqui o valor do cookie sp_dc do seu navegador"
            value={spDc}
            onChange={(e) => handleSpDcChange(e.target.value)}
            disabled={loading}
          />
          <small className="field-hint">
            Como obter: abra <strong>open.spotify.com</strong>, faça login, pressione <kbd>F12</kbd> →
            Application → Cookies → open.spotify.com → copie o valor de <code>sp_dc</code>
          </small>
        </div>
      )}

      <div className="form-row">
        <div className="field" style={{ flex: 1 }}>
          <label>Pasta de destino</label>
          <div className="input-row">
            <input
              type="text"
              value={outputDir}
              onChange={(e) => setOutputDir(e.target.value)}
              disabled={loading}
            />
            <button className="btn-browse" onClick={browseFolderDir} disabled={loading}>
              📁
            </button>
          </div>
        </div>

        <div className="field field-limit">
          <label>Limite <span className="optional">(opcional)</span></label>
          <input
            type="number"
            min="1"
            placeholder="∞"
            value={playlistEnd}
            onChange={(e) => setPlaylistEnd(e.target.value)}
            disabled={loading}
          />
        </div>
      </div>

      {!isSpotify && (
        <div className="field">
          <label>cookies.txt <span className="optional">(opcional — necessário para playlists privadas)</span></label>
          <div className="input-row">
            <input
              type="text"
              placeholder="C:\caminho\para\cookies.txt"
              value={cookiesPath}
              onChange={(e) => setCookiesPath(e.target.value)}
              disabled={loading}
            />
            <button className="btn-browse" onClick={browseCookiesFile} disabled={loading}>
              📄
            </button>
          </div>
        </div>
      )}

      <div className="buttons">
        <button onClick={handleSubmit} disabled={loading || !url.trim() || isSpotifyInvalid || (isSpotify && !spDc.trim())}>
          {loading ? "Baixando..." : isSpotify ? "Baixar do Spotify" : mode === "original" ? "Baixar Original" : "Baixar Playlist"}
        </button>
        {loading && (
          <button className="btn-cancel" onClick={onCancel}>
            ⛔ Cancelar
          </button>
        )}
      </div>
    </div>
  );
}
