import React from "react";
import { Newspaper, BookOpen, GitFork, Package } from "lucide-react";
import { useTranslation } from "react-i18next";
import { PostList } from "../../components/PostList";
import { ResourceList } from "../../components/ResourceList";
import { GroupList } from "../../components/GroupList";
import { Post, Resource, CommunityGroup, Community } from "../../types";

export const CommunityTabsContent = ({
  activeTab,
  posts,
  resources,
  groups,
  community,
  platformNameMap,
  onLike,
  onCommentClick,
}: {
  activeTab: string;
  posts: Post[];
  resources: Resource[];
  groups: CommunityGroup[];
  community: Community;
  platformNameMap: Record<string, string>;
  onLike: (postId: string) => void;
  onCommentClick: (postId: string) => void;
}) => {
  const { t } = useTranslation();

  return (
    <div className="flex-1 overflow-y-auto w-full">
      {activeTab === "feeds" ? (
        <PostList posts={posts} onLike={onLike} onCommentClick={onCommentClick} />
      ) : activeTab === "resources" ? (
        <ResourceList resources={resources} />
      ) : activeTab === "groups" ? (
        <GroupList
          groups={groups}
          communityId={community.id}
          platformNameMap={platformNameMap}
        />
      ) : activeTab === "news" ? (
        <div className="flex flex-col items-center justify-center h-48 text-text-sub gap-3">
          <Newspaper className="w-10 h-10 opacity-30" />
          {t("community.auto_n47c208ec", "暂无新闻资讯")}
        </div>
      ) : activeTab === "docs" ? (
        <div className="flex flex-col items-center justify-center h-48 text-text-sub gap-3">
          <BookOpen className="w-10 h-10 opacity-30" />
          {t("community.auto_302413da", "暂无文档")}
        </div>
      ) : activeTab === "repos" ? (
        <div className="flex flex-col items-center justify-center h-48 text-text-sub gap-3">
          <GitFork className="w-10 h-10 opacity-30" />
          {t("community.auto_n4b67f9b2", "暂无开源仓库")}
        </div>
      ) : activeTab === "software" ? (
        <div className="flex flex-col items-center justify-center h-48 text-text-sub gap-3">
          <Package className="w-10 h-10 opacity-30" />
          {t("community.auto_n35d7ab13", "暂无软件推荐")}
        </div>
      ) : null}
    </div>
  );
};
