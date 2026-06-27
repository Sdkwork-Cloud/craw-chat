import React, { useState } from "react";
import { motion } from "motion/react";
import { X, Zap } from "lucide-react";
import { Device, deviceService } from "../services/DeviceService";
import { toast } from "@sdkwork/im-pc-chat";

interface ActivationModalProps {
  showActivationModal: boolean;
  setShowActivationModal: (show: boolean) => void;
  selectedDevice: Device | null;
  onActivated: (d: Device) => void;
}

export const ActivationModal: React.FC<ActivationModalProps> = ({
  showActivationModal, setShowActivationModal, selectedDevice, onActivated
}) => {
  const [activationCode, setActivationCode] = useState("");
  const [isActivating, setIsActivating] = useState(false);

  // If there's no selected device, there shouldn't be an active modal, but
  // keep animation working on exit
  if (!selectedDevice) return null;

  const handleActivate = async () => {
    if (!activationCode) {
      toast("请输入激活码", "error");
      return;
    }
    
    setIsActivating(true);
    try {
      await deviceService.activateDevice(selectedDevice.id, activationCode);
      toast("设备激活成功", "success");
      onActivated({ ...selectedDevice, status: 'offline' }); // Active device but might be offline initially
      setShowActivationModal(false);
      setActivationCode("");
    } catch (e: any) {
      toast(e.message || "激活失败，请检查激活码", "error");
    } finally {
      setIsActivating(false);
    }
  };

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4"
    >
      <motion.div
        initial={{ opacity: 0, scale: 0.95, y: 10 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.95, y: 10 }}
        className="w-full max-w-md bg-white dark:bg-[#252528] border border-gray-200 dark:border-white/10 rounded-2xl shadow-2xl overflow-hidden"
      >
        <div className="flex justify-between items-center p-6 border-b border-gray-200 dark:border-white/5">
          <h2 className="text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
            <Zap size={20} className="text-amber-500" />
            激活设备
          </h2>
          <button
            onClick={() => setShowActivationModal(false)}
            className="text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
          >
            <X size={20} />
          </button>
        </div>
        <div className="p-6 space-y-6">
          <div className="bg-amber-50 dark:bg-amber-500/10 border border-amber-200 dark:border-amber-500/20 rounded-xl p-4 text-sm text-amber-600 dark:text-amber-500 text-center">
            正在激活设备：<span className="font-bold">{selectedDevice.name}</span>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">激活码</label>
            <input
              type="text"
              value={activationCode}
              onChange={(e) => setActivationCode(e.target.value)}
              className="w-full bg-gray-50 dark:bg-[#1a1a1c] border border-gray-200 dark:border-white/10 rounded-xl px-4 py-3 text-sm text-gray-900 dark:text-gray-200 focus:border-amber-500 outline-none font-mono tracking-widest text-center"
              placeholder="请输入6位或以上激活码"
            />
          </div>
          
          <button
            className="w-full py-3 bg-amber-600 hover:bg-amber-500 text-white rounded-xl font-medium transition-colors disabled:opacity-50 flex items-center justify-center gap-2"
            onClick={handleActivate}
            disabled={isActivating || !activationCode}
          >
            {isActivating ? (
              <>
                <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                激活中...
              </>
            ) : (
              "验证激活码并激活"
            )}
          </button>
        </div>
      </motion.div>
    </motion.div>
  );
};
