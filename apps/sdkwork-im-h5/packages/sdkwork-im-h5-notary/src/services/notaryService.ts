export interface NotaryFileTag {
  label: string;
  color: "blue" | "green" | "orange" | "red" | "gray";
}

export interface NotaryFile {
  id: string;
  name: string;
  size: string;
  uploadTime: string;
  fileType: "image" | "video" | "pdf" | "word" | "excel" | "zip" | "unknown";
  tags: NotaryFileTag[];
  uploader: string;
}

export interface NotaryDetailData {
  id: string;
  title: string;
  time: string;
  item: string;
  notaryName: string;
  notaryNo: string;
  status: string;
  remarks: string;
  parties: any[];
  materials: NotaryFile[];
}

const STORAGE_KEY = "sdkwork_im_h5_notary_records_v2";
const CLOUD_FILES_KEY = "sdkwork_im_h5_notary_cloud_files";

const INITIAL_RECORDS = Array.from({ length: 250 }).map((_, i) => {
  const typeMap = [
    "证据保全",
    "存证公证",
    "家事公证",
    "委托公证",
    "声明书公证",
  ];
  const titleMap = [
    "著作权电子取证公证",
    "电子合同存证公证",
    "网页取证保全公证",
    "批量房产继承公证",
    "委托书公证",
  ];
  const statusMap = ["processing", "completed", "cancelled"];
  const date = new Date(
    Date.now() - Math.floor(Math.random() * 30 * 24 * 60 * 60 * 1000),
  );
  return {
    id: String(i + 1),
    title: titleMap[i % titleMap.length],
    type: typeMap[i % typeMap.length],
    date: date.toISOString().split("T")[0],
    status: statusMap[i % statusMap.length],
  };
});

const INITIAL_CLOUD_FILES = [
  {
    id: "1",
    type: "folder",
    name: "著作权电子取证案卷 (2026)",
    date: "2026-04-18 10:42",
    size: "-",
    iconColor: "text-yellow-400",
    fill: "fill-yellow-400",
  },
  {
    id: "1_1",
    parentId: "1",
    type: "image",
    name: "网页存证截图_1.png",
    date: "2026-04-18 10:50",
    size: "1.2 MB",
    iconColor: "text-green-500",
    bg: "bg-green-500/10",
  },
  {
    id: "1_2",
    parentId: "1",
    type: "pdf",
    name: "著作权登记证书.pdf",
    date: "2026-04-18 11:05",
    size: "3.4 MB",
    iconColor: "text-red-500",
    bg: "bg-red-500/10",
  },
  {
    id: "2",
    type: "folder",
    name: "合同存证案卷",
    date: "2026-04-15 14:20",
    size: "-",
    iconColor: "text-yellow-400",
    fill: "fill-yellow-400",
  },
  {
    id: "2_1",
    parentId: "2",
    type: "doc",
    name: "租赁合同原件扫描.docx",
    date: "2026-04-16 09:12",
    size: "820 KB",
    iconColor: "text-blue-500",
    bg: "bg-blue-500/10",
  },
  {
    id: "3",
    type: "folder",
    name: "历史纸质扫描归档",
    date: "2026-01-10 09:00",
    size: "-",
    iconColor: "text-yellow-400",
    fill: "fill-yellow-400",
  },
  {
    id: "4",
    type: "image",
    name: "现场取证照片_001.jpg",
    date: "昨天 16:30",
    size: "4.2 MB",
    iconColor: "text-green-500",
    bg: "bg-green-500/10",
  },
  {
    id: "5",
    type: "pdf",
    name: "电子签名公证书_张三.pdf",
    date: "2026-04-10 11:20",
    size: "1.8 MB",
    iconColor: "text-red-500",
    bg: "bg-red-500/10",
  },
  {
    id: "6",
    type: "doc",
    name: "公证申请表_已签字.docx",
    date: "2026-04-09 15:45",
    size: "245 KB",
    iconColor: "text-blue-500",
    bg: "bg-blue-500/10",
  },
  {
    id: "7",
    type: "image",
    name: "护照扫描件.png",
    date: "2026-03-21 09:12",
    size: "2.1 MB",
    iconColor: "text-green-500",
    bg: "bg-green-500/10",
  },
];

let MOCK_RECORDS: any[] = [];
let MOCK_CLOUD_FILES: any[] = [];

const loadRecords = () => {
  if (MOCK_RECORDS.length > 0) return MOCK_RECORDS;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      MOCK_RECORDS = JSON.parse(data);
    } else {
      MOCK_RECORDS = [...INITIAL_RECORDS];
      saveRecords();
    }
  } catch (e) {
    MOCK_RECORDS = [...INITIAL_RECORDS];
  }
  return MOCK_RECORDS;
};

const saveRecords = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(MOCK_RECORDS));
  } catch (e) {
    console.error("Failed to save notary records", e);
  }
};

const loadCloudFiles = () => {
  if (MOCK_CLOUD_FILES.length > 0) return MOCK_CLOUD_FILES;
  try {
    const data = localStorage.getItem(CLOUD_FILES_KEY);
    if (data) {
      MOCK_CLOUD_FILES = JSON.parse(data);
    } else {
      MOCK_CLOUD_FILES = [...INITIAL_CLOUD_FILES];
      saveCloudFiles();
    }
  } catch (e) {
    MOCK_CLOUD_FILES = [...INITIAL_CLOUD_FILES];
  }
  return MOCK_CLOUD_FILES;
};

const saveCloudFiles = () => {
  try {
    localStorage.setItem(CLOUD_FILES_KEY, JSON.stringify(MOCK_CLOUD_FILES));
  } catch (e) {
    console.error("Failed to save cloud files", e);
  }
};

export const notaryService = {
  addRecord: async (record: Record<string, unknown>) => {
    loadRecords();
    MOCK_RECORDS = [
      {
        ...record,
        id: `n_${Date.now()}`,
        date: new Date().toISOString().split("T")[0],
        status: "processing",
      },
      ...MOCK_RECORDS,
    ];
    saveRecords();
  },
  updateRecordStatus: async (id: string, status: string) => {
    return new Promise((resolve) => {
      setTimeout(() => {
        loadRecords();
        const index = MOCK_RECORDS.findIndex((r) => r.id === id);
        if (index !== -1) {
          MOCK_RECORDS[index].status = status;
          saveRecords();
        }
        resolve(true);
      }, 300);
    });
  },
  getNotaryTypes: async () => {
    return [
      {
        id: "1",
        title: "证据保全公证",
        description:
          "对有可能灭失或难以取得的证据进行保全，如聊天记录、网页、录音等",
      },
      {
        id: "2",
        title: "委托公证",
        description: "用于办理房产买卖、诉讼、金融等相关委托事宜",
      },
      {
        id: "3",
        title: "声明书公证",
        description: "包括放弃继承权声明、单身声明、同意未成年子女出国声明等",
      },
      {
        id: "4",
        title: "财产协议公证",
        description: "婚前/婚内财产约定、离婚财产分割、家庭财产分配等",
      },
      {
        id: "5",
        title: "遗嘱公证",
        description: "立遗嘱人设立遗嘱，明确遗产分配方案",
      },
      {
        id: "6",
        title: "企业法人相关",
        description: "营业执照、法定代表人身份、公司章程、董事会决议等",
      },
      {
        id: "7",
        title: "金融业务相关",
        description: "赋予强制执行效力债权文书、抵押借款合同等",
      },
      {
        id: "8",
        title: "涉外民事公证",
        description: "出生、亲属关系、学历学位、无犯罪记录出国使用",
      },
    ];
  },
  getRecordTabs: async () => {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([
          { id: "all", label: "全部" },
          { id: "processing", label: "处理中" },
          { id: "completed", label: "已完成" },
          { id: "cancelled", label: "已取消" },
        ]);
      }, 50);
    });
  },
  getNotaryRecords: async (tab: string, page: number = 1) => {
    // Mock simulation
    return new Promise((resolve) => {
      setTimeout(() => {
        const records = loadRecords();

        const filtered = records.filter((r) => {
          if (tab === "all") return true;
          return r.status === tab;
        });

        // Simulate sort by date desc
        filtered.sort(
          (a, b) => new Date(b.date).getTime() - new Date(a.date).getTime(),
        );

        const pageSize = 15;
        const startIndex = (page - 1) * pageSize;
        const pagedRecords = filtered.slice(startIndex, startIndex + pageSize);

        resolve({
          records: pagedRecords,
          hasMore: startIndex + pageSize < filtered.length,
        });
      }, 500);
    });
  },
  getNotarySearchList: async (query: string = "") => {
    return new Promise((resolve) => {
      setTimeout(() => {
        const mockNotaries = [
          {
            id: "n6",
            name: "陈公证员",
            org: "成都市蜀都公证处",
            loc: "成都",
            active: true,
            initial: "C",
          },
          {
            id: "n2",
            name: "李公证员",
            org: "上海市东方公证处",
            loc: "上海",
            active: true,
            initial: "L",
          },
          {
            id: "n5",
            name: "刘公证员",
            org: "杭州市互联网公证处",
            loc: "杭州",
            active: true,
            initial: "L",
          },
          {
            id: "n3",
            name: "王公证员",
            org: "广州市南方公证处",
            loc: "广州",
            active: true,
            initial: "W",
          },
          {
            id: "n1",
            name: "张公证员",
            org: "北京市中信公证处",
            loc: "北京",
            active: true,
            initial: "Z",
          },
          {
            id: "n4",
            name: "赵公证员",
            org: "深圳市前海公证处",
            loc: "深圳",
            active: false,
            initial: "Z",
          },
          {
            id: "n7",
            name: "周公证员",
            org: "重庆市渝商公证处",
            loc: "重庆",
            active: false,
            initial: "Z",
          },
        ];
        const res = query
          ? mockNotaries.filter(
              (n) =>
                n.name.includes(query) ||
                n.org.includes(query) ||
                n.loc.includes(query),
            )
          : mockNotaries;
        resolve(res);
      }, 300);
    });
  },
  getNotaryDetail: async (id: string): Promise<NotaryDetailData> => {
    // Mocking an API call
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve({
          id,
          title: "公证 2026-03-19 17:13",
          time: "2026-03-19 17:13:46",
          item: "金融业务",
          notaryName: "公*员",
          notaryNo: "5391899435143469751",
          status: "待公证",
          remarks: "在我评论区进行骚扰诽谤造谣诋毁",
          parties: [
            {
              id: "p1",
              name: "刘*",
              gender: "男",
              status: "身份验证通过",
              avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/notaryparty/200x200.png",
              phone: "13800138000",
              idCard: "11010519900101234X",
              dob: "1990-01-01",
              idStartDate: "2020-01-01",
              idEndDate: "2040-01-01",
              address: "北京市朝阳区建国路88号",
              faceScore: "98.5",
              facePreview: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/notaryparty/200x200.png",
            },
          ],
          materials: [
            {
              id: "m1",
              name: "转账记录截图.png",
              size: "2.4 MB",
              uploadTime: "2026-03-19 17:10",
              fileType: "image",
              tags: [
                { label: "原件", color: "blue" },
                { label: "资金证明", color: "green" },
              ],
              uploader: "刘*",
            },
            {
              id: "m2",
              name: "被侵权网页长截图_完整版.jpg",
              size: "5.1 MB",
              uploadTime: "2026-03-19 17:11",
              fileType: "image",
              tags: [
                { label: "侵权证据", color: "red" },
                { label: "已校验", color: "gray" },
              ],
              uploader: "刘*",
            },
            {
              id: "m3",
              name: "借款合同扫描件_签署版.pdf",
              size: "12.8 MB",
              uploadTime: "2026-03-19 17:12",
              fileType: "pdf",
              tags: [
                { label: "复印件", color: "orange" },
                { label: "核心材料", color: "red" },
              ],
              uploader: "刘*",
            },
            {
              id: "m4",
              name: "侵权人在平台发布的视频证据.mp4",
              size: "45.2 MB",
              uploadTime: "2026-03-19 17:12",
              fileType: "video",
              tags: [{ label: "视听资料", color: "blue" }],
              uploader: "刘*",
            },
            {
              id: "m5",
              name: "聊天记录导出数据.zip",
              size: "1.2 GB",
              uploadTime: "2026-03-19 17:14",
              fileType: "zip",
              tags: [{ label: "归档包", color: "gray" }],
              uploader: "刘*",
            },
          ],
        });
      }, 500); // 500ms network delay simulation
    });
  },
  getNotaryMessages: async () => {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([
          {
            id: "1",
            title: "补充材料通知",
            content:
              "您申请的公证事项“金融业务”还需要补充双方的居住证明，请尽快在材料中心上传。",
            time: "10:45",
            unread: true,
          },
          {
            id: "2",
            title: "预约成功提醒",
            content:
              "您的视频通话预约已成功，时间为：2026-03-20 14:00。请确保证明材料和网络环境准备妥当。",
            time: "昨天",
            unread: false,
          },
          {
            id: "3",
            title: "申办进度更新",
            content:
              "您的公证办理状态已更新为：材料审核中。预计需要1-2个工作日，请耐心等待。",
            time: "星期一",
            unread: false,
          },
        ]);
      }, 300);
    });
  },
  getCloudFiles: async () => {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([...loadCloudFiles()]);
      }, 300);
    });
  },
  addCloudFile: async (file: Record<string, unknown>) => {
    return new Promise((resolve) => {
      setTimeout(() => {
        loadCloudFiles();
        MOCK_CLOUD_FILES = [file, ...MOCK_CLOUD_FILES];
        saveCloudFiles();
        resolve(true);
      }, 300);
    });
  },
  renameCloudFile: async (id: string, newName: string) => {
    return new Promise((resolve) => {
      setTimeout(() => {
        loadCloudFiles();
        const fileIndex = MOCK_CLOUD_FILES.findIndex((f) => f.id === id);
        if (fileIndex !== -1) {
          MOCK_CLOUD_FILES[fileIndex].name = newName;
          saveCloudFiles();
        }
        resolve(true);
      }, 300);
    });
  },
  deleteCloudFile: async (id: string) => {
    return new Promise((resolve) => {
      setTimeout(() => {
        loadCloudFiles();
        MOCK_CLOUD_FILES = MOCK_CLOUD_FILES.filter((f) => f.id !== id);
        saveCloudFiles();
        resolve(true);
      }, 300);
    });
  },
  getNotaryRoles: async () => {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([
          {
            name: "公证员",
            color:
              "bg-indigo-500/10 text-indigo-500 border border-indigo-500/20",
          },
          {
            name: "助理",
            color:
              "bg-emerald-500/10 text-emerald-600 border border-emerald-500/20",
          },
        ]);
      }, 300);
    });
  },
};
