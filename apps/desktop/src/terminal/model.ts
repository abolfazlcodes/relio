import { FitAddon } from "@xterm/addon-fit";
import { Terminal, type ITerminalOptions, type Terminal as XtermTerminal } from "@xterm/xterm";
import type { TerminalChannelEvent } from "../generated/ipc/TerminalChannelEvent";
import type { TerminalTransport } from "./transport";
import { approvedExternalUri, sanitizeRemoteTitle } from "./policy";

export const TERMINAL_SCROLLBACK_LINES = 10_000;
export const INITIAL_OUTPUT_CREDIT_BYTES = 1024 * 1024;

export type TerminalState =
  | { kind: "idle" }
  | { kind: "running"; sessionId: string }
  | { kind: "exited"; exitCode: number; signal: string | null }
  | { kind: "failed"; messageKey: string };

export interface TerminalModelEvents {
  onExternalUri: (uri: string) => void;
  onOutputGap: (firstMissingSequence: string, nextAvailableSequence: string) => void;
  onState: (state: TerminalState) => void;
  onTitle: (title: string) => void;
}

interface TerminalLike {
  cols: number;
  element: HTMLElement | undefined;
  rows: number;
  loadAddon(addon: FitAddon): void;
  open(element: HTMLElement): void;
  onBinary(listener: (data: string) => void): { dispose(): void };
  onData(listener: (data: string) => void): { dispose(): void };
  onTitleChange(listener: (title: string) => void): { dispose(): void };
  parser: XtermTerminal["parser"];
  write(data: Uint8Array, callback?: () => void): void;
}

export class TerminalModel {
  readonly terminal: TerminalLike;
  private readonly detachedHost = document.createDocumentFragment();
  private expectedOutputSequence = 0n;
  private fitAddon: FitAddon;
  private inputSequence = 0n;
  private mounted = false;
  private resizeFrame: number | null = null;
  private resizeObserver: ResizeObserver | null = null;
  private sessionId: string | null = null;

  constructor(
    private readonly transport: TerminalTransport,
    private readonly events: TerminalModelEvents,
    options: Pick<ITerminalOptions, "fontFamily" | "fontSize" | "screenReaderMode"> = {},
    terminal?: TerminalLike,
    fitAddon = new FitAddon(),
  ) {
    this.fitAddon = fitAddon;
    this.terminal =
      terminal ??
      new Terminal({
        allowProposedApi: false,
        convertEol: false,
        cursorBlink: true,
        disableStdin: false,
        drawBoldTextInBrightColors: false,
        fontFamily: options.fontFamily ?? "ui-monospace, SFMono-Regular, Consolas, monospace",
        fontSize: options.fontSize ?? 13,
        minimumContrastRatio: 4.5,
        linkHandler: terminalLinkHandler(events.onExternalUri),
        screenReaderMode: options.screenReaderMode ?? false,
        scrollback: TERMINAL_SCROLLBACK_LINES,
        windowOptions: {},
      });
    this.terminal.loadAddon(this.fitAddon);
    this.terminal.parser.registerOscHandler(52, () => true);
    this.terminal.onTitleChange((title) => {
      const safeTitle = sanitizeRemoteTitle(title);
      if (safeTitle) this.events.onTitle(safeTitle);
    });
    this.terminal.onData((data) => {
      void this.sendInput(new TextEncoder().encode(data)).catch(() => this.fail());
    });
    this.terminal.onBinary((data) => {
      const bytes = Uint8Array.from(data, (character) => character.charCodeAt(0) & 0xff);
      void this.sendInput(bytes).catch(() => this.fail());
    });
  }

  async start(profileId: string): Promise<void> {
    if (this.sessionId !== null) return;
    const started = await this.transport.start(
      {
        columns: this.terminal.cols,
        pixel_height: 0,
        pixel_width: 0,
        profile_id: profileId,
        rows: this.terminal.rows,
      },
      (event) => this.handleEvent(event),
    );
    this.sessionId = started.session_id;
    this.inputSequence = BigInt(started.initial_input_sequence);
    this.events.onState({ kind: "running", sessionId: started.session_id });
    try {
      await this.transport.grantCredit({
        bytes: Math.min(INITIAL_OUTPUT_CREDIT_BYTES, started.maximum_output_credit_bytes),
        session_id: started.session_id,
      });
    } catch (error) {
      await this.transport.stop({ session_id: started.session_id }).catch(() => undefined);
      this.sessionId = null;
      this.fail();
      throw error;
    }
  }

  attach(host: HTMLElement): void {
    if (!this.mounted) {
      this.terminal.open(host);
      this.mounted = true;
    } else if (this.terminal.element) {
      host.append(this.terminal.element);
    }
    if (typeof ResizeObserver !== "undefined") {
      this.resizeObserver = new ResizeObserver(() => this.scheduleFit());
      this.resizeObserver.observe(host);
    }
    this.scheduleFit();
    queueMicrotask(() => {
      this.terminal.element
        ?.querySelector<HTMLTextAreaElement>(".xterm-helper-textarea")
        ?.setAttribute("aria-label", "Terminal input");
    });
  }

  detach(): void {
    this.resizeObserver?.disconnect();
    this.resizeObserver = null;
    if (this.resizeFrame !== null) {
      cancelAnimationFrame(this.resizeFrame);
      this.resizeFrame = null;
    }
    if (this.terminal.element) this.detachedHost.append(this.terminal.element);
  }

  async stop(): Promise<void> {
    if (this.sessionId === null) return;
    await this.transport.stop({ session_id: this.sessionId });
  }

  async sendText(value: string): Promise<void> {
    await this.sendInput(new TextEncoder().encode(value));
  }

  selection(): string {
    const terminal = this.terminal as TerminalLike & { getSelection?: () => string };
    return terminal.getSelection?.() ?? "";
  }

  private handleEvent(event: TerminalChannelEvent): void {
    if (event.event === "output") {
      const sequence = BigInt(event.sequence);
      if (sequence !== this.expectedOutputSequence) {
        this.events.onOutputGap(this.expectedOutputSequence.toString(), event.sequence);
        this.expectedOutputSequence = sequence;
      }
      this.expectedOutputSequence += 1n;
      const bytes = Uint8Array.from(event.bytes);
      this.terminal.write(bytes, () => {
        if (this.sessionId !== null) {
          void this.transport.grantCredit({
            bytes: bytes.byteLength,
            session_id: this.sessionId,
          }).catch(() => this.fail());
        }
      });
      return;
    }
    if (event.event === "output_gap") {
      this.expectedOutputSequence = BigInt(event.next_available_sequence);
      this.events.onOutputGap(event.first_missing_sequence, event.next_available_sequence);
      return;
    }
    if (event.event === "exited") {
      this.events.onState({
        exitCode: event.exit_code,
        kind: "exited",
        signal: event.signal,
      });
      this.sessionId = null;
      return;
    }
    this.events.onState({ kind: "failed", messageKey: event.safe_message_key });
    this.sessionId = null;
  }

  private async sendInput(bytes: Uint8Array): Promise<void> {
    if (this.sessionId === null || bytes.byteLength === 0) return;
    const sequence = this.inputSequence;
    this.inputSequence += 1n;
    await this.transport.sendInput({
      bytes: [...bytes],
      sequence: sequence.toString(),
      session_id: this.sessionId,
    });
  }

  private fail(): void {
    this.events.onState({ kind: "failed", messageKey: "terminal.session_failed" });
  }

  private scheduleFit(): void {
    if (this.resizeFrame !== null) cancelAnimationFrame(this.resizeFrame);
    this.resizeFrame = requestAnimationFrame(() => {
      this.resizeFrame = null;
      this.fitAddon.fit();
      if (this.sessionId !== null) {
        void this.transport.resize({
          columns: this.terminal.cols,
          pixel_height: 0,
          pixel_width: 0,
          rows: this.terminal.rows,
          session_id: this.sessionId,
        }).catch(() => this.fail());
      }
    });
  }
}

export function terminalLinkHandler(onUri: (uri: string) => void): NonNullable<ITerminalOptions["linkHandler"]> {
  return {
    activate: (_event, uri) => {
      const approved = approvedExternalUri(uri);
      if (approved) onUri(approved);
    },
    allowNonHttpProtocols: false,
  };
}
