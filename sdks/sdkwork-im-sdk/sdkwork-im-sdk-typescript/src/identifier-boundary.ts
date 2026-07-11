export function requireStringIdentifier(value: unknown, fieldName: string): string {
  if (typeof value !== 'string') {
    throw new TypeError(`${fieldName} must be a string identifier`);
  }
  const trimmed = value.trim();
  if (!trimmed) {
    throw new TypeError(`${fieldName} must be a non-empty string identifier`);
  }
  return trimmed;
}
