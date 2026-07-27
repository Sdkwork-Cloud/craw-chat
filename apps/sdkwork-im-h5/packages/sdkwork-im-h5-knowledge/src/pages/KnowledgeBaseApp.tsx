import React, { useState, useEffect } from "react";
import { PageLayout, ActionSheet, showPrompt, showConfirm, showToast } from "@sdkwork/im-h5-commons";
import { Plus, Database } from "lucide-react";
import { KnowledgeBaseService, KnowledgeBase } from "../services/KnowledgeBaseService";
import { motion, AnimatePresence } from "motion/react";
import { useNavigate } from "react-router";
import { useTranslation } from "react-i18next";
import { KnowledgeBaseCard } from "../components/KnowledgeBaseCard";
import { KnowledgeBaseHeaderFilter } from "../components/KnowledgeBaseHeaderFilter";
import { BatchActionBar } from "../components/BatchActionBar";
import { EmptyKnowledgeBaseState } from "../components/EmptyKnowledgeBaseState";

export const KnowledgeBaseApp = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();

  const [knowledgeBases, setKnowledgeBases] = useState<KnowledgeBase[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [activeKb, setActiveKb] = useState<KnowledgeBase | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [activeFilter, setActiveFilter] = useState("all");

  // Batch Selection State
  const [isSelectionMode, setIsSelectionMode] = useState(false);
  const [selectedIds, setSelectedIds] = useState<string[]>([]);

  const loadKbs = () => {
    KnowledgeBaseService.getKnowledgeBases().then((data) => {
      setKnowledgeBases(data);
      setIsLoading(false);
    });
  };

  useEffect(() => {
    setIsLoading(true);
    loadKbs();
  }, []);

  const getFilteredAndSortedKbs = () => {
    let filtered = knowledgeBases;

    // Filter by archived status
    if (activeFilter === "archived") {
      filtered = filtered.filter((kb) => kb.isArchived);
    } else {
      filtered = filtered.filter((kb) => !kb.isArchived);
    }

    if (searchQuery.trim()) {
      filtered = filtered.filter(
        (kb) =>
          kb.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
          (kb.description && kb.description.toLowerCase().includes(searchQuery.toLowerCase()))
      );
    }

    return filtered.sort((a, b) => {
      if (activeFilter === "newest") {
        return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
      } else if (activeFilter === "oldest") {
        return new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime();
      } else if (activeFilter === "recently_updated") {
        return new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime();
      }
      return 0;
    });
  };

  const displayedKbs = getFilteredAndSortedKbs();

  const toggleSelectKb = (id: string) => {
    setSelectedIds((prev) =>
      prev.includes(id) ? prev.filter((i) => i !== id) : [...prev, id]
    );
  };

  const handleToggleSelectAll = () => {
    const displayedIds = displayedKbs.map((kb) => kb.id);
    const allSelected = displayedIds.every((id) => selectedIds.includes(id));
    if (allSelected) {
      setSelectedIds((prev) => prev.filter((id) => !displayedIds.includes(id)));
    } else {
      setSelectedIds((prev) => Array.from(new Set([...prev, ...displayedIds])));
    }
  };

  const handleBatchDelete = async () => {
    if (selectedIds.length === 0) return;
    const confirmed = await showConfirm(
      `确定要批量删除选中的 ${selectedIds.length} 个知识库及其包含的所有文档吗？`
    );
    if (confirmed) {
      await KnowledgeBaseService.deleteKnowledgeBases(selectedIds);
      showToast("已成功删除选中的知识库");
      setSelectedIds([]);
      setIsSelectionMode(false);
      loadKbs();
    }
  };

  const handleBatchArchive = async () => {
    if (selectedIds.length === 0) return;
    await KnowledgeBaseService.archiveKnowledgeBases(selectedIds);
    showToast(`已归档 ${selectedIds.length} 个知识库`);
    setSelectedIds([]);
    setIsSelectionMode(false);
    loadKbs();
  };

  const handleBatchUnarchive = async () => {
    if (selectedIds.length === 0) return;
    await KnowledgeBaseService.unarchiveKnowledgeBases(selectedIds);
    showToast(`已恢复 ${selectedIds.length} 个知识库`);
    setSelectedIds([]);
    setIsSelectionMode(false);
    loadKbs();
  };

  const handleRename = async () => {
    if (!activeKb) return;
    const currentKb = activeKb;
    setActiveKb(null);
    const newName = await showPrompt(
      t("knowledge.rename_kb_title", "Rename Knowledge Base"),
      currentKb.name
    );
    if (newName && newName.trim() !== "" && newName !== currentKb.name) {
      await KnowledgeBaseService.updateKnowledgeBase(currentKb.id, { name: newName });
      loadKbs();
    }
  };

  const handleSingleArchiveToggle = async () => {
    if (!activeKb) return;
    const currentKb = activeKb;
    setActiveKb(null);
    if (currentKb.isArchived) {
      await KnowledgeBaseService.unarchiveKnowledgeBases([currentKb.id]);
      showToast("知识库已取消归档");
    } else {
      await KnowledgeBaseService.archiveKnowledgeBases([currentKb.id]);
      showToast("知识库已归档");
    }
    loadKbs();
  };

  const handleDelete = async () => {
    if (!activeKb) return;
    const currentKb = activeKb;
    setActiveKb(null);
    const confirmed = await showConfirm(
      t(
        "knowledge.delete_kb_confirm",
        "Are you sure you want to delete this entire knowledge base and all its documents?"
      )
    );
    if (confirmed) {
      await KnowledgeBaseService.deleteKnowledgeBase(currentKb.id);
      loadKbs();
    }
  };

  const isAllDisplayedSelected =
    displayedKbs.length > 0 &&
    displayedKbs.every((kb) => selectedIds.includes(kb.id));

  return (
    <PageLayout title={t("knowledge.kb_title", "Knowledge Bases")}>
      <div className="flex flex-col h-full bg-[#f8f9fc] dark:bg-[#121214] relative">
        <KnowledgeBaseHeaderFilter
          searchQuery={searchQuery}
          setSearchQuery={setSearchQuery}
          activeFilter={activeFilter}
          setActiveFilter={(filter) => {
            setActiveFilter(filter);
            setSelectedIds([]);
          }}
          isSelectionMode={isSelectionMode}
          setIsSelectionMode={(val) => {
            setIsSelectionMode(val);
            if (!val) setSelectedIds([]);
          }}
          totalCount={displayedKbs.length}
        />

        <div className="flex-1 overflow-y-auto px-4 pb-28">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {isLoading ? (
              <div className="col-span-full flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-primary-blue/30 border-t-primary-blue animate-spin mb-3"></div>
                <span className="text-[14px]">{t("knowledge.loading", "Loading...")}</span>
              </div>
            ) : displayedKbs.length > 0 ? (
              displayedKbs.map((kb) => (
                <KnowledgeBaseCard
                  key={kb.id}
                  kb={kb}
                  isSelectionMode={isSelectionMode}
                  isSelected={selectedIds.includes(kb.id)}
                  onToggleSelect={() => toggleSelectKb(kb.id)}
                  onClickCard={() => navigate(`/workspace/knowledge/${kb.id}`)}
                  onMoreClick={() => setActiveKb(kb)}
                />
              ))
            ) : (
              <EmptyKnowledgeBaseState
                activeFilter={activeFilter}
                onCreateNew={() => navigate("/workspace/knowledge/create")}
              />
            )}
          </div>
        </div>

        {/* Floating Add Button when NOT in selection mode */}
        {!isSelectionMode && (
          <motion.button
            whileTap={{ scale: 0.9 }}
            whileHover={{ scale: 1.05 }}
            onClick={() => navigate("/workspace/knowledge/create")}
            className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/40 z-10"
          >
            <Plus className="w-7 h-7" />
          </motion.button>
        )}

        {/* Batch Action Bar */}
        <AnimatePresence>
          {isSelectionMode && (
            <BatchActionBar
              selectedCount={selectedIds.length}
              totalCount={displayedKbs.length}
              isAllSelected={isAllDisplayedSelected}
              onToggleSelectAll={handleToggleSelectAll}
              onBatchDelete={handleBatchDelete}
              onBatchArchive={handleBatchArchive}
              onBatchUnarchive={handleBatchUnarchive}
              onCancel={() => {
                setIsSelectionMode(false);
                setSelectedIds([]);
              }}
              isArchivedTab={activeFilter === "archived"}
            />
          )}
        </AnimatePresence>

        <ActionSheet
          isOpen={!!activeKb}
          onClose={() => setActiveKb(null)}
          title={activeKb?.name}
          options={[
            {
              label: t("common.rename", "Rename"),
              onClick: handleRename,
            },
            {
              label: activeKb?.isArchived ? "取消归档" : "归档",
              onClick: handleSingleArchiveToggle,
            },
            {
              label: t("common.delete", "Delete"),
              danger: true,
              onClick: handleDelete,
            },
          ]}
        />
      </div>
    </PageLayout>
  );
};
