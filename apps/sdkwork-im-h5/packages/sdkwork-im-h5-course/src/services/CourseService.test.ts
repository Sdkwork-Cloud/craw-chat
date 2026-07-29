import assert from "node:assert/strict";
import test from "node:test";

import { CourseCapabilityUnavailableError, CourseService } from "./CourseService";

test("course operations fail closed", async () => {
  for (const operation of [
    () => CourseService.getCourses(),
    () => CourseService.getCourseDetail("course-id"),
    () => CourseService.getMyCourses(),
    () => CourseService.purchaseCourse("course-id", "payment-method"),
    () => CourseService.getCourseDiscussions("course-id", "lesson-id"),
    () => CourseService.postDiscussion("course-id", "lesson-id", "Comment"),
  ]) {
    await assert.rejects(operation, CourseCapabilityUnavailableError);
  }
});
