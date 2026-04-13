import { useEffect, useState } from "react";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

type UpdateState =
  | { status: "idle" }
  | { status: "available"; version: string; body: string | null | undefined }
  | { status: "downloading"; percent: number }
  | { status: "error"; message: string };

export function UpdateChecker() {
  const [state, setState] = useState<UpdateState>({ status: "idle" });

  useEffect(() => {
    async function checkUpdate() {
      try {
        const update = await check();
        if (update?.available) {
          setState({ status: "available", version: update.version, body: update.body });
        }
      } catch {
      }
    }
    checkUpdate();
  }, []);

  async function handleInstall() {
    if (state.status !== "available") return;
    try {
      const update = await check();
      if (!update?.available) return;

      let totalSize = 0;
      let downloaded = 0;

      await update.downloadAndInstall((event) => {
        if (event.event === "Started") {
          totalSize = event.data.contentLength ?? 0;
        } else if (event.event === "Progress") {
          downloaded += event.data.chunkLength;
          const percent = totalSize > 0 ? Math.round((downloaded / totalSize) * 100) : 0;
          setState({ status: "downloading", percent });
        }
      });

      await relaunch();
    } catch (e) {
      setState({ status: "error", message: String(e) });
    }
  }

  if (state.status === "idle" || state.status === "error") return null;

  if (state.status === "downloading") {
    return (
      <div className="update-banner">
        <span>⬇️ Baixando atualização... {state.percent}%</span>
      </div>
    );
  }

  return (
    <div className="update-banner">
      <span>🆕 Nova versão disponível: <strong>v{state.version}</strong></span>
      <button className="btn-update" onClick={handleInstall}>
        Atualizar agora
      </button>
      <button className="btn-update-dismiss" onClick={() => setState({ status: "idle" })}>
        ✕
      </button>
    </div>
  );
}
