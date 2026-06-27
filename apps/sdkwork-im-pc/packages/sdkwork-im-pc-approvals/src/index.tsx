import React, { useState, useEffect } from 'react';
import { 
  FileText, CheckCircle, XCircle, Clock,
  Plus, Briefcase, DollarSign
} from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { toast } from '@sdkwork/im-pc-chat';
import { ApprovalsService, ApprovalItem } from './services/ApprovalsService';
import { CreateApprovalModal } from './components/CreateApprovalModal';
import { ApprovalListPanel } from './components/ApprovalListPanel';
import { ApprovalDetailPanel } from './components/ApprovalDetailPanel';

export const ApprovalsView: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'pending' | 'handled'>('pending');
  const [approvals, setApprovals] = useState<ApprovalItem[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [comment, setComment] = useState('');

  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newApprovalType, setNewApprovalType] = useState<'leave' | 'reimbursement' | 'purchase' | 'other'>('leave');
  const [newApprovalTitle, setNewApprovalTitle] = useState('');
  const [newApprovalDesc, setNewApprovalDesc] = useState('');
  const [newApprovalAmount, setNewApprovalAmount] = useState('');

  useEffect(() => {
    const fetchData = async () => {
      const lists = activeTab === 'pending' 
        ? await ApprovalsService.getPendingApprovals() 
        : await ApprovalsService.getHandledApprovals();
      setApprovals(lists);
    };
    fetchData();
  }, [activeTab]);

  const displayList = approvals;
  const selectedItem = approvals.find(a => a.id === selectedId);

  const handleSubmitNewApproval = async () => {
    if (!newApprovalTitle.trim() || !newApprovalDesc.trim()) {
      toast('请填写标题和详细说明', 'error');
      return;
    }
    try {
      await ApprovalsService.submitApproval({
        id: '',
        title: newApprovalTitle,
        type: newApprovalType,
        status: 'pending',
        applicant: { id: 'current-user', name: '当前用户' },
        submitTime: '',
        description: newApprovalDesc,
        amount: newApprovalAmount ? parseFloat(newApprovalAmount) : undefined,
      });
      setShowCreateModal(false);
      setNewApprovalTitle('');
      setNewApprovalDesc('');
      setNewApprovalAmount('');
      toast('审批已提交', 'success');
    } catch (error) {
      toast(error instanceof Error ? error.message : '操作失败', 'error');
    }
  };

  const handleAction = async (action: 'approve' | 'reject') => {
    if (!selectedId) return;
    try {
      if (action === 'approve') await ApprovalsService.approve(selectedId, comment);
      else await ApprovalsService.reject(selectedId, comment);

      toast(action === 'approve' ? '已同意该申请' : '已拒绝该申请', 'success');
      setSelectedId(null);
      setComment('');
      const lists = activeTab === 'pending'
        ? await ApprovalsService.getPendingApprovals()
        : await ApprovalsService.getHandledApprovals();
      setApprovals(lists);
    } catch (e) {
      toast(e instanceof Error ? e.message : '操作失败', 'error');
    }
  };

  const getStatusIcon = (status: string) => {
     switch(status) {
       case 'pending': return <Clock size={16} className="text-amber-500" />;
       case 'approved': return <CheckCircle size={16} className="text-green-500" />;
       case 'rejected': return <XCircle size={16} className="text-red-500" />;
       default: return <FileText size={16} className="text-gray-400" />;
     }
  };

  const getTypeIcon = (type: string) => {
     switch(type) {
       case 'leave': return <Briefcase size={18} className="text-blue-400" />;
       case 'reimbursement': return <DollarSign size={18} className="text-emerald-400" />;
       case 'purchase': return <FileText size={18} className="text-indigo-400" />;
       default: return <FileText size={18} className="text-gray-400" />;
     }
  };

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 animate-in fade-in">
      {/* Header */}
      <div className="h-16 border-b border-white/5 flex items-center justify-between px-6 shrink-0 bg-[#1e1e1e]/80 backdrop-blur-md">
        <div className="flex items-center gap-6">
          <h2 className="text-lg font-medium text-gray-200">审批流</h2>
          <div className="flex items-center gap-1 bg-black/20 p-1 rounded-lg">
            <button 
              onClick={() => { setActiveTab('pending'); setSelectedId(null); }}
              className={cn("px-4 py-1.5 rounded-md text-sm font-medium transition-colors", activeTab === 'pending' ? "bg-[#3a3a3a] text-white shadow-sm" : "text-gray-400 hover:text-gray-200")}
            > 待处理 </button>
            <button 
              onClick={() => { setActiveTab('handled'); setSelectedId(null); }}
              className={cn("px-4 py-1.5 rounded-md text-sm font-medium transition-colors", activeTab === 'handled' ? "bg-[#3a3a3a] text-white shadow-sm" : "text-gray-400 hover:text-gray-200")}
            > 已处理 </button>
          </div>
        </div>
        <div className="flex items-center gap-3">
           <button onClick={() => setShowCreateModal(true)} className="flex items-center gap-2 bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-lg text-sm font-medium transition-all shadow-lg shadow-indigo-500/20">
              <Plus size={16} /> 发起审批
           </button>
        </div>
      </div>

      <CreateApprovalModal 
        show={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        newApprovalType={newApprovalType}
        setNewApprovalType={setNewApprovalType}
        newApprovalTitle={newApprovalTitle}
        setNewApprovalTitle={setNewApprovalTitle}
        newApprovalAmount={newApprovalAmount}
        setNewApprovalAmount={setNewApprovalAmount}
        newApprovalDesc={newApprovalDesc}
        setNewApprovalDesc={setNewApprovalDesc}
        onSubmit={handleSubmitNewApproval}
      />

      {/* Main Content */}
      <div className="flex-1 flex overflow-hidden relative">
        <ApprovalListPanel 
          displayList={displayList}
          selectedId={selectedId}
          setSelectedId={setSelectedId}
          getTypeIcon={getTypeIcon}
          getStatusIcon={getStatusIcon}
        />

        {selectedItem ? (
          <ApprovalDetailPanel 
            selectedItem={selectedItem}
            setSelectedId={setSelectedId}
            comment={comment}
            setComment={setComment}
            handleAction={handleAction}
          />
        ) : (
          <div className="hidden lg:flex flex-1 flex-col items-center justify-center text-gray-500 p-8">
            <div className="w-24 h-24 mb-6 rounded-full bg-white/5 flex items-center justify-center">
              <FileText size={48} className="text-white/20" />
            </div>
            <p className="text-lg font-medium text-gray-400 mb-2">选择一个审批查看详情</p>
            <p className="text-sm text-center max-w-sm">在左侧列表中点击审批条目可以查看详细信息、附件内容并进行审批操作。</p>
          </div>
        )}
      </div>
    </div>
  );
};
