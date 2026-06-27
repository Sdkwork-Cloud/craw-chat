import React, { useState, useEffect } from "react";
import { AnimatePresence } from "motion/react";
import { Server } from "lucide-react";
import { toast } from "@sdkwork/im-pc-chat";
import { deviceService, Device } from "../services/DeviceService";
import { DeviceSidebar } from "./DeviceSidebar";
import { DeviceFormPanel } from "./DeviceFormPanel";
import { DeviceDetailPanel } from "./DeviceDetailPanel";
import { BindAgentModal } from "./BindAgentModal";
import { ActivationModal } from "./ActivationModal";

interface DevicesViewProps {
  onEditAgent?: (agentId: string) => void;
}

export const DevicesView: React.FC<DevicesViewProps> = ({ onEditAgent }) => {
  const [devices, setDevices] = useState<Device[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);
  const [isAdding, setIsAdding] = useState(false);
  const [isEditing, setIsEditing] = useState(false);
  const [showBindModal, setShowBindModal] = useState<Device | null>(null);
  const [showActivationModal, setShowActivationModal] = useState(false);

  useEffect(() => {
    loadDevices();
  }, []);

  const loadDevices = async () => {
    setLoading(true);
    try {
      const data = await deviceService.getDevices();
      setDevices(data);
    } catch (e) {
      toast("加载设备失败", "error");
    } finally {
      setLoading(false);
    }
  };

  const handleDeviceSelected = (device: Device) => {
    setSelectedDevice(device);
    setIsAdding(false);
    setIsEditing(false);
  };

  const handleDeviceUpdate = (updatedDevice: Device) => {
    setSelectedDevice(updatedDevice);
    setDevices(devices.map(d => d.id === updatedDevice.id ? updatedDevice : d));
  };

  const handleDeleteDevice = async (device: Device) => {
    if (window.confirm(`确定要删除设备 ${device.name} 吗？`)) {
      try {
        await deviceService.deleteDevice(device.id);
        const nextDevices = devices.filter(d => d.id !== device.id);
        setDevices(nextDevices);
        setSelectedDevice(null);
        toast("设备删除成功", "success");
      } catch (e) {
        toast("设备删除失败", "error");
      }
    }
  };

  return (
    <div className="flex-1 flex bg-white dark:bg-[#1e1e20] min-w-0 h-full overflow-hidden text-gray-900 dark:text-gray-200">
      <DeviceSidebar
        devices={devices}
        loading={loading}
        searchTerm={searchTerm}
        setSearchTerm={setSearchTerm}
        selectedDevice={selectedDevice}
        setSelectedDevice={handleDeviceSelected}
        setIsAdding={(v) => { setIsAdding(v); setIsEditing(false); }}
      />

      <div className="flex-1 flex flex-col bg-gray-50 dark:bg-[#1e1e20] overflow-y-auto custom-scrollbar relative">
        {isAdding || isEditing ? (
          <DeviceFormPanel 
            device={isEditing ? selectedDevice : null}
            onClose={() => {
              setIsAdding(false);
              setIsEditing(false);
            }} 
            onSave={(dev: Device) => {
              if (isAdding) {
                setDevices([...devices, dev]);
                setSelectedDevice(dev);
                setIsAdding(false);
              } else {
                handleDeviceUpdate(dev);
                setIsEditing(false);
              }
            }} 
          />
        ) : selectedDevice ? (
          <DeviceDetailPanel
            selectedDevice={selectedDevice}
            setSelectedDevice={handleDeviceUpdate}
            setShowBindModal={setShowBindModal}
            setShowActivationModal={setShowActivationModal}
            onEdit={() => setIsEditing(true)}
            onDelete={() => handleDeleteDevice(selectedDevice)}
            onConfigureAgent={() => onEditAgent && selectedDevice.agentId && onEditAgent(selectedDevice.agentId)}
          />
        ) : (
          <div className="flex-1 flex flex-col items-center justify-center text-center p-8">
            <Server size={48} className="text-gray-400 dark:text-gray-600 mb-4" />
            <h2 className="text-xl font-bold text-gray-900 dark:text-gray-300 mb-2">未选择设备</h2>
            <p className="text-gray-500 max-w-sm">
              请在左侧列表中选择一个智能硬件进行查看和管理。
            </p>
          </div>
        )}
      </div>

      <AnimatePresence>
        {showBindModal && (
          <BindAgentModal
            key="bind-modal"
            device={showBindModal}
            onClose={() => setShowBindModal(null)}
            onSelect={(agentIds) => {
              handleDeviceUpdate({
                ...showBindModal,
                agentId: agentIds[0] || undefined
              });
            }}
            multiple={false}
          />
        )}
        {showActivationModal && (
          <ActivationModal
            key="activation-modal"
            showActivationModal={showActivationModal}
            setShowActivationModal={setShowActivationModal}
            selectedDevice={selectedDevice}
            onActivated={handleDeviceUpdate}
          />
        )}
      </AnimatePresence>
    </div>
  );
};
