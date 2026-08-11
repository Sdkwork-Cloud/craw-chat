import React from "react";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";

type ComponentName = "CourseHome" | "MyCourses" | "CourseDetail" | "CoursePurchase" | "CoursePlayer" | "CourseLiveRoom" | "CourseSearch";

function lazyComponent(name: ComponentName) {
  return React.lazy(async () => {
    const mod = await import("@sdkwork/course-mobile-react-courses");
    return { default: mod[name] };
  });
}

const CourseHome = lazyComponent("CourseHome");
const MyCourses = lazyComponent("MyCourses");
const CourseDetail = lazyComponent("CourseDetail");
const CoursePurchase = lazyComponent("CoursePurchase");
const CoursePlayer = lazyComponent("CoursePlayer");
const CourseLiveRoom = lazyComponent("CourseLiveRoom");
const CourseSearch = lazyComponent("CourseSearch");

export const courseModule: ImH5CapabilityModule = {
  id: "course",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.courseHome, render: () => <CourseHome /> },
    { ...IM_H5_ROUTE_DEFINITIONS.courseMy, render: () => <MyCourses /> },
    { ...IM_H5_ROUTE_DEFINITIONS.courseDetail, render: () => <CourseDetail /> },
    { ...IM_H5_ROUTE_DEFINITIONS.coursePurchase, render: () => <CoursePurchase /> },
    { ...IM_H5_ROUTE_DEFINITIONS.coursePlayer, render: () => <CoursePlayer /> },
    { ...IM_H5_ROUTE_DEFINITIONS.courseLive, render: () => <CourseLiveRoom /> },
    { ...IM_H5_ROUTE_DEFINITIONS.courseSearch, render: () => <CourseSearch /> },
  ],
};
