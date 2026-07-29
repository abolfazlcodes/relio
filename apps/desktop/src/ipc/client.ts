import type { RequestEnvelope } from "../generated/ipc/RequestEnvelope";
import {
  RelioClientError,
  isRecord,
  parsePublicError,
} from "./public-error";

export const CONTRACT_VERSION = 1;
export const MAX_METADATA_BYTES = 5 * 1024 * 1024;

export interface IpcTransport {
  invoke(command: string, payload: unknown, signal?: AbortSignal): Promise<unknown>;
}

export interface RequestOptions {
  windowId: string;
  profileId?: string;
  expectedRevision?: number;
  idempotencyKey?: string;
  signal?: AbortSignal;
}

export class IpcClient {
  public constructor(private readonly transport: IpcTransport) {}

  public async query<TRequest, TResponse>(
    command: string,
    body: TRequest,
    options: RequestOptions,
  ): Promise<TResponse> {
    return this.send(command, "query", body, options);
  }

  public async command<TRequest, TResponse>(
    command: string,
    body: TRequest,
    options: RequestOptions,
  ): Promise<TResponse> {
    return this.send(command, "command", body, options);
  }

  public async decision<TRequest, TResponse>(
    command: string,
    body: TRequest,
    options: RequestOptions,
  ): Promise<TResponse> {
    return this.send(command, "decision", body, options);
  }

  private async send<TRequest, TResponse>(
    command: string,
    kind: "query" | "command" | "decision",
    body: TRequest,
    options: RequestOptions,
  ): Promise<TResponse> {
    validateCommand(command);
    const request: RequestEnvelope<TRequest> = {
      contract_version: CONTRACT_VERSION,
      request_id: crypto.randomUUID(),
      window_id: options.windowId,
      profile_id: options.profileId ?? null,
      expected_revision: options.expectedRevision ?? null,
      idempotency_key: options.idempotencyKey ?? null,
      kind,
      body,
    };
    enforceSize(request);
    const response = await this.transport.invoke(command, request, options.signal);
    return parseResponse<TResponse>(response, request.request_id);
  }
}

function parseResponse<T>(value: unknown, requestId: string): T {
  enforceSize(value);
  if (
    !isRecord(value) ||
    value.contract_version !== CONTRACT_VERSION ||
    value.request_id !== requestId ||
    (value.outcome !== "ok" && value.outcome !== "error")
  ) {
    throw new RelioClientError(parsePublicError(undefined));
  }
  if (value.outcome === "ok") return value.body as T;
  if (value.outcome === "error") {
    throw new RelioClientError(parsePublicError(value.body));
  }
  throw new RelioClientError(parsePublicError(undefined));
}

function validateCommand(command: string): void {
  if (!/^[a-z][a-z0-9_]*\.[a-z][a-z0-9_]*$/u.test(command)) {
    throw new RelioClientError(parsePublicError(undefined));
  }
}

function enforceSize(value: unknown): void {
  const encoded = new TextEncoder().encode(JSON.stringify(value));
  if (encoded.byteLength > MAX_METADATA_BYTES) {
    throw new RelioClientError({
      ...parsePublicError(undefined),
      code: "ipc.payload_too_large",
      safe_message_key: "ipc.payload_too_large",
    });
  }
}
