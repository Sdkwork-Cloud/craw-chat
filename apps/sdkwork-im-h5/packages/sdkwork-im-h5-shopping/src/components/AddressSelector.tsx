import { useTranslation } from "react-i18next";
import React, { useState } from "react";
import { ChevronRight, MapPin, Check, Plus, Edit2, ChevronLeft } from "lucide-react";
import { useAddressStore } from "../store/useAddressStore";
import { Address } from "../types";

interface AddressSelectorProps {
  onAddressChange?: (address: Address | null) => void;
}

export const AddressSelector: React.FC<AddressSelectorProps> = ({ onAddressChange }) => {
  const { t } = useTranslation();
const { addresses, getDefaultOrSelectedAddress, selectAddress, deleteAddress } = useAddressStore();
  const currentAddress = getDefaultOrSelectedAddress();

  const [isListVisible, setIsListVisible] = useState(false);
  const [editingAddress, setEditingAddress] = useState<Address | null | 'new'>(null);

  const handleSelect = (id: string) => {
  selectAddress(id);
    setIsListVisible(false);
  };

  return (
    <>
      <div 
        className="bg-chat-other-bg rounded-xl p-4 mb-3 flex items-center gap-3 cursor-pointer active:scale-[0.98] transition-transform"
        onClick={() => setIsListVisible(true)}
      >
        <div className="w-8 h-8 rounded-full bg-[#FA5151]/10 flex items-center justify-center shrink-0">
          <MapPin className="w-4 h-4 text-[#FA5151]" />
        </div>
        <div className="flex-1">
          {currentAddress ? (
            <>
              <div className="flex items-center gap-2 mb-1">
                <span className="text-[16px] font-medium text-text-main">
                  {currentAddress.name}
                </span>
                <span className="text-[14px] text-text-sub">{currentAddress.phone}</span>
                {currentAddress.isDefault && (
                  <span className="text-[10px] bg-[#FA5151]/10 text-[#FA5151] px-1 rounded-sm">{t('shopping.auto_13c7cc', '默认')}</span>
                )}
              </div>
              <div className="text-[13px] text-text-main line-clamp-2 leading-tight">
                {currentAddress.province} {currentAddress.city} {currentAddress.district} {currentAddress.detail}
              </div>
            </>
          ) : (
            <span className="text-[15px] font-medium text-text-main">{t('shopping.auto_77761738', '请选择收货地址')}</span>
          )}
        </div>
        <ChevronRight className="w-5 h-5 text-text-sub/60 shrink-0" />
      </div>

      {isListVisible && (
        <AddressListModal
          addresses={addresses}
          selectedId={currentAddress?.id}
          onClose={() => setIsListVisible(false)}
          onSelect={handleSelect}
          onEdit={(address) => setEditingAddress(address)}
          onAdd={() => setEditingAddress('new')}
        />
      )}

      {editingAddress && (
        <AddressEditModal
          address={editingAddress === 'new' ? undefined : editingAddress}
          onClose={() => setEditingAddress(null)}
          onSave={() => setEditingAddress(null)} // state handles itself
          onDelete={(id) => {
            deleteAddress(id);
            setEditingAddress(null);
          }}
        />
      )}
    </>
  );
};

const AddressListModal = ({ addresses, selectedId, onClose, onSelect, onEdit, onAdd }: any) => {
  const { t } = useTranslation();
  
return (
    <div className="fixed inset-0 z-[100] flex flex-col justify-end">
      <div className="absolute inset-0 bg-black/60 transition-opacity" onClick={onClose} />
      <div className="bg-bg-color w-full h-[75vh] rounded-t-2xl flex flex-col relative animate-in slide-in-from-bottom duration-300">
        <div className="flex items-center justify-between p-4 border-b border-border-color shrink-0">
          <span className="w-6" />
          <span className="text-[16px] font-medium text-text-main">{t('shopping.auto_6b26a081', '选择收货地址')}</span>
          <span onClick={onClose} className="w-6 text-[22px] text-text-sub cursor-pointer leading-none">&times;</span>
        </div>
        
        <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-3">
          {addresses.map((addr: Address) => (
            <div key={addr.id} className="bg-chat-other-bg rounded-xl p-4 flex items-center border border-border-color/30" onClick={() => onSelect(addr.id)}>
              <div className="w-6 shrink-0 mr-2 flex justify-center">
                {selectedId === addr.id && <Check className="w-5 h-5 text-[#FA5151]" />}
              </div>
              <div className="flex-1 mr-2">
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-[15px] font-medium text-text-main">{addr.name}</span>
                  <span className="text-[13px] text-text-sub">{addr.phone}</span>
                  {addr.isDefault && (
                    <span className="text-[10px] bg-[#FA5151]/10 text-[#FA5151] px-1 rounded-sm">{t('shopping.auto_13c7cc', '默认')}</span>
                  )}
                </div>
                <div className="text-[13px] text-text-main line-clamp-2 leading-tight">
                  {addr.province}{addr.city}{addr.district} {addr.detail}
                </div>
              </div>
              <div 
                className="w-10 h-10 flex items-center justify-center shrink-0 cursor-pointer border-l border-border-color/50"
                onClick={(e) => {
                  e.stopPropagation();
                  onEdit(addr);
                }}
              >
                <Edit2 className="w-4 h-4 text-text-sub" />
              </div>
            </div>
          ))}
          {addresses.length === 0 && (
            <div className="py-10 flex flex-col items-center justify-center text-text-sub">
              <MapPin className="w-10 h-10 mb-2 opacity-20" />
              <span className="text-[14px]">{t('shopping.auto_n482162e1', '暂无收货地址')}</span>
            </div>
          )}
        </div>

        <div className="p-4 bg-bg-color pb-safe border-t border-border-color shrink-0">
          <button 
            className="w-full h-11 bg-[#FA5151] text-white rounded-full font-medium flex items-center justify-center gap-1 active:scale-[0.98] transition-transform"
            onClick={onAdd}
          >
            <Plus className="w-5 h-5" />{t('shopping.auto_n694eb191', '新增收货地址')}</button>
        </div>
      </div>
    </div>
  );
};

const AddressEditModal = ({ address, onClose, onSave, onDelete }: any) => {
  const { t } = useTranslation();
  
const { addAddress, updateAddress } = useAddressStore();
  
  const isEdit = !!address;
  const [formData, setFormData] = useState({
    name: address?.name || "",
    phone: address?.phone || "",
    province: address?.province || "",
    city: address?.city || "",
    district: address?.district || "",
    detail: address?.detail || "",
    isDefault: address?.isDefault || false,
  });

  const handleSave = () => {
  if (!formData.name || !formData.phone || !formData.detail) {
      alert(t('shopping.auto_fn_7cdd838c', '请填写完整收货信息'));
      return;
    }
    
    // Auto-fill some regions if user just typed the detail (simplified for demo)
    const toSave = {
      ...formData,
      province: formData.province || "浙江省",
      city: formData.city || "杭州市",
      district: formData.district || "余杭区",
    };

    if (isEdit) {
      updateAddress(address.id, toSave);
    } else {
      addAddress(toSave);
    }
    onSave();
  };

  return (
    <div className="fixed inset-0 z-[110] bg-bg-color animate-in slide-in-from-right duration-300 flex flex-col">
      <header className="flex items-center justify-between px-2 pt-safe h-[56px] border-b border-border-color bg-chat-other-bg shrink-0">
        <div className="w-10 h-10 flex items-center justify-center cursor-pointer" onClick={onClose}>
          <ChevronLeft className="w-6 h-6 text-text-main" />
        </div>
        <span className="text-[17px] font-medium text-text-main">{isEdit ? t('shopping.edit_address', '编辑收货地址') : t('shopping.auto_n694eb191', '新增收货地址')}</span>
        <div className="w-10 h-10 flex items-center justify-center text-[14px] text-[#FA5151]" onClick={handleSave}>{t('shopping.auto_a071b', '保存')}</div>
      </header>

      <div className="flex-1 overflow-y-auto">
        <div className="bg-chat-other-bg mt-3">
          <div className="flex items-center px-4 min-h-[50px] border-b border-border-color">
            <span className="w-[80px] text-[15px] text-text-main">{t('shopping.auto_18d5629', '收货人')}</span>
            <input 
              type="text" 
              placeholder={t('shopping.auto_prop_a88ea', '名字')}
              className="flex-1 bg-transparent border-none outline-none text-[15px] text-text-main"
              value={formData.name}
              onChange={(e) => setFormData({...formData, name: e.target.value})}
            />
          </div>
          <div className="flex items-center px-4 min-h-[50px] border-b border-border-color">
            <span className="w-[80px] text-[15px] text-text-main">{t('shopping.auto_2e3c9979', '手机号码')}</span>
            <input 
              type="tel" 
              placeholder={t('shopping.auto_prop_17dcf88', '手机号')}
              className="flex-1 bg-transparent border-none outline-none text-[15px] text-text-main"
              value={formData.phone}
              onChange={(e) => setFormData({...formData, phone: e.target.value})}
            />
          </div>
          <div className="flex items-start px-4 py-3 border-b border-border-color">
            <span className="w-[80px] text-[15px] text-text-main mt-0.5">{t('shopping.auto_417eedb0', '详细地址')}</span>
            <textarea 
              placeholder={t('shopping.auto_prop_50c1bf2e', '如街道、门牌号、小区、乡镇、村等')}
              className="flex-1 bg-transparent border-none outline-none text-[15px] text-text-main resize-none h-[80px]"
              value={formData.detail}
              onChange={(e) => setFormData({...formData, detail: e.target.value})}
            />
          </div>
        </div>

        <div className="bg-chat-other-bg mt-3 px-4 min-h-[50px] flex items-center justify-between">
          <span className="text-[15px] text-text-main">{t('shopping.auto_6ce56b89', '设为默认收货地址')}</span>
          <div 
            className={`w-[42px] h-[24px] rounded-full transition-colors relative cursor-pointer ${formData.isDefault ? 'bg-[#FA5151]' : 'bg-gray-300 dark:bg-zinc-700'}`}
            onClick={() => setFormData({...formData, isDefault: !formData.isDefault})}
          >
            <div className={`absolute top-[2px] transition-all duration-300 w-[20px] h-[20px] bg-white rounded-full shadow-sm ${formData.isDefault ? 'left-[20px]' : 'left-[2px]'}`} />
          </div>
        </div>

        {isEdit && (
          <div 
            className="bg-chat-other-bg mt-3 px-4 min-h-[50px] flex items-center text-[#FA5151] text-[15px] cursor-pointer active:bg-black/5 dark:active:bg-white/5"
            onClick={() => {
              if (window.confirm(t('shopping.confirm_delete_address', '确定要删除该地址吗？'))) {
                onDelete(address.id);
              }
            }}
          >{t('shopping.auto_n649c6d3b', '删除收货地址')}</div>
        )}
      </div>
    </div>
  );
};
