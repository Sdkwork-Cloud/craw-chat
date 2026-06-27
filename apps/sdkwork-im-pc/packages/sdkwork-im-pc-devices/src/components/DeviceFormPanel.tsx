import React, { useState, useEffect } from "react";
import { toast } from "@sdkwork/im-pc-chat";
import { Device, deviceService } from "../services/DeviceService";

interface DeviceFormPanelProps {
  device?: Device | null;
  onClose: () => void;
  onSave: (device: Device) => void;
}

export const DeviceFormPanel: React.FC<DeviceFormPanelProps> = ({ device, onClose, onSave }) => {
  const [formData, setFormData] = useState<Partial<Device>>({
    name: "",
    macAddress: "",
    type: "camera",
    status: "unactivated",
    firmwareVersion: "1.0.0"
  });
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (device) {
      setFormData(device);
    }
  }, [device]);

  const handleSave = async () => {
    if (!formData.name || !formData.macAddress) {
      toast("请填写完整的设备信息", "error");
      return;
    }

    setIsSaving(true);
    try {
      if (device) {
        // Edit mode
        await deviceService.updateDevice(device.id, formData);
        toast("设备更新成功", "success");
        onSave({ ...device, ...formData } as Device);
      } else {
        // Add mode
        const newDevice = await deviceService.addDevice(formData as Omit<Device, "id">);
        toast("设备添加成功", "success");
        onSave(newDevice);
      }
    } catch (e) {
      toast("保存失败", "error");
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="p-8 max-w-2xl mx-auto w-full">
      <h2 className="text-2xl font-bold mb-8 text-gray-900 dark:text-gray-100">{device ? "编辑设备" : "添加新设备"}</h2>
      <div className="space-y-6">
        <div>
          <label className="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">设备名称</label>
          <input
            type="text"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            className="w-full bg-white dark:bg-[#2b2b2d] border border-gray-200 dark:border-white/10 rounded-xl px-4 py-3 text-sm text-gray-900 dark:text-gray-200 focus:border-indigo-500 outline-none"
            placeholder="输入设备名称，如“会议室摄像头”"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">MAC 地址</label>
          <input
            type="text"
            value={formData.macAddress}
            onChange={(e) => setFormData({ ...formData, macAddress: e.target.value })}
            className="w-full bg-white dark:bg-[#2b2b2d] border border-gray-200 dark:border-white/10 rounded-xl px-4 py-3 text-sm text-gray-900 dark:text-gray-200 focus:border-indigo-500 outline-none font-mono uppercase"
            placeholder="00:1A:2B:3C:4D:5E"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">设备类型</label>
          <select 
            value={formData.type}
            onChange={(e) => setFormData({ ...formData, type: e.target.value as any })}
            className="w-full bg-white dark:bg-[#2b2b2d] border border-gray-200 dark:border-white/10 rounded-xl px-4 py-3 text-sm text-gray-900 dark:text-gray-200 focus:border-indigo-500 outline-none"
          >
            <option value="camera">摄像头</option>
            <option value="speaker">智能音箱</option>
            <option value="display">显示设备</option>
            <option value="sensor">传感器</option>
          </select>
        </div>
        <div className="pt-4 flex gap-4">
          <button
            className="flex-1 py-3 bg-indigo-600 hover:bg-indigo-700 dark:hover:bg-indigo-500 text-white rounded-xl font-medium transition-colors flex items-center justify-center disabled:opacity-50"
            onClick={handleSave}
            disabled={isSaving}
          >
            {isSaving ? (
              <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
            ) : (
              "保存设备"
            )}
          </button>
          <button
            className="flex-1 py-3 bg-gray-100 dark:bg-[#2b2b2d] hover:bg-gray-200 dark:hover:bg-white/10 text-gray-700 dark:text-gray-300 rounded-xl font-medium transition-colors"
            onClick={onClose}
          >
            取消
          </button>
        </div>
      </div>
    </div>
  );
};
