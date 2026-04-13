import { useState } from "react";
import { DownloadForm } from "./components/DownloadForm";
import { LogConsole } from "./components/LogConsole";
import { HelpModal } from "./components/HelpModal";
import { UpdateChecker } from "./components/UpdateChecker";
import { useDownload } from "./hooks/useDownload";
import "./App.css";

function App() {
  const { logs, loading, done, progress, playlistInfo, startDownload, cancelDownload, clearLogs } = useDownload();
  const [showHelp, setShowHelp] = useState(false);

  const progressLabel = playlistInfo
    ? `${playlistInfo.current} de ${playlistInfo.total}`
    : progress !== null
    ? `${progress.toFixed(1)}%`
    : "";

  return (
    <div className="app">
      <header>
        <div className="header-top">
          <div>
            <h1>SYT Playlist Downloader</h1>
            <p>for Multimídia mp5 · Cole a URL da playlist e escolha onde salvar</p>
          </div>
          <button className="btn-help" onClick={() => setShowHelp(true)}>❓ Ajuda</button>
        </div>
      </header>
      {showHelp && <HelpModal onClose={() => setShowHelp(false)} />}
      <UpdateChecker />

      <main>
        <DownloadForm onSubmit={startDownload} onCancel={cancelDownload} loading={loading} />
        {(loading || done) && progress !== null && (
          <div className="progress-wrap">
            <div className="progress-bar" style={{ width: `${done ? 100 : progress}%` }} />
            <span className="progress-label">{done && playlistInfo ? `${playlistInfo.total} de ${playlistInfo.total}` : progressLabel}</span>
          </div>
        )}
        {done && <div className="badge-done">✅ Todos os vídeos baixados!</div>}
        <LogConsole logs={logs} onClear={clearLogs} />
      </main>
    </div>
  );
}

export default App;
