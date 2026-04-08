import { useEffect, useRef } from "react";

interface Props {
  logs: string[];
  onClear: () => void;
}

export function LogConsole({ logs, onClear }: Props) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [logs]);

  return (
    <div className="console-wrap">
      <div className="console-toolbar">
        <span className="console-title">📋 Log</span>
        {logs.length > 0 && (
          <button className="btn-clear" onClick={onClear} title="Limpar logs">
            🗑️ Limpar
          </button>
        )}
      </div>
      <div className="console">
        {logs.length === 0 && <span className="placeholder">Logs aparecerão aqui...</span>}
        {logs.map((log, i) => (
          <div key={i} className="log-line">{log}</div>
        ))}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
