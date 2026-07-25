import { useTranslation } from "react-i18next";
export interface CloudFile {
  id: string;
  name: string;
  type: string;
  size: string;
  date: string;
  owner: string;
}

const STORAGE_KEY = "sdkwork_im_h5_cloud_drive_files";

let _files: CloudFile[] = [];

const INITIAL_FILES: CloudFile[] = [
  {
    id: "1",
    name: "2023年度总结报告.pdf",
    type: "pdf",
    size: "2.4 MB",
    date: "2023-10-24 10:30",
    owner: "我",
  },
  {
    id: "2",
    name: "Q3研发部门OKR.xlsx",
    type: "excel",
    size: "156 KB",
    date: "2023-10-23 15:45",
    owner: "张三",
  },
  {
    id: "3",
    name: "产品原型设计",
    type: "folder",
    size: "-",
    date: "2023-10-20 09:15",
    owner: "李四",
  },
  {
    id: "4",
    name: "企业宣传片素材.mp4",
    type: "video",
    size: "128 MB",
    date: "2023-10-18 14:20",
    owner: "市场部",
  },
  {
    id: "5",
    name: "活动现场照片_01.jpg",
    type: "image",
    size: "3.2 MB",
    date: "2023-10-15 11:10",
    owner: "我",
  },
];

const loadFiles = () => {
  if (_files.length > 0) return _files;
  try {
    const data = localStorage.getItem(STORAGE_KEY);
    if (data) {
      _files = JSON.parse(data);
    } else {
      _files = [...INITIAL_FILES];
      saveFiles();
    }
  } catch (e) {
    _files = [...INITIAL_FILES];
  }
  return _files;
};

const saveFiles = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(_files));
  } catch (e) {}
};

export class CloudDriveService {
  static async getFiles(): Promise<CloudFile[]> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve([...loadFiles()]);
      }, 300);
    });
  }

  static async uploadFile(file: File): Promise<CloudFile> {
    loadFiles();
    const newFile: CloudFile = {
      id: Math.random().toString(36).substring(7),
      name: file.name,
      type: file.type.split("/")[0] || "file",
      size: `${(file.size / 1024 / 1024).toFixed(2)} MB`,
      date: new Date().toISOString().split("T")[0],
      owner: "我",
    };
    _files.push(newFile);
    saveFiles();
    return newFile;
  }

  static async createFolder(name: string): Promise<CloudFile> {
    loadFiles();
    const newFolder: CloudFile = {
      id: Math.random().toString(36).substring(7),
      name: name,
      type: "folder",
      size: "-",
      date: new Date().toISOString().split("T")[0],
      owner: "我",
    };
    _files.unshift(newFolder);
    saveFiles();
    return newFolder;
  }

  static async deleteFile(id: string): Promise<void> {
    loadFiles();
    _files = _files.filter((f) => f.id !== id);
    saveFiles();
  }

  static async renameFile(id: string, newName: string): Promise<void> {
    loadFiles();
    const file = _files.find((f) => f.id === id);
    if (file) {
      file.name = newName;
      saveFiles();
    }
  }
}
