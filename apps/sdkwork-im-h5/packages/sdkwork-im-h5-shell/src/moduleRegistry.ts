import type { ImH5CapabilityModule, ImH5ModuleId } from "./contracts";
import {
  DEFAULT_IM_H5_MODULES,
} from "./moduleCatalog";
import {
  requireImH5ShellModule,
  validateImH5ShellModules,
} from "./moduleValidation";
import { approvalModule } from "./modules/approvalModule";
import { agentsModule } from "./modules/agentsModule";
import { attendanceModule } from "./modules/attendanceModule";
import { calendarModule } from "./modules/calendarModule";
import { chatModule } from "./modules/chatModule";
import { communityModule } from "./modules/communityModule";
import { contactsModule } from "./modules/contactsModule";
import { courseModule } from "./modules/courseModule";
import { devicesModule } from "./modules/devicesModule";
import { driveModule } from "./modules/driveModule";
import { enterpriseModule } from "./modules/enterpriseModule";
import { imagegenModule } from "./modules/imagegenModule";
import { knowledgeModule } from "./modules/knowledgeModule";
import { meetingModule } from "./modules/meetingModule";
import { musicModule } from "./modules/musicModule";
import { membershipModule } from "./modules/membershipModule";
import { musicgenModule } from "./modules/musicgenModule";
import { notaryModule } from "./modules/notaryModule";
import { ordersModule } from "./modules/ordersModule";
import { recruitmentModule } from "./modules/recruitmentModule";
import { reportModule } from "./modules/reportModule";
import { shopModule } from "./modules/shopModule";
import { userModule } from "./modules/userModule";
import { videogenModule } from "./modules/videogenModule";
import { voiceModule } from "./modules/voiceModule";
import { writingModule } from "./modules/writingModule";

export * from "./moduleCatalog";
export { validateImH5ShellModules } from "./moduleValidation";

export const BUILTIN_IM_H5_MODULE_REGISTRY: Readonly<Partial<Record<ImH5ModuleId, ImH5CapabilityModule>>> = {
  agents: agentsModule,
  approval: approvalModule,
  attendance: attendanceModule,
  calendar: calendarModule,
  chat: chatModule,
  community: communityModule,
  contacts: contactsModule,
  course: courseModule,
  devices: devicesModule,
  drive: driveModule,
  enterprise: enterpriseModule,
  imagegen: imagegenModule,
  knowledge: knowledgeModule,
  meeting: meetingModule,
  music: musicModule,
  membership: membershipModule,
  musicgen: musicgenModule,
  notary: notaryModule,
  orders: ordersModule,
  recruitment: recruitmentModule,
  report: reportModule,
  shop: shopModule,
  user: userModule,
  videogen: videogenModule,
  voice: voiceModule,
  writing: writingModule,
};

export function resolveImH5ShellModules(
  moduleIds: readonly ImH5ModuleId[] = DEFAULT_IM_H5_MODULES,
  registry: Readonly<Partial<Record<ImH5ModuleId, ImH5CapabilityModule>>> = BUILTIN_IM_H5_MODULE_REGISTRY,
): ImH5CapabilityModule[] {
  const modules = moduleIds.map((moduleId) => requireImH5ShellModule(moduleId, registry));
  validateImH5ShellModules(modules);
  return modules;
}
