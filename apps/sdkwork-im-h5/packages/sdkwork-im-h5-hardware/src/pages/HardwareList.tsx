import React, { useState, useEffect, useRef } from "react";
import { useNavigate } from "react-router";
import { HardwareService } from "../services/HardwareService";
import { Hardware } from "../types";
import { cn, IconButton, showToast, ActionSheet, useLongPress } from "@sdkwork/im-h5-commons";
import { ChevronLeft, Plus, Cpu, Speaker, Camera, Bot, Settings, Trash2, Link } from "lucide-react";
import { HardwareCard } from "../components/HardwareCard";
import { useTranslation } from "react-i18next";

export const HardwareList: React.FC = () => {
  const { t } = useTranslation();

  
const navigate = useNavigate();
  
  const [hardwareList, setHardwareList] = useState<Hardware[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [actionSheetItem, setActionSheetItem] = useState<Hardware | null>(null);
  const [isLongPressed, setIsLongPressed] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setIsLoading(true);
    try {
      const list = await HardwareService.getHardwareList();
      setHardwareList(list);
    } catch {
      showToast(t('hardware.failedToLoad'));
    } finally {
      setIsLoading(false);
    }
  };

  const longPressItemRef = useRef<Hardware | null>(null);
  const longPressHandlers = useLongPress({
    delay: 500,
    onLongPress: () => {
      const hw = longPressItemRef.current;
      if (hw) {
        setIsLongPressed(true);
        setActionSheetItem(hw);
      }
    },
  });

  const startLongPress = (hw: Hardware) => ({
    onPointerDown: () => {
      longPressItemRef.current = hw;
      setIsLongPressed(false);
      longPressHandlers.onPointerDown();
    },
    onPointerUp: longPressHandlers.onPointerUp,
    onPointerLeave: () => {
      longPressHandlers.onPointerLeave();
      setIsLongPressed(false);
    },
    onContextMenu: (e: React.MouseEvent) => {
      e.preventDefault();
      longPressHandlers.onPointerUp();
      setIsLongPressed(true);
      setActionSheetItem(hw);
    },
  });

  const handleActionSheetSelect = (action: string) => {
  if (!actionSheetItem) return;
    if (action === 'edit') {
       navigate(`/hardware/${actionSheetItem.id}`);
    } else if (action === 'delete') {
       setHardwareList(prev => prev.filter(c => c.id !== actionSheetItem.id));
       showToast(t('hardware.unbindSuccess'));
    }
    setActionSheetItem(null);
  };

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-hidden w-full relative">
      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe bg-bg-color">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          className="bg-transparent w-10 h-10 -ml-2"
          onClick={() => navigate(-1)}
        />
        <h1 className="text-[17px] font-semibold text-text-main">{t('hardware.title')}</h1>
        <IconButton
          icon={<Plus className="w-5 h-5 text-text-main" />}
          className="bg-black/5 dark:bg-white/5 w-8 h-8 rounded-full"
          onClick={() => navigate("/hardware/bind")}
        />
      </header>

      <div className="flex-1 overflow-y-auto px-4 py-4 pb-12 w-full">
        {isLoading ? (
          <div className="flex flex-col h-40 items-center justify-center text-text-sub opacity-70">
            <div className="w-6 h-6 rounded-full border-2 border-text-sub border-t-transparent animate-spin mb-2"></div>
            <span className="text-[14px]">{t('hardware.loading')}</span>
          </div>
        ) : hardwareList.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-60 text-text-sub">
            <Cpu className="w-16 h-16 opacity-30 mb-4" />
            <span className="text-[15px]">{t('hardware.empty')}</span>
            <button 
              className="mt-6 px-6 py-2 bg-gradient-to-r from-blue-500 to-indigo-500 text-white rounded-full text-[14px] font-medium shadow-md shadow-blue-500/20 active:scale-95 transition-transform"
              onClick={() => navigate("/hardware/bind")}
            >
              {t('hardware.bindNew')}
            </button>
          </div>
        ) : (
          <div className="flex flex-col gap-3 w-full">
            {hardwareList.map(hw => (
               <HardwareCard 
                  key={hw.id}
                  hardware={hw}
                  onClick={() => {
                    if (isLongPressed) {
                      setIsLongPressed(false);
                      return;
                    }
                    navigate(`/hardware/${hw.id}`)
                  }}
                  onLongPressProps={startLongPress(hw)}
               />
            ))}
          </div>
        )}
      </div>

      {actionSheetItem && (
        <ActionSheet
          isOpen={true}
          title={t('hardware.actionTitle', { name: actionSheetItem.name })}
          options={[
            { label: t('hardware.unbindDevice'), danger: true, onClick: () => handleActionSheetSelect('delete') }
          ]}
          onClose={() => setActionSheetItem(null)}
        />
      )}
    </div>
  );
};
