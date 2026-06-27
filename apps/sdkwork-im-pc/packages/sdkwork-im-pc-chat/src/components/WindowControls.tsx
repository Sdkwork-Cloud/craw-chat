import React, { useState } from 'react';
import { Pin, Minus, Square, X, Copy } from 'lucide-react';
import { toast } from './Toast';
import { cn } from '@sdkwork/im-pc-commons';

type NativeWindowControlAction = 'closeToTray' | 'minimize' | 'show' | 'toggleMaximize';
type TauriInvoke = (command: string, args?: Record<string, unknown>) => Promise<unknown>;

function resolveTauriInvoke(): TauriInvoke | null {
  return (globalThis as {
    __TAURI__?: {
      core?: {
        invoke?: TauriInvoke;
      };
    };
  }).__TAURI__?.core?.invoke ?? null;
}

async function invokeDesktopWindowControl(action: NativeWindowControlAction): Promise<boolean> {
  const invoke = resolveTauriInvoke();
  if (!invoke) {
    return false;
  }

  await invoke('sdkwork_chat_pc_window_control', { action });
  return true;
}

export const WindowControls: React.FC = () => {
  const [isPinned, setIsPinned] = useState(false);
  const [isMaximized, setIsMaximized] = useState(false);

  const handlePin = () => {
    setIsPinned(!isPinned);
    toast(!isPinned ? '窗口已置顶' : '已取消置顶', 'success');
  };

  const toggleMaximize = () => {
    void invokeDesktopWindowControl('toggleMaximize')
      .then((handled) => {
        if (handled) {
          setIsMaximized((current) => !current);
        }
      })
      .catch(() => {
        toast('窗口最大化失败', 'error');
      });
  };

  const handleMinimize = () => {
    void invokeDesktopWindowControl('minimize')
      .catch(() => {
        toast('窗口最小化失败', 'error');
      });
  };

  const handleClose = () => {
    void invokeDesktopWindowControl('closeToTray')
      .catch(() => {
        toast('关闭到托盘失败', 'error');
      });
  };

  return (
    <div className="flex items-center h-full">
      <button 
        className={cn("w-[46px] h-full flex items-center justify-center transition-colors", isPinned ? "bg-white/10 text-white" : "text-gray-400 hover:bg-white/10")} 
        title={isPinned ? "取消置顶" : "置顶"} 
        onClick={handlePin}
      >
        <Pin size={14} className={cn(isPinned && "rotate-45 transition-transform")} />
      </button>
      <button 
        className="w-[46px] h-full flex items-center justify-center text-gray-400 hover:bg-white/10 transition-colors" 
        title="最小化" 
        onClick={handleMinimize}
      >
        <Minus size={16} />
      </button>
      <button 
        className="w-[46px] h-full flex items-center justify-center text-gray-400 hover:bg-white/10 transition-colors" 
        title={isMaximized ? "还原" : "最大化"} 
        onClick={toggleMaximize}
      >
        {isMaximized ? <Copy size={12} className="rotate-180" /> : <Square size={12} />}
      </button>
      <button 
        className="w-[46px] h-full flex items-center justify-center text-gray-400 hover:bg-[#e81123] hover:text-white transition-colors" 
        title="关闭" 
        onClick={handleClose}
      >
        <X size={16} />
      </button>
    </div>
  );
};
