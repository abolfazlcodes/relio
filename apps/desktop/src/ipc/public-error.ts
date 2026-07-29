import type { PublicError } from "../generated/ipc/PublicError";

const fallbackError: PublicError = {
  code: "ipc.response_malformed",
  subsystem: "ipc",
  operation_id: null,
  retryable: false,
  user_action: "none",
  safe_message_key: "ipc.response_malformed",
  safe_parameters: {},
  diagnostic_id: "00000000-0000-0000-0000-000000000000",
};

export class RelioClientError extends Error {
  public readonly detail: PublicError;

  public constructor(detail: PublicError) {
    super(detail.safe_message_key);
    this.name = "RelioClientError";
    this.detail = detail;
  }
}

export function parsePublicError(value: unknown): PublicError {
  if (!isRecord(value)) return fallbackError;
  if (
    typeof value.code !== "string" ||
    typeof value.safe_message_key !== "string" ||
    typeof value.diagnostic_id !== "string" ||
    typeof value.retryable !== "boolean" ||
    !isSubsystem(value.subsystem) ||
    !isUserAction(value.user_action) ||
    !isSafeParameters(value.safe_parameters)
  ) {
    return fallbackError;
  }
  return {
    code: value.code,
    subsystem: value.subsystem,
    operation_id:
      typeof value.operation_id === "string" ? value.operation_id : null,
    retryable: value.retryable,
    user_action: value.user_action,
    safe_message_key: value.safe_message_key,
    safe_parameters: value.safe_parameters,
    diagnostic_id: value.diagnostic_id,
  };
}

export function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function isSubsystem(value: unknown): value is PublicError["subsystem"] {
  return (
    value === "ipc" ||
    value === "operation" ||
    value === "stream" ||
    value === "policy"
  );
}

function isUserAction(value: unknown): value is PublicError["user_action"] {
  return (
    value === "none" ||
    value === "retry" ||
    value === "refresh" ||
    value === "review"
  );
}

function isSafeParameters(value: unknown): value is Record<string, string> {
  return (
    isRecord(value) &&
    Object.values(value).every((parameter) => typeof parameter === "string")
  );
}
