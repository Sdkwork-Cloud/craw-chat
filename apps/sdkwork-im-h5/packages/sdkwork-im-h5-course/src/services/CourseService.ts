import { useTranslation } from "react-i18next";
export interface CourseData {
  id: string;
  title: string;
  instructor: string;
  rating: number;
  students: number;
  price: number;
  originalPrice: number;
  duration: string;
  totalLessons?: number;
  cover: string;
  category: string;
  type?: 'live' | 'recorded' | string;
  liveStatus?: string;
  instructorDesc?: string;
  advantages?: string[];
  isPurchased?: boolean;
  curriculum?: any[];
}

export interface MyCourseData {
  id: string;
  title: string;
  cover: string;
  progress: number;
  totalLessons: number;
  completedLessons: number;
  lastWatched: string;
  isLive?: boolean;
}

const mockCourses: CourseData[] = [
  {
    id: "c5",
    title: "【直播】2026 前端架构演进与高频面试题精讲",
    instructor: "张工 - 资深架构师",
    instructorDesc: "前一线大厂前端架构师，拥有10年以上的研发经验，曾主导多个亿级用户产品的研发。",
    rating: 4.9,
    students: 3200,
    price: 9.9,
    originalPrice: 99,
    duration: "正在直播中",
    totalLessons: 1,
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c5/600x400.png",
    category: "live",
    type: "live",
    liveStatus: "live",
    isPurchased: false,
    advantages: [
      "2026 最新前端架构演进趋势解析",
      "高频面试题现场解答与连麦",
      "加入讲师专属直播答疑群",
      "干货满满，带源码与 PPT"
    ],
    curriculum: [
      { section: "直播大纲", lessons: [{ id: "l-live1", title: "前端架构的发展与演进", free: true, duration: "45:00" }, { id:"l-live2", title: "2026 高频面试题解析", free: true, duration: "60:00" }, { id:"l-live3", title: "互动连麦答疑", free: true, duration: "30:00" }] },
    ]
  },
  {
    id: "c6",
    title: "【直播预约】2027 Serverless 架构与前端未来",
    instructor: "李工 - 云原生专家",
    instructorDesc: "多年的云原生及前端基础架构研发经验。",
    rating: 4.9,
    students: 1500,
    price: 1,
    originalPrice: 99,
    duration: "周五 20:00 开播",
    totalLessons: 1,
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c6/600x400.png",
    category: "live",
    type: "live",
    liveStatus: "upcoming",
    isPurchased: true,
    advantages: [
      "Serverless 应用实战",
      "结合云函数进行低成本开发",
      "全栈化视野下的大前端演进"
    ],
    curriculum: [
      { section: "直播大纲", lessons: [{ id: "l-l1", title: "无服务器时代的前端", free: true, duration: "10:00" }] },
    ]
  },
  {
    id: "c7",
    title: "【直播预约】现代 AI 前端应用构建指南",
    instructor: "AI架构师",
    instructorDesc: "前大厂AIGC业务核心骨干",
    rating: 4.8,
    students: 2310,
    price: 0,
    originalPrice: 199,
    duration: "下周一 19:30 开播",
    totalLessons: 1,
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c7/600x400.png",
    category: "live",
    type: "live",
    liveStatus: "upcoming",
    isPurchased: false,
    advantages: [
      "AI 前端工程化技术揭秘",
      "基于大模型快速生成UI组件",
    ],
    curriculum: [
      { section: "直播大纲", lessons: [{ id: "l-l2", title: "如何构建生成式UI", free: true, duration: "30:00" }] },
    ]
  },
  {
    id: "c1",
    title: "零基础全栈开发大师课 - React19 + TS 深度解析",
    instructor: "张工 - 资深架构师",
    instructorDesc: "前一线大厂前端架构师，拥有10年以上的研发经验，曾主导多个亿级用户产品的研发。",
    rating: 4.8,
    students: 12500,
    price: 299,
    originalPrice: 599,
    duration: "45小时",
    totalLessons: 128,
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c1/600x400.png",
    category: "tech",
    type: "recorded",
    isPurchased: true,
    advantages: [
      "从零基础到独立开发的完整闭环",
      "基于 React 19 最新特性与 TailwindCSS",
      "配备专属班主任与答疑辅导",
      "实战级企业项目源码解析"
    ],
    curriculum: [
      { 
        section: "第一章：基础预备", 
        lessons: [
          { id: "l1", title: "开发环境搭建", free: true, duration: "12:45", completed: true, videoUrl: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/elephants-dream.mp4" }, 
          { id: "l2", title: "现代 JS 核心语法", free: true, duration: "24:30", completed: false, videoUrl: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/big-buck-bunny.mp4" }, 
          { id: "l3", title: "TS 速成", free: true, duration: "45:10", completed: false, videoUrl: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/sintel.mp4" }
        ] 
      },
      { 
        section: "第二章：React 核心", 
        lessons: [
          { id: "l4", title: "组件化思想", free: false, duration: "18:20", completed: false, videoUrl: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/tears-of-steel.mp4" }, 
          { id: "l5", title: "Hooks 深度解析", free: false, duration: "32:15", completed: false, videoUrl: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/elephants-dream.mp4" }, 
          { id: "l6", title: "状态管理与数据流", free: false, duration: "41:00", completed: false, videoUrl: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/videos/big-buck-bunny.mp4" }
        ] 
      },
    ]
  },
  {
    id: "c2",
    title: "企业级 UI 设计实战 - Figma 高级进阶指南",
    instructor: "Lisa UI总监",
    rating: 4.9,
    students: 8300,
    price: 199,
    originalPrice: 399,
    duration: "20小时",
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c2/600x400.png",
    category: "design",
    type: "recorded",
  },
  {
    id: "c3",
    title: "产品经理的核心素养：从需求到产品落地",
    instructor: "王总 - 前产品副总裁",
    rating: 4.7,
    students: 15600,
    price: 159,
    originalPrice: 299,
    duration: "15小时",
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c3/600x400.png",
    category: "business",
    type: "recorded",
  },
  {
    id: "c4",
    title: "商业插画与视觉表现速成班",
    instructor: "Mike 自由插画师",
    rating: 4.9,
    students: 5400,
    price: 259,
    originalPrice: 499,
    duration: "12小时",
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c4/600x400.png",
    category: "design",
    type: "recorded",
  }
];

const mockMyCourses: MyCourseData[] = [
  {
    id: "c6",
    title: "【直播预约】2027 Serverless 架构与前端未来",
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c6/300x200.png",
    progress: 0,
    totalLessons: 1,
    completedLessons: 0,
    lastWatched: "已预约直播",
    isLive: true,
  },
  {
    id: "c1",
    title: "零基础全栈开发大师课 - React19 + TS 深度解析",
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c1/300x200.png",
    progress: 35,
    totalLessons: 128,
    completedLessons: 45,
    lastWatched: "第3章 第2节: React Hooks 深度解析",
  },
  {
    id: "c5",
    title: "【直播】2026 前端架构演进与高频面试题精讲",
    cover: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/c5/300x200.png",
    progress: 0,
    totalLessons: 1,
    completedLessons: 0,
    lastWatched: "正在直播中",
    isLive: true,
  }
];

export interface CourseDiscussion {
  id: string;
  user: {
    name: string;
    avatar: string;
  };
  content: string;
  likes: number;
  time: string;
  reply?: {
    author: string;
    content: string;
  };
}

const mockDiscussions: CourseDiscussion[] = [
  {
    id: "d1",
    user: { name: "飞奔的蜗牛", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/u1/100x100.png" },
    content: "老师讲得太透彻了！特别是 Hooks 闭包陷阱这里，彻底理解了 useEffect 的依赖数组原理。",
    likes: 12,
    time: "昨天 14:20"
  },
  {
    id: "d2",
    user: { name: "前端小菜鸟", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/u2/100x100.png" },
    content: "请问 12:45 演示的代码怎么拿到最新 state？",
    likes: 5,
    time: "2天前",
    reply: {
      author: "讲师回复",
      content: "可以使用 useRef 来保存最新的值，或者在下一节课的状态管理章节会有更详细的 useReducer 方案说明。"
    }
  }
];

export class CourseService {
  /**
   * Fetch all courses for the home page, optionally filtered by category
   */
  static async getCourses(category: string = 'all'): Promise<CourseData[]> {
    // Simulate network delay
    await new Promise(resolve => setTimeout(resolve, 300));
    if (category === 'all') {
      return mockCourses;
    }
    return mockCourses.filter(c => c.category === category);
  }

  /**
   * Get detailed information for a specific course
   */
  static async getCourseDetail(courseId: string): Promise<CourseData | undefined> {
    await new Promise(resolve => setTimeout(resolve, 200));
    return mockCourses.find(c => c.id === courseId);
  }

  /**
   * Get courses the user has purchased
   */
  static async getMyCourses(): Promise<MyCourseData[]> {
    await new Promise(resolve => setTimeout(resolve, 300));
    return mockMyCourses;
  }

  /**
   * Process a purchase
   */
  static async purchaseCourse(courseId: string, paymentMethod: string): Promise<boolean> {
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    // Update local mock state for UI consistency
    const course = mockCourses.find(c => c.id === courseId);
    if (course) {
       course.isPurchased = true;
       // Add to my courses if not exists
       const existing = mockMyCourses.find(c => c.id === courseId);
       if (!existing) {
          mockMyCourses.push({
             id: course.id,
             title: course.title,
             cover: course.cover,
             progress: 0,
             totalLessons: course.totalLessons || 1,
             completedLessons: 0,
             lastWatched: "刚刚购买",
             isLive: course.type === 'live'
          });
       }
    }
    return true; // Simulate success
  }

  /**
   * Get discussions for a course
   */
  static async getCourseDiscussions(courseId: string, lessonId?: string): Promise<CourseDiscussion[]> {
    await new Promise(resolve => setTimeout(resolve, 300));
    return mockDiscussions;
  }

  /**
   * Post a discussion comment
   */
  static async postDiscussion(courseId: string, lessonId: string | undefined, content: string): Promise<CourseDiscussion> {
    await new Promise(resolve => setTimeout(resolve, 500));
    const newComment: CourseDiscussion = {
      id: "d" + Date.now(),
      user: { name: "我", avatar: "https://cdn.sdkwork.com/apps/sdkwork-im-h5/mock/images/me/100x100.png" },
      content,
      likes: 0,
      time: "刚刚"
    };
    mockDiscussions.unshift(newComment);
    return newComment;
  }
}

export const useCourseService = () => {
  const { t } = useTranslation();
return CourseService;
}
