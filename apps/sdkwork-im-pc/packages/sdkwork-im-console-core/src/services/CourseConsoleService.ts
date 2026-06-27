import { getCourseBackendSdkClientWithSession } from '@sdkwork/im-pc-core';
import {
  readNumber,
  readRecords,
  readSingleRecord,
  readString,
  unwrapCourseBackendEnvelope,
  asRecord,
} from './courseBackendResponse';

export interface ConsoleCourseItem {
  id: string;
  courseCode: string;
  title: string;
  status: string;
  studentsCount: number;
  lessonsCount: number;
  category?: string;
  instructor?: string;
}

export interface ConsoleCourseListResult {
  items: ConsoleCourseItem[];
  total: number;
}

export interface CreateConsoleCourseInput {
  title: string;
  subtitle?: string;
  description?: string;
}

export interface ConsoleCourseCategoryItem {
  id: string;
  name: string;
  slug: string;
  status: string;
  sortOrder: number;
}

export interface ConsoleCourseCategoryListResult {
  items: ConsoleCourseCategoryItem[];
  total: number;
}

export interface CreateConsoleCourseCategoryInput {
  name: string;
  slug?: string;
  description?: string;
}

export interface ConsoleCourseSectionItem {
  id: string;
  courseId: string;
  title: string;
  lessonCount: number;
  sortOrder: number;
  status: string;
}

export interface ConsoleCourseLessonItem {
  id: string;
  courseId: string;
  sectionId?: string;
  title: string;
  durationSeconds: number;
  freePreview: boolean;
  status: string;
}

export interface CreateConsoleCourseSectionInput {
  title: string;
  description?: string;
}

export interface CreateConsoleCourseLessonInput {
  title: string;
  sectionId?: string;
  description?: string;
  freePreview?: boolean;
}

function mapCategoryItem(record: ReturnType<typeof asRecord>): ConsoleCourseCategoryItem {
  return {
    id: readString(record, ['id', 'categoryId']),
    name: readString(record, ['name', 'title'], '未命名分类'),
    slug: readString(record, ['slug', 'code']),
    status: readString(record, ['status'], 'active'),
    sortOrder: readNumber(record, ['sortOrder', 'sort_order', 'order']),
  };
}

function mapSectionItem(record: ReturnType<typeof asRecord>): ConsoleCourseSectionItem {
  return {
    id: readString(record, ['id', 'sectionId']),
    courseId: readString(record, ['courseId', 'course_id']),
    title: readString(record, ['title', 'name'], '未命名章节'),
    lessonCount: readNumber(record, ['lessonCount', 'lesson_count']),
    sortOrder: readNumber(record, ['sortWeight', 'sort_weight', 'sortOrder']),
    status: readString(record, ['status'], 'active'),
  };
}

function mapLessonItem(record: ReturnType<typeof asRecord>): ConsoleCourseLessonItem {
  return {
    id: readString(record, ['id', 'lessonId']),
    courseId: readString(record, ['courseId', 'course_id']),
    sectionId: readString(record, ['sectionId', 'section_id']) || undefined,
    title: readString(record, ['title', 'name'], '未命名课时'),
    durationSeconds: readNumber(record, ['durationSeconds', 'duration_seconds']),
    freePreview: Boolean(record.freePreview ?? record.free_preview),
    status: readString(record, ['status'], 'draft'),
  };
}

function mapCourseItem(record: ReturnType<typeof asRecord>): ConsoleCourseItem {
  return {
    id: readString(record, ['id', 'courseId']),
    courseCode: readString(record, ['courseCode', 'course_code', 'code']),
    title: readString(record, ['title', 'name'], '未命名课程'),
    status: readString(record, ['status'], 'draft'),
    studentsCount: readNumber(record, ['studentsCount', 'students_count', 'studentCount']),
    lessonsCount: readNumber(record, ['lessonsCount', 'lessons_count', 'lessonCount']),
    category: readString(record, ['category', 'categoryName']) || undefined,
    instructor: readString(record, ['instructor', 'instructorName']) || undefined,
  };
}

export const courseConsoleService = {
  async listCourses(params?: { q?: string; status?: string; limit?: number }): Promise<ConsoleCourseListResult> {
    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courses.list({
      q: params?.q,
      status: params?.status,
      limit: params?.limit ?? 50,
    });
    const items = readRecords(response, ['items', 'records', 'data', 'list', 'rows', 'content', 'courses'])
      .map(mapCourseItem)
      .filter((item) => item.id);
    const page = asRecord(unwrapCourseBackendEnvelope(response));
    const total = readNumber(page, ['total'], items.length);
    return { items, total };
  },

  async createCourse(input: CreateConsoleCourseInput): Promise<ConsoleCourseItem> {
    const title = input.title.trim();
    if (!title) {
      throw new Error('课程标题不能为空');
    }

    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courses.create({
      title,
      subtitle: input.subtitle?.trim() || undefined,
      description: input.description?.trim() || undefined,
      tags: [],
    });
    return mapCourseItem(readSingleRecord(response));
  },

  async publishCourse(courseId: string): Promise<ConsoleCourseItem> {
    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courses.publish(courseId, {});
    return mapCourseItem(readSingleRecord(response));
  },

  async unpublishCourse(courseId: string): Promise<ConsoleCourseItem> {
    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courses.unpublish(courseId, {});
    return mapCourseItem(readSingleRecord(response));
  },

  async listCategories(params?: { q?: string; limit?: number }): Promise<ConsoleCourseCategoryListResult> {
    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courseCategories.list({
      q: params?.q,
      limit: params?.limit ?? 100,
    });
    const items = readRecords(response, ['items', 'records', 'data', 'list', 'rows', 'content', 'categories'])
      .map(mapCategoryItem)
      .filter((item) => item.id);
    const page = asRecord(unwrapCourseBackendEnvelope(response));
    const total = readNumber(page, ['total'], items.length);
    return { items, total };
  },

  async createCategory(input: CreateConsoleCourseCategoryInput): Promise<ConsoleCourseCategoryItem> {
    const name = input.name.trim();
    if (!name) {
      throw new Error('分类名称不能为空');
    }

    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courseCategories.create({
      name,
      slug: input.slug?.trim() || undefined,
      description: input.description?.trim() || undefined,
    });
    return mapCategoryItem(readSingleRecord(response));
  },

  async listSections(courseId: string): Promise<ConsoleCourseSectionItem[]> {
    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courseSections.list(courseId, { limit: 200 });
    return readRecords(response, ['items', 'records', 'data', 'list', 'rows', 'content', 'sections'])
      .map(mapSectionItem)
      .filter((item) => item.id);
  },

  async createSection(
    courseId: string,
    input: CreateConsoleCourseSectionInput,
  ): Promise<ConsoleCourseSectionItem> {
    const title = input.title.trim();
    if (!title) {
      throw new Error('章节标题不能为空');
    }

    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courseSections.create(courseId, {
      title,
      description: input.description?.trim() || undefined,
    });
    return mapSectionItem(readSingleRecord(response));
  },

  async listLessons(courseId: string): Promise<ConsoleCourseLessonItem[]> {
    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courseLessons.list(courseId, { limit: 500 });
    return readRecords(response, ['items', 'records', 'data', 'list', 'rows', 'content', 'lessons'])
      .map(mapLessonItem)
      .filter((item) => item.id);
  },

  async createLesson(
    courseId: string,
    input: CreateConsoleCourseLessonInput,
  ): Promise<ConsoleCourseLessonItem> {
    const title = input.title.trim();
    if (!title) {
      throw new Error('课时标题不能为空');
    }

    const client = getCourseBackendSdkClientWithSession();
    const response = await client.courseLessons.create(courseId, {
      title,
      sectionId: input.sectionId,
      description: input.description?.trim() || undefined,
      freePreview: input.freePreview ?? false,
    });
    return mapLessonItem(readSingleRecord(response));
  },
};
