export const PC_CONSOLE_API_CONTRACT_UNAVAILABLE =
  'pc console api contract is not available';

export const PC_ADMIN_API_CONTRACT_UNAVAILABLE =
  'pc admin api contract is not available';

export function assertPcConsoleApiContractAvailable(): never {
  throw new Error(PC_CONSOLE_API_CONTRACT_UNAVAILABLE);
}

export function assertPcAdminApiContractAvailable(): never {
  throw new Error(PC_ADMIN_API_CONTRACT_UNAVAILABLE);
}
