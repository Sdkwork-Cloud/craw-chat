import React, { useCallback, useEffect, useState } from 'react';
import { Plus, Search, Video } from 'lucide-react';
import { cn, ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';
import {
  courseConsoleService,
  type ConsoleCourseCategoryItem,
  type ConsoleCourseItem,
} from './services/CourseConsoleService';

type ConsoleCourseTab = 'courses' | 'categories';

function formatStatus(status: string): string {
  switch (status) {
    case 'published':
      return '已发布';
    case 'draft':
      return '草稿';
    case 'archived':
      return '已归档';
    default:
      return status;
  }
}

export const ConsoleCourse: React.FC = () => {
  const [activeTab, setActiveTab] = useState<ConsoleCourseTab>('courses');
  const [courses, setCourses] = useState<ConsoleCourseItem[]>([]);
  const [categories, setCategories] = useState<ConsoleCourseCategoryItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [mutatingCourseId, setMutatingCourseId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [query, setQuery] = useState('');
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [createTitle, setCreateTitle] = useState('');
  const [createDescription, setCreateDescription] = useState('');
  const [createCategoryName, setCreateCategoryName] = useState('');
  const [creating, setCreating] = useState(false);
  const [creatingCategory, setCreatingCategory] = useState(false);
  const [selectedCourseId, setSelectedCourseId] = useState<string | null>(null);
  const [sections, setSections] = useState<Awaited<ReturnType<typeof courseConsoleService.listSections>>>([]);
  const [lessons, setLessons] = useState<Awaited<ReturnType<typeof courseConsoleService.listLessons>>>([]);
  const [curriculumLoading, setCurriculumLoading] = useState(false);
  const [newSectionTitle, setNewSectionTitle] = useState('');
  const [newLessonTitle, setNewLessonTitle] = useState('');
  const [newLessonSectionId, setNewLessonSectionId] = useState('');
  const [creatingSection, setCreatingSection] = useState(false);
  const [creatingLesson, setCreatingLesson] = useState(false);

  const selectedCourse = courses.find((course) => course.id === selectedCourseId) ?? null;

  const loadCourses = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await courseConsoleService.listCourses({
        q: query.trim() || undefined,
        limit: 50,
      });
      setCourses(result.items);
    } catch (loadError) {
      setError(loadError instanceof Error ? loadError.message : '课程列表加载失败');
      setCourses([]);
    } finally {
      setLoading(false);
    }
  }, [query]);

  const loadCategories = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await courseConsoleService.listCategories({
        q: query.trim() || undefined,
        limit: 100,
      });
      setCategories(result.items);
    } catch (loadError) {
      setError(loadError instanceof Error ? loadError.message : '分类列表加载失败');
      setCategories([]);
    } finally {
      setLoading(false);
    }
  }, [query]);

  useEffect(() => {
    if (activeTab === 'courses') {
      void loadCourses();
      return;
    }
    void loadCategories();
  }, [activeTab, loadCourses, loadCategories]);

  const loadCurriculum = useCallback(async (courseId: string) => {
    setCurriculumLoading(true);
    setError(null);
    try {
      const [sectionItems, lessonItems] = await Promise.all([
        courseConsoleService.listSections(courseId),
        courseConsoleService.listLessons(courseId),
      ]);
      setSections(sectionItems);
      setLessons(lessonItems);
    } catch (loadError) {
      setError(loadError instanceof Error ? loadError.message : '课程结构加载失败');
      setSections([]);
      setLessons([]);
    } finally {
      setCurriculumLoading(false);
    }
  }, []);

  useEffect(() => {
    if (!selectedCourseId) {
      setSections([]);
      setLessons([]);
      return;
    }
    void loadCurriculum(selectedCourseId);
  }, [selectedCourseId, loadCurriculum]);

  const handleCreateCourse = async () => {
    setCreating(true);
    setError(null);
    try {
      await courseConsoleService.createCourse({
        title: createTitle,
        description: createDescription,
      });
      setCreateTitle('');
      setCreateDescription('');
      setShowCreateForm(false);
      await loadCourses();
    } catch (createError) {
      setError(createError instanceof Error ? createError.message : '创建课程失败');
    } finally {
      setCreating(false);
    }
  };

  const handleCreateCategory = async () => {
    setCreatingCategory(true);
    setError(null);
    try {
      await courseConsoleService.createCategory({ name: createCategoryName });
      setCreateCategoryName('');
      setShowCreateForm(false);
      await loadCategories();
    } catch (createError) {
      setError(createError instanceof Error ? createError.message : '创建分类失败');
    } finally {
      setCreatingCategory(false);
    }
  };

  const handleCreateSection = async () => {
    if (!selectedCourseId) {
      return;
    }
    setCreatingSection(true);
    setError(null);
    try {
      await courseConsoleService.createSection(selectedCourseId, { title: newSectionTitle });
      setNewSectionTitle('');
      await loadCurriculum(selectedCourseId);
      await loadCourses();
    } catch (createError) {
      setError(createError instanceof Error ? createError.message : '创建章节失败');
    } finally {
      setCreatingSection(false);
    }
  };

  const handleCreateLesson = async () => {
    if (!selectedCourseId) {
      return;
    }
    setCreatingLesson(true);
    setError(null);
    try {
      await courseConsoleService.createLesson(selectedCourseId, {
        title: newLessonTitle,
        sectionId: newLessonSectionId || undefined,
      });
      setNewLessonTitle('');
      await loadCurriculum(selectedCourseId);
      await loadCourses();
    } catch (createError) {
      setError(createError instanceof Error ? createError.message : '创建课时失败');
    } finally {
      setCreatingLesson(false);
    }
  };

  const handlePublishToggle = async (course: ConsoleCourseItem) => {
    setMutatingCourseId(course.id);
    setError(null);
    try {
      if (course.status === 'published') {
        await courseConsoleService.unpublishCourse(course.id);
      } else {
        await courseConsoleService.publishCourse(course.id);
      }
      await loadCourses();
    } catch (mutationError) {
      setError(mutationError instanceof Error ? mutationError.message : '更新课程状态失败');
    } finally {
      setMutatingCourseId(null);
    }
  };

  return (
    <div className="flex flex-col h-full bg-console-bg-panel rounded-xl border border-console-border overflow-hidden">
      <div className="p-6 border-b border-console-border flex flex-wrap gap-4 items-center justify-between bg-console-bg-panel/50">
        <div className="flex items-center gap-3">
          <Video size={22} className="text-blue-600" />
          <div>
            <h2 className="text-lg font-semibold text-console-text-main mb-1">课程管理</h2>
            <p className="text-sm text-console-text-muted">
              通过 sdkwork-course-backend-sdk 管理课程、分类与发布流程
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <div className="inline-flex rounded-lg border border-console-border overflow-hidden">
            <button
              type="button"
              onClick={() => setActiveTab('courses')}
              className={cn(
                'px-3 py-1.5 text-xs font-medium',
                activeTab === 'courses'
                  ? 'bg-blue-600 text-white'
                  : 'text-console-text-muted hover:text-console-text-main',
              )}
            >
              课程
            </button>
            <button
              type="button"
              onClick={() => setActiveTab('categories')}
              className={cn(
                'px-3 py-1.5 text-xs font-medium',
                activeTab === 'categories'
                  ? 'bg-blue-600 text-white'
                  : 'text-console-text-muted hover:text-console-text-main',
              )}
            >
              分类
            </button>
          </div>
          <button
            type="button"
            onClick={() => setShowCreateForm((current) => !current)}
            className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2"
          >
            <Plus size={16} />
            {activeTab === 'categories' ? '创建分类' : '创建课程'}
          </button>
        </div>
      </div>

      {showCreateForm ? (
        <div className="p-4 border-b border-console-border bg-console-bg-root/30">
          <div className="grid gap-3 max-w-xl">
            {activeTab === 'categories' ? (
              <>
                <input
                  type="text"
                  value={createCategoryName}
                  onChange={(event) => setCreateCategoryName(event.target.value)}
                  placeholder="分类名称"
                  className="w-full bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none"
                />
                <div className="flex gap-2">
                  <button
                    type="button"
                    disabled={creatingCategory || !createCategoryName.trim()}
                    onClick={() => void handleCreateCategory()}
                    className="bg-blue-600 hover:bg-blue-700 disabled:opacity-60 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors"
                  >
                    {creatingCategory ? '创建中...' : '保存分类'}
                  </button>
                  <button
                    type="button"
                    onClick={() => setShowCreateForm(false)}
                    className="px-4 py-2 rounded-lg text-sm font-medium border border-console-border text-console-text-muted hover:text-console-text-main"
                  >
                    取消
                  </button>
                </div>
              </>
            ) : (
              <>
            <input
              type="text"
              value={createTitle}
              onChange={(event) => setCreateTitle(event.target.value)}
              placeholder="课程标题"
              className="w-full bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none"
            />
            <textarea
              value={createDescription}
              onChange={(event) => setCreateDescription(event.target.value)}
              placeholder="课程简介（可选）"
              rows={3}
              className="w-full bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none resize-none"
            />
            <div className="flex gap-2">
              <button
                type="button"
                disabled={creating || !createTitle.trim()}
                onClick={() => void handleCreateCourse()}
                className="bg-blue-600 hover:bg-blue-700 disabled:opacity-60 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors"
              >
                {creating ? '创建中...' : '保存草稿'}
              </button>
              <button
                type="button"
                onClick={() => setShowCreateForm(false)}
                className="px-4 py-2 rounded-lg text-sm font-medium border border-console-border text-console-text-muted hover:text-console-text-main"
              >
                取消
              </button>
            </div>
              </>
            )}
          </div>
        </div>
      ) : null}

      <div className="p-4 border-b border-console-border">
        <div className="relative max-w-md">
          <Search size={14} className="absolute left-3 top-1/2 -translate-y-1/2 text-console-text-muted" />
          <input
            type="search"
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="搜索课程名称或编码..."
            className="w-full bg-console-input-bg border border-console-border rounded-lg py-2 pl-9 pr-3 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none"
          />
        </div>
      </div>

      {error ? (
        <div className="px-4 pt-3 text-sm text-red-600 dark:text-red-400">{error}</div>
      ) : null}

      {loading ? (
        <div className="p-8 text-center text-console-text-muted">
          {activeTab === 'categories' ? '加载分类中...' : '加载课程中...'}
        </div>
      ) : activeTab === 'categories' ? (
        categories.length === 0 ? (
          <ConsoleContractEmptyState
            title="暂无分类"
            description="点击“创建分类”开始维护课程目录结构。"
          />
        ) : (
          <div className="flex-1 overflow-y-auto p-4">
            <div className="grid gap-3">
              {categories.map((category) => (
                <div
                  key={category.id}
                  className="rounded-xl border border-console-border bg-console-bg-root/40 p-4 flex items-center justify-between gap-4"
                >
                  <div className="min-w-0">
                    <h3 className="text-sm font-semibold text-console-text-main truncate">{category.name}</h3>
                    <p className="text-xs text-console-text-muted truncate">
                      {category.slug || category.id}
                      {category.sortOrder ? ` · 排序 ${category.sortOrder}` : ''}
                    </p>
                  </div>
                  <span className="text-[10px] px-2 py-0.5 rounded-full border text-console-text-muted bg-console-bg-panel border-console-border">
                    {category.status}
                  </span>
                </div>
              ))}
            </div>
          </div>
        )
      ) : courses.length === 0 ? (
        <ConsoleContractEmptyState
          title="暂无课程"
          description="点击“创建课程”开始录入第一门企业培训课程。"
        />
      ) : (
        <div className="flex-1 overflow-y-auto p-4">
          <div className="grid gap-3">
            {courses.map((course) => (
              <div
                key={course.id}
                className={cn(
                  'rounded-xl border bg-console-bg-root/40 p-4 flex items-start justify-between gap-4',
                  selectedCourseId === course.id
                    ? 'border-blue-500 ring-1 ring-blue-500/30'
                    : 'border-console-border',
                )}
              >
                <button
                  type="button"
                  className="min-w-0 text-left"
                  onClick={() => setSelectedCourseId(course.id)}
                >
                  <div className="flex items-center gap-2 mb-1">
                    <h3 className="text-sm font-semibold text-console-text-main truncate">{course.title}</h3>
                    <span
                      className={cn(
                        'text-[10px] px-2 py-0.5 rounded-full border',
                        course.status === 'published'
                          ? 'text-emerald-700 bg-emerald-50 border-emerald-200 dark:text-emerald-300 dark:bg-emerald-500/10 dark:border-emerald-500/20'
                          : 'text-console-text-muted bg-console-bg-panel border-console-border',
                      )}
                    >
                      {formatStatus(course.status)}
                    </span>
                  </div>
                  <p className="text-xs text-console-text-muted truncate">
                    {course.courseCode || course.id}
                    {course.category ? ` · ${course.category}` : ''}
                    {course.instructor ? ` · ${course.instructor}` : ''}
                  </p>
                </button>
                <div className="flex items-start gap-3 shrink-0">
                  <div className="text-right text-xs text-console-text-muted">
                    <div>{course.lessonsCount} 课时</div>
                    <div>{course.studentsCount} 学员</div>
                  </div>
                  <button
                    type="button"
                    disabled={mutatingCourseId === course.id}
                    onClick={() => void handlePublishToggle(course)}
                    className="px-3 py-1.5 rounded-lg text-xs font-medium border border-console-border text-console-text-main hover:bg-console-bg-hover disabled:opacity-60"
                  >
                    {mutatingCourseId === course.id
                      ? '处理中...'
                      : course.status === 'published'
                        ? '下架'
                        : '发布'}
                  </button>
                </div>
              </div>
            ))}
          </div>

          {selectedCourse ? (
            <div className="mt-6 rounded-xl border border-console-border bg-console-bg-panel/40 p-4">
              <div className="flex items-center justify-between gap-3 mb-4">
                <div>
                  <h3 className="text-sm font-semibold text-console-text-main">课程结构：{selectedCourse.title}</h3>
                  <p className="text-xs text-console-text-muted">管理章节与课时内容</p>
                </div>
                <button
                  type="button"
                  onClick={() => setSelectedCourseId(null)}
                  className="text-xs text-console-text-muted hover:text-console-text-main"
                >
                  关闭
                </button>
              </div>

              {curriculumLoading ? (
                <div className="py-6 text-center text-sm text-console-text-muted">加载课程结构中...</div>
              ) : (
                <div className="grid gap-6 lg:grid-cols-2">
                  <div>
                    <div className="flex items-center justify-between mb-2">
                      <h4 className="text-xs font-semibold text-console-text-main">章节</h4>
                      <span className="text-xs text-console-text-muted">{sections.length} 个</span>
                    </div>
                    <div className="flex gap-2 mb-3">
                      <input
                        type="text"
                        value={newSectionTitle}
                        onChange={(event) => setNewSectionTitle(event.target.value)}
                        placeholder="新章节标题"
                        className="flex-1 bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none"
                      />
                      <button
                        type="button"
                        disabled={creatingSection || !newSectionTitle.trim()}
                        onClick={() => void handleCreateSection()}
                        className="px-3 py-2 rounded-lg text-xs font-medium bg-blue-600 text-white disabled:opacity-60"
                      >
                        添加
                      </button>
                    </div>
                    <div className="space-y-2">
                      {sections.map((section) => (
                        <div
                          key={section.id}
                          className="rounded-lg border border-console-border px-3 py-2 text-sm text-console-text-main"
                        >
                          {section.title}
                          <span className="ml-2 text-xs text-console-text-muted">
                            {section.lessonCount} 课时
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>

                  <div>
                    <div className="flex items-center justify-between mb-2">
                      <h4 className="text-xs font-semibold text-console-text-main">课时</h4>
                      <span className="text-xs text-console-text-muted">{lessons.length} 个</span>
                    </div>
                    <div className="grid gap-2 mb-3">
                      <input
                        type="text"
                        value={newLessonTitle}
                        onChange={(event) => setNewLessonTitle(event.target.value)}
                        placeholder="新课时标题"
                        className="w-full bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none"
                      />
                      <div className="flex gap-2">
                        <select
                          value={newLessonSectionId}
                          onChange={(event) => setNewLessonSectionId(event.target.value)}
                          className="flex-1 bg-console-input-bg border border-console-border rounded-lg py-2 px-3 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none"
                        >
                          <option value="">未指定章节</option>
                          {sections.map((section) => (
                            <option key={section.id} value={section.id}>
                              {section.title}
                            </option>
                          ))}
                        </select>
                        <button
                          type="button"
                          disabled={creatingLesson || !newLessonTitle.trim()}
                          onClick={() => void handleCreateLesson()}
                          className="px-3 py-2 rounded-lg text-xs font-medium bg-blue-600 text-white disabled:opacity-60"
                        >
                          添加
                        </button>
                      </div>
                    </div>
                    <div className="space-y-2">
                      {lessons.map((lesson) => (
                        <div
                          key={lesson.id}
                          className="rounded-lg border border-console-border px-3 py-2 text-sm text-console-text-main"
                        >
                          {lesson.title}
                          {lesson.sectionId ? (
                            <span className="ml-2 text-xs text-console-text-muted">章节 {lesson.sectionId}</span>
                          ) : null}
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              )}
            </div>
          ) : null}
        </div>
      )}
    </div>
  );
};
