import React, { useState } from "react";
import { Edit2, Trash2, Bot, Plug, Zap, Activity, Battery, Wifi, Cpu, AlertCircle, SignalHigh, CheckCircle2, ChevronRight, ActivitySquare } from "lucide-react";
import { Device, deviceService } from "../services/DeviceService";
import { getDeviceIcon, getStatusDisplay } from "./DeviceSidebar";
import { toast } from "@sdkwork/im-pc-chat";

interface DeviceDetailPanelProps {
  selectedDevice: Device;
  setSelectedDevice: (d: Device) => void;
  setShowBindModal: (d: Device) => void;
  setShowActivationModal: (show: boolean) => void;
  onEdit: () => void;
  onDelete: () => void;
  onConfigureAgent?: () => void;
}

export const DeviceDetailPanel: React.FC<DeviceDetailPanelProps> = ({
  selectedDevice, setSelectedDevice, setShowBindModal, setShowActivationModal, onEdit, onDelete, onConfigureAgent
}) => {
  const isOnline = selectedDevice.status === 'online';
  const [unbindingAgent, setUnbindingAgent] = useState(false);

  const handleUnbindAgent = async () => {
    if (unbindingAgent) {
      return;
    }
    setUnbindingAgent(true);
    try {
      await deviceService.unbindAgent(selectedDevice.id);
      setSelectedDevice({ ...selectedDevice, agentId: undefined });
      toast("Unbinding submitted", "success");
    } catch (error) {
      const message = error instanceof Error && error.message ? error.message : "Unbinding failed";
      toast(message, "error");
    } finally {
      setUnbindingAgent(false);
    }
  };
  
  return (
    <div className="p-8 pb-32 max-w-5xl mx-auto w-full h-full overflow-y-auto custom-scrollbar">
      {/* Header Section */}
      <div className="flex flex-wrap items-start justify-between gap-6 mb-8">
        <div className="flex items-center gap-6">
          <div className={`w-20 h-20 rounded-2xl flex items-center justify-center shrink-0 border shadow-lg ${isOnline ? 'bg-indigo-50 dark:bg-indigo-500/10 border-indigo-200 dark:border-indigo-500/20 text-indigo-600 dark:text-indigo-400 shadow-indigo-500/10' : 'bg-gray-100 dark:bg-gray-800 border-gray-200 dark:border-white/5 text-gray-500'}`}>
            {getDeviceIcon(selectedDevice.type)}
          </div>
          <div>
            <h1 className="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-2 flex items-center gap-3 tracking-tight">
              {selectedDevice.name}
              <div className="scale-90 origin-left">{getStatusDisplay(selectedDevice.status)}</div>
            </h1>
            <div className="flex items-center gap-4 text-sm font-mono tracking-wider">
              <span className="text-gray-400 flex items-center gap-1.5"><span className="w-1.5 h-1.5 rounded-full bg-gray-500"></span> MAC: {selectedDevice.macAddress}</span>
              <span className="text-gray-500">|</span>
              <span className="text-gray-400 flex items-center gap-1.5">FW: {selectedDevice.firmwareVersion || 'v1.0.0'}</span>
            </div>
          </div>
        </div>
        <div className="flex items-center gap-2">
          {selectedDevice.status === 'unactivated' && (
            <button
              onClick={() => setShowActivationModal(true)}
              className="px-4 py-2 bg-amber-50 dark:bg-amber-500/10 hover:bg-amber-100 dark:hover:bg-amber-500/20 border border-amber-200 dark:border-amber-500/20 text-amber-600 dark:text-amber-500 text-sm font-medium rounded-lg transition-all flex items-center gap-2 shadow-sm"
            >
              <Zap size={16} /> 激活动力模块
            </button>
          )}
          <button 
            className="px-3 py-2 bg-white dark:bg-[#252528] border border-gray-200 dark:border-white/5 shadow-sm hover:bg-gray-50 dark:hover:bg-white/10 rounded-lg text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors flex items-center gap-2 text-sm font-medium" 
            onClick={onEdit}
          >
            <Edit2 size={16} /> 编辑配置
          </button>
          <button 
            className="px-3 py-2 bg-white dark:bg-[#252528] border border-gray-200 dark:border-white/5 shadow-sm hover:bg-red-50 dark:hover:bg-red-500/10 rounded-lg text-gray-500 dark:text-gray-400 hover:text-red-600 dark:hover:text-red-400 transition-colors flex items-center gap-2 text-sm font-medium" 
            onClick={onDelete}
          >
            <Trash2 size={16} />
          </button>
        </div>
      </div>

      {/* Metrics Bento Board */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <div className="bg-white dark:bg-[#212124] rounded-2xl p-5 border border-gray-200 dark:border-white/5 shadow-sm dark:shadow-md flex flex-col justify-between group hover:border-indigo-300 dark:hover:border-indigo-500/30 transition-colors">
          <div className="flex items-center justify-between mb-4">
            <span className="text-xs font-medium text-gray-500 tracking-wider">NETWORK PING</span>
            <Activity className={`w-4 h-4 ${isOnline ? 'text-green-500' : 'text-gray-400 dark:text-gray-600'}`} />
          </div>
          <div className="flex items-baseline gap-1">
            <span className="text-2xl font-bold font-mono text-gray-900 dark:text-gray-200">{isOnline ? '24' : '--'}</span>
            <span className="text-xs text-gray-500">ms</span>
          </div>
          <div className="mt-2 text-[10px] text-gray-500">US-East-1 Data Center</div>
        </div>
        
        <div className="bg-white dark:bg-[#212124] rounded-2xl p-5 border border-gray-200 dark:border-white/5 shadow-sm dark:shadow-md flex flex-col justify-between group hover:border-indigo-300 dark:hover:border-indigo-500/30 transition-colors">
          <div className="flex items-center justify-between mb-4">
            <span className="text-xs font-medium text-gray-500 tracking-wider">SIGNAL STRENGTH</span>
            <Wifi className={`w-4 h-4 ${isOnline ? 'text-indigo-500 dark:text-indigo-400' : 'text-gray-400 dark:text-gray-600'}`} />
          </div>
          <div className="flex items-baseline gap-1">
            <span className="text-2xl font-bold font-mono text-gray-900 dark:text-gray-200">{isOnline ? '-68' : '--'}</span>
            <span className="text-xs text-gray-500">dBm</span>
          </div>
          <div className="mt-2 w-full bg-gray-100 dark:bg-gray-800 rounded-full h-1.5 overflow-hidden">
            <div className={`h-full rounded-full ${isOnline ? 'bg-indigo-500 w-3/4' : 'bg-transparent w-0'}`}></div>
          </div>
        </div>

        <div className="bg-white dark:bg-[#212124] rounded-2xl p-5 border border-gray-200 dark:border-white/5 shadow-sm dark:shadow-md flex flex-col justify-between group hover:border-indigo-300 dark:hover:border-indigo-500/30 transition-colors">
          <div className="flex items-center justify-between mb-4">
            <span className="text-xs font-medium text-gray-500 tracking-wider">CPU USAGE</span>
            <Cpu className={`w-4 h-4 ${isOnline ? 'text-blue-500 dark:text-blue-400' : 'text-gray-400 dark:text-gray-600'}`} />
          </div>
          <div className="flex items-baseline gap-1">
            <span className="text-2xl font-bold font-mono text-gray-900 dark:text-gray-200">{isOnline ? '12.4' : '--'}</span>
            <span className="text-xs text-gray-500">%</span>
          </div>
          <div className="mt-2 w-full flex gap-0.5">
            {[...Array(12)].map((_, i) => (
              <div key={i} className={`h-1.5 flex-1 rounded-sm ${!isOnline ? 'bg-gray-100 dark:bg-gray-800' : i < 3 ? 'bg-blue-500' : 'bg-gray-100 dark:bg-gray-800'}`}></div>
            ))}
          </div>
        </div>

        <div className="bg-white dark:bg-[#212124] rounded-2xl p-5 border border-gray-200 dark:border-white/5 shadow-sm dark:shadow-md flex flex-col justify-between group hover:border-indigo-300 dark:hover:border-indigo-500/30 transition-colors">
          <div className="flex items-center justify-between mb-4">
            <span className="text-xs font-medium text-gray-500 tracking-wider">POWER STATE</span>
            <Battery className={`w-4 h-4 ${isOnline ? 'text-green-500 dark:text-green-400' : 'text-gray-400 dark:text-gray-600'}`} />
          </div>
          <div className="flex items-baseline gap-1">
             <span className="text-2xl font-bold font-mono text-gray-900 dark:text-gray-200">{isOnline ? '98' : '--'}</span>
             <span className="text-xs text-gray-500">%</span>
          </div>
          <div className="mt-2 text-[10px] text-gray-500">{isOnline ? 'Mains Power Connected' : 'Offline'}</div>
        </div>
      </div>

      {/* Core Panels Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-8">
        {/* Device Identity Panel */}
        <div className="bg-white dark:bg-[#252528] rounded-2xl border border-gray-200 dark:border-white/5 shadow-lg dark:shadow-xl overflow-hidden flex flex-col">
          <div className="px-6 py-4 flex items-center gap-3 border-b border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-[#2a2a2d]">
             <div className="w-1.5 h-4 bg-gray-400 dark:bg-gray-500 rounded-full"></div>
             <h3 className="font-semibold text-sm text-gray-900 dark:text-gray-200 uppercase tracking-widest">Device Identity</h3>
          </div>
          <div className="p-6 flex-1 flex flex-col justify-center space-y-5">
            <div className="flex justify-between items-center pb-4 border-b border-gray-100 dark:border-white/5">
              <span className="text-gray-500 text-sm font-medium">Device Type</span>
              <span className="text-gray-900 dark:text-gray-200 text-sm capitalize px-2.5 py-1 bg-gray-100 dark:bg-white/5 rounded-md border border-gray-200 dark:border-white/5">{selectedDevice.type}</span>
            </div>
            <div className="flex justify-between items-center pb-4 border-b border-gray-100 dark:border-white/5">
              <span className="text-gray-500 text-sm font-medium">Assigned IP</span>
              <span className="text-gray-900 dark:text-gray-200 text-sm font-mono">{isOnline ? '192.168.1.104' : 'N/A'}</span>
            </div>
            <div className="flex justify-between items-center pb-4 border-b border-gray-100 dark:border-white/5">
              <span className="text-gray-500 text-sm font-medium">Uptime</span>
              <span className="text-gray-900 dark:text-gray-200 text-sm font-mono">{isOnline ? '14d 08h 22m' : 'N/A'}</span>
            </div>
            <div className="flex justify-between items-center">
              <span className="text-gray-500 text-sm font-medium">Protocol</span>
              <span className="text-blue-600 dark:text-gray-200 text-sm font-mono px-2.5 py-1 bg-blue-50 dark:bg-blue-500/10 dark:text-blue-400 rounded-md border border-blue-200 dark:border-blue-500/10">MQTT / WSS</span>
            </div>
          </div>
        </div>

        {/* AI Agent Panel */}
        <div className="bg-white dark:bg-[#252528] rounded-2xl border border-gray-200 dark:border-white/5 shadow-lg dark:shadow-xl relative overflow-hidden group min-h-[280px] flex flex-col">
          {selectedDevice.status === 'unactivated' ? (
            <div className="flex-1 flex flex-col items-center justify-center p-8 text-center bg-gradient-to-b from-transparent to-amber-50 dark:to-amber-500/5">
              <div className="w-16 h-16 bg-amber-100 dark:bg-amber-500/10 rounded-2xl flex items-center justify-center text-amber-500 mb-4 shadow-[0_0_30px_rgba(245,158,11,0.1)]">
                <Zap size={32} />
              </div>
              <h3 className="text-base font-bold text-gray-900 dark:text-gray-200 mb-2">硬件核心未激活</h3>
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-6 max-w-[200px] leading-relaxed">激活后即可下载指令集，接入 AI Agent 托管系统</p>
              <button
                className="px-6 py-2.5 bg-amber-600 hover:bg-amber-700 dark:hover:bg-amber-500 text-white text-sm font-semibold rounded-xl transition-all shadow-md shadow-amber-500/20 hover:shadow-amber-500/40 hover:-translate-y-0.5"
                onClick={() => setShowActivationModal(true)}
              >
                启动激活序列
              </button>
            </div>
          ) : selectedDevice.agentId ? (
            <>
              <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-500/10 dark:bg-indigo-500/20 rounded-full blur-[80px] -translate-y-1/2 translate-x-1/2 pointer-events-none group-hover:bg-indigo-500/20 dark:group-hover:bg-indigo-500/30 transition-all duration-700"></div>
              
              <div className="px-6 py-4 flex items-center justify-between border-b border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-[#2a2a2d] relative z-10">
                 <div className="flex items-center gap-3">
                   <div className="w-1.5 h-4 bg-indigo-500 rounded-full shadow-[0_0_10px_rgba(99,102,241,0.4)] dark:shadow-[0_0_10px_rgba(99,102,241,0.8)]"></div>
                   <h3 className="font-semibold text-sm text-gray-900 dark:text-gray-200 uppercase tracking-widest">Agent Hosting</h3>
                 </div>
                 <span className="flex items-center gap-1.5 text-xs bg-indigo-50 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 px-2.5 py-1 rounded-md border border-indigo-200 dark:border-indigo-500/30 font-medium tracking-wide">
                   <span className="w-1.5 h-1.5 rounded-full bg-indigo-500 dark:bg-indigo-400 animate-pulse"></span> ACTIVE
                 </span>
              </div>
              
              <div className="p-6 relative z-10 flex-1 flex flex-col justify-between">
                <div className="flex items-start gap-4">
                  <div className="w-12 h-12 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-xl flex items-center justify-center text-white shrink-0 shadow-lg shadow-indigo-500/20">
                    <Bot size={24} />
                  </div>
                  <div>
                    <h4 className="text-lg font-bold text-gray-900 dark:text-gray-100 flex items-center gap-2">
                       Vision 视觉处理向导
                    </h4>
                    <div className="flex items-center gap-2 mt-1.5">
                       <span className="text-xs px-2 py-0.5 bg-gray-100 dark:bg-white/10 text-gray-600 dark:text-gray-300 rounded font-mono">ID: {selectedDevice.agentId}</span>
                       <span className="text-xs text-gray-400 dark:text-gray-500">•</span>
                       <span className="text-xs text-indigo-600 dark:text-indigo-400 font-medium">Model: Gemini-v1.5-Pro</span>
                    </div>
                  </div>
                </div>
                
                <div className="mt-8 flex gap-3">
                  <button 
                    className="flex-[2] py-3 bg-gray-50 dark:bg-[#2a2a2d] hover:bg-gray-100 dark:hover:bg-white/10 rounded-xl text-sm text-gray-800 dark:text-gray-200 font-medium transition-colors border border-gray-200 dark:border-white/10 hover:border-gray-300 dark:hover:border-white/20 flex items-center justify-center gap-2"
                    onClick={() => onConfigureAgent && onConfigureAgent()}
                  >
                    配置 Agent 技能 <ChevronRight size={16} className="text-gray-400 dark:text-gray-500" />
                  </button>
                  <button
                    className="flex-1 py-3 bg-red-50 dark:bg-red-500/5 hover:bg-red-100 dark:hover:bg-red-500/10 rounded-xl text-sm text-red-600 dark:text-red-500 font-medium transition-colors border border-red-200 dark:border-red-500/10 hover:border-red-300 dark:hover:border-red-500/30"
                    onClick={handleUnbindAgent}
                    disabled={unbindingAgent}
                  >
                    {unbindingAgent ? "Submitting..." : "解除绑定"}
                  </button>
                </div>
              </div>
            </>
          ) : (
            <div className="flex-1 flex flex-col items-center justify-center p-8 text-center bg-gradient-to-b from-transparent to-indigo-50 dark:to-indigo-500/5">
              <div className="w-16 h-16 bg-gray-50 dark:bg-white/5 border border-gray-200 dark:border-white/10 rounded-2xl flex items-center justify-center text-gray-400 dark:text-gray-500 mb-4 shadow-md dark:shadow-xl">
                <Plug size={32} />
              </div>
              <h3 className="text-base font-bold text-gray-900 dark:text-gray-200 mb-2">独立运行模式</h3>
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-6 max-w-[200px] leading-relaxed">设备当前未关联大脑，接入 Agent 后可获得空间感知与决策能力</p>
              <button
                className="px-6 py-2.5 bg-indigo-600 hover:bg-indigo-700 dark:hover:bg-indigo-500 text-white text-sm font-semibold rounded-xl transition-all shadow-md shadow-indigo-500/20 hover:shadow-indigo-500/40 hover:-translate-y-0.5"
                onClick={() => setShowBindModal(selectedDevice)}
              >
                云端绑定 Agent
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Capabilities Section */}
      <div className="bg-white dark:bg-[#252528] rounded-2xl border border-gray-200 dark:border-white/5 shadow-lg dark:shadow-xl overflow-hidden">
         <div className="px-6 py-4 flex items-center gap-3 border-b border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-[#2a2a2d]">
           <div className="w-1.5 h-4 bg-green-500 rounded-full shadow-[0_0_10px_rgba(34,197,94,0.3)] dark:shadow-[0_0_10px_rgba(34,197,94,0.5)]"></div>
           <h3 className="font-semibold text-sm text-gray-900 dark:text-gray-200 uppercase tracking-widest">Active Modules</h3>
        </div>
        <div className="p-2">
          {selectedDevice.status === 'unactivated' ? (
             <div className="text-center py-12 flex flex-col items-center">
               <AlertCircle size={32} className="text-gray-400 dark:text-gray-600 mb-3" />
               <p className="text-gray-500 text-sm font-medium tracking-wide">SYSTEM OFFLINE</p>
             </div>
          ) : selectedDevice.agentId ? (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2 p-4">
              {/* Capability Item */}
              <div className="flex items-start justify-between p-4 bg-gray-50 dark:bg-[#1e1e20] rounded-xl border border-gray-200 dark:border-white/5 hover:border-indigo-300 dark:hover:border-indigo-500/30 transition-colors group cursor-crosshair">
                <div className="flex items-start gap-4">
                  <div className="w-10 h-10 rounded-lg bg-green-50 dark:bg-green-500/10 border border-green-200 dark:border-green-500/20 text-green-600 dark:text-green-400 flex items-center justify-center shrink-0">
                     <ActivitySquare size={20} />
                  </div>
                  <div>
                    <h4 className="text-sm font-bold text-gray-900 dark:text-gray-200 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">实时人员检测 (Person Tracking)</h4>
                    <p className="text-xs text-gray-500 mt-1.5 leading-relaxed">调用视觉端侧模型，实时分析画面中出现的人员轨迹与停留情况。</p>
                  </div>
                </div>
                <button className="px-3 py-1.5 bg-indigo-50 dark:bg-indigo-500/10 hover:bg-indigo-100 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 text-xs font-medium rounded-lg border border-indigo-200 dark:border-indigo-500/20 transition-colors whitespace-nowrap">
                  仿真测试
                </button>
              </div>
              
              {/* Capability Item */}
              <div className="flex items-start justify-between p-4 bg-gray-50 dark:bg-[#1e1e20] rounded-xl border border-gray-200 dark:border-white/5 hover:border-indigo-300 dark:hover:border-indigo-500/30 transition-colors group cursor-crosshair">
                <div className="flex items-start gap-4">
                  <div className="w-10 h-10 rounded-lg bg-amber-50 dark:bg-amber-500/10 border border-amber-200 dark:border-amber-500/20 text-amber-600 dark:text-amber-500 flex items-center justify-center shrink-0">
                     <AlertCircle size={20} />
                  </div>
                  <div>
                    <h4 className="text-sm font-bold text-gray-900 dark:text-gray-200 group-hover:text-amber-600 dark:group-hover:text-amber-400 transition-colors">异常行为告警 (Behavior Alert)</h4>
                    <p className="text-xs text-gray-500 mt-1.5 leading-relaxed">连续侦测画面行为，识别跌倒、争执等关键异常并自动拉起事件流。</p>
                  </div>
                </div>
                <button className="px-3 py-1.5 bg-indigo-50 dark:bg-indigo-500/10 hover:bg-indigo-100 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 text-xs font-medium rounded-lg border border-indigo-200 dark:border-indigo-500/20 transition-colors whitespace-nowrap">
                  仿真测试
                </button>
              </div>
            </div>
          ) : (
            <div className="text-center py-12 flex flex-col items-center">
              <Bot size={32} className="text-gray-400 dark:text-gray-700 mb-3" />
              <p className="text-gray-500 text-sm font-medium tracking-wide">NO AGENT MODULE LOADED</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

