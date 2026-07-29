import { create } from "zustand";

import type { Address } from "../types";
import { ShoppingCapabilityUnavailableError } from "../services/ShoppingCapabilityUnavailableError";

interface AddressState {
  addresses: Address[];
  selectedAddressId: string | null;
  addAddress: (address: Omit<Address, "id">) => void;
  updateAddress: (id: string, address: Omit<Address, "id">) => void;
  deleteAddress: (id: string) => void;
  setDefaultAddress: (id: string) => void;
  selectAddress: (id: string) => void;
  getDefaultOrSelectedAddress: () => Address | null;
}

function unavailable(): never {
  throw new ShoppingCapabilityUnavailableError();
}

export const useAddressStore = create<AddressState>(() => ({
  addresses: [],
  selectedAddressId: null,
  addAddress: () => unavailable(),
  updateAddress: () => unavailable(),
  deleteAddress: () => unavailable(),
  setDefaultAddress: () => unavailable(),
  selectAddress: () => unavailable(),
  getDefaultOrSelectedAddress: () => unavailable(),
}));
