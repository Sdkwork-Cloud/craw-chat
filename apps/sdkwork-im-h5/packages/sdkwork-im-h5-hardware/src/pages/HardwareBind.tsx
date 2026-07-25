import React, { useState } from "react";
import { useNavigate } from "react-router";
import { HardwareService } from "../services/HardwareService";
import { cn, IconButton, showToast } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Cpu, Smartphone, Scan, QrCode } from "lucide-react";
import { useTranslation } from "react-i18next";

export const HardwareBind: React.FC = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  
  const [deviceParams, setDeviceParams] = useState({ name: "", type: "camera", activationCode: "" });
  const [isBinding, setIsBinding] = useState(false);

  const handleBind = async () => {
    if (!deviceParams.activationCode.trim()) {
      showToast(t('hardware.bind.activationCodeEmpty'));
      return;
    }
    if (!deviceParams.name.trim()) {
      showToast(t('hardware.bind.deviceNameEmpty'));
      return;
    }
    setIsBinding(true);
    try {
      await HardwareService.bindHardware(deviceParams.name, deviceParams.type, deviceParams.activationCode);
      showToast(t('hardware.bind.bindSuccess'));
      navigate("/hardware");
    } catch (e: any) {
      showToast(e.message || t('hardware.bind.bindFailed'));
    } finally {
      setIsBinding(false);
    }
  };

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-hidden">
      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-bg-color">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          className="bg-transparent w-10 h-10 -ml-2"
          onClick={() => navigate(-1)}
        />
        <h1 className="text-[17px] font-semibold text-text-main">{t('hardware.bind.activationTitle')}</h1>
        <div className="w-10"></div>
      </header>

      <div className="flex-1 overflow-y-auto px-6 py-6 pb-12 flex flex-col items-center">
         <div className="w-24 h-24 bg-gradient-to-br from-blue-500 to-indigo-500 rounded-[28px] shadow-lg shadow-blue-500/30 flex items-center justify-center mb-8">
            <QrCode className="w-12 h-12 text-white" />
         </div>
         <h2 className="text-[20px] font-bold text-text-main mb-2">{t('hardware.bind.activationLabel')}</h2>
         <p className="text-[14px] text-text-sub text-center mb-8">
           {t('hardware.bind.activationDesc')}
         </p>

         <div className="w-full flex flex-col gap-4">
            <div className="flex flex-col gap-2">
               <label className="text-[14px] font-medium text-text-main pl-1">{t('hardware.bind.activationCodeLabel')}</label>
               <input 
                 type="text"
                 placeholder={t('hardware.bind.activationCodePlaceholder')}
                 className="w-full bg-chat-other-bg border-none border border-transparent rounded-2xl px-4 py-3.5 text-[15px] outline-none text-text-main focus:ring-2 focus:ring-blue-500/50 transition-all font-mono uppercase tracking-wider"
                 value={deviceParams.activationCode}
                 onChange={e => setDeviceParams(p => ({...p, activationCode: e.target.value.toUpperCase()}))}
               />
            </div>

            <div className="flex flex-col gap-2 mt-2">
              <label className="text-[14px] font-medium text-text-main pl-1">{t('hardware.bind.deviceTypeLabel')}</label>
              <div className="grid grid-cols-3 gap-3">
                 {[
                   { id: "camera", name: t('hardware.bind.typeCamera'), icon: <Scan className="w-5 h-5 mb-1" /> },
                   { id: "speaker", name: t('hardware.bind.typeSpeaker'), icon: <Cpu className="w-5 h-5 mb-1" /> },
                   { id: "robot", name: t('hardware.bind.typeRobot'), icon: <Smartphone className="w-5 h-5 mb-1" /> }
                 ].map(item => (
                   <div 
                     key={item.id}
                     className={cn(
                       "flex flex-col items-center justify-center py-4 rounded-2xl border transition-all cursor-pointer",
                       deviceParams.type === item.id 
                         ? "border-blue-500 bg-blue-500/5 text-blue-500" 
                         : "border-black/10 dark:border-white/10 text-text-sub"
                     )}
                     onClick={() => setDeviceParams(p => ({...p, type: item.id}))}
                   >
                     {item.icon}
                     <span className="text-[12px] font-medium">{item.name}</span>
                   </div>
                 ))}
              </div>
            </div>

            <div className="flex flex-col gap-2 mt-2">
               <label className="text-[14px] font-medium text-text-main pl-1">{t('hardware.bind.deviceNameLabel')}</label>
               <input 
                 type="text"
                 placeholder={t('hardware.bind.deviceNamePlaceholder')}
                 className="w-full bg-chat-other-bg border-none rounded-2xl px-4 py-3.5 text-[15px] outline-none text-text-main focus:ring-2 focus:ring-blue-500/50 transition-all font-medium"
                 value={deviceParams.name}
                 onChange={e => setDeviceParams(p => ({...p, name: e.target.value}))}
               />
            </div>
            
            <div className="h-24"></div>
         </div>
      </div>

      <div className="fixed bottom-0 left-0 right-0 p-4 bg-bg-color border-t border-black/5 dark:border-white/5 pb-safe z-10">
        <button 
          className={cn(
            "w-full rounded-full py-3.5 text-white text-[16px] font-medium shadow-md transition-all flex items-center justify-center gap-2",
            isBinding || !deviceParams.name.trim() || !deviceParams.activationCode.trim()
              ? "bg-blue-300 shadow-blue-300/30 pointer-events-none" 
              : "bg-gradient-to-r from-blue-500 to-indigo-500 shadow-blue-500/30 active:scale-[0.98]"
          )}
          onClick={handleBind}
        >
          {isBinding && <div className="w-5 h-5 rounded-full border-2 border-white/50 border-t-white animate-spin" />}
          {isBinding ? t('hardware.bind.activating') : t('hardware.bind.activateNow')}
        </button>
      </div>
    </div>
  );
};
