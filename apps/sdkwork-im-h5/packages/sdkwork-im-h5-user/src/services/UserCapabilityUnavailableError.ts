export class UserCapabilityUnavailableError extends Error {
  constructor(capability: string) {
    super(
      `${capability} is unavailable because its owner SDK and authorization model are not composed.`,
    );
    this.name = "UserCapabilityUnavailableError";
  }
}
