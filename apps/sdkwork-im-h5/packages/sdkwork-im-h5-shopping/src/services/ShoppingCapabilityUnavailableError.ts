export class ShoppingCapabilityUnavailableError extends Error {
  constructor() {
    super("Shopping is unavailable because the Shop and Order owner SDKs are not composed.");
    this.name = "ShoppingCapabilityUnavailableError";
  }
}
