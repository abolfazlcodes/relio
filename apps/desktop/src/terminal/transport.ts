import { Channel, invoke, isTauri } from "@tauri-apps/api/core";
import type { LocalTerminalStarted } from "../generated/ipc/LocalTerminalStarted";
import type { ShellProfileSummary } from "../generated/ipc/ShellProfileSummary";
import type { StartLocalTerminalRequest } from "../generated/ipc/StartLocalTerminalRequest";
import type { TerminalChannelEvent } from "../generated/ipc/TerminalChannelEvent";
import type { TerminalCreditRequest } from "../generated/ipc/TerminalCreditRequest";
import type { TerminalInputRequest } from "../generated/ipc/TerminalInputRequest";
import type { TerminalResizeRequest } from "../generated/ipc/TerminalResizeRequest";
import type { TerminalSessionRequest } from "../generated/ipc/TerminalSessionRequest";

export interface TerminalTransport {
  grantCredit(request: TerminalCreditRequest): Promise<void>;
  listProfiles(): Promise<Array<ShellProfileSummary>>;
  resize(request: TerminalResizeRequest): Promise<void>;
  sendInput(request: TerminalInputRequest): Promise<void>;
  start(
    request: StartLocalTerminalRequest,
    onEvent: (event: TerminalChannelEvent) => void,
  ): Promise<LocalTerminalStarted>;
  stop(request: TerminalSessionRequest): Promise<void>;
}

export class TauriTerminalTransport implements TerminalTransport {
  async listProfiles(): Promise<Array<ShellProfileSummary>> {
    requireDesktopRuntime();
    return invoke("terminal_list_shell_profiles");
  }

  async start(
    request: StartLocalTerminalRequest,
    onEvent: (event: TerminalChannelEvent) => void,
  ): Promise<LocalTerminalStarted> {
    requireDesktopRuntime();
    const events = new Channel<TerminalChannelEvent>();
    events.onmessage = onEvent;
    return invoke("terminal_start_local", { events, request });
  }

  async grantCredit(request: TerminalCreditRequest): Promise<void> {
    return invoke("terminal_grant_output_credit", { request });
  }

  async sendInput(request: TerminalInputRequest): Promise<void> {
    return invoke("terminal_send_input", { request });
  }

  async resize(request: TerminalResizeRequest): Promise<void> {
    return invoke("terminal_resize", { request });
  }

  async stop(request: TerminalSessionRequest): Promise<void> {
    return invoke("terminal_stop", { request });
  }
}

function requireDesktopRuntime(): void {
  if (!isTauri()) {
    throw new Error("terminal.desktop_runtime_required");
  }
}
