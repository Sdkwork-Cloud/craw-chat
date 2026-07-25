import { useTranslation } from "react-i18next";
import { Hardware, Agent } from '../types';

let MOCK_HARDWARE: Hardware[] = [
  {
    id: "hw_1",
    name: "前台迎宾机器人",
    type: "robot",
    status: "online",
    boundAt: "2026-05-20T10:00:00Z",
    agentId: "agent_1",
    agentName: "智能客服小Claw"
  },
  {
    id: "hw_2",
    name: "会议室智能音箱",
    type: "speaker",
    status: "offline",
    boundAt: "2026-05-21T14:30:00Z"
  }
];

let MOCK_AGENTS: Agent[] = [
  { id: "agent_1", name: "智能客服小Claw", capabilities: ["语音交流", "访客登记"] },
  { id: "agent_2", name: "会议纪要助手", capabilities: ["录音总结", "待办提取"] },
  { id: "agent_3", name: "安防监控AI", capabilities: ["异常检测", "人脸识别"] }
];

export const HardwareService = {
  getHardwareList: async (): Promise<Hardware[]> => {
    return new Promise((resolve) => setTimeout(() => resolve([...MOCK_HARDWARE]), 300));
  },
  
  getHardwareById: async (id: string): Promise<Hardware | undefined> => {
    return new Promise((resolve) => setTimeout(() => resolve(MOCK_HARDWARE.find(h => h.id === id)), 200));
  },

  bindHardware: async (name: string, type: string, activationCode: string): Promise<Hardware> => {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (!activationCode) {
           return reject(new Error("请输入激活码"));
        }
        if (activationCode.length < 4) {
           return reject(new Error("无效的激活码"));
        }
        const newHw: Hardware = {
          id: `hw_${Date.now()}`,
          name,
          type,
          status: "online",
          boundAt: new Date().toISOString()
        };
        MOCK_HARDWARE.push(newHw);
        resolve({...newHw});
      }, 500);
    });
  },

  deleteHardware: async (id: string): Promise<void> => {
    return new Promise((resolve) => {
      setTimeout(() => {
        MOCK_HARDWARE = MOCK_HARDWARE.filter(h => h.id !== id);
        resolve();
      }, 400);
    });
  },

  updateHardwareName: async (id: string, name: string): Promise<Hardware> => {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            const hw = MOCK_HARDWARE.find(h => h.id === id);
            if (hw) {
                hw.name = name;
                resolve({...hw});
            } else {
                reject(new Error("Hardware not found"));
            }
        }, 400);
    });
  },

  getAllAgents: async (): Promise<Agent[]> => {
    return new Promise(resolve => setTimeout(() => resolve([...MOCK_AGENTS]), 200));
  },

  associateAgent: async (hardwareId: string, agentId: string | undefined): Promise<Hardware> => {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const hwIndex = MOCK_HARDWARE.findIndex(h => h.id === hardwareId);
        if (hwIndex > -1) {
          const agent = MOCK_AGENTS.find(a => a.id === agentId);
          MOCK_HARDWARE[hwIndex].agentId = agent?.id;
          MOCK_HARDWARE[hwIndex].agentName = agent?.name;
          resolve({...MOCK_HARDWARE[hwIndex]});
        } else {
          reject(new Error("Hardware not found"));
        }
      }, 400);
    });
  }
};
