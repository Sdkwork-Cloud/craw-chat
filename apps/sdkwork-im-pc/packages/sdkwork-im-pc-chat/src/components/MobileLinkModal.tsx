import React from 'react';
import { motion } from 'motion/react';
import { Smartphone } from 'lucide-react';
import { toast } from './Toast';

interface MobileLinkModalProps {
  onClose: () => void;
}

export const MobileLinkModal: React.FC<MobileLinkModalProps> = ({ onClose }) => {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.2 }}
      className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-md"
      onClick={onClose}
    >
      <motion.div
        initial={{ scale: 0.95, opacity: 0 }}
        animate={{ scale: 1, opacity: 1 }}
        exit={{ scale: 0.95, opacity: 0 }}
        transition={{
          type: "spring",
          damping: 25,
          stiffness: 300,
          duration: 0.2,
        }}
        className="bg-[#1e1e1e] border border-white/10 rounded-3xl w-[340px] shadow-2xl flex flex-col overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="p-8 text-center relative overflow-hidden">
          <div className="absolute top-0 inset-x-0 h-32 bg-gradient-to-b from-indigo-500/20 to-transparent pointer-events-none"></div>
          <div className="w-16 h-16 rounded-full bg-[#2b2b2d] border border-indigo-500/30 flex items-center justify-center mx-auto mb-4 text-indigo-400 relative z-10 shadow-[0_0_20px_rgba(79,70,229,0.2)]">
            <Smartphone size={28} />
          </div>
          <h3 className="text-xl font-semibold text-gray-100 mb-2 relative z-10">
            连接移动设备
          </h3>
          <p className="text-sm text-gray-400 mb-8 relative z-10">
            在手机端打开 Sdkwork IM 扫码登录，即可同步会话与文件。
          </p>

          <div className="w-48 h-48 bg-white mx-auto rounded-2xl p-3 mb-8 shadow-xl relative z-10">
            <img
              src="https://api.qrserver.com/v1/create-qr-code/?size=180x180&data=im://login/client_route_sync&color=000000&bgcolor=ffffff"
              alt="QR Code"
              className="w-full h-full"
            />
          </div>

          <button
            onClick={() => {
              toast("已发送推送请求到您的默认移动设备", "success");
              onClose();
            }}
            className="w-full py-3 rounded-xl bg-white/5 hover:bg-white/10 text-white font-medium transition-colors border border-white/10 relative z-10"
          >
            发送连接校验指令
          </button>
        </div>
      </motion.div>
    </motion.div>
  );
};
