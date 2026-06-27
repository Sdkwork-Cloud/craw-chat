import React, { useState, useEffect } from "react";
import { motion } from "motion/react";
import {
  Users,
  Search,
  Plus,
  Compass,
  Star,
  TrendingUp,
  ChevronLeft,
  Image as ImageIcon,
  Link2,
  FileText,
  Heart,
  MessageCircle,
  Share2,
  Download,
  MoreHorizontal,
  Send,
  Trash2,
  Edit2,
  QrCode,
  MessageSquare,
  Video,
  FileBox,
  Filter,
  Code,
  Package,
  Upload,
  Flame,
  Clock,
  Eye,
  GitFork,
  ExternalLink,
  Folder,
  FolderOpen,
  ChevronRight,
  ChevronDown,
  ThumbsUp,
  X,
} from "lucide-react";
import { toast } from "@sdkwork/im-pc-chat";
import { Avatar } from "@sdkwork/im-pc-commons";
import { readAppSdkSessionTokens } from "@sdkwork/im-pc-core";
import {
  communityService,
  Community,
  Post,
  ResourceItem,
  ChatGroup,
  GroupQRCode,
  PlatformType,
  PC_COMMUNITY_COMMENTS_UNAVAILABLE,
  PC_COMMUNITY_MEDIA_UNAVAILABLE,
} from "../services/CommunityService";

import { CommunitySettings } from "./CommunitySettings";

interface CommunityViewProps {
  initialCommunityId?: string;
  onInitialCommunityHandled?: () => void;
}

export const CommunityView = ({
  initialCommunityId,
  onInitialCommunityHandled,
}: CommunityViewProps = {}) => {
  const [activeCommunity, setActiveCommunity] = useState<Community | null>(
    null,
  );

  useEffect(() => {
    if (!initialCommunityId) {
      return;
    }

    let isMounted = true;
    const openInitialCommunity = async () => {
      try {
        const community = await communityService.getCommunity(initialCommunityId);
        if (isMounted && community) {
          setActiveCommunity(community);
        }
      } finally {
        if (isMounted) {
          onInitialCommunityHandled?.();
        }
      }
    };

    void openInitialCommunity();
    return () => {
      isMounted = false;
    };
  }, [initialCommunityId, onInitialCommunityHandled]);

  if (activeCommunity) {
    return (
      <CommunityDetail
        community={activeCommunity}
        onBack={() => setActiveCommunity(null)}
        onUpdate={(c) => {
          setActiveCommunity(c);
        }}
      />
    );
  }

  return <CommunityHome onSelectCommunity={setActiveCommunity} />;
};

const CommunityHome = ({
  onSelectCommunity,
}: {
  onSelectCommunity: (c: Community) => void;
}) => {
  const [communities, setCommunities] = useState<Community[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");

  const loadCommunities = async () => {
    const data = await communityService.getCommunities();
    setCommunities(data);
    setLoading(false);
  };

  useEffect(() => {
    loadCommunities();
  }, []);

  const filteredCommunities = communities.filter(
    (c) =>
      c.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      c.description.toLowerCase().includes(searchTerm.toLowerCase()),
  );

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e20] text-gray-200 overflow-y-auto custom-scrollbar h-full">
      <div className="sticky top-0 z-10 bg-[#1e1e20]/80 backdrop-blur-md border-b border-white/5 p-6 flex items-center justify-between">
        <h1 className="text-2xl font-bold flex items-center gap-3">
          <Users className="text-indigo-400" />
          全员社群圈子
        </h1>
        <div className="flex items-center gap-4">
          <div className="relative">
            <Search
              className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
              size={16}
            />
            <input
              type="text"
              placeholder="搜索感兴趣的圈子..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="bg-[#2b2b2d] border border-white/5 rounded-full pl-10 pr-4 py-2 text-sm w-64 focus:border-indigo-500 outline-none transition-colors"
            />
          </div>
          <button className="bg-indigo-600 hover:bg-indigo-500 text-white rounded-full px-4 py-2 text-sm font-medium flex items-center gap-2 transition-colors">
            <Plus size={16} />
            创建圈子
          </button>
        </div>
      </div>

      <div className="p-8 max-w-7xl mx-auto w-full">
        {/* Categories / Nav */}
        <div className="flex gap-4 mb-8 overflow-x-auto custom-scrollbar pb-2">
          <button className="bg-white/10 hover:bg-white/15 px-5 py-2.5 rounded-full text-sm font-medium flex items-center gap-2 whitespace-nowrap transition-colors border border-white/5">
            <Compass size={16} className="text-blue-400" /> 发现圈子
          </button>
          <button className="bg-[#2b2b2d] hover:bg-white/5 px-5 py-2.5 rounded-full text-sm font-medium flex items-center gap-2 whitespace-nowrap transition-colors border border-transparent">
            <Star size={16} className="text-yellow-400" /> 我加入的
          </button>
          <button className="bg-[#2b2b2d] hover:bg-white/5 px-5 py-2.5 rounded-full text-sm font-medium flex items-center gap-2 whitespace-nowrap transition-colors border border-transparent">
            <TrendingUp size={16} className="text-green-400" /> 热门推荐
          </button>
        </div>

        {loading ? (
          <div className="flex justify-center p-20">
            <div className="w-8 h-8 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {filteredCommunities.map((community) => (
              <motion.div
                whileHover={{ y: -4 }}
                key={community.id}
                onClick={() => onSelectCommunity(community)}
                className="bg-[#2b2b2d] rounded-2xl overflow-hidden border border-white/5 hover:border-indigo-500/30 transition-colors cursor-pointer group flex flex-col"
              >
                <div className="h-32 relative">
                  <img
                    src={community.cover}
                    alt="Cover"
                    className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                  />
                  <div className="absolute inset-0 bg-gradient-to-t from-[#2b2b2d] to-transparent"></div>
                  <div className="absolute -bottom-6 left-4 p-1 bg-[#2b2b2d] rounded-full">
                    <img
                      src={community.avatar}
                      alt={community.name}
                      className="w-16 h-16 rounded-full border-2 border-[#2b2b2d] object-cover"
                    />
                  </div>
                </div>
                <div className="pt-8 p-5 flex-1 flex flex-col">
                  <h3 className="font-bold text-lg text-gray-100 mb-2 truncate group-hover:text-indigo-400 transition-colors">
                    {community.name}
                  </h3>
                  <p className="text-sm text-gray-400 line-clamp-2 mb-4 flex-1">
                    {community.description}
                  </p>

                  <div className="flex items-center justify-between text-xs text-gray-500 pt-4 border-t border-white/5">
                    <span className="flex items-center gap-1">
                      <Users size={14} /> {community.membersCount} 成员
                    </span>
                    <div className="flex gap-1.5">
                      {community.tags.slice(0, 2).map((tag) => (
                        <span
                          key={tag}
                          className="bg-white/5 px-2 py-0.5 rounded-md"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>
              </motion.div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

const CommunityDetail = ({
  community,
  onBack,
  onUpdate,
}: {
  community: Community;
  onBack: () => void;
  onUpdate: (community: Community) => void;
}) => {
  const sessionUser = readAppSdkSessionTokens()?.user;
  const currentUserName = sessionUser?.displayName ?? sessionUser?.nickname ?? sessionUser?.name ?? "Me";
  const currentUserAvatar = sessionUser?.avatar ?? "";
  const [activeTab, setActiveTab] = useState<string>("feeds");
  const [showSettings, setShowSettings] = useState(false);
  const [activeSoftwareCategory, setActiveSoftwareCategory] = useState("全部分类");
  const [repoSearch, setRepoSearch] = useState("");
  const [posts, setPosts] = useState<Post[]>([]);
  const [resources, setResources] = useState<ResourceItem[]>([]);
  const [groups, setGroups] = useState<ChatGroup[]>([]);
  const [newsList, setNewsList] = useState<any[]>([]);
  const [docsOutline, setDocsOutline] = useState<any[]>([]);
  const [reposList, setReposList] = useState<any[]>([]);
  const [softwareList, setSoftwareList] = useState<any[]>([]);

  const [selectedGroup, setSelectedGroup] = useState<ChatGroup | null>(null);
  const [editingGroup, setEditingGroup] = useState<Partial<ChatGroup> | null>(
    null,
  );
  const [isGroupFormOpen, setIsGroupFormOpen] = useState(false);
  const [resourceFilter, setResourceFilter] = useState<
    "all" | "docs" | "images" | "videos" | "others"
  >("all");
  const [resourceSearch, setResourceSearch] = useState("");
  const [resourcePage, setResourcePage] = useState(1);
  const [isUploadModalOpen, setIsUploadModalOpen] = useState(false);
  const [uploadForm, setUploadForm] = useState({ name: "", type: "PDF" });
  const [selectedNews, setSelectedNews] = useState<any>(null);
  const [activeDocId, setActiveDocId] = useState<string>("doc_1");
  const [expandedDocFolders, setExpandedDocFolders] = useState<
    Record<string, boolean>
  >({ folder_1: true });
  const [newPostContent, setNewPostContent] = useState("");
  const [newPostImages, setNewPostImages] = useState<string[]>([]);
  const [isPosting, setIsPosting] = useState(false);
  const [likedPosts, setLikedPosts] = useState<Record<string, boolean>>({});
  const [expandedComments, setExpandedComments] = useState<Record<string, boolean>>({});
  const [commentInputs, setCommentInputs] = useState<Record<string, string>>({});
  const [previewImage, setPreviewImage] = useState<string | null>(null);
  const [openPostDropdown, setOpenPostDropdown] = useState<string | null>(null);

  useEffect(() => {
    communityService.getPosts(community.id).then(setPosts);
    communityService.getResources(community.id).then(setResources);
    communityService.getGroups(community.id).then(setGroups);
    communityService.getNews(community.id).then(setNewsList);
    communityService.getDocsOutline(community.id).then(setDocsOutline);
    communityService.getRepos(community.id).then(setReposList);
    communityService.getSoftware(community.id).then(setSoftwareList);
  }, [community.id]);

  const handlePost = async () => {
    if (!newPostContent.trim() && newPostImages.length === 0) return;
    setIsPosting(true);
    try {
      const newPost = await communityService.createPost(
        community.id,
        newPostContent,
        newPostImages.length > 0 ? newPostImages : undefined
      );
      setPosts([newPost, ...posts]);
      setNewPostContent("");
      setNewPostImages([]);
      toast("发布成功", "success");
    } catch {
      toast("发布失败", "error");
    } finally {
      setIsPosting(false);
    }
  };

  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files) {
      if (newPostImages.length + files.length > 9) {
        toast("最多只能上传 9 张图片", "error");
        return;
      }
      const newImages = Array.from<File>(files).map(file => URL.createObjectURL(file));
      setNewPostImages(prev => [...prev, ...newImages].slice(0, 9));
    }
  };

  const removeImage = (index: number) => {
    setNewPostImages(prev => prev.filter((_, i) => i !== index));
  };

  const handleLikePost = async (postId: string) => {
    setLikedPosts((prev) => ({ ...prev, [postId]: !prev[postId] }));
    setPosts((prev) =>
      prev.map((p) => {
        if (p.id === postId) {
          return {
            ...p,
            likes: likedPosts[postId] ? p.likes - 1 : p.likes + 1,
          };
        }
        return p;
      }),
    );
    await communityService.toggleLikePost(postId);
  };

  const handleDeletePost = async (postId: string) => {
    try {
      await communityService.deletePost(community.id, postId);
      setPosts((prev) => prev.filter((p) => p.id !== postId));
      toast("删除成功", "success");
    } catch {
      toast("删除失败", "error");
    } finally {
      setOpenPostDropdown(null);
    }
  };

  const handleUploadResource = async () => {
    if (!uploadForm.name.trim()) {
      toast("请输入资料名称", "error");
      return;
    }
    try {
      const newResource = await communityService.uploadResource(community.id, {
        name: uploadForm.name,
        type: uploadForm.type,
        size: "2.5 MB",
        uploader: "当前用户",
      });
      setResources([newResource, ...resources]);
      setIsUploadModalOpen(false);
      setUploadForm({ name: "", type: "PDF" });
      toast("资料上传成功", "success");
    } catch (e) {
      toast("上传失败", "error");
    }
  };

  const handleSaveGroup = async () => {
    if (!editingGroup || !editingGroup.name || !editingGroup.platform) return;
    try {
      if (editingGroup.id) {
        // update
        await communityService.updateGroup(
          community.id,
          editingGroup.id,
          editingGroup,
        );
        toast("群组更新成功", "success");
      } else {
        // create
        await communityService.createGroup(community.id, editingGroup as any);
        toast("群组创建成功", "success");
      }
      setIsGroupFormOpen(false);
      setEditingGroup(null);
      communityService.getGroups(community.id).then(setGroups);
    } catch {
      toast("保存失败", "error");
    }
  };

  const handleDeleteGroup = async (groupId: string, e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await communityService.deleteGroup(community.id, groupId);
      toast("群组已删除", "success");
      communityService.getGroups(community.id).then(setGroups);
    } catch {
      toast("删除失败", "error");
    }
  };

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e20] h-full overflow-hidden relative text-gray-200">
      <div
        className={`absolute inset-0 ${activeTab === "resources" || activeTab === "docs" ? "overflow-hidden" : "overflow-y-auto custom-scrollbar"} flex flex-col z-0`}
      >
        {/* Persistent top bar if cover header is hidden */}
        {activeTab !== "feeds" && (
          <div className="flex items-center gap-4 p-6 shrink-0 bg-transparent relative z-10">
            <button
              onClick={onBack}
              className="w-10 h-10 bg-white/5 hover:bg-white/10 backdrop-blur-md rounded-full flex items-center justify-center text-white transition-colors border border-white/10"
            >
              <ChevronLeft size={20} />
            </button>
            <div className="flex items-center gap-3">
              <img
                src={community.avatar}
                alt={community.name}
                className="w-8 h-8 rounded-full"
              />
              <h1 className="text-lg font-bold text-white">{community.name}</h1>
            </div>
            <div className="ml-auto flex items-center gap-2">
              <button className="bg-white/10 hover:bg-white/20 text-white px-4 py-1.5 rounded-full text-sm font-medium transition-colors">
                已加入
              </button>
            </div>
          </div>
        )}

        {/* Cover Header */}
        {activeTab === "feeds" && (
          <div className="h-64 relative shrink-0">
            <img
              src={community.cover}
              alt="Cover"
              className="w-full h-full object-cover"
            />
            <div className="absolute inset-0 bg-black/40"></div>
            <div className="absolute inset-0 bg-gradient-to-t from-[#1e1e20] via-[#1e1e20]/60 to-transparent"></div>

            <button
              onClick={onBack}
              className="absolute top-6 left-6 w-10 h-10 bg-black/40 hover:bg-black/60 backdrop-blur-md rounded-full flex items-center justify-center text-white transition-colors"
            >
              <ChevronLeft size={20} />
            </button>

            <div className="absolute bottom-6 left-8 right-8 flex items-end gap-6">
              <img
                src={community.avatar}
                alt={community.name}
                className="w-24 h-24 rounded-lg border-4 border-[#1e1e20] object-cover shadow-2xl"
              />
              <div className="flex-1 mb-1">
                <h1 className="text-3xl font-bold text-white mb-2">
                  {community.name}
                </h1>
                <div className="flex items-center gap-4 text-sm text-gray-300">
                  <span className="flex items-center gap-1.5">
                    <Users size={16} /> {community.membersCount} 成员
                  </span>
                  <span>•</span>
                  <span>{community.description}</span>
                </div>
              </div>
              <button className="bg-white text-black hover:bg-gray-200 px-6 py-2.5 rounded-full font-bold text-sm transition-colors shadow-lg mb-2">
                加入圈子
              </button>
              <button
                onClick={() => setShowSettings(true)}
                className="bg-white/10 hover:bg-white/20 backdrop-blur-md text-white px-6 py-2.5 rounded-full font-bold text-sm transition-colors shadow-lg mb-2 flex items-center gap-2 border border-white/10"
              >
                <Edit2 size={16} /> 管理圈子
              </button>
            </div>
          </div>
        )}

        <div
          className={`px-8 ${activeTab === "feeds" ? "mt-6" : "mt-0"} shrink-0`}
        >
          <div className="flex border-b border-white/10 gap-8">
            <button
              onClick={() => {
                setActiveTab("feeds");
                setSelectedNews(null);
              }}
              className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === "feeds" || !activeTab ? "text-indigo-400" : "text-gray-400 hover:text-gray-200"}`}
            >
              动态
              {(activeTab === "feeds" || !activeTab) && (
                <div className="absolute bottom-[-1px] left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"></div>
              )}
            </button>

            {(!community.tabs || community.tabs.includes("resources")) && (
              <button
                onClick={() => {
                  setActiveTab("resources");
                  setSelectedNews(null);
                }}
                className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === "resources" ? "text-indigo-400" : "text-gray-400 hover:text-gray-200"}`}
              >
                资源共享{" "}
                <span className="ml-1 bg-white/10 text-xs px-1.5 py-0.5 rounded-full text-gray-400">
                  {resources.length}
                </span>
                {activeTab === "resources" && (
                  <div className="absolute bottom-[-1px] left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"></div>
                )}
              </button>
            )}

            {(!community.tabs || community.tabs.includes("groups")) && (
              <button
                onClick={() => {
                  setActiveTab("groups");
                  setSelectedNews(null);
                }}
                className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === "groups" ? "text-indigo-400" : "text-gray-400 hover:text-gray-200"}`}
              >
                聊天群组{" "}
                <span className="ml-1 bg-white/10 text-xs px-1.5 py-0.5 rounded-full text-gray-400">
                  {groups.length}
                </span>
                {activeTab === "groups" && (
                  <div className="absolute bottom-[-1px] left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"></div>
                )}
              </button>
            )}

            {(!community.tabs || community.tabs.includes("news")) && (
              <button
                onClick={() => {
                  setActiveTab("news");
                  setSelectedNews(null);
                }}
                className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === "news" ? "text-indigo-400" : "text-gray-400 hover:text-gray-200"}`}
              >
                新闻资讯
                {activeTab === "news" && (
                  <div className="absolute bottom-[-1px] left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"></div>
                )}
              </button>
            )}

            {(!community.tabs || community.tabs.includes("docs")) && (
              <button
                onClick={() => {
                  setActiveTab("docs");
                  setSelectedNews(null);
                }}
                className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === "docs" ? "text-indigo-400" : "text-gray-400 hover:text-gray-200"}`}
              >
                文档
                {activeTab === "docs" && (
                  <div className="absolute bottom-[-1px] left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"></div>
                )}
              </button>
            )}

            {(!community.tabs || community.tabs.includes("repos")) && (
              <button
                onClick={() => {
                  setActiveTab("repos");
                  setSelectedNews(null);
                }}
                className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === "repos" ? "text-indigo-400" : "text-gray-400 hover:text-gray-200"}`}
              >
                开源仓库
                {activeTab === "repos" && (
                  <div className="absolute bottom-[-1px] left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"></div>
                )}
              </button>
            )}

            {(!community.tabs || community.tabs.includes("software")) && (
              <button
                onClick={() => {
                  setActiveTab("software");
                  setSelectedNews(null);
                }}
                className={`pb-3 text-sm font-medium transition-colors relative ${activeTab === "software" ? "text-indigo-400" : "text-gray-400 hover:text-gray-200"}`}
              >
                软件推荐
                {activeTab === "software" && (
                  <div className="absolute bottom-[-1px] left-0 right-0 h-0.5 bg-indigo-500 rounded-t-full"></div>
                )}
              </button>
            )}
          </div>
        </div>

        <div
          className={`flex-1 flex flex-col w-full ${
            activeTab === "resources" || activeTab === "docs"
              ? ""
              : activeTab === "news" ||
                  activeTab === "repos" ||
                  activeTab === "software"
                ? "max-w-6xl mx-auto p-8 border-x border-white/5 bg-[#1e1e20]" // Added border-x and bg for max width constraints
                : "max-w-4xl mx-auto p-8"
          }`}
        >
          {activeTab === "feeds" && (
            <div className="space-y-6">
              {/* Create Post */}
              <div className="bg-[#2b2b2d] rounded-2xl p-5 border border-white/5">
                <div className="flex gap-4">
                  <Avatar
                    src={currentUserAvatar || undefined}
                    alt={currentUserName}
                    fallback={currentUserName.charAt(0) || "?"}
                    size="md"
                    shape="circle"
                  />
                  <div className="flex-1 flex flex-col gap-3">
                    <textarea
                      placeholder="分享你的想法、问题或经验..."
                      className="w-full bg-transparent border-none outline-none resize-none text-gray-200 text-sm placeholder-gray-500 min-h-[60px]"
                      value={newPostContent}
                      onChange={(e) => setNewPostContent(e.target.value)}
                    />
                    
                    {/* Image Previews */}
                    {newPostImages.length > 0 && (
                      <div className="grid grid-cols-3 gap-2 mt-2">
                        {newPostImages.map((img, idx) => (
                          <div key={idx} className="relative group aspect-square rounded-xl overflow-hidden border border-white/10">
                            <img src={img} alt={`upload-${idx}`} className="w-full h-full object-cover" />
                            <button
                              onClick={() => removeImage(idx)}
                              className="absolute top-1.5 right-1.5 bg-black/60 p-1.5 rounded-full text-white opacity-0 group-hover:opacity-100 transition-opacity hover:bg-black/80"
                            >
                              <X size={14} />
                            </button>
                          </div>
                        ))}
                      </div>
                    )}
                    
                    <div className="flex justify-between items-center pt-3 border-t border-white/5">
                      <div className="flex gap-2">
                        <label className="text-gray-400 hover:text-indigo-400 p-2 hover:bg-white/5 rounded-xl transition-colors cursor-pointer">
                          <ImageIcon size={18} />
                          <input 
                            type="file" 
                            multiple 
                            accept="image/*" 
                            className="hidden" 
                            onChange={handleImageUpload} 
                            disabled={newPostImages.length >= 9} 
                          />
                        </label>
                        <button className="text-gray-400 hover:text-indigo-400 p-2 hover:bg-white/5 rounded-xl transition-colors">
                          <Link2 size={18} />
                        </button>
                      </div>
                      <button
                        onClick={handlePost}
                        disabled={(!newPostContent.trim() && newPostImages.length === 0) || isPosting}
                        className="bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 disabled:hover:bg-indigo-600 text-white px-5 py-1.5 rounded-full text-sm font-medium transition-colors flex items-center gap-2"
                      >
                        {isPosting ? (
                          <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                        ) : (
                          <Send size={14} />
                        )}
                        发布
                      </button>
                    </div>
                  </div>
                </div>
              </div>

              {/* Feeds List */}
              {posts.map((post) => (
                <div
                  key={post.id}
                  className="bg-[#2b2b2d] rounded-2xl p-5 border border-white/5"
                >
                  <div className="flex justify-between items-start mb-4">
                    <div className="flex items-center gap-3">
                      <Avatar
                        src={post.author.avatar || undefined}
                        alt={post.author.name}
                        fallback={post.author.name?.charAt(0) || "?"}
                        size="md"
                        shape="circle"
                      />
                      <div>
                        <div className="font-bold text-sm text-gray-200">
                          {post.author.name}
                        </div>
                        <div className="text-xs text-gray-500">
                          {post.createdAt}
                        </div>
                      </div>
                    </div>
                    <div className="relative">
                      <button 
                        onClick={() => setOpenPostDropdown(openPostDropdown === post.id ? null : post.id)}
                        className="text-gray-500 hover:text-gray-300 p-1 rounded-full hover:bg-white/5 transition-colors"
                      >
                        <MoreHorizontal size={18} />
                      </button>
                      
                      {openPostDropdown === post.id && (
                        <div className="absolute right-0 mt-1 w-32 bg-[#252528] border border-white/5 rounded-xl shadow-2xl py-1 z-10 animate-in zoom-in-95">
                          {post.author.name === "我" || post.author.name === "Me" ? (
                            <button 
                              onClick={() => handleDeletePost(post.id)}
                              className="w-full text-left px-4 py-2 text-sm text-red-400 hover:bg-white/5 transition-colors flex items-center gap-2"
                            >
                              <Trash2 size={14} /> 删除动态
                            </button>
                          ) : (
                            <button className="w-full text-left px-4 py-2 text-sm text-gray-300 hover:bg-white/5 transition-colors flex items-center gap-2">
                              隐蔽此动态
                            </button>
                          )}
                        </div>
                      )}
                    </div>
                  </div>
                  <div className="text-sm text-gray-300 mb-4 whitespace-pre-wrap leading-relaxed">
                    {post.content}
                  </div>
                  {post.images && post.images.length > 0 && (
                    <div className={`mb-4 grid gap-2 ${post.images.length === 1 ? 'grid-cols-1' : post.images.length === 2 ? 'grid-cols-2' : 'grid-cols-3'}`}>
                      {post.images.map((img, i) => (
                        <div key={i} className={`rounded-xl overflow-hidden border border-white/5 ${post.images!.length === 1 ? 'max-w-md max-h-96' : 'aspect-square'}`}>
                          <img
                            src={img}
                            alt="Post"
                            onClick={() => setPreviewImage(img)}
                            className="w-full h-full object-cover cursor-pointer hover:opacity-90 transition-opacity"
                          />
                        </div>
                      ))}
                    </div>
                  )}
                  <div className="flex items-center gap-6 text-gray-500 pt-3 border-t border-white/5">
                    <button 
                      onClick={() => handleLikePost(post.id)}
                      className={`flex items-center gap-2 text-sm transition-colors group ${likedPosts[post.id] ? 'text-pink-500' : 'hover:text-pink-500'}`}
                    >
                      <Heart size={18} className={likedPosts[post.id] ? 'fill-pink-500' : 'group-hover:fill-pink-500'} />{" "}
                      {post.likes}
                    </button>
                    <button 
                      onClick={() => setExpandedComments(prev => ({...prev, [post.id]: !prev[post.id]}))}
                      className="flex items-center gap-2 text-sm hover:text-indigo-400 transition-colors"
                    >
                      <MessageCircle size={18} /> {post.comments}
                    </button>
                    <button className="flex items-center gap-2 text-sm hover:text-green-400 transition-colors ml-auto">
                      <Share2 size={18} /> 分享
                    </button>
                  </div>
                  {expandedComments[post.id] && (
                    <div className="mt-4 pt-4 border-t border-white/5 animate-in fade-in slide-in-from-top-2 duration-200">
                       <div className="flex gap-3 mb-4">
                         <Avatar
                           src={currentUserAvatar || undefined}
                           alt={currentUserName}
                           fallback={currentUserName.charAt(0) || "?"}
                           size="sm"
                           shape="circle"
                         />
                         <div className="flex-1 flex gap-2">
                            <input 
                              type="text" 
                              placeholder="发表你的评论..." 
                              className="flex-1 bg-[#1e1e20] border border-white/10 rounded-full px-4 py-1.5 text-sm outline-none focus:border-indigo-500 transition-colors text-gray-200"
                              value={commentInputs[post.id] || ""}
                              onChange={e => setCommentInputs(prev => ({...prev, [post.id]: e.target.value}))}
                              onKeyDown={async (e) => {
                                if (e.key === 'Enter' && commentInputs[post.id]?.trim()) {
                                  try {
                                    await communityService.createComment(community.id, post.id, commentInputs[post.id].trim());
                                  } catch (error) {
                                    toast(error instanceof Error ? error.message : PC_COMMUNITY_COMMENTS_UNAVAILABLE, "error");
                                  }
                                }
                              }}
                            />
                            <button 
                              disabled={!commentInputs[post.id]?.trim()}
                              onClick={async () => {
                                if (!commentInputs[post.id]?.trim()) return;
                                try {
                                  await communityService.createComment(community.id, post.id, commentInputs[post.id].trim());
                                } catch (error) {
                                  toast(error instanceof Error ? error.message : PC_COMMUNITY_COMMENTS_UNAVAILABLE, "error");
                                }
                              }}
                              className="bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white p-2 rounded-full transition-colors shrink-0"
                            >
                              <Send size={16} className="-ml-0.5 mt-0.5" />
                            </button>
                         </div>
                       </div>
                       
                       <div className="space-y-4">
                         <div className="text-center text-sm text-gray-500 py-4">
                           {post.comments > 0
                             ? `已有 ${post.comments} 条评论，评论详情合约暂未开放`
                             : "暂无评论，快来抢沙发吧~"}
                         </div>
                       </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}

          {activeTab === "resources" &&
            (() => {
              const filteredResources = resources.filter((res) => {
                if (
                  resourceSearch &&
                  !res.name.toLowerCase().includes(resourceSearch.toLowerCase())
                ) {
                  return false;
                }
                if (resourceFilter === "docs") {
                  return (
                    res.type === "PDF" ||
                    res.type === "Markdown" ||
                    res.type === "Word" ||
                    !!res.name.match(/\.(pdf|md|doc|docx|txt)$/i)
                  );
                } else if (resourceFilter === "images") {
                  return (
                    res.type === "Image" ||
                    !!res.name.match(/\.(png|jpg|jpeg|gif|webp)$/i)
                  );
                } else if (resourceFilter === "videos") {
                  return (
                    res.type === "Video" ||
                    !!res.name.match(/\.(mp4|mov|avi)$/i)
                  );
                } else if (resourceFilter === "others") {
                  return (
                    !res.name.match(
                      /\.(pdf|md|doc|docx|txt|png|jpg|jpeg|gif|webp|mp4|mov|avi)$/i,
                    ) &&
                    res.type !== "PDF" &&
                    res.type !== "Image" &&
                    res.type !== "Video" &&
                    res.type !== "Markdown" &&
                    res.type !== "Word"
                  );
                }
                return true;
              });

              const ITEMS_PER_PAGE = 10;
              const totalPages =
                Math.ceil(filteredResources.length / ITEMS_PER_PAGE) || 1;
              const paginatedResources = filteredResources.slice(
                (resourcePage - 1) * ITEMS_PER_PAGE,
                resourcePage * ITEMS_PER_PAGE,
              );

              return (
                <div className="flex bg-[#1e1e20] flex-1 overflow-hidden border-t border-white/5">
                  {/* Left Sidebar Layout */}
                  <div className="w-64 bg-[#252528] border-r border-white/5 shrink-0 p-4 flex flex-col gap-1.5 overflow-y-auto custom-scrollbar">
                    <div className="mb-4 text-xs font-bold text-gray-500 tracking-wider uppercase px-2">
                      资源分类
                    </div>

                    <button
                      onClick={() => {
                        setResourceFilter("all");
                        setResourcePage(1);
                      }}
                      className={`px-3 py-2.5 rounded-xl text-sm font-medium transition-colors flex items-center justify-between ${resourceFilter === "all" ? "bg-indigo-500/10 text-indigo-400" : "text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
                    >
                      <span className="flex items-center gap-2.5">
                        <FileBox size={16} /> 全部文件
                      </span>
                    </button>

                    <button
                      onClick={() => {
                        setResourceFilter("docs");
                        setResourcePage(1);
                      }}
                      className={`px-3 py-2.5 rounded-xl text-sm font-medium transition-colors flex items-center justify-between ${resourceFilter === "docs" ? "bg-indigo-500/10 text-indigo-400" : "text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
                    >
                      <span className="flex items-center gap-2.5">
                        <FileText size={16} /> 文档
                      </span>
                    </button>

                    <button
                      onClick={() => {
                        setResourceFilter("images");
                        setResourcePage(1);
                      }}
                      className={`px-3 py-2.5 rounded-xl text-sm font-medium transition-colors flex items-center justify-between ${resourceFilter === "images" ? "bg-indigo-500/10 text-indigo-400" : "text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
                    >
                      <span className="flex items-center gap-2.5">
                        <ImageIcon size={16} /> 图片
                      </span>
                    </button>

                    <button
                      onClick={() => {
                        setResourceFilter("videos");
                        setResourcePage(1);
                      }}
                      className={`px-3 py-2.5 rounded-xl text-sm font-medium transition-colors flex items-center justify-between ${resourceFilter === "videos" ? "bg-indigo-500/10 text-indigo-400" : "text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
                    >
                      <span className="flex items-center gap-2.5">
                        <Video size={16} /> 视频
                      </span>
                    </button>

                    <button
                      onClick={() => {
                        setResourceFilter("others");
                        setResourcePage(1);
                      }}
                      className={`px-3 py-2.5 rounded-xl text-sm font-medium transition-colors flex items-center justify-between ${resourceFilter === "others" ? "bg-indigo-500/10 text-indigo-400" : "text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
                    >
                      <span className="flex items-center gap-2.5">
                        <Filter size={16} /> 其他
                      </span>
                    </button>
                  </div>

                  {/* Main Content Area */}
                  <div className="flex-1 flex flex-col bg-[#1e1e20] h-full">
                    {/* Header: Search & Upload */}
                    <div className="p-4 border-b border-white/5 flex items-center justify-between gap-4 bg-[#252528]/50">
                      <div className="relative w-full max-w-sm">
                        <Search
                          className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                          size={14}
                        />
                        <input
                          type="text"
                          placeholder="在当前分类中搜索文件..."
                          value={resourceSearch}
                          onChange={(e) => {
                            setResourceSearch(e.target.value);
                            setResourcePage(1);
                          }}
                          className="w-full bg-[#1e1e20] border border-white/10 rounded-full pl-9 pr-4 py-2 text-sm text-gray-200 focus:border-indigo-500 outline-none transition-colors"
                        />
                      </div>
                      <button
                        onClick={() => setIsUploadModalOpen(true)}
                        className="bg-indigo-600 hover:bg-indigo-500 text-white px-5 py-2 rounded-full text-sm font-bold transition-all flex items-center gap-2 shadow-md hover:shadow-lg shrink-0"
                      >
                        <Plus size={16} /> 上传资料
                      </button>
                    </div>

                    {/* List */}
                    <div className="flex-1 overflow-y-auto custom-scrollbar">
                      <div className="divide-y divide-white/5">
                        {paginatedResources.map((res) => (
                          <div
                            key={res.id}
                            className="p-4 flex items-center justify-between hover:bg-white/5 transition-colors group cursor-pointer"
                          >
                            <div className="flex items-center gap-4">
                              <div className="w-10 h-10 rounded-xl bg-indigo-500/10 text-indigo-400 flex items-center justify-center shrink-0 border border-indigo-500/20 shadow-sm">
                                {(() => {
                                  const type = res.type?.toLowerCase() || "";
                                  if (
                                    type.includes("pdf") ||
                                    type.includes("doc") ||
                                    type.includes("mark")
                                  )
                                    return <FileText size={20} />;
                                  if (type.includes("image"))
                                    return <ImageIcon size={20} />;
                                  if (type.includes("video"))
                                    return <Video size={20} />;
                                  return <FileBox size={20} />;
                                })()}
                              </div>
                              <div>
                                <div className="text-sm font-medium text-gray-200 group-hover:text-indigo-400 transition-colors">
                                  {res.name}
                                </div>
                                <div className="text-xs text-gray-500 mt-1 flex gap-3">
                                  <span className="font-mono">{res.size}</span>
                                  <span>•</span>
                                  <span>
                                    {res.uploader} 上传于 {res.uploadTime}
                                  </span>
                                </div>
                              </div>
                            </div>
                            <div className="flex items-center gap-2">
                              <button
                                onClick={() => toast("开始预览文件", "success")}
                                className="p-2 text-gray-400 hover:text-indigo-400 hover:bg-indigo-500/10 rounded-lg transition-colors opacity-0 group-hover:opacity-100"
                                title="预览"
                              >
                                <FileText size={18} />
                              </button>
                              <button
                                onClick={() => toast("已开始下载 " + res.name, "success")}
                                className="p-2 text-gray-400 hover:text-white hover:bg-white/10 rounded-lg transition-colors opacity-0 group-hover:opacity-100"
                                title="下载"
                              >
                                <Download size={18} />
                              </button>
                              {(res.uploader === "我" || res.uploader === "Me") && (
                                <button
                                  onClick={async () => {
                                    try {
                                      await communityService.deleteResource(community.id, res.id);
                                      setResources(prev => prev.filter(r => r.id !== res.id));
                                      toast("文件已删除", "success");
                                    } catch {
                                      toast("删除失败", "error");
                                    }
                                  }}
                                  className="p-2 text-gray-400 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors opacity-0 group-hover:opacity-100"
                                  title="删除"
                                >
                                  <Trash2 size={18} />
                                </button>
                              )}
                            </div>
                          </div>
                        ))}
                        {filteredResources.length === 0 && (
                          <div className="p-16 text-center text-gray-500 text-sm flex flex-col items-center">
                            <FileBox size={40} className="opacity-20 mb-4" />
                            未找到匹配的文件
                          </div>
                        )}
                      </div>
                    </div>

                    {/* Pagination */}
                    {totalPages > 1 && (
                      <div className="p-4 border-t border-white/5 flex items-center justify-between text-sm text-gray-400 bg-[#252528]/50">
                        <div>共 {filteredResources.length} 个文件</div>
                        <div className="flex items-center gap-2">
                          <button
                            disabled={resourcePage === 1}
                            onClick={() =>
                              setResourcePage((p) => Math.max(1, p - 1))
                            }
                            className="px-3 py-1.5 rounded-lg border border-white/10 hover:bg-white/5 disabled:opacity-30 disabled:hover:bg-transparent transition-colors"
                          >
                            上一页
                          </button>
                          <span className="px-2">
                            {" "}
                            {resourcePage} / {totalPages}{" "}
                          </span>
                          <button
                            disabled={resourcePage === totalPages}
                            onClick={() =>
                              setResourcePage((p) =>
                                Math.min(totalPages, p + 1),
                              )
                            }
                            className="px-3 py-1.5 rounded-lg border border-white/10 hover:bg-white/5 disabled:opacity-30 disabled:hover:bg-transparent transition-colors"
                          >
                            下一页
                          </button>
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              );
            })()}

          {activeTab === "groups" && (
            <div className="space-y-6">
              <div className="flex justify-end mb-4">
                <button
                  className="bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-full text-sm font-medium transition-colors flex items-center gap-2 shadow-sm"
                  onClick={() => {
                    setEditingGroup({
                      name: "",
                      platform: "wechat",
                      memberCount: 0,
                      description: "",
                      qrCodes: [
                        {
                          url: "",
                          description: "扫码加入群聊",
                        },
                      ],
                    });
                    setIsGroupFormOpen(true);
                  }}
                >
                  <Plus size={16} /> 创建/绑定群组
                </button>
              </div>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {groups.map((group) => (
                  <div
                    key={group.id}
                    className="bg-[#2b2b2d] rounded-2xl border border-white/5 p-5 hover:border-indigo-500/30 transition-colors group/group relative overflow-hidden flex flex-col justify-between h-48"
                  >
                    <div className="absolute top-2 right-2 opacity-0 group-hover/group:opacity-100 transition-opacity flex bg-[#1e1e20]/80 rounded-lg backdrop-blur-sm z-10 border border-white/10">
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          setEditingGroup(group);
                          setIsGroupFormOpen(true);
                        }}
                        className="p-1.5 text-gray-400 hover:text-indigo-400 transition-colors"
                        title="编辑"
                      >
                        <Edit2 size={14} />
                      </button>
                      <div className="w-px bg-white/10 my-1"></div>
                      <button
                        onClick={(e) => handleDeleteGroup(group.id, e)}
                        className="p-1.5 text-gray-400 hover:text-red-400 transition-colors"
                        title="删除"
                      >
                        <Trash2 size={14} />
                      </button>
                    </div>
                    <div>
                      <div className="flex justify-between items-start mb-2 pr-16">
                        <h3 className="font-bold text-gray-100 text-base line-clamp-1">
                          {group.name}
                        </h3>
                        <span
                          className={`text-xs px-2 py-1 rounded-md font-medium border shrink-0 ${group.platform === "wechat" ? "text-green-500 bg-green-500/10 border-green-500/20" : group.platform === "qq" ? "text-blue-500 bg-blue-500/10 border-blue-500/20" : group.platform === "dingtalk" ? "text-blue-400 bg-blue-400/10 border-blue-400/20" : group.platform === "feishu" ? "text-cyan-500 bg-cyan-500/10 border-cyan-500/20" : "text-gray-400 bg-gray-500/10 border-gray-500/20"}`}
                        >
                          {group.platform === "wechat"
                            ? "微信群"
                            : group.platform === "qq"
                              ? "QQ群"
                              : group.platform === "dingtalk"
                                ? "钉钉群"
                                : group.platform === "feishu"
                                  ? "飞书群"
                                  : "群组"}
                        </span>
                      </div>
                      <p className="text-sm text-gray-400 line-clamp-2 leading-relaxed">
                        {group.description}
                      </p>
                    </div>

                    <div className="flex items-center justify-between mt-4">
                      <div className="flex items-center gap-1.5 text-xs text-gray-400">
                        <Users size={14} /> {group.memberCount} 人
                      </div>
                      <button
                        onClick={() => setSelectedGroup(group)}
                        className="opacity-0 group-hover/group:opacity-100 bg-white text-black hover:bg-gray-200 px-4 py-1.5 rounded-full text-xs font-bold transition-all shadow-md transform translate-y-2 group-hover/group:translate-y-0"
                      >
                        扫码加入
                      </button>
                    </div>
                  </div>
                ))}
                {groups.length === 0 && (
                  <div className="col-span-2 py-12 text-center text-gray-500 text-sm flex flex-col items-center">
                    <MessageSquare size={32} className="opacity-50 mb-3" />
                    还没有绑定任何群组
                  </div>
                )}
              </div>
            </div>
          )}

          {activeTab === "news" && !selectedNews && (
            <div className="flex gap-6 items-start">
              {/* Main News List */}
              <div className="flex-1 space-y-4">
                {newsList.map((news) => (
                  <div
                    key={news.id}
                    onClick={() => setSelectedNews(news)}
                    className="bg-[#2b2b2d] rounded-2xl border border-white/5 p-5 hover:bg-white/5 transition-colors cursor-pointer group flex gap-6"
                  >
                    <div className="flex-1 mb-1">
                      <h3 className="text-lg font-bold text-gray-100 group-hover:text-indigo-400 transition-colors mb-2 leading-snug">
                        {news.title}
                      </h3>
                      <p className="text-sm text-gray-400 line-clamp-2 mb-4 leading-relaxed">
                        {news.summary}
                      </p>
                      <div className="flex items-center gap-4 text-xs text-gray-500">
                        <span className="font-medium text-gray-300">
                          {news.source}
                        </span>
                        <span>{news.time}</span>
                        <span className="flex items-center gap-1.5 ml-2">
                          <Eye size={14} /> {news.views}
                        </span>
                        <span className="flex items-center gap-1.5">
                          <MessageCircle size={14} /> {news.comments}
                        </span>
                      </div>
                    </div>
                    {news.cover && (
                      <div className="w-32 h-24 shrink-0 overflow-hidden rounded-xl border border-white/5">
                        <img
                          src={news.cover}
                          alt="Cover"
                          className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                        />
                      </div>
                    )}
                  </div>
                ))}
              </div>

              {/* Right Sidebar */}
              <div className="w-80 shrink-0 space-y-6 hidden lg:block">
                {/* 推荐 Recommend */}
                <div className="bg-[#2b2b2d] rounded-2xl border border-white/5 overflow-hidden">
                  <div className="p-4 border-b border-white/5 bg-[#252528] flex items-center gap-2">
                    <Flame size={18} className="text-orange-500" />
                    <h3 className="font-bold text-gray-100">为您推荐</h3>
                  </div>
                  <div className="p-4 space-y-4">
                    {[
                      {
                        id: "r1",
                        title: "英伟达推出新一代 AI 芯片，算力再飙升",
                      },
                      { id: "r2", title: "苹果 WWDC 带来全面 AI 升级" },
                      {
                        id: "r3",
                        title: "如何看待 Devin 程序员 AI 的实际表现？",
                      },
                      { id: "r4", title: "开源大模型性能逼近 GPT-4" },
                    ].map((item, idx) => (
                      <div
                        key={item.id}
                        onClick={() => setSelectedNews({ ...item, summary: "这是自动生成的侧边栏预览数据摘要内容", source: "系统推荐", time: "刚才", views: 99, comments: 0, content: "侧边栏推荐文章正文...", id: item.id })}
                        className="flex gap-3 group cursor-pointer"
                      >
                        <span
                          className={`font-mono font-bold text-base mt-0.5 ${idx < 3 ? "text-orange-500" : "text-gray-600"}`}
                        >
                          {idx + 1}
                        </span>
                        <span className="text-sm text-gray-300 group-hover:text-indigo-400 transition-colors">
                          {item.title}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>

                {/* 最新 Latest */}
                <div className="bg-[#2b2b2d] rounded-2xl border border-white/5 overflow-hidden">
                  <div className="p-4 border-b border-white/5 bg-[#252528] flex items-center gap-2">
                    <Clock size={18} className="text-blue-400" />
                    <h3 className="font-bold text-gray-100">最新资讯</h3>
                  </div>
                  <div className="p-4 space-y-4">
                    {[
                      {
                        time: "10分钟前",
                        title: "微软宣布将 Copilot 深度集成到 Windows 核心",
                      },
                      {
                        time: "半小时前",
                        title: "Vite 6.0 规划曝光，将有哪些重磅更新？",
                      },
                      {
                        time: "1小时前",
                        title: "谷歌发布新一代基础模型，支持超长上下文",
                      },
                      {
                        time: "2小时前",
                        title: "Next.js App Router 最佳实践指南更新",
                      },
                    ].map((item, idx) => (
                      <div 
                        key={idx} 
                        className="group cursor-pointer"
                        onClick={() => setSelectedNews({ ...item, id: `l${idx}`, summary: "这是最新资讯的预览数据摘要。", source: "最新资讯", views: 24, comments: 2, content: "资讯正文内容..." })}
                      >
                        <div className="text-xs text-blue-400 mb-1">
                          {item.time}
                        </div>
                        <div className="text-sm text-gray-300 leading-snug group-hover:text-indigo-400 transition-colors">
                          {item.title}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === "news" && selectedNews && (
            <div className="bg-[#2b2b2d] rounded-2xl border border-white/5 p-8 max-w-4xl mx-auto w-full animate-in fade-in slide-in-from-bottom-4 duration-300">
              <button
                onClick={() => setSelectedNews(null)}
                className="flex items-center gap-2 text-sm text-gray-400 hover:text-gray-200 transition-colors mb-6"
              >
                <ChevronLeft size={16} /> 返回新闻列表
              </button>

              <h1 className="text-3xl font-bold text-gray-100 mb-6 leading-snug">
                {selectedNews.title}
              </h1>

              <div className="flex items-center gap-6 text-sm text-gray-400 mb-8 pb-8 border-b border-white/5">
                <span className="font-medium text-indigo-400">
                  {selectedNews.source}
                </span>
                <span className="flex items-center gap-1.5">
                  <Clock size={14} /> {selectedNews.time}
                </span>
                <span className="flex items-center gap-1.5">
                  <Eye size={14} /> {selectedNews.views}
                </span>
                <span className="flex items-center gap-1.5">
                  <MessageCircle size={14} /> {selectedNews.comments}
                </span>
              </div>

              {selectedNews.cover && (
                <div className="w-full h-80 rounded-2xl overflow-hidden border border-white/5 mb-10">
                  <img
                    src={selectedNews.cover}
                    alt="Cover"
                    className="w-full h-full object-cover"
                  />
                </div>
              )}

              <div className="prose prose-invert max-w-none">
                <p className="text-gray-300 text-lg leading-relaxed whitespace-pre-wrap">
                  {selectedNews.content}
                </p>
              </div>

              <div className="mt-12 pt-8 border-t border-white/5 flex items-center justify-between">
                <button 
                  onClick={() => toast("已点赞此篇资讯", "success")}
                  className="flex items-center gap-2 bg-white/5 hover:bg-white/10 px-6 py-2.5 rounded-full text-sm font-bold text-gray-200 transition-colors"
                >
                  <Heart size={16} /> 点赞支持
                </button>
                <button 
                  onClick={() => toast("分享链接已复制到剪贴板", "success")}
                  className="flex items-center gap-2 text-gray-400 hover:text-gray-200 text-sm font-medium transition-colors"
                >
                  <Share2 size={16} /> 转发分享
                </button>
              </div>

              {/* Comments Section */}
              <div className="mt-12 pt-8 border-t border-white/5">
                <h3 className="text-xl font-bold text-gray-100 mb-6 flex items-center gap-2">
                  全部评论
                </h3>

                <div className="flex gap-4 mb-8">
                  <Avatar
                    src={currentUserAvatar || undefined}
                    alt={currentUserName}
                    fallback={currentUserName.charAt(0) || "?"}
                    size="md"
                    shape="circle"
                  />
                  <div className="flex-1 bg-[#1e1e20] border border-white/10 rounded-2xl p-3 focus-within:border-indigo-500 transition-colors">
                    <textarea
                      placeholder="分享你的见解..."
                      id="newsCommentInput"
                      className="w-full bg-transparent outline-none text-sm text-gray-200 resize-none min-h-[60px]"
                    />
                    <div className="flex justify-between items-center mt-2 pt-2 border-t border-white/5">
                      <div className="flex gap-2 text-gray-500">
                        <button type="button" className="p-1.5 hover:bg-white/5 rounded-lg transition-colors hover:text-gray-300">
                          <ImageIcon size={16} />
                        </button>
                        <button type="button" className="p-1.5 hover:bg-white/5 rounded-lg transition-colors hover:text-gray-300">
                          <Link2 size={16} />
                        </button>
                      </div>
                      <button
                        type="button"
                        onClick={() => {
                          toast(PC_COMMUNITY_COMMENTS_UNAVAILABLE, "error");
                        }}
                        className="bg-indigo-600 hover:bg-indigo-500 text-white px-5 py-1.5 rounded-full text-sm font-bold transition-all disabled:opacity-50"
                      >
                        发布
                      </button>
                    </div>
                  </div>
                </div>

                <div className="py-8 text-center text-sm text-gray-500">
                  资讯评论合约暂未开放
                </div>
              </div>
            </div>
          )}

          {activeTab === "docs" && (
            <div className="flex bg-[#1e1e20] flex-1 overflow-hidden border-t border-white/5">
              {/* Docs Sidebar */}
              <div className="w-64 bg-[#252528] border-r border-white/5 shrink-0 flex flex-col h-full overflow-hidden">
                <div className="p-3 border-b border-white/5 shrink-0 flex flex-col gap-2 bg-[#252528]/50">
                  <div className="flex justify-between items-center px-1">
                    <span className="text-sm font-bold text-gray-200">
                      知识库大纲
                    </span>
                    <button 
                      onClick={() => toast("创建新文档", "success")}
                      className="text-gray-400 hover:text-white p-1 hover:bg-white/10 rounded transition-colors"
                      title="新建文档"
                    >
                      <Plus size={16} />
                    </button>
                  </div>
                  <div className="relative">
                    <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 text-gray-500" size={14} />
                    <input 
                      type="text" 
                      placeholder="搜索文档..." 
                      className="w-full bg-[#1e1e20] border border-white/10 rounded-md pl-8 pr-3 py-1.5 text-xs text-gray-200 focus:border-indigo-500 outline-none transition-colors"
                    />
                  </div>
                </div>
                <div className="flex-1 overflow-y-auto custom-scrollbar p-3 space-y-1">
                    {docsOutline.map((folder) => (
                      <div key={folder.id}>
                        <button
                          onClick={() =>
                            setExpandedDocFolders((prev) => ({
                              ...prev,
                              [folder.id]: !prev[folder.id],
                            }))
                          }
                          className="w-full flex items-center gap-2 px-2 py-1.5 text-sm font-medium text-gray-300 hover:text-gray-100 hover:bg-white/5 rounded-lg transition-colors group"
                        >
                          <ChevronRight
                            size={14}
                            className={`transition-transform ${expandedDocFolders[folder.id] ? "rotate-90 text-gray-400" : "text-gray-600"}`}
                          />
                          {expandedDocFolders[folder.id] ? (
                            <FolderOpen size={16} className="text-indigo-400" />
                          ) : (
                            <Folder size={16} className="text-indigo-400" />
                          )}
                          {folder.title}
                        </button>
                        {expandedDocFolders[folder.id] && folder.children && (
                          <div className="pl-6 mt-1 space-y-1">
                            {folder.children.map((child: any) => (
                              <button
                                key={child.id}
                                onClick={() => setActiveDocId(child.id)}
                                className={`w-full flex items-center gap-2 px-2 py-1.5 text-sm transition-colors rounded-lg ${activeDocId === child.id ? "bg-indigo-500/10 text-indigo-400 font-medium" : "text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
                              >
                                <FileText size={14} />
                                {child.title}
                              </button>
                            ))}
                          </div>
                        )}
                      </div>
                    ))}
                </div>
              </div>

              {/* Docs Main Content */}
              <div className="flex-1 flex flex-col bg-[#1e1e20] overflow-y-auto custom-scrollbar">
                <div className="max-w-4xl w-full mx-auto p-12">
                  {activeDocId === "doc_1" && (
                    <div className="prose prose-invert max-w-none text-gray-300 animate-in fade-in">
                      <div className="text-sm font-mono text-indigo-400 mb-4 flex items-center gap-2">
                        快速入门{" "}
                        <ChevronRight size={12} className="text-gray-600" />{" "}
                        Sdkwork IM 介绍
                      </div>
                      <h1 className="text-4xl font-bold text-gray-100 mb-6 border-b border-white/10 pb-4">
                        Sdkwork IM 社区版介绍
                      </h1>
                      <p className="text-lg leading-relaxed mb-6">
                        Sdkwork IM
                        社区版是一个高性能、可扩展的即时通讯与在线社区协作平台。它不仅提供了基础的聊天引擎，还通过「圈子」概念，将信息流（Feed）、资源共享（Drive）与多人文档等功能模块化，赋能团队开发者快速集成所需功能。
                      </p>
                      <h2 className="text-2xl font-bold text-gray-100 mb-4 mt-8">
                        核心特性
                      </h2>
                      <ul className="list-disc pl-6 space-y-2 mb-8">
                        <li>
                          <strong>极致性能</strong>: 基于 Rust 构建的网络层与
                          WebAssembly 客户端解析，极速处理万人级群聊。
                        </li>
                        <li>
                          <strong>模块化圈子</strong>:
                          自由拼装新闻资讯、开源代码仓库、文档库等应用。
                        </li>
                        <li>
                          <strong>专业的技术栈</strong>: 拥抱 React 19 和 Vite
                          的全栈前沿生态。
                        </li>
                      </ul>
                      <div className="bg-[#2b2b2d] p-4 rounded-xl border border-white/5 mb-6 text-sm font-mono">
                        npm install @sdkwork/im-pc-community
                      </div>
                    </div>
                  )}
                  {activeDocId !== "doc_1" && (
                    <div className="prose prose-invert max-w-none text-gray-300 animate-in fade-in">
                      <div className="text-sm font-mono text-indigo-400 mb-4 flex items-center gap-2">
                        导航{" "}
                        <ChevronRight size={12} className="text-gray-600" />{" "}
                        正在阅读中...
                      </div>
                      <h1 className="text-4xl font-bold text-gray-100 mb-6 border-b border-white/10 pb-4">
                        正文内容加载中
                      </h1>
                      <p className="text-lg leading-relaxed mb-6">
                        该文档内容正在编写完善中，请稍后再来查看。
                      </p>
                    </div>
                  )}
                </div>
              </div>
            </div>
          )}

          {activeTab === "repos" && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300">
              <div className="flex justify-between items-center bg-[#252528] p-6 rounded-2xl border border-white/5 shadow-sm">
                <div>
                  <h3 className="text-xl font-bold mb-1">开源仓库</h3>
                  <p className="text-sm text-gray-400">
                    发现和贡献社区维护的优秀开源项目
                  </p>
                </div>
                <div className="relative">
                  <Search
                    className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                    size={16}
                  />
                  <input
                    type="text"
                    value={repoSearch}
                    onChange={(e) => setRepoSearch(e.target.value)}
                    placeholder="搜索仓库..."
                    className="bg-[#1e1e20] border border-white/10 rounded-full pl-9 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 w-64 transition-colors"
                  />
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {reposList.filter(repo => repo.name.toLowerCase().includes(repoSearch.toLowerCase()) || repo.desc.toLowerCase().includes(repoSearch.toLowerCase())).map((repo, idx) => (
                  <div
                    key={idx}
                    className="bg-[#2b2b2d] rounded-2xl border border-white/5 p-6 hover:bg-white/5 transition-colors group cursor-pointer flex flex-col h-full"
                  >
                    <div className="flex items-start justify-between mb-3">
                      <h4 className="text-lg font-bold text-indigo-400 flex items-center gap-2 group-hover:text-indigo-300 transition-colors">
                        <Folder size={18} /> {repo.name}
                      </h4>
                      <span className="flex items-center gap-1.5 text-xs font-mono bg-[#1e1e20] px-2 py-1 rounded-md text-gray-400 border border-white/5">
                        <div
                          className={`w-2 h-2 rounded-full ${repo.color}`}
                        ></div>
                        {repo.lang}
                      </span>
                    </div>
                    <p className="text-sm text-gray-400 leading-relaxed mb-6 flex-1">
                      {repo.desc}
                    </p>
                    <div className="flex items-center gap-5 text-sm text-gray-500 font-mono">
                      <button 
                        onClick={(e) => { e.stopPropagation(); toast("已添星标", "success"); }}
                        className="flex items-center gap-1.5 hover:text-yellow-400 transition-colors"
                      >
                        <Star size={14} /> {repo.stars}
                      </button>
                      <button 
                        onClick={(e) => { e.stopPropagation(); toast("已克隆仓库", "success"); }}
                        className="flex items-center gap-1.5 hover:text-indigo-400 transition-colors"
                      >
                        <GitFork size={14} /> {repo.forks}
                      </button>
                      <span className="ml-auto text-xs text-gray-600">
                        Updated {repo.updated}
                      </span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {activeTab === "software" && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-300">
              <div className="flex justify-between items-center bg-[#252528] p-6 rounded-2xl border border-white/5 shadow-sm">
                <div>
                  <h3 className="text-xl font-bold mb-1">软件与工具推荐</h3>
                  <p className="text-sm text-gray-400">
                    精选提升生产力的必备开发与设计工具
                  </p>
                </div>
                <div className="flex gap-2">
                  <button 
                    onClick={() => setActiveSoftwareCategory("全部分类")}
                    className={`${activeSoftwareCategory === "全部分类" ? "bg-indigo-500 text-white" : "bg-[#2b2b2d] text-gray-400 hover:text-gray-200 border border-white/5"} px-4 py-2 rounded-lg text-sm font-medium transition-colors`}
                  >
                    全部分类
                  </button>
                  <button 
                    onClick={() => setActiveSoftwareCategory("生产力")}
                    className={`${activeSoftwareCategory === "生产力" ? "bg-indigo-500 text-white" : "bg-[#2b2b2d] text-gray-400 hover:text-gray-200 border border-white/5"} px-4 py-2 rounded-lg text-sm font-medium transition-colors`}
                  >
                    生产力
                  </button>
                  <button 
                    onClick={() => setActiveSoftwareCategory("开发工具")}
                    className={`${activeSoftwareCategory === "开发工具" ? "bg-indigo-500 text-white" : "bg-[#2b2b2d] text-gray-400 hover:text-gray-200 border border-white/5"} px-4 py-2 rounded-lg text-sm font-medium transition-colors`}
                  >
                    开发工具
                  </button>
                  <button 
                    onClick={() => setActiveSoftwareCategory("设计工具")}
                    className={`${activeSoftwareCategory === "设计工具" ? "bg-indigo-500 text-white" : "bg-[#2b2b2d] text-gray-400 hover:text-gray-200 border border-white/5"} px-4 py-2 rounded-lg text-sm font-medium transition-colors`}
                  >
                    设计工具
                  </button>
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {softwareList.filter(s => activeSoftwareCategory === "全部分类" || s.cat === activeSoftwareCategory).map((soft, idx) => (
                  <div
                    key={idx}
                    className="bg-[#2b2b2d] rounded-2xl border border-white/5 p-5 hover:bg-white/5 transition-colors group cursor-pointer flex items-start gap-4 h-full"
                  >
                    <img
                      src={soft.icon}
                      alt={soft.name}
                      className="w-14 h-14 rounded-xl object-cover border border-white/10 shrink-0 shadow-sm"
                    />
                    <div className="flex-1 flex flex-col h-full">
                      <div className="flex justify-between items-start mb-1">
                        <h4 className="font-bold text-gray-100 group-hover:text-indigo-400 transition-colors">
                          {soft.name}
                        </h4>
                        <span className="text-[10px] font-medium bg-white/5 px-2 py-0.5 rounded text-gray-400 border border-white/5">
                          {soft.cat}
                        </span>
                      </div>
                      <p className="text-xs text-gray-400 leading-relaxed mb-3 flex-1">
                        {soft.desc}
                      </p>
                      <button 
                        onClick={() => toast(`已开始下载 ${soft.name}`, "success")}
                        className="mt-auto self-start flex items-center gap-1.5 text-xs font-bold text-indigo-400 hover:text-indigo-300 transition-colors bg-indigo-500/10 hover:bg-indigo-500/20 px-3 py-1.5 rounded-lg"
                      >
                        获取 <ExternalLink size={12} />
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>

      {showSettings && (
        <CommunitySettings
          community={community}
          onClose={() => setShowSettings(false)}
          onUpdate={(c) => {
            setShowSettings(false);
            onUpdate(c);
          }}
        />
      )}

      {/* Upload Modal */}
      {isUploadModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in">
          <div className="bg-[#1e1e20] border border-white/10 rounded-2xl w-full max-w-md shadow-2xl overflow-hidden animate-in zoom-in-95">
            <div className="p-5 border-b border-white/5 bg-[#252528] flex justify-between items-center">
              <h3 className="text-lg font-bold text-gray-100">上传资料</h3>
              <button
                onClick={() => setIsUploadModalOpen(false)}
                className="text-gray-500 hover:text-white transition-colors"
              >
                <Plus size={20} className="rotate-45" />
              </button>
            </div>
            <div className="p-6 space-y-6">
              <div>
                <label className="block text-sm font-medium text-gray-400 mb-2">
                  文件名称
                </label>
                <input
                  type="text"
                  value={uploadForm.name}
                  onChange={(e) =>
                    setUploadForm({ ...uploadForm, name: e.target.value })
                  }
                  placeholder="输入或上传后自动获取名称"
                  className="w-full bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors text-sm"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-400 mb-2">
                  文件类型
                </label>
                <select
                  value={uploadForm.type}
                  onChange={(e) =>
                    setUploadForm({ ...uploadForm, type: e.target.value })
                  }
                  className="w-full bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors text-sm appearance-none"
                >
                  <option value="PDF">PDF 文档</option>
                  <option value="Word">Word 文档</option>
                  <option value="Markdown">Markdown 文档</option>
                  <option value="Image">图片 (PNG/JPG等)</option>
                  <option value="Video">视频文件</option>
                  <option value="Other">其他文件</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-400 mb-2">
                  选择文件
                </label>
                <div className="w-full border-2 border-dashed border-white/10 rounded-xl p-8 hover:bg-white/5 hover:border-indigo-500/50 transition-colors cursor-pointer text-center group">
                  <Upload
                    size={32}
                    className="text-gray-500 group-hover:text-indigo-400 mx-auto mb-3 transition-colors"
                  />
                  <div className="text-sm font-medium text-gray-300">
                    点击或将文件拖曳至此
                  </div>
                  <div className="text-xs text-gray-500 mt-1">
                    支持所有常见格式，最大 100MB
                  </div>
                </div>
              </div>
            </div>
            <div className="p-5 border-t border-white/5 bg-[#252528] flex justify-end gap-3">
              <button
                onClick={() => setIsUploadModalOpen(false)}
                className="px-5 py-2 rounded-full text-sm font-medium text-gray-400 hover:text-white hover:bg-white/5 transition-colors border border-transparent"
              >
                取消
              </button>
              <button
                onClick={handleUploadResource}
                className="bg-indigo-600 hover:bg-indigo-500 text-white px-6 py-2 rounded-full text-sm font-bold transition-all shadow-md flex items-center gap-2"
              >
                <Plus size={16} /> 确认上传
              </button>
            </div>
          </div>
        </div>
      )}

      {/* QR Code Modal */}
      {selectedGroup && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in">
          <div className="bg-[#1e1e20] border border-white/10 rounded-3xl w-full max-w-sm shadow-2xl p-8 flex flex-col items-center relative animate-in zoom-in-95">
            <button
              onClick={() => setSelectedGroup(null)}
              className="absolute top-4 right-4 text-gray-500 hover:text-white p-2 rounded-full hover:bg-white/10 transition-colors"
            >
              <Plus size={20} className="rotate-45" />
            </button>

            <div
              className={`w-12 h-12 rounded-xl flex items-center justify-center mb-4 border ${selectedGroup.platform === "wechat" ? "text-green-500 bg-green-500/10 border-green-500/20" : selectedGroup.platform === "qq" ? "text-blue-500 bg-blue-500/10 border-blue-500/20" : selectedGroup.platform === "dingtalk" ? "text-blue-400 bg-blue-400/10 border-blue-400/20" : selectedGroup.platform === "feishu" ? "text-cyan-500 bg-cyan-500/10 border-cyan-500/20" : "text-gray-400 bg-gray-500/10 border-gray-500/20"}`}
            >
              <MessageSquare size={24} />
            </div>

            <h3 className="text-xl font-bold text-gray-100 mb-1 text-center">
              {selectedGroup.name}
            </h3>
            <p className="text-sm text-gray-400 mb-6 text-center">
              {selectedGroup.description}
            </p>

            <div className="flex gap-4 overflow-x-auto max-w-full pb-2 custom-scrollbar justify-center">
              {selectedGroup.qrCodes?.map((qr, idx) => (
                <div key={idx} className="flex flex-col items-center">
                  <div className="bg-white p-3 rounded-2xl shadow-inner border-[4px] border-white/5 relative group mb-2 shrink-0">
                    <img
                      src={qr.url}
                      alt={qr.description || "QR Code"}
                      className="w-40 h-40 object-cover rounded-xl border border-gray-200"
                    />
                    <div className="absolute inset-0 bg-white/80 backdrop-blur-sm opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center rounded-2xl">
                      <QrCode size={40} className="text-indigo-600" />
                    </div>
                  </div>
                  {qr.description && (
                    <div className="text-xs text-gray-400 text-center max-w-[160px] truncate">
                      {qr.description}
                    </div>
                  )}
                </div>
              ))}
            </div>

            <div className="mt-6 text-xs text-gray-500 text-center flex items-center justify-center gap-2">
              <QrCode size={14} />
              请使用
              {selectedGroup.platform === "wechat"
                ? "微信"
                : selectedGroup.platform === "qq"
                  ? "QQ"
                  : selectedGroup.platform === "dingtalk"
                    ? "钉钉"
                    : selectedGroup.platform === "feishu"
                      ? "飞书"
                      : "对应 APP"}
              扫码加入
            </div>
          </div>
        </div>
      )}

      {/* Group CRUD Modal */}
      {isGroupFormOpen && editingGroup && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 animate-in fade-in">
          <div className="bg-[#1e1e20] border border-white/10 rounded-2xl w-full max-w-md shadow-2xl overflow-hidden animate-in zoom-in-95">
            <div className="p-5 border-b border-white/5 bg-[#252528] flex justify-between items-center">
              <h3 className="text-lg font-bold text-gray-100">
                {editingGroup.id ? "编辑群组" : "创建群组"}
              </h3>
              <button
                onClick={() => {
                  setIsGroupFormOpen(false);
                  setEditingGroup(null);
                }}
                className="text-gray-500 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors"
              >
                <Plus size={20} className="rotate-45" />
              </button>
            </div>

            <div className="p-6 space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-400 mb-1.5">
                  群组名称 *
                </label>
                <input
                  type="text"
                  value={editingGroup.name || ""}
                  onChange={(e) =>
                    setEditingGroup({ ...editingGroup, name: e.target.value })
                  }
                  className="w-full bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors"
                  placeholder="例如：官方技术交流群"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-400 mb-1.5">
                  平台类型 *
                </label>
                <select
                  value={editingGroup.platform || "wechat"}
                  onChange={(e) =>
                    setEditingGroup({
                      ...editingGroup,
                      platform: e.target.value as PlatformType,
                    })
                  }
                  className="w-full bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors appearance-none"
                >
                  <option value="wechat">微信</option>
                  <option value="qq">QQ</option>
                  <option value="feishu">飞书</option>
                  <option value="dingtalk">钉钉</option>
                  <option value="telegram">Telegram</option>
                  <option value="discord">Discord</option>
                  <option value="other">其他</option>
                </select>
              </div>

              <div>
                <div className="flex justify-between items-center mb-1.5">
                  <label className="block text-sm font-medium text-gray-400">
                    二维码图片 URL 列表
                  </label>
                  <button
                    onClick={() => {
                      const newQrCodes = [...(editingGroup.qrCodes || [])];
                      newQrCodes.push({ url: "", description: "" });
                      setEditingGroup({ ...editingGroup, qrCodes: newQrCodes });
                    }}
                    className="text-xs text-indigo-400 hover:text-indigo-300 flex items-center gap-1"
                  >
                    <Plus size={12} /> 添加二维码
                  </button>
                </div>
                <div className="space-y-3">
                  {(editingGroup.qrCodes || []).map((qr, idx) => (
                    <div key={idx} className="flex gap-2">
                      <div className="flex-1 space-y-2">
                        <div className="flex gap-2">
                          <input
                            type="text"
                            value={qr.url || ""}
                            onChange={(e) => {
                              const newQrCodes = [
                                ...(editingGroup.qrCodes || []),
                              ];
                              newQrCodes[idx].url = e.target.value;
                              setEditingGroup({
                                ...editingGroup,
                                qrCodes: newQrCodes,
                              });
                            }}
                            className="flex-1 bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-2 text-gray-200 outline-none focus:border-indigo-500 transition-colors text-sm"
                            placeholder="图片 URL (或点击模拟上传)"
                          />
                          <button
                            type="button"
                            onClick={() => {
                              toast(PC_COMMUNITY_MEDIA_UNAVAILABLE, "error");
                            }}
                            className="bg-[#3b3b3d] hover:bg-[#4b4b4d] text-gray-300 px-3 py-2 rounded-xl text-sm font-medium transition-colors"
                          >
                            上传
                          </button>
                        </div>
                        <input
                          type="text"
                          value={qr.description || ""}
                          onChange={(e) => {
                            const newQrCodes = [
                              ...(editingGroup.qrCodes || []),
                            ];
                            newQrCodes[idx].description = e.target.value;
                            setEditingGroup({
                              ...editingGroup,
                              qrCodes: newQrCodes,
                            });
                          }}
                          className="w-full bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-2 text-gray-200 outline-none focus:border-indigo-500 transition-colors text-sm"
                          placeholder="说明文字 (例如: 微信群二维码)"
                        />
                      </div>
                      {(editingGroup.qrCodes?.length || 0) > 1 && (
                        <button
                          onClick={() => {
                            const newQrCodes = [
                              ...(editingGroup.qrCodes || []),
                            ];
                            newQrCodes.splice(idx, 1);
                            setEditingGroup({
                              ...editingGroup,
                              qrCodes: newQrCodes,
                            });
                          }}
                          className="text-gray-500 hover:text-red-400 transition-colors p-2"
                        >
                          <Trash2 size={16} />
                        </button>
                      )}
                    </div>
                  ))}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-400 mb-1.5">
                  群简介
                </label>
                <textarea
                  value={editingGroup.description || ""}
                  onChange={(e) =>
                    setEditingGroup({
                      ...editingGroup,
                      description: e.target.value,
                    })
                  }
                  className="w-full bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors resize-none min-h-[80px]"
                  placeholder="群组规则或讨论内容简介..."
                ></textarea>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-400 mb-1.5">
                  现有成员数
                </label>
                <input
                  type="number"
                  value={editingGroup.memberCount ?? 0}
                  onChange={(e) =>
                    setEditingGroup({
                      ...editingGroup,
                      memberCount: parseInt(e.target.value) || 0,
                    })
                  }
                  className="w-full bg-[#2b2b2d] border border-white/10 rounded-xl px-4 py-2.5 text-gray-200 outline-none focus:border-indigo-500 transition-colors"
                />
              </div>
            </div>

            <div className="p-5 border-t border-white/5 bg-[#252528] flex justify-end gap-3">
              <button
                onClick={() => {
                  setIsGroupFormOpen(false);
                  setEditingGroup(null);
                }}
                className="px-5 py-2.5 rounded-full text-sm font-medium text-gray-400 hover:text-white hover:bg-white/5 transition-colors border border-transparent"
              >
                取消
              </button>
              <button
                onClick={handleSaveGroup}
                disabled={!editingGroup.name || !editingGroup.platform}
                className="bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 disabled:hover:bg-indigo-600 text-white px-6 py-2.5 rounded-full text-sm font-bold transition-all shadow-md"
              >
                保存群组
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Image Preview Modal */}
      {previewImage && (
        <div 
          className="fixed inset-0 z-[100] flex items-center justify-center bg-black/90 backdrop-blur-sm cursor-zoom-out p-4 animate-in fade-in duration-200"
          onClick={() => setPreviewImage(null)}
        >
          <img 
            src={previewImage} 
            alt="Preview" 
            className="max-w-full max-h-full object-contain cursor-default"
            onClick={(e) => e.stopPropagation()} 
          />
          <button 
            className="absolute top-6 right-6 text-white/50 hover:text-white p-2 bg-white/10 hover:bg-white/20 rounded-full transition-colors cursor-pointer"
            onClick={() => setPreviewImage(null)}
          >
            <X size={24} />
          </button>
        </div>
      )}
    </div>
  );
};
