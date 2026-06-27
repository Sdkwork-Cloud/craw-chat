import React from "react";
import { motion } from "motion/react";
import { X, Check, Edit2, Trash2, Plus, Save } from "lucide-react";
import { useTranslation } from "react-i18next";

export const CheckoutAddressModal = ({
  showAddressModal,
  setShowAddressModal,
  addressForm,
  setAddressForm,
  handleSaveAddress,
  addresses,
  selectedAddressId,
  setSelectedAddressId,
  handleSetDefault,
  handleDeleteAddress,
}: any) => {
  const { t } = useTranslation(["checkout", "common"]);

  if (!showAddressModal) return null;

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="absolute inset-0 bg-black/60 z-50 flex items-center justify-center p-4 backdrop-blur-sm"
    >
      <motion.div
        initial={{ scale: 0.95, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        exit={{ scale: 0.95, opacity: 0 }}
        className="bg-[#2b2b2d] rounded-2xl w-full max-w-lg overflow-hidden shadow-2xl border border-white/10 flex flex-col max-h-[80vh]"
      >
        <div className="flex items-center justify-between p-6 border-b border-white/5 shrink-0">
          <h3 className="text-lg font-medium text-gray-100">
            {t("checkout:selectAddress")}
          </h3>
          <button
            onClick={() => setShowAddressModal(false)}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <X size={20} />
          </button>
        </div>
        <div className="flex-1 overflow-y-auto p-4 custom-scrollbar space-y-3">
          {addressForm ? (
            <form onSubmit={handleSaveAddress} className="space-y-4">
              <div>
                <label className="text-sm font-medium text-gray-400 mb-1.5 block">收货人</label>
                <input type="text" value={addressForm.name || ""} onChange={e => setAddressForm({...addressForm, name: e.target.value})} className="w-full bg-black/20 border border-white/10 focus:border-pink-500/50 outline-none rounded-lg px-4 py-2.5 text-white text-sm transition-colors" placeholder="真实姓名" required />
              </div>
              <div>
                <label className="text-sm font-medium text-gray-400 mb-1.5 block">手机号</label>
                <input type="text" value={addressForm.phone || ""} onChange={e => setAddressForm({...addressForm, phone: e.target.value})} className="w-full bg-black/20 border border-white/10 focus:border-pink-500/50 outline-none rounded-lg px-4 py-2.5 text-white text-sm transition-colors" placeholder="11位手机号" required />
              </div>
              <div>
                <label className="text-sm font-medium text-gray-400 mb-1.5 block">详细地址</label>
                <textarea value={addressForm.address || ""} onChange={e => setAddressForm({...addressForm, address: e.target.value})} className="w-full bg-black/20 border border-white/10 focus:border-pink-500/50 outline-none rounded-lg px-4 py-2.5 text-white text-sm h-24 resize-none transition-colors custom-scrollbar" placeholder="省市区 街道门牌号" required />
              </div>
              <div className="flex items-center gap-2 mt-2 cursor-pointer w-fit" onClick={() => setAddressForm({...addressForm, isDefault: !addressForm.isDefault})}>
                <div className={`w-4 h-4 rounded border flex items-center justify-center transition-colors ${addressForm.isDefault ? "bg-pink-500 border-pink-500" : "border-white/20"}`}>
                  {addressForm.isDefault && <Check size={12} className="text-white" />}
                </div>
                <span className="text-sm text-gray-300">设为默认收货地址</span>
              </div>
              <div className="flex justify-end gap-3 pt-4 border-t border-white/5 mt-6">
                <button type="button" onClick={() => setAddressForm(null)} className="px-5 py-2.5 rounded-lg text-sm font-medium text-gray-400 hover:text-white bg-white/5 hover:bg-white/10 transition-colors">取消</button>
                <button type="submit" className="px-5 py-2.5 rounded-lg text-sm font-bold text-white bg-pink-500 hover:bg-pink-600 transition-colors flex items-center gap-2">
                  <Save size={16} /> 保存地址
                </button>
              </div>
            </form>
          ) : (
            <>
              {addresses.map((addr: any) => (
                <div
                  key={addr.id}
                  onClick={() => {
                    setSelectedAddressId(addr.id);
                    setShowAddressModal(false);
                  }}
                  className={`group p-4 rounded-xl border flex items-start gap-4 cursor-pointer transition-all ${selectedAddressId === addr.id ? 'bg-pink-500/10 border-pink-500/50' : 'bg-white/5 border-transparent hover:bg-white/10 hover:border-white/20'}`}
                >
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-3 mb-2">
                      <span className="text-base font-medium text-gray-200">
                        {addr.name}
                      </span>
                      <span className="text-sm text-gray-400">
                        {addr.phone}
                      </span>
                      {addr.isDefault && (
                        <span className="text-xs bg-pink-500/20 text-pink-400 px-2 py-0.5 rounded-full font-medium">
                          {t("checkout:default")}
                        </span>
                      )}
                    </div>
                    <p className="text-sm text-gray-400 leading-relaxed truncate mb-3">
                      {addr.address}
                    </p>
                    
                    <div className="flex items-center gap-4 opacity-0 group-hover:opacity-100 transition-opacity">
                      {!addr.isDefault && (
                        <span onClick={(e) => handleSetDefault(e, addr.id)} className="text-xs text-blue-400 hover:text-blue-300 transition-colors">设为默认</span>
                      )}
                      <span onClick={(e) => { e.stopPropagation(); setAddressForm(addr); }} className="text-xs text-gray-400 hover:text-white flex items-center gap-1 transition-colors">
                        <Edit2 size={12} /> 编辑
                      </span>
                      {addresses.length > 1 && (
                        <span onClick={(e) => handleDeleteAddress(e, addr.id)} className="text-xs text-red-500/80 hover:text-red-400 flex items-center gap-1 transition-colors">
                          <Trash2 size={12} /> 删除
                        </span>
                      )}
                    </div>
                  </div>
                  {selectedAddressId === addr.id && (
                    <Check
                      size={20}
                      className="text-pink-500 shrink-0 mt-1"
                    />
                  )}
                </div>
              ))}

              <button onClick={() => setAddressForm({})} className="w-full p-4 rounded-xl border border-dashed border-white/20 flex flex-col items-center justify-center gap-2 text-gray-400 hover:text-white hover:border-white/40 transition-colors mt-4 bg-white/5 hover:bg-white/10">
                <Plus size={24} />
                <span className="text-sm">{t("checkout:addNewAddress")}</span>
              </button>
            </>
          )}
        </div>
      </motion.div>
    </motion.div>
  );
};
