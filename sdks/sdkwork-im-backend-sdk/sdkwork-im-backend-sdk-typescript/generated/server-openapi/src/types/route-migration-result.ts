export interface RouteMigrationResult {
  migratedRouteCount: string;
  sourceDrainStatus: string;
  sourceNodeId: string;
  sourceRebalanceState: string;
  targetDrainStatus: string;
  targetNodeId: string;
  targetRebalanceState: string;
}
