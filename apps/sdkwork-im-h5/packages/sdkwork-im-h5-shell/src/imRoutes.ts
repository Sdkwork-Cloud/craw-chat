export interface ImH5RouteDefinition {
  path: string;
  label: string;
}

export function createImH5AppRoutes(): ImH5RouteDefinition[] {
  return [
    {
      path: "#/chat/inbox",
      label: "Inbox",
    },
  ];
}
