import React, { useEffect, useState } from "react";
import { Community, communityService, CommunityMember, PC_COMMUNITY_MEMBERS_UNAVAILABLE } from "../services/CommunityService";
import {
  ChevronLeft,
  Save,
  Upload,
  Users,
  X,
  Info,
  Settings2,
  Trash2,
  Search,
} from "lucide-react";
import { toast } from "@sdkwork/im-pc-chat";
import { Avatar } from "@sdkwork/im-pc-commons";

const TAB_OPTIONS = [
  { id: "feeds", name: "动态" },
  { id: "resources", name: "资源共享" },
  { id: "groups", name: "聊天群组" },
  { id: "news", name: "新闻资讯" },
  { id: "docs", name: "文档" },
  { id: "repos", name: "开源仓库" },
  { id: "software", name: "软件推荐" },
];

export const CommunitySettings = ({
  community,
  onClose,
  onUpdate,
}: {
  community: Community;
  onClose: () => void;
  onUpdate: (c: Community) => void;
}) => {
  const [activeTab, setActiveTab] = useState<"info" | "members">("info");
  const [members, setMembers] = useState<CommunityMember[]>([]);
  const [membersLoading, setMembersLoading] = useState(false);
  const [form, setForm] = useState<Partial<Community>>({
    name: community.name,
    description: community.description,
    avatar: community.avatar,
    cover: community.cover,
    tags: community.tags || [],
    tabs: community.tabs || [
      "feeds",
      "resources",
      "groups",
      "news",
      "docs",
      "repos",
      "software",
    ],
  });
  const [isSaving, setIsSaving] = useState(false);
  const [newTag, setNewTag] = useState("");

  useEffect(() => {
    if (activeTab !== "members") {
      return;
    }

    let mounted = true;
    const loadMembers = async () => {
      setMembersLoading(true);
      try {
        const items = await communityService.getMembers(community.id);
        if (mounted) {
          setMembers(items);
        }
      } catch {
        if (mounted) {
          setMembers([]);
        }
      } finally {
        if (mounted) {
          setMembersLoading(false);
        }
      }
    };

    void loadMembers();
    return () => {
      mounted = false;
    };
  }, [activeTab, community.id]);

  const handleSave = async () => {
    setIsSaving(true);
    try {
      const updated = await communityService.updateCommunity(
        community.id,
        form,
      );
      toast("保存成功", "success");
      onUpdate(updated);
    } catch (e) {
      toast("保存失败", "error");
    } finally {
      setIsSaving(false);
    }
  };

  const handleToggleTab = (tabId: string) => {
    const currentTabs = form.tabs || [];
    if (currentTabs.includes(tabId)) {
      setForm({ ...form, tabs: currentTabs.filter((t) => t !== tabId) });
    } else {
      setForm({ ...form, tabs: [...currentTabs, tabId] });
    }
  };

  const handleAddTag = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && newTag.trim()) {
      const currentTags = form.tags || [];
      if (!currentTags.includes(newTag.trim())) {
        setForm({ ...form, tags: [...currentTags, newTag.trim()] });
      }
      setNewTag("");
    }
  };

  const removeTag = (tag: string) => {
    setForm({ ...form, tags: (form.tags || []).filter((t) => t !== tag) });
  };

  return (
    <div className="fixed inset-0 z-[100] bg-[#1e1e20] text-gray-200 overflow-hidden flex flex-col animate-in fade-in slide-in-from-bottom-4 duration-300">
      <div className="flex h-full">
        {/* Sidebar */}
        <div className="w-64 bg-[#252528] border-r border-white/5 flex flex-col">
          <div className="p-6 flex items-center gap-3 border-b border-white/5">
            <button
              onClick={onClose}
              className="w-8 h-8 rounded-full bg-white/5 hover:bg-white/10 flex items-center justify-center transition-colors"
            >
              <ChevronLeft size={18} />
            </button>
            <h2 className="font-bold text-lg">圈子管理</h2>
          </div>
          <div className="p-4 flex flex-col gap-2">
            <button
              onClick={() => setActiveTab("info")}
              className={`flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-colors ${activeTab === "info" ? "bg-indigo-500/10 text-indigo-400" : "hover:bg-white/5 text-gray-400 hover:text-gray-200"}`}
            >
              <Settings2 size={18} /> 基本信息与模块
            </button>
            <button
              onClick={() => setActiveTab("members")}
              className={`flex items-center gap-3 px-4 py-3 rounded-xl text-sm font-medium transition-colors ${activeTab === "members" ? "bg-indigo-500/10 text-indigo-400" : "hover:bg-white/5 text-gray-400 hover:text-gray-200"}`}
            >
              <Users size={18} /> 成员管理
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-10 bg-[#1e1e20]">
          <div className="max-w-3xl mx-auto space-y-8">
            {activeTab === "info" && (
              <>
                <div className="flex justify-between items-center bg-[#252528] p-6 rounded-2xl border border-white/5 shadow-sm sticky top-0 z-10">
                  <div>
                    <h3 className="text-xl font-bold mb-1">基本信息与模块</h3>
                    <p className="text-sm text-gray-400">
                      修改圈子的展示信息、封面图和功能模块
                    </p>
                  </div>
                  <button
                    onClick={handleSave}
                    disabled={isSaving}
                    className="bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white px-6 py-2.5 rounded-full font-bold text-sm transition-all shadow-md flex items-center gap-2"
                  >
                    {isSaving ? (
                      "保存中..."
                    ) : (
                      <>
                        <Save size={16} /> 保存修改
                      </>
                    )}
                  </button>
                </div>

                <div className="bg-[#2b2b2d] rounded-2xl border border-white/5 p-8 space-y-8">
                  {/* Visuals */}
                  <div className="space-y-4">
                    <h4 className="text-base font-bold text-gray-100 flex items-center gap-2">
                      <Upload size={18} className="text-indigo-400" /> 视觉形象
                    </h4>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                      <div>
                        <label className="block text-sm font-medium text-gray-400 mb-2">
                          圈子头像 URL
                        </label>
                        <div className="flex gap-4 items-center">
                          <img
                            src={form.avatar}
                            alt="Avatar Preview"
                            className="w-16 h-16 rounded-xl object-cover border border-white/10"
                          />
                          <input
                            type="text"
                            value={form.avatar}
                            onChange={(e) =>
                              setForm({ ...form, avatar: e.target.value })
                            }
                            className="flex-1 bg-[#252528] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors text-sm"
                          />
                        </div>
                      </div>
                      <div>
                        <label className="block text-sm font-medium text-gray-400 mb-2">
                          背景封面 URL
                        </label>
                        <div className="flex gap-4 items-center">
                          <img
                            src={form.cover}
                            alt="Cover Preview"
                            className="w-24 h-16 rounded-xl object-cover border border-white/10"
                          />
                          <input
                            type="text"
                            value={form.cover}
                            onChange={(e) =>
                              setForm({ ...form, cover: e.target.value })
                            }
                            className="flex-1 bg-[#252528] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors text-sm"
                          />
                        </div>
                      </div>
                    </div>
                  </div>

                  <div className="h-px bg-white/5 w-full"></div>

                  {/* Basic Info */}
                  <div className="space-y-4">
                    <h4 className="text-base font-bold text-gray-100 flex items-center gap-2">
                      <Info size={18} className="text-indigo-400" /> 圈子信息
                    </h4>

                    <div className="space-y-5">
                      <div>
                        <label className="block text-sm font-medium text-gray-400 mb-2">
                          圈子名称
                        </label>
                        <input
                          type="text"
                          value={form.name}
                          onChange={(e) =>
                            setForm({ ...form, name: e.target.value })
                          }
                          className="w-full bg-[#252528] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors text-sm"
                        />
                      </div>

                      <div>
                        <label className="block text-sm font-medium text-gray-400 mb-2">
                          圈子简介
                        </label>
                        <textarea
                          value={form.description}
                          onChange={(e) =>
                            setForm({ ...form, description: e.target.value })
                          }
                          className="w-full bg-[#252528] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors text-sm min-h-[100px] resize-none"
                        />
                      </div>

                      <div>
                        <label className="block text-sm font-medium text-gray-400 mb-2">
                          标签分类 (回车添加)
                        </label>
                        <div className="bg-[#252528] border border-white/10 rounded-xl p-2 flex flex-wrap gap-2 items-center focus-within:border-indigo-500 transition-colors">
                          {form.tags?.map((tag) => (
                            <span
                              key={tag}
                              className="bg-white/10 text-gray-200 text-xs px-2.5 py-1 rounded-md flex items-center gap-1.5"
                            >
                              {tag}
                              <button
                                onClick={() => removeTag(tag)}
                                className="hover:text-red-400"
                              >
                                <X size={12} />
                              </button>
                            </span>
                          ))}
                          <input
                            type="text"
                            value={newTag}
                            onChange={(e) => setNewTag(e.target.value)}
                            onKeyDown={handleAddTag}
                            placeholder="输入标签后按回车"
                            className="flex-1 bg-transparent px-2 py-1 text-sm outline-none text-gray-200 min-w-[150px]"
                          />
                        </div>
                      </div>
                    </div>
                  </div>

                  <div className="h-px bg-white/5 w-full"></div>

                  {/* Tabs configuration */}
                  <div className="space-y-4">
                    <h4 className="text-base font-bold text-gray-100 flex items-center gap-2">
                      <Settings2 size={18} className="text-indigo-400" />{" "}
                      显示模块
                    </h4>
                    <p className="text-sm text-gray-400 mb-4">
                      勾选需要在圈子详情页展示的功能模块
                    </p>

                    <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
                      {TAB_OPTIONS.map((opt) => (
                        <label
                          key={opt.id}
                          className={`flex items-center gap-3 p-4 border rounded-xl cursor-pointer transition-colors ${form.tabs?.includes(opt.id) ? "bg-indigo-500/10 border-indigo-500/40" : "bg-[#252528] border-white/5 hover:border-white/20"}`}
                        >
                          <input
                            type="checkbox"
                            className="hidden"
                            checked={form.tabs?.includes(opt.id) || false}
                            onChange={() => handleToggleTab(opt.id)}
                          />
                          <div
                            className={`w-5 h-5 rounded flex items-center justify-center border transition-colors ${form.tabs?.includes(opt.id) ? "bg-indigo-500 border-indigo-500" : "border-gray-500"}`}
                          >
                            {form.tabs?.includes(opt.id) && (
                              <svg
                                className="w-3.5 h-3.5 text-white"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                              >
                                <path
                                  strokeLinecap="round"
                                  strokeLinejoin="round"
                                  strokeWidth={3}
                                  d="M5 13l4 4L19 7"
                                />
                              </svg>
                            )}
                          </div>
                          <span className="text-sm font-medium text-gray-200">
                            {opt.name}
                          </span>
                        </label>
                      ))}
                    </div>
                  </div>
                </div>
              </>
            )}

            {activeTab === "members" && (
              <div className="bg-[#2b2b2d] rounded-2xl border border-white/5 overflow-hidden">
                <div className="p-6 border-b border-white/5 bg-[#252528] flex justify-between items-center">
                  <div>
                    <h3 className="text-xl font-bold mb-1">成员列表</h3>
                    <p className="text-sm text-gray-400">
                      管理圈子成员 ({community.membersCount} 人)
                    </p>
                  </div>
                  <div className="relative">
                    <Search
                      className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                      size={16}
                    />
                    <input
                      type="text"
                      placeholder="搜索成员..."
                      className="bg-[#1e1e20] border border-white/10 rounded-full pl-9 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 w-64"
                    />
                  </div>
                </div>

                <div className="divide-y divide-white/5">
                  {membersLoading ? (
                    <div className="p-8 text-center text-sm text-gray-500">加载中...</div>
                  ) : members.length === 0 ? (
                    <div className="p-8 text-center text-sm text-gray-500">暂无成员数据，成员管理合约暂未开放</div>
                  ) : (
                  members.map((member) => (
                    <div
                      key={member.id}
                      className="p-4 flex items-center justify-between hover:bg-white/5 transition-colors"
                    >
                      <div className="flex items-center gap-4">
                        <Avatar
                          src={member.avatar || undefined}
                          alt={member.name}
                          fallback={member.name?.charAt(0) || "?"}
                          size="md"
                          shape="circle"
                        />
                        <div>
                          <div className="text-sm font-bold text-gray-200 flex items-center gap-2">
                            {member.name}
                            {member.role === "admin" && (
                              <span className="bg-indigo-500/20 text-indigo-400 text-[10px] px-1.5 py-0.5 rounded font-bold uppercase tracking-wider">
                                Admin
                              </span>
                            )}
                          </div>
                          <div className="text-xs text-gray-500">
                            加入时间: {member.joinedAt}
                          </div>
                        </div>
                      </div>
                      <div>
                        {member.role !== "admin" && (
                          <button
                            type="button"
                            onClick={() => toast(PC_COMMUNITY_MEMBERS_UNAVAILABLE, "error")}
                            className="text-gray-400 hover:text-red-400 p-2 rounded-lg hover:bg-white/5 transition-colors"
                          >
                            <Trash2 size={16} />
                          </button>
                        )}
                      </div>
                    </div>
                  ))
                  )}
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
