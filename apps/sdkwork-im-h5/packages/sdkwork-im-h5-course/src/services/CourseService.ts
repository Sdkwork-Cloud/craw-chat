export interface CourseLesson {
  id: string;
  title: string;
  duration: string;
  completed?: boolean;
  free?: boolean;
  videoUrl?: string;
}

export interface CourseSection {
  section: string;
  lessons: CourseLesson[];
}

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
  type?: "live" | "recorded" | string;
  liveStatus?: string;
  instructorDesc?: string;
  advantages?: string[];
  isPurchased?: boolean;
  curriculum?: CourseSection[];
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

export class CourseCapabilityUnavailableError extends Error {
  constructor() {
    super("Courses are unavailable because the Course owner SDK and payment flow are not composed.");
    this.name = "CourseCapabilityUnavailableError";
  }
}

export class CourseService {
  static async getCourses(_category = "all"): Promise<CourseData[]> {
    throw new CourseCapabilityUnavailableError();
  }

  static async getCourseDetail(_courseId: string): Promise<CourseData | undefined> {
    throw new CourseCapabilityUnavailableError();
  }

  static async getMyCourses(): Promise<MyCourseData[]> {
    throw new CourseCapabilityUnavailableError();
  }

  static async purchaseCourse(_courseId: string, _paymentMethod: string): Promise<boolean> {
    throw new CourseCapabilityUnavailableError();
  }

  static async getCourseDiscussions(
    _courseId: string,
    _lessonId?: string,
  ): Promise<CourseDiscussion[]> {
    throw new CourseCapabilityUnavailableError();
  }

  static async postDiscussion(
    _courseId: string,
    _lessonId: string | undefined,
    _content: string,
  ): Promise<CourseDiscussion> {
    throw new CourseCapabilityUnavailableError();
  }
}

export function useCourseService() {
  return CourseService;
}
