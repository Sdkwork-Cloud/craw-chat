import React from "react";
import { Search, Plus } from "lucide-react";
import { Device } from "../services/DeviceService";
import { Camera, Speaker, MonitorSmartphone, Cpu, Wifi, WifiOff, AlertCircle } from "lucide-react";

interface DeviceSidebarProps {
  devices: Device[];
  loading: boolean;
  searchTerm: string;
  setSearchTerm: (v: string) => void;
  selectedDevice: Device | null;
  setSelectedDevice: (d: Device) => void;
  setIsAdding: (v: boolean) => void;
}

export const getDeviceIcon = (type: string) => {
  switch (type) {
  case "camera": return <Camera size={24} />;
  case "speaker": return <Speaker size={24} />;
  case "display": return <MonitorSmartphone size={24} />;
  case "sensor": return <Cpu size={24} />;
  default: return <Cpu size={24} />;
  }
};

export const getStatusDisplay = (status: string) => {
  switch (status) {
    case "online":
      return <span className="flex items-center gap-1.5 text-xs text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-500/10 px-2 py-1 rounded-full"><Wifi size={12} /> 在线</span>;
    case "offline":
      return <span className="flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-500/10 px-2 py-1 rounded-full"><WifiOff size={12} /> 离线</span>;
    case "error":
      return <span className="flex items-center gap-1.5 text-xs text-red-600 dark:text-red-400 bg-red-50 dark:bg-red-500/10 px-2 py-1 rounded-full"><AlertCircle size={12} /> 异常</span>;
    case "unactivated":
      return <span className="flex items-center gap-1.5 text-xs text-amber-600 dark:text-amber-400 bg-amber-50 dark:bg-amber-500/10 px-2 py-1 rounded-full"><AlertCircle size={12} /> 未激活</span>;
    default:
      return null;
  }
};

export const DeviceSidebar: React.FC<DeviceSidebarProps> = ({
  devices, loading, searchTerm, setSearchTerm, selectedDevice, setSelectedDevice, setIsAdding
}) => {
  const filteredDevices = devices.filter(
    (d) => d.name.toLowerCase().includes(searchTerm.toLowerCase()) || d.macAddress.toLowerCase().includes(searchTerm.toLowerCase())
  );

  return (
    <div className="w-80 flex flex-col border-r border-gray-200 dark:border-white/10 bg-white dark:bg-[#252528] shrink-0">
      <div className="p-4 border-b border-gray-200 dark:border-white/10 shrink-0">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-bold text-gray-900 dark:text-gray-100 flex items-center gap-2">
            <Cpu size={20} className="text-indigo-600 dark:text-indigo-400" />
            智能硬件管理
          </h2>
          <button
            onClick={() => setIsAdding(true)}
            className="w-8 h-8 rounded-full bg-indigo-50 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 hover:bg-indigo-100 dark:hover:bg-indigo-500/30 flex items-center justify-center transition-colors"
          >
            <Plus size={18} />
          </button>
        </div>
        <div className="relative">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500" />
          <input
            type="text"
            placeholder="搜索设备名称或 MAC 地址..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
            className="w-full bg-gray-50 dark:bg-[#1a1a1c] border border-gray-200 dark:border-white/5 focus:border-indigo-500/50 rounded-xl pl-9 pr-4 py-2 text-sm text-gray-900 dark:text-gray-200 outline-none transition-colors"
          />
        </div>
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar">
        {loading ? (
          <div className="flex justify-center p-8">
            <div className="w-6 h-6 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
          </div>
        ) : filteredDevices.length === 0 ? (
          <div className="text-center p-8 text-gray-500 text-sm">
            未找到相关设备
          </div>
        ) : (
          <div className="divide-y divide-gray-100 dark:divide-white/5">
            {filteredDevices.map((device) => (
              <div
                key={device.id}
                onClick={() => setSelectedDevice(device)}
                className={`p-4 cursor-pointer hover:bg-gray-50 dark:hover:bg-white/5 transition-colors flex items-center gap-4 ${selectedDevice?.id === device.id ? "bg-indigo-50 dark:bg-indigo-500/10 border-l-2 border-indigo-600 dark:border-indigo-500" : "border-l-2 border-transparent"}`}
              >
                <div className={`w-12 h-12 rounded-xl flex items-center justify-center shrink-0 ${device.status === "online" ? "bg-indigo-100 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400" : "bg-gray-100 dark:bg-gray-800 text-gray-400 dark:text-gray-500"}`}>
                  {getDeviceIcon(device.type)}
                </div>
                <div className="flex-1 min-w-0">
                  <h3 className="text-sm font-medium text-gray-900 dark:text-gray-200 truncate">
                    {device.name}
                  </h3>
                  <div className="text-xs text-gray-500 font-mono mt-1 opacity-80">
                    {device.macAddress}
                  </div>
                </div>
                <div>{getStatusDisplay(device.status)}</div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};
