import { useTranslation } from "react-i18next";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import { Address } from "../types";

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

const INITIAL_ADDRESSES: Address[] = [
  {
    id: "addr_1",
    name: "张三",
    phone: "13800138000",
    province: "浙江省",
    city: "杭州市",
    district: "余杭区",
    detail: "仓前街道 梦想小镇天使村11号",
    isDefault: true,
  },
  {
    id: "addr_2",
    name: "李四",
    phone: "13912345678",
    province: "北京市",
    city: "北京市",
    district: "朝阳区",
    detail: "望京SOHO T3 A座",
    isDefault: false,
  }
];

export const useAddressStore = create<AddressState>()(
  persist(
    (set, get) => ({
      addresses: INITIAL_ADDRESSES,
      selectedAddressId: INITIAL_ADDRESSES[0].id,
      
      addAddress: (addressData) => set((state) => {
        const newId = `addr_${Date.now()}`;
        const isFirst = state.addresses.length === 0;
        const newAddress = { 
          ...addressData, 
          id: newId,
          isDefault: isFirst ? true : addressData.isDefault
        };
        
        let newAddresses = [...state.addresses, newAddress];
        if (newAddress.isDefault) {
          newAddresses = newAddresses.map(a => ({ ...a, isDefault: a.id === newId }));
        }
        
        return { addresses: newAddresses };
      }),
      
      updateAddress: (id, addressData) => set((state) => {
        let newAddresses = state.addresses.map(a => 
          a.id === id ? { ...a, ...addressData } : a
        );
        
        if (addressData.isDefault) {
          newAddresses = newAddresses.map(a => ({ ...a, isDefault: a.id === id }));
        }
        
        return { addresses: newAddresses };
      }),
      
      deleteAddress: (id) => set((state) => {
        const newAddresses = state.addresses.filter(a => a.id !== id);
        
        // If we deleted the default, make the first one default
        const hadDefault = newAddresses.some(a => a.isDefault);
        if (newAddresses.length > 0 && !hadDefault) {
          newAddresses[0].isDefault = true;
        }

        let newSelectedId = state.selectedAddressId;
        if (newSelectedId === id) {
          newSelectedId = newAddresses.find(a => a.isDefault)?.id || (newAddresses.length > 0 ? newAddresses[0].id : null);
        }
        
        return { 
          addresses: newAddresses,
          selectedAddressId: newSelectedId
        };
      }),
      
      setDefaultAddress: (id) => set((state) => ({
        addresses: state.addresses.map(a => ({ ...a, isDefault: a.id === id }))
      })),
      
      selectAddress: (id) => set({ selectedAddressId: id }),
      
      getDefaultOrSelectedAddress: () => {
        const state = get();
        if (!state.addresses || state.addresses.length === 0) return null;
        
        if (state.selectedAddressId) {
          const selected = state.addresses.find(a => a.id === state.selectedAddressId);
          if (selected) return selected;
        }
        
        return state.addresses.find(a => a.isDefault) || state.addresses[0];
      }
    }),
    {
      name: "sdkwork_im_h5-address-storage"
    }
  )
);
