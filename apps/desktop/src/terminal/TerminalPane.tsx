import { isTauri } from "@tauri-apps/api/core";
import { useEffect, useMemo, useRef, useState } from "react";
import { Button, Dialog, StatusMessage } from "../ui";
import { TerminalModel, type TerminalState } from "./model";
import { approvedExternalUri, reviewPaste, type PasteReview } from "./policy";
import { TauriTerminalTransport } from "./transport";
import "@xterm/xterm/css/xterm.css";
import "./terminal.css";

interface UriReview {
  uri: string;
}

export function TerminalPane() {
  if (!isTauri()) {
    return <StatusMessage tone="info">Local terminals are available only in the Relio desktop runtime.</StatusMessage>;
  }
  return <DesktopTerminalPane />;
}

function DesktopTerminalPane() {
  const hostRef = useRef<HTMLDivElement>(null);
  const [profiles, setProfiles] = useState<Array<{ id: string; display_name: string }>>([]);
  const [state, setState] = useState<TerminalState>({ kind: "idle" });
  const [title, setTitle] = useState("Local terminal");
  const [gap, setGap] = useState<string | null>(null);
  const [pasteReview, setPasteReview] = useState<PasteReview | null>(null);
  const [uriReview, setUriReview] = useState<UriReview | null>(null);
  const model = useMemo(() => {
    const transport = new TauriTerminalTransport();
    return new TerminalModel(
      transport,
      {
        onExternalUri: (uri) => {
          const approved = approvedExternalUri(uri);
          if (approved) setUriReview({ uri: approved });
        },
        onOutputGap: (first, next) => setGap(`Terminal output is incomplete (${first}–${next}).`),
        onState: setState,
        onTitle: setTitle,
      },
      { screenReaderMode: typeof window.matchMedia === "function" && window.matchMedia("(forced-colors: active)").matches },
    );
  }, []);

  useEffect(() => {
    const host = hostRef.current;
    if (!host) return;
    model.attach(host);
    void new TauriTerminalTransport()
      .listProfiles()
      .then(setProfiles)
      .catch(() => setState({ kind: "failed", messageKey: "terminal.desktop_runtime_required" }));
    return () => model.detach();
  }, [model]);

  const copySelection = async () => {
    const selection = model.selection();
    if (selection) await navigator.clipboard.writeText(selection);
  };

  const requestPaste = async () => {
    const value = await navigator.clipboard.readText();
    const review = reviewPaste(value);
    if (review.requiresConfirmation) setPasteReview(review);
    else await model.sendText(review.value);
  };

  return (
    <div className="terminal-pane">
      <div className="terminal-toolbar" aria-label="Terminal controls">
        <span className="terminal-title" title={title}>{title}</span>
        <div>
          <Button
            disabled={profiles.length === 0 || state.kind === "running"}
            onClick={() => {
              const profile = profiles[0];
              if (profile) void model.start(profile.id).catch(() => setState({ kind: "failed", messageKey: "terminal.start_failed" }));
            }}
          >
            Start local shell
          </Button>
          <Button disabled={state.kind !== "running"} onClick={() => void copySelection().catch(() => setState({ kind: "failed", messageKey: "terminal.clipboard_failed" }))} variant="quiet">
            Copy
          </Button>
          <Button disabled={state.kind !== "running"} onClick={() => void requestPaste().catch(() => setState({ kind: "failed", messageKey: "terminal.clipboard_failed" }))} variant="quiet">
            Paste
          </Button>
          <Button disabled={state.kind !== "running"} onClick={() => void model.stop().catch(() => setState({ kind: "failed", messageKey: "terminal.stop_failed" }))} variant="quiet">
            Stop
          </Button>
        </div>
      </div>
      {gap ? <StatusMessage tone="warning">{gap}</StatusMessage> : null}
      {state.kind === "failed" ? (
        <StatusMessage tone="danger">
          {state.messageKey === "terminal.desktop_runtime_required"
            ? "Local terminals are available only in the Relio desktop runtime."
            : "The local terminal could not continue. Retry from a new session."}
        </StatusMessage>
      ) : null}
      <div
        aria-label="Local terminal screen"
        className="terminal-host"
        onPaste={(event) => {
          event.preventDefault();
          const review = reviewPaste(event.clipboardData.getData("text/plain"));
          if (review.requiresConfirmation) setPasteReview(review);
          else void model.sendText(review.value).catch(() => setState({ kind: "failed", messageKey: "terminal.input_failed" }));
        }}
        ref={hostRef}
      />
      <p aria-live="polite" className="terminal-status">
        {state.kind === "running" ? "Terminal running" : state.kind === "exited" ? `Terminal exited with code ${state.exitCode}` : "Terminal stopped"}
      </p>
      <Dialog
        actions={
          <>
            <Button onClick={() => setPasteReview(null)} variant="quiet">Cancel</Button>
            <Button onClick={() => {
              if (pasteReview) void model.sendText(pasteReview.value).catch(() => setState({ kind: "failed", messageKey: "terminal.input_failed" }));
              setPasteReview(null);
            }}>Send exactly as shown</Button>
          </>
        }
        onCancel={() => setPasteReview(null)}
        open={pasteReview !== null}
        title="Review terminal paste"
      >
        <p>Multiline and control-character input may execute commands immediately.</p>
        <pre className="terminal-review">{pasteReview?.preview}</pre>
      </Dialog>
      <Dialog
        actions={
          <>
            <Button onClick={() => setUriReview(null)} variant="quiet">Cancel</Button>
            <Button onClick={() => {
              if (uriReview) void navigator.clipboard.writeText(uriReview.uri).catch(() => setState({ kind: "failed", messageKey: "terminal.clipboard_failed" }));
              setUriReview(null);
            }}>Copy destination</Button>
          </>
        }
        onCancel={() => setUriReview(null)}
        open={uriReview !== null}
        title="Terminal link"
      >
        <p>Relio does not open terminal-provided links automatically. Inspect the full destination before copying it.</p>
        <code className="terminal-uri">{uriReview?.uri}</code>
      </Dialog>
    </div>
  );
}

