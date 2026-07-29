export type ReadinessState =
  | "bootstrapping"
  | "locked"
  | "opening"
  | "recovering"
  | "restoring"
  | "ready"
  | "recovery-mode";

export type ShutdownState =
  | "running"
  | "reviewing"
  | "quiescing"
  | "draining"
  | "forced-cleanup"
  | "persisting"
  | "exiting";

export interface CloseBlockers {
  dirtyRemoteBuffers: number;
  transfers: number;
  tunnels: number;
  recordings: number;
  protectedMigration: boolean;
}

export const emptyCloseBlockers: CloseBlockers = {
  dirtyRemoteBuffers: 0,
  transfers: 0,
  tunnels: 0,
  recordings: 0,
  protectedMigration: false,
};

export function hasCloseBlockers(blockers: CloseBlockers): boolean {
  return (
    blockers.dirtyRemoteBuffers > 0 ||
    blockers.transfers > 0 ||
    blockers.tunnels > 0 ||
    blockers.recordings > 0 ||
    blockers.protectedMigration
  );
}

export class FrontendLifecycleCoordinator {
  private readiness: ReadinessState = "bootstrapping";
  private shutdown: ShutdownState = "running";
  private confirmationEpoch = 0;

  get snapshot(): Readonly<{
    readiness: ReadinessState;
    shutdown: ShutdownState;
    confirmationEpoch: number;
  }> {
    return {
      readiness: this.readiness,
      shutdown: this.shutdown,
      confirmationEpoch: this.confirmationEpoch,
    };
  }

  setReadiness(readiness: ReadinessState): void {
    this.readiness = readiness;
  }

  requestClose(): void {
    if (this.shutdown !== "running") {
      throw new Error("Close review is already active.");
    }
    this.shutdown = "reviewing";
  }

  cancelClose(): void {
    if (this.shutdown !== "reviewing") {
      throw new Error("There is no close review to cancel.");
    }
    this.shutdown = "running";
  }

  beginShutdown(): void {
    if (this.shutdown !== "reviewing") {
      throw new Error("Shutdown requires an active close review.");
    }
    this.shutdown = "quiescing";
  }

  onOsSessionLocked(): void {
    this.confirmationEpoch += 1;
    if (this.shutdown === "reviewing") {
      this.shutdown = "running";
    }
  }

  onWebviewLost(): void {
    this.confirmationEpoch += 1;
  }
}
