import { useState, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface ProgressPayload {
  msg: string;
  done: boolean;
  error?: string;
}

interface DownloadParams {
  url: string;
  outputDir: string;
  cookiesPath?: string;
  playlistEnd?: number | null;
  clientId?: string;
  clientSecret?: string;
  spDc?: string;
  mode?: string;
}

export interface PlaylistInfo {
  current: number;
  total: number;
}

const PROGRESS_REGEX    = /\[download\]\s+([\d.]+)%/;
const ITEM_REGEX        = /\[download\] Downloading item (\d+) of (\d+)/;
const MERGER_REGEX      = /\[Merger\]|\[ffmpeg\]|Deleting original file/;
const CONVERTING_MSG    = "🔄 Convertendo...";

const SPOTIFY_TOTAL_REGEX     = /📋 (\d+) músicas encontradas/;
const SPOTIFY_TRACK_DONE_REGEX = /\[ExtractAudio\] Destination:/;

const STATUS_PATTERNS: Array<[RegExp, string | ((m: RegExpMatchArray) => string)]> = [
  [/\[youtube(?::tab)?\].*Downloading API JSON/,         "🔍 Buscando playlist..."],
  [/\[youtube:search\] Extracting URL: ytsearch1:(.+)/,  (m) => `🔍 ${m[1].trim()}`],
  [/\[youtube\].*Downloading webpage/,                   "🌐 Carregando página..."],
  [/\[youtube\].*Downloading.*player API/,               "🌐 Carregando..."],
  [/\[youtube\].*Solving JS challenges/,                 "⚙️ Processando..."],
  [/\[youtube\].*Downloading m3u8/,                      "⚙️ Processando..."],
  [/\[info\].*Downloading \d+ format/,                   "📥 Preparando download..."],
  [/\[download\] Downloading item (\d+) of (\d+)/,       (m) => `📦 Item ${m[1]} de ${m[2]}`],
  [/\[youtube\].*Extracting URL/,                        "🔗 Lendo URL..."],
  [/\[youtube\].*Downloading tv downgraded/,             "🌐 Carregando..."],
];

function toFriendlyStatus(msg: string): string | null {
  for (const [regex, label] of STATUS_PATTERNS) {
    const m = msg.match(regex);
    if (m) return typeof label === "function" ? label(m) : label;
  }
  return null;
}

const STATUS_PREFIXES = ["🔍", "🌐", "⚙️", "📥", "📦", "🔗"];
function isStatusLine(line: string) {
  return STATUS_PREFIXES.some((p) => line.startsWith(p));
}

export function useDownload() {
  const [logs, setLogs] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [done, setDone] = useState(false);
  const [playlistInfo, setPlaylistInfo] = useState<PlaylistInfo | null>(null);
  const [fileProgress, setFileProgress] = useState(0);

  const unlistenRef = useRef<(() => void) | null>(null);
  const isSpotifyModeRef = useRef(false);

  const progress: number | null = playlistInfo
    ? isSpotifyModeRef.current
      ? (playlistInfo.current / playlistInfo.total) * 100
      : ((playlistInfo.current - 1) + fileProgress / 100) / playlistInfo.total * 100
    : fileProgress > 0 ? fileProgress : null;

  const addLog = (msg: string) => setLogs((prev) => [...prev, msg]);

  const handleLog = (msg: string) => {
    const spotifyTotalMatch = SPOTIFY_TOTAL_REGEX.exec(msg);
    if (spotifyTotalMatch) {
      const total = parseInt(spotifyTotalMatch[1]);
      isSpotifyModeRef.current = true;
      setPlaylistInfo({ current: 0, total });
      addLog(msg);
      return;
    }

    if (isSpotifyModeRef.current && SPOTIFY_TRACK_DONE_REGEX.test(msg)) {
      setPlaylistInfo((prev) =>
        prev ? { current: prev.current + 1, total: prev.total } : prev
      );
      return;
    }

    if (isSpotifyModeRef.current && /Deleting original file/.test(msg)) {
      return;
    }

    const progressMatch = PROGRESS_REGEX.exec(msg);
    if (progressMatch) {
      setFileProgress(parseFloat(progressMatch[1]));
      setLogs((prev) => {
        const last = prev[prev.length - 1] ?? "";
        if (PROGRESS_REGEX.test(last)) return [...prev.slice(0, -1), msg];
        return [...prev, msg];
      });
      return;
    }

    if (!isSpotifyModeRef.current && MERGER_REGEX.test(msg)) {
      setLogs((prev) => {
        const last = prev[prev.length - 1] ?? "";
        if (last === CONVERTING_MSG) return prev;
        return [...prev, CONVERTING_MSG];
      });
      return;
    }

    const friendly = toFriendlyStatus(msg);
    if (friendly) {
      const itemMatch = ITEM_REGEX.exec(msg);
      if (itemMatch) {
        const current = parseInt(itemMatch[1]);
        const total   = parseInt(itemMatch[2]);
        setPlaylistInfo((prev) => {
          if (!prev) return current === 1 ? { current, total } : null;
          return total === prev.total ? { current, total } : prev;
        });
        setFileProgress(0);
      }
      setLogs((prev) => {
        const last = prev[prev.length - 1] ?? "";
        if (isStatusLine(last)) return [...prev.slice(0, -1), friendly];
        return [...prev, friendly];
      });
      return;
    }

    addLog(msg);
  };

  const startDownload = useCallback(async ({
    url, outputDir, cookiesPath, playlistEnd, clientId, clientSecret, spDc, mode,
  }: DownloadParams) => {
    if (unlistenRef.current) {
      unlistenRef.current();
      unlistenRef.current = null;
    }

    setLogs([]);
    setDone(false);
    setLoading(true);
    setPlaylistInfo(null);
    setFileProgress(0);
    isSpotifyModeRef.current = false;

    const unlisten = await listen<ProgressPayload>("download-progress", (event) => {
      const { msg, done: isDone, error } = event.payload;

      if (error) {
        addLog(`❌ ${error}`);
        setLoading(false);
        unlistenRef.current = null;
        unlisten();
        return;
      }

      if (isDone) {
        addLog("✅ Download concluído!");
        setLoading(false);
        setDone(true);
        setFileProgress(100);
        unlistenRef.current = null;
        unlisten();
        return;
      }

      if (msg) handleLog(msg);
    });

    unlistenRef.current = unlisten;

    try {
      await invoke("download_playlist", {
        url,
        outputDir,
        cookiesPath:  cookiesPath  || null,
        playlistEnd:  playlistEnd  ?? null,
        clientId:     clientId     || null,
        clientSecret: clientSecret || null,
        spDc:         spDc         || null,
        mode:         mode         || null,
      });
    } catch (err) {
      addLog(`❌ Erro: ${err}`);
      setLoading(false);
      unlistenRef.current = null;
      unlisten();
    }
  }, []);

  const cancelDownload = useCallback(async () => {
    try {
      await invoke("cancel_download");
      setLogs((prev) => [...prev, "⛔ Download cancelado."]);
      setLoading(false);
    } catch (err) {
      setLogs((prev) => [...prev, `❌ Erro ao cancelar: ${err}`]);
    } finally {
      if (unlistenRef.current) {
        unlistenRef.current();
        unlistenRef.current = null;
      }
    }
  }, []);

  const clearLogs = useCallback(() => setLogs([]), []);

  return { logs, loading, done, progress, playlistInfo, startDownload, cancelDownload, clearLogs };
}
