import { Component, type ErrorInfo, type ReactNode } from "react";
import { FatalBootstrap } from "./FatalBootstrap";

interface BootstrapBoundaryProps {
  children: ReactNode;
}

interface BootstrapBoundaryState {
  failed: boolean;
}

export class BootstrapBoundary extends Component<
  BootstrapBoundaryProps,
  BootstrapBoundaryState
> {
  public state: BootstrapBoundaryState = { failed: false };

  public static getDerivedStateFromError(): BootstrapBoundaryState {
    return { failed: true };
  }

  public componentDidCatch(error: Error, info: ErrorInfo): void {
    console.error("Relio interface bootstrap failed.", {
      message: error.message,
      componentStack: info.componentStack,
    });
  }

  public render(): ReactNode {
    return this.state.failed ? <FatalBootstrap /> : this.props.children;
  }
}
