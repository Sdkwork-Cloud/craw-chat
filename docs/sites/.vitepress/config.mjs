import { defineConfig } from "vitepress";
import { apiReferenceSidebar } from "./api-reference-sidebar.mjs";

const nav = [
  { text: "Getting Started", link: "/getting-started/index" },
  { text: "Architecture", link: "/architecture/overview" },
  { text: "Features", link: "/features/index" },
  { text: "API Reference", link: "/api-reference/index" },
  { text: "SDK", link: "/sdk/index" },
  { text: "Deployment", link: "/deployment/index" },
  { text: "Reference", link: "/reference/cli-and-scripts" },
];

const sidebar = {
  "/getting-started/": [
    {
      text: "Getting Started",
      items: [
        { text: "Overview", link: "/getting-started/index" },
        { text: "Quick Start", link: "/getting-started/quick-start" },
      ],
    },
  ],
  "/architecture/": [
    {
      text: "Architecture",
      items: [
        { text: "Overview", link: "/architecture/overview" },
        { text: "Runtime Topology", link: "/architecture/runtime-topology" },
        { text: "Module Map", link: "/architecture/module-map" },
        { text: "Storage Management", link: "/architecture/storage-management" },
      ],
    },
  ],
  "/features/": [
    {
      text: "Features",
      items: [
        { text: "Overview", link: "/features/index" },
        { text: "Capability Matrix", link: "/features/capabilities" },
      ],
    },
  ],
  "/api-reference/": apiReferenceSidebar,
  "/sdk/": [
    {
      text: "SDK",
      items: [
        { text: "Overview", link: "/sdk/index" },
        { text: "IM Standard SDK", link: "/sdk/typescript-sdk" },
        { text: "App API SDK", link: "/sdk/app-sdk" },
        { text: "Backend SDK", link: "/sdk/backend-sdk" },
        { text: "RTC SDK", link: "/sdk/rtc-sdk" },
        { text: "TypeScript SDK", link: "/sdk/typescript-sdk" },
        { text: "Flutter SDK", link: "/sdk/flutter-sdk" },
        { text: "Rust SDK", link: "/sdk/rust-sdk" },
        { text: "Java SDK", link: "/sdk/java-sdk" },
        { text: "C# SDK", link: "/sdk/csharp-sdk" },
        { text: "Swift SDK", link: "/sdk/swift-sdk" },
        { text: "Kotlin SDK", link: "/sdk/kotlin-sdk" },
        { text: "Go SDK", link: "/sdk/go-sdk" },
        { text: "Python SDK", link: "/sdk/python-sdk" },
        { text: "Generator Boundary", link: "/sdk/generator-boundary" },
        { text: "Language Support", link: "/sdk/language-support" },
        { text: "TypeScript Quick Start", link: "/sdk/typescript-quick-start" },
        { text: "Flutter Quick Start", link: "/sdk/flutter-quick-start" },
        { text: "Rust Quick Start", link: "/sdk/rust-quick-start" },
        { text: "Auth and Client Init", link: "/sdk/auth-and-client-init" },
        { text: "Module Map", link: "/sdk/module-map" },
        { text: "Generation and Ownership", link: "/sdk/generation-and-ownership" },
        { text: "Realtime Presence", link: "/sdk/modules/session-and-presence" },
        { text: "Realtime", link: "/sdk/modules/realtime" },
        { text: "Conversations", link: "/sdk/modules/conversations" },
        { text: "Rooms", link: "/sdk/modules/rooms" },
        { text: "Messages", link: "/sdk/modules/messages" },
        { text: "Media", link: "/sdk/modules/media" },
        { text: "Streams", link: "/sdk/modules/streams" },
        { text: "RTC", link: "/sdk/modules/rtc" },
        { text: "Realtime Presence Bootstrap", link: "/sdk/examples/session-bootstrap" },
        { text: "Conversation Workflow", link: "/sdk/examples/conversation-workflow" },
        { text: "Message and Media", link: "/sdk/examples/message-and-media" },
        { text: "Stream and RTC", link: "/sdk/examples/stream-and-rtc" },
      ],
    },
  ],
  "/deployment/": [
    {
      text: "Deployment",
      items: [
        { text: "Overview", link: "/deployment/index" },
        { text: "Local Binary", link: "/deployment/local-binary" },
        { text: "Server Lifecycle", link: "/deployment/server-lifecycle" },
        { text: "Docker", link: "/deployment/docker" },
        { text: "Profiles and Environment", link: "/deployment/profiles-and-env" },
        { text: "Runtime Operations", link: "/deployment/runtime-operations" },
      ],
    },
  ],
  "/reference/": [
    {
      text: "Reference",
      items: [
        { text: "CLI and Scripts", link: "/reference/cli-and-scripts" },
        { text: "Admin Storage Contract", link: "/reference/admin-storage-contract" },
        { text: "Runtime Directory", link: "/reference/runtime-directory" },
      ],
    },
  ],
};

export default defineConfig({
  lang: "en-US",
  title: "Sdkwork IM",
  description:
    "Open-source product documentation for Sdkwork IM, aligned to the currently implemented architecture, runtime behavior, APIs, SDKs, and deployment workflows.",
  cleanUrls: true,
  lastUpdated: true,
  head: [
    ["meta", { name: "theme-color", content: "#8f5b34" }],
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:title", content: "Sdkwork IM Docs" }],
  ],
  themeConfig: {
    siteTitle: "Sdkwork IM",
    nav,
    sidebar,
    logo: {
      light: "/logo-light.svg",
      dark: "/logo-dark.svg",
    },
    search: {
      provider: "local",
    },
    outline: {
      level: [2, 3],
      label: "On This Page",
    },
    docFooter: {
      prev: "Previous page",
      next: "Next page",
    },
    lastUpdated: {
      text: "Last updated",
    },
    footer: {
      message:
        "This site documents only behavior that is implemented and verified in the current repository state.",
      copyright: "AGPL-3.0-or-later; commercial use requires a separate commercial license",
    },
  },
});
