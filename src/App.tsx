import { useEffect, useRef, useState } from "react";
import * as bridge from "./bridge";
import { Icon } from "./components/Icon";
import { messages, systemLocale } from "./i18n";
import type { AppSnapshot, AwakeMode, Language } from "./model";

const modes: AwakeMode[] = ["off", "system", "display"];

export default function App() {
  const [snapshot, setSnapshot] = useState<AppSnapshot | null>(null);
  const [busy, setBusy] = useState(false);
  const [ready, setReady] = useState(!bridge.desktop);
  const [error, setError] = useState("");
  const [previewLanguage, setPreviewLanguage] = useState<Language>("auto");
  const revision = useRef(-1);
  const locale =
    snapshot?.locale ??
    (previewLanguage === "auto" ? systemLocale() : previewLanguage);
  const text = messages[locale];

  function accept(next: AppSnapshot) {
    if (next.revision >= revision.current) {
      revision.current = next.revision;
      setSnapshot(next);
    }
  }

  async function reconnect() {
    setBusy(true);
    try {
      accept(await bridge.getState());
      setReady(true);
      setError("");
    } catch {
      setError(messages[locale].reconnect);
    } finally {
      setBusy(false);
    }
  }

  useEffect(() => {
    document.documentElement.lang = locale === "zh" ? "zh-CN" : "en";
  }, [locale]);

  useEffect(() => {
    if (!bridge.desktop) return;
    let disposed = false;
    let unlisten: (() => void) | undefined;
    // Subscribe before fetching so tray changes cannot be lost during startup.
    void (async () => {
      try {
        const stop = await bridge.onStateChanged((next) => {
          if (!disposed) accept(next);
        });
        if (disposed) {
          stop();
          return;
        }
        unlisten = stop;
        const initial = await bridge.getState();
        if (!disposed) {
          accept(initial);
          setReady(true);
        }
      } catch {
        if (!disposed) {
          setReady(true);
          setError(messages[systemLocale()].reconnect);
        }
      }
    })();
    return () => {
      disposed = true;
      unlisten?.();
    };
  }, []);

  async function change(operation: () => Promise<AppSnapshot>) {
    setBusy(true);
    setError("");
    try {
      accept(await operation());
    } catch {
      setError(text.failed);
    } finally {
      setBusy(false);
    }
  }

  return (
    <main className="panel" aria-busy={busy || !ready}>
      <header className="header">
        <div className="brand">
          <Icon name="coffee" size={34} />
          <div>
            <h1 title={`Caffeine ${snapshot?.version ?? "2.0.0"}`}>Caffeine</h1>
            <p
              className={`status status-${snapshot?.mode ?? "off"}`}
              role="status"
            >
              {!ready ? text.loading : text.status[snapshot?.mode ?? "off"]}
            </p>
          </div>
        </div>
        <select
          aria-label={text.language}
          value={snapshot?.language ?? previewLanguage}
          disabled={busy || !ready}
          onChange={(event) => {
            const language = event.target.value as Language;
            if (bridge.desktop) void change(() => bridge.setLanguage(language));
            else setPreviewLanguage(language);
          }}
        >
          <option value="auto">{text.automatic}</option>
          <option value="zh">中文</option>
          <option value="en">English</option>
        </select>
      </header>

      <fieldset
        className="modes"
        disabled={!bridge.desktop || !snapshot || busy}
      >
        <legend className="sr-only">{text.modeLabel}</legend>
        {modes.map((mode) => (
          <label
            key={mode}
            className={`mode${(snapshot?.mode ?? "off") === mode ? " selected" : ""}`}
            title={text.help[mode]}
          >
            <Icon name={mode} />
            <span>{text.modes[mode]}</span>
            <input
              type="radio"
              name="mode"
              value={mode}
              checked={(snapshot?.mode ?? "off") === mode}
              onChange={() => void change(() => bridge.setMode(mode))}
              aria-description={text.help[mode]}
            />
          </label>
        ))}
      </fieldset>

      {!bridge.desktop ? (
        <p className="notice" role="note">
          {text.preview}
        </p>
      ) : null}
      {snapshot?.warning ? (
        <p className="notice" role="alert">
          {text.saveWarning}
        </p>
      ) : null}
      {error || snapshot?.error ? (
        <div className="error" role="alert">
          <span title={snapshot?.error ?? undefined}>
            {error || text.failed}
          </span>
          <button onClick={() => void reconnect()} disabled={busy}>
            {text.retry}
          </button>
        </div>
      ) : null}

      <footer>
        <div className="footer-note">
          <span>{text.running}</span>
          <button
            className="info"
            aria-label={text.details}
            title={text.details}
          >
            <Icon name="info" size={14} />
          </button>
        </div>
        <button
          className="quit"
          disabled={!bridge.desktop || busy}
          onClick={() => {
            setBusy(true);
            void bridge.quit().catch(() => {
              setError(text.failed);
              setBusy(false);
            });
          }}
        >
          {text.quit}
        </button>
      </footer>
    </main>
  );
}
