export interface PortalDataAvailability {
  state: 'available' | 'partial' | 'unavailable';
  source: string;
  complete: boolean;
  reason?: string;
}
