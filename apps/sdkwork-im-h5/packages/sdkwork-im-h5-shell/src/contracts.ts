import type { ComponentType, ReactNode } from "react";

import type { ImH5RouteMetadata } from "@sdkwork/im-h5-core/routes";

export interface ImH5RouteContribution extends ImH5RouteMetadata {
  readonly index?: boolean;
  readonly relativePath?: string;
  readonly render: () => ReactNode;
  readonly children?: readonly ImH5RouteContribution[];
}

export interface ImH5NavigationContribution {
  readonly id: string;
  readonly moduleId: string;
  readonly path: string;
  readonly labelKey: string;
  readonly icon: ComponentType<{
    className?: string;
    strokeWidth?: number;
  }>;
  /** Filled variant rendered when the tab is selected (original tab bar UI). */
  readonly activeIcon?: ComponentType<{
    className?: string;
    strokeWidth?: number;
  }>;
}

export interface ImH5CapabilityModule {
  readonly id: ImH5ModuleId;
  readonly routes: readonly ImH5RouteContribution[];
  readonly navigation?: readonly ImH5NavigationContribution[];
  readonly lifecycle?: ComponentType;
}

export type ImH5ModuleId =
  | "chat"
  | "contacts"
  | "user"
  | "agents"
  | "knowledge"
  | "drive"
  | "orders"
  | "shop"
  | "calendar"
  | "notary"
  | "approval"
  | "report"
  | "attendance"
  | "enterprise"
  | "devices"
  | "community"
  | "voice"
  | "course"
  | "videogen"
  | "imagegen"
  | "musicgen"
  | "writing"
  | "meeting"
  | "moments"
  | "music"
  | "channels"
  | "recruitment"
  | "membership";
