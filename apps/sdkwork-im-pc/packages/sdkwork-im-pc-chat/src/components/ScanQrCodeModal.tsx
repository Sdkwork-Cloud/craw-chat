import React, { useCallback, useEffect, useRef, useState } from 'react';
import {
  AlertCircle,
  Camera,
  Copy,
  ExternalLink,
  Globe,
  Link,
  Loader2,
  QrCode,
  Upload,
  UserPlus,
  Users,
} from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { Avatar } from '@sdkwork/im-pc-commons';
import type { Chat, User } from '@sdkwork/im-pc-types';
import { communityService, type Community } from '@sdkwork/im-pc-community';
import { contactService } from '../services/ContactService';
import { groupService } from '../services/GroupService';
import { createQrCameraScanner, decodeQrCodeFromImageFile, type QrCameraScanner } from '../services/QrCodeDecodeService';
import {
  getQrCodeResultLabelKey,
  getQrCodeScanActions,
  parseQrCodeContent,
  type QrCodeScanActionKind,
  type QrCodeScanPayload,
} from '../services/QrCodeScanService';
import { ModalWrapper } from './ModalWrapper';
import { toast } from './Toast';

type ScanMode = 'upload' | 'camera';
type CameraState = 'idle' | 'starting' | 'active';
type UnknownQrCodePayload = Extract<QrCodeScanPayload, { kind: 'unknown' }>;

interface ResolvedQrCodeResult {
  community?: Community | null;
  group?: Chat | null;
  isResolving: boolean;
  payload: QrCodeScanPayload;
  resolveError?: string;
  user?: User | null;
}

interface ScanQrCodeModalProps {
  isOpen: boolean;
  onClose: () => void;
  onOpenCommunity?: (communityId: string) => void;
  onOpenGroup?: (group: Chat) => void | Promise<void>;
}

function findQrCodeAction(payload: QrCodeScanPayload, actionKind: QrCodeScanActionKind) {
  return getQrCodeScanActions(payload).find((action) => action.kind === actionKind);
}

function buildUserSubtitle(user: User | null | undefined, payload: Extract<QrCodeScanPayload, { kind: 'user' }>): string {
  return user?.chatId ?? payload.chatId ?? user?.email ?? user?.phone ?? payload.userId;
}

export const ScanQrCodeModal: React.FC<ScanQrCodeModalProps> = ({
  isOpen,
  onClose,
  onOpenCommunity,
  onOpenGroup,
}) => {
  const { t } = useTranslation();
  const [scanMode, setScanMode] = useState<ScanMode>('upload');
  const [cameraState, setCameraState] = useState<CameraState>('idle');
  const [isDecodingFile, setIsDecodingFile] = useState(false);
  const [statusMessage, setStatusMessage] = useState<string | null>(null);
  const [result, setResult] = useState<ResolvedQrCodeResult | null>(null);
  const [embeddedBrowserUrl, setEmbeddedBrowserUrl] = useState<string | null>(null);
  const [unknownContentModalPayload, setUnknownContentModalPayload] = useState<UnknownQrCodePayload | null>(null);
  const [userProfileModalUser, setUserProfileModalUser] = useState<User | null>(null);
  const [cameraDevices, setCameraDevices] = useState<MediaDeviceInfo[]>([]);
  const [selectedCameraDeviceId, setSelectedCameraDeviceId] = useState('');
  const [isLoadingCameraDevices, setIsLoadingCameraDevices] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const videoRef = useRef<HTMLVideoElement>(null);
  const scannerRef = useRef<QrCameraScanner | null>(null);

  const stopCamera = useCallback(() => {
    scannerRef.current?.stop();
    scannerRef.current = null;
    setCameraState('idle');
  }, []);

  const resolvePayload = useCallback(async (payload: QrCodeScanPayload) => {
    setResult({ payload, isResolving: true });
    try {
      if (payload.kind === 'user') {
        const user = await contactService.getUserById(payload.userId)
          ?? (payload.chatId ? await contactService.getUserById(payload.chatId) : null);
        setResult({ payload, user, isResolving: false });
        return;
      }

      if (payload.kind === 'group') {
        const groups = await groupService.getGroups();
        const group = groups.find((item) => (
          item.id === payload.groupId
          || item.id === payload.conversationId
        )) ?? null;
        setResult({ payload, group, isResolving: false });
        return;
      }

      if (payload.kind === 'community') {
        const community = await communityService.getCommunity(payload.communityId);
        setResult({ payload, community: community ?? null, isResolving: false });
        return;
      }

      setResult({ payload, isResolving: false });
    } catch {
      setResult({
        payload,
        isResolving: false,
        resolveError: t('scanQr.state.resolveFailed'),
      });
    }
  }, [t]);

  const handleDecodedContent = useCallback(async (rawContent: string) => {
    setEmbeddedBrowserUrl(null);
    setUnknownContentModalPayload(null);
    setUserProfileModalUser(null);
    setStatusMessage(null);
    try {
      const payload = parseQrCodeContent(rawContent);
      if (payload.kind === 'unknown') {
        setUnknownContentModalPayload(payload);
      }
      await resolvePayload(payload);
    } catch {
      setStatusMessage(t('scanQr.state.invalidContent'));
      setResult(null);
    }
  }, [resolvePayload, t]);

  const refreshCameraDevices = useCallback(async () => {
    if (!navigator.mediaDevices?.enumerateDevices) {
      setCameraDevices([]);
      return;
    }

    setIsLoadingCameraDevices(true);
    try {
      const devices = await navigator.mediaDevices.enumerateDevices();
      const videoDevices = devices.filter((device) => device.kind === 'videoinput');
      setCameraDevices(videoDevices);
      setSelectedCameraDeviceId((currentDeviceId) => {
        if (!currentDeviceId || videoDevices.some((device) => device.deviceId === currentDeviceId)) {
          return currentDeviceId;
        }
        return '';
      });
    } catch {
      setCameraDevices([]);
    } finally {
      setIsLoadingCameraDevices(false);
    }
  }, []);

  const startCamera = useCallback(async () => {
    if (!videoRef.current || cameraState !== 'idle') {
      return;
    }

    setStatusMessage(null);
    setCameraState('starting');
    try {
      const scanner = await createQrCameraScanner({
        deviceId: selectedCameraDeviceId || undefined,
        videoElement: videoRef.current,
        onDecode: (content) => {
          stopCamera();
          void handleDecodedContent(content);
        },
        onError: () => undefined,
      });
      scannerRef.current = scanner;
      setCameraState('active');
    } catch {
      setCameraState('idle');
      setStatusMessage(t('scanQr.state.cameraUnavailable'));
    }
  }, [cameraState, handleDecodedContent, selectedCameraDeviceId, stopCamera, t]);

  const decodeFile = async (file: File | undefined) => {
    if (!file) {
      return;
    }

    stopCamera();
    setIsDecodingFile(true);
    setStatusMessage(null);
    try {
      const content = await decodeQrCodeFromImageFile(file);
      await handleDecodedContent(content);
    } catch {
      setResult(null);
      setStatusMessage(t('scanQr.state.decodeFailed'));
    } finally {
      setIsDecodingFile(false);
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    }
  };

  const copyText = async (content: string) => {
    try {
      await navigator.clipboard.writeText(content);
      toast(t('scanQr.toast.copied'), 'success');
    } catch {
      toast(t('scanQr.toast.copyFailed'), 'error');
    }
  };

  const handleAddFriend = async () => {
    const targetUserId = userProfileModalUser?.id
      ?? (result?.payload.kind === 'user' ? result.user?.id ?? result.payload.userId : null);
    if (!targetUserId) {
      return;
    }
    const targetUserName = userProfileModalUser?.name
      ?? (result?.payload.kind === 'user' ? result.user?.name : null)
      ?? targetUserId;
    try {
      await contactService.addFriend(targetUserId);
      toast(t('scanQr.toast.friendRequestSent', { name: targetUserName }), 'success');
    } catch {
      toast(t('scanQr.toast.friendRequestFailed'), 'error');
    }
  };

  const handleOpenGroup = async () => {
    if (!result || result.payload.kind !== 'group') {
      return;
    }
    if (!result.group) {
      toast(t('scanQr.toast.joinCapabilityUnavailable'), 'error');
      return;
    }
    await onOpenGroup?.(result.group);
    onClose();
  };

  const handleOpenCommunity = () => {
    if (!result || result.payload.kind !== 'community') {
      return;
    }
    if (!result.community) {
      toast(t('scanQr.toast.joinCapabilityUnavailable'), 'error');
      return;
    }
    onOpenCommunity?.(result.community.id);
    onClose();
  };

  useEffect(() => {
    if (!isOpen) {
      stopCamera();
      setScanMode('upload');
      setStatusMessage(null);
      setResult(null);
      setEmbeddedBrowserUrl(null);
      setUnknownContentModalPayload(null);
      setUserProfileModalUser(null);
      setSelectedCameraDeviceId('');
    }
  }, [isOpen, stopCamera]);

  useEffect(() => () => stopCamera(), [stopCamera]);

  useEffect(() => {
    if (isOpen && scanMode === 'camera') {
      void refreshCameraDevices();
    }
  }, [isOpen, refreshCameraDevices, scanMode]);

  const renderModeTabs = () => (
    <div className="grid h-9 grid-cols-2 rounded-lg border border-white/10 bg-[#181818] p-1">
      <button
        type="button"
        onClick={() => {
          stopCamera();
          setScanMode('upload');
        }}
        className={`flex items-center justify-center gap-2 rounded-md text-sm transition-colors ${
          scanMode === 'upload'
            ? 'bg-white/10 text-gray-100'
            : 'text-gray-400 hover:text-gray-200'
        }`}
      >
        <Upload size={15} />
        {t('scanQr.tabs.upload')}
      </button>
      <button
        type="button"
        onClick={() => {
          setScanMode('camera');
          void refreshCameraDevices();
        }}
        className={`flex items-center justify-center gap-2 rounded-md text-sm transition-colors ${
          scanMode === 'camera'
            ? 'bg-white/10 text-gray-100'
            : 'text-gray-400 hover:text-gray-200'
        }`}
      >
        <Camera size={15} />
        {t('scanQr.tabs.camera')}
      </button>
    </div>
  );

  const renderUploadPane = () => (
    <div
      className="flex min-h-[220px] flex-col items-center justify-center rounded-lg border border-dashed border-white/10 bg-[#181818] px-6 text-center transition-colors hover:border-white/20"
      onDrop={(event) => {
        event.preventDefault();
        void decodeFile(event.dataTransfer.files[0]);
      }}
      onDragOver={(event) => event.preventDefault()}
    >
      <div className="mb-4 flex h-14 w-14 items-center justify-center rounded-lg bg-white/5 text-gray-300">
        {isDecodingFile ? <Loader2 size={24} className="animate-spin" /> : <QrCode size={24} />}
      </div>
      <div className="mb-2 text-sm font-medium text-gray-200">{t('scanQr.upload.title')}</div>
      <div className="mb-5 max-w-[340px] text-xs leading-5 text-gray-500">{t('scanQr.upload.description')}</div>
      <input
        ref={fileInputRef}
        type="file"
        accept=".png,.jpg,.jpeg,.webp"
        className="hidden"
        onChange={(event) => void decodeFile(event.target.files?.[0])}
      />
      <button
        type="button"
        onClick={() => fileInputRef.current?.click()}
        disabled={isDecodingFile}
        className="flex items-center gap-2 rounded bg-[#00b42a] px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-[#009a24] disabled:bg-[#00b42a]/50"
      >
        <Upload size={15} />
        {isDecodingFile ? t('scanQr.actions.decoding') : t('scanQr.actions.chooseFile')}
      </button>
    </div>
  );

  const renderCameraPane = () => (
    <div className="overflow-hidden rounded-lg border border-white/10 bg-[#181818]">
      <div className="relative h-[260px] bg-black">
        <video
          ref={videoRef}
          className="h-full w-full object-cover"
          muted
          playsInline
        />
        {cameraState !== 'active' && (
          <div className="absolute inset-0 flex flex-col items-center justify-center bg-[#181818] text-center">
            <Camera size={28} className="mb-4 text-gray-400" />
            <div className="mb-2 text-sm font-medium text-gray-200">{t('scanQr.camera.title')}</div>
            <div className="max-w-[340px] text-xs leading-5 text-gray-500">{t('scanQr.camera.description')}</div>
          </div>
        )}
      </div>
      <div className="flex items-center gap-3 border-t border-white/5 px-4 py-3">
        <label htmlFor="scan-qr-camera-device" className="shrink-0 text-xs text-gray-500">
          {t('scanQr.camera.device')}
        </label>
        <select
          id="scan-qr-camera-device"
          value={selectedCameraDeviceId}
          onChange={(event) => {
            stopCamera();
            setSelectedCameraDeviceId(event.target.value);
          }}
          disabled={cameraState === 'starting'}
          className="min-w-0 flex-1 rounded border border-white/10 bg-[#202020] px-2 py-1.5 text-xs text-gray-200 outline-none transition-colors focus:border-[#00b42a] disabled:opacity-50"
        >
          <option value="">
            {cameraDevices.length > 0
              ? t('scanQr.camera.defaultDevice')
              : t(isLoadingCameraDevices ? 'scanQr.camera.loadingDevices' : 'scanQr.camera.noDevices')}
          </option>
          {cameraDevices.map((device, index) => (
            <option key={device.deviceId || `camera-${index}`} value={device.deviceId}>
              {device.label || t('scanQr.camera.deviceName', { index: index + 1 })}
            </option>
          ))}
        </select>
      </div>
      <div className="flex items-center justify-between border-t border-white/5 px-4 py-3">
        <div className="flex items-center gap-2 text-xs text-gray-500">
          {cameraState === 'starting' && <Loader2 size={14} className="animate-spin" />}
          <span>{t(`scanQr.camera.state.${cameraState}`)}</span>
        </div>
        {cameraState === 'active' ? (
          <button
            type="button"
            onClick={stopCamera}
            className="rounded bg-white/10 px-3 py-1.5 text-sm text-gray-200 transition-colors hover:bg-white/15"
          >
            {t('scanQr.actions.stopCamera')}
          </button>
        ) : (
          <button
            type="button"
            onClick={() => void startCamera()}
            disabled={cameraState === 'starting'}
            className="flex items-center gap-2 rounded bg-[#00b42a] px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-[#009a24] disabled:bg-[#00b42a]/50"
          >
            {cameraState === 'starting' ? <Loader2 size={14} className="animate-spin" /> : <Camera size={14} />}
            {t('scanQr.actions.startCamera')}
          </button>
        )}
      </div>
    </div>
  );

  const renderStatus = () => {
    if (!statusMessage) {
      return null;
    }
    return (
      <div className="flex items-center gap-2 rounded-lg border border-red-500/20 bg-red-500/10 px-3 py-2 text-xs text-red-200">
        <AlertCircle size={14} />
        <span>{statusMessage}</span>
      </div>
    );
  };

  const renderResultHeader = (payload: QrCodeScanPayload) => (
    <div className="mb-4 flex items-center gap-2">
      <div className="flex h-8 w-8 items-center justify-center rounded bg-white/5 text-gray-300">
        {payload.kind === 'user' && <UserPlus size={16} />}
        {payload.kind === 'group' && <Users size={16} />}
        {payload.kind === 'community' && <Globe size={16} />}
        {payload.kind === 'url' && <Link size={16} />}
        {payload.kind === 'unknown' && <QrCode size={16} />}
      </div>
      <div>
        <div className="text-sm font-medium text-gray-100">{t(getQrCodeResultLabelKey(payload))}</div>
        <div className="text-xs text-gray-500">{t('scanQr.result.source', { source: payload.source })}</div>
      </div>
    </div>
  );

  const renderUserResult = (resolvedResult: ResolvedQrCodeResult) => {
    if (resolvedResult.payload.kind !== 'user') {
      return null;
    }
    const user = resolvedResult.user ?? {
      id: resolvedResult.payload.userId,
      chatId: resolvedResult.payload.chatId,
      name: resolvedResult.payload.userId,
      status: 'offline' as const,
    };
    const viewProfileAction = findQrCodeAction(resolvedResult.payload, 'viewUserProfile');
    const addFriendAction = findQrCodeAction(resolvedResult.payload, 'sendFriendRequest');
    return (
      <div className="flex items-center justify-between gap-4 rounded-lg border border-white/5 bg-[#181818] p-4">
        <div className="flex min-w-0 items-center gap-3">
          <Avatar src={user.avatar} alt={user.name} className="h-12 w-12 rounded-lg" />
          <div className="min-w-0">
            <div className="truncate text-sm font-medium text-gray-100">{user.name}</div>
            <div className="mt-1 truncate text-xs text-gray-500">{buildUserSubtitle(user, resolvedResult.payload)}</div>
          </div>
        </div>
        <div className="flex shrink-0 flex-wrap justify-end gap-2">
          <button
            type="button"
            onClick={() => setUserProfileModalUser(user)}
            className="flex items-center gap-2 rounded bg-white/10 px-3 py-2 text-sm text-gray-200 transition-colors hover:bg-white/15"
          >
            <QrCode size={15} />
            {t(viewProfileAction?.labelKey ?? 'scanQr.actions.viewUserProfile')}
          </button>
          <button
            type="button"
            onClick={() => void handleAddFriend()}
            className="flex items-center gap-2 rounded bg-[#00b42a] px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-[#009a24]"
          >
            <UserPlus size={15} />
            {t(addFriendAction?.labelKey ?? 'scanQr.actions.addFriend')}
          </button>
        </div>
      </div>
    );
  };

  const renderGroupResult = (resolvedResult: ResolvedQrCodeResult) => {
    if (resolvedResult.payload.kind !== 'group') {
      return null;
    }
    const group = resolvedResult.group;
    const groupName = group?.name ?? t('scanQr.group.unknownName');
    const groupSubtitle = group
      ? t('scanQr.group.memberCount', { count: group.memberCount ?? 0 })
      : t('scanQr.group.unresolved');
    const action = findQrCodeAction(resolvedResult.payload, group ? 'openGroup' : 'joinGroup');
    return (
      <div className="space-y-3">
        <div className="flex items-center justify-between gap-4 rounded-lg border border-white/5 bg-[#181818] p-4">
          <div className="flex min-w-0 items-center gap-3">
            <Avatar src={group?.avatar} alt={groupName} className="h-12 w-12 rounded-lg" />
            <div className="min-w-0">
              <div className="truncate text-sm font-medium text-gray-100">{groupName}</div>
              <div className="mt-1 truncate text-xs text-gray-500">
                {groupSubtitle}
              </div>
            </div>
          </div>
          <button
            type="button"
            onClick={() => void handleOpenGroup()}
            className="flex shrink-0 items-center gap-2 rounded bg-[#00b42a] px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-[#009a24]"
          >
            <Users size={15} />
            {t(action?.labelKey ?? (group ? 'scanQr.actions.openGroup' : 'scanQr.actions.joinGroup'))}
          </button>
        </div>
        {!group && (
          <div className="rounded-lg border border-amber-500/20 bg-amber-500/10 px-3 py-2 text-xs leading-5 text-amber-200">
            {t('scanQr.state.joinCapabilityUnavailable')}
          </div>
        )}
      </div>
    );
  };

  const renderCommunityResult = (resolvedResult: ResolvedQrCodeResult) => {
    if (resolvedResult.payload.kind !== 'community') {
      return null;
    }
    const community = resolvedResult.community;
    const communityName = community?.name ?? t('scanQr.community.unknownName');
    const communitySubtitle = community
      ? t('scanQr.community.memberCount', { count: community.membersCount })
      : t('scanQr.community.unresolved');
    const action = findQrCodeAction(resolvedResult.payload, community ? 'openCommunity' : 'joinCommunity');
    return (
      <div className="space-y-3">
        <div className="flex items-center justify-between gap-4 rounded-lg border border-white/5 bg-[#181818] p-4">
          <div className="flex min-w-0 items-center gap-3">
            <Avatar src={community?.avatar} alt={communityName} className="h-12 w-12 rounded-lg" />
            <div className="min-w-0">
              <div className="truncate text-sm font-medium text-gray-100">{communityName}</div>
              <div className="mt-1 truncate text-xs text-gray-500">
                {communitySubtitle}
              </div>
            </div>
          </div>
          <button
            type="button"
            onClick={handleOpenCommunity}
            className="flex shrink-0 items-center gap-2 rounded bg-[#00b42a] px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-[#009a24]"
          >
            <Globe size={15} />
            {t(action?.labelKey ?? (community ? 'scanQr.actions.openCommunity' : 'scanQr.actions.joinCommunity'))}
          </button>
        </div>
        {!community && (
          <div className="rounded-lg border border-amber-500/20 bg-amber-500/10 px-3 py-2 text-xs leading-5 text-amber-200">
            {t('scanQr.state.joinCapabilityUnavailable')}
          </div>
        )}
      </div>
    );
  };

  const renderUrlResult = (payload: Extract<QrCodeScanPayload, { kind: 'url' }>) => {
    const openLinkAction = findQrCodeAction(payload, 'openEmbeddedBrowser');
    const copyAction = findQrCodeAction(payload, 'copyRawContent');
    return (
      <div className="space-y-3">
        <div className="rounded-lg border border-white/5 bg-[#181818] p-4">
          <div className="mb-3 break-all text-sm text-gray-200">{payload.url}</div>
          <div className="flex flex-wrap gap-2">
            <button
              type="button"
              onClick={() => setEmbeddedBrowserUrl(payload.url)}
              className="flex items-center gap-2 rounded bg-[#00b42a] px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-[#009a24]"
            >
              <ExternalLink size={15} />
              {t(openLinkAction?.labelKey ?? 'scanQr.actions.openLink')}
            </button>
            <button
              type="button"
              onClick={() => void copyText(payload.url)}
              className="flex items-center gap-2 rounded bg-white/10 px-3 py-2 text-sm text-gray-200 transition-colors hover:bg-white/15"
            >
              <Copy size={15} />
              {t(copyAction?.labelKey ?? 'scanQr.actions.copyContent')}
            </button>
          </div>
        </div>
        {embeddedBrowserUrl && (
          <div className="overflow-hidden rounded-lg border border-white/10 bg-[#181818]">
            <div className="flex h-10 items-center justify-between border-b border-white/5 px-3">
              <div className="truncate text-xs text-gray-400">{t('scanQr.browser.title')}</div>
              <button
                type="button"
                onClick={() => setEmbeddedBrowserUrl(null)}
                className="text-xs text-gray-500 transition-colors hover:text-gray-200"
              >
                {t('scanQr.actions.closeBrowser')}
              </button>
            </div>
            <iframe
              src={embeddedBrowserUrl}
              title={t('scanQr.browser.title')}
              className="h-[360px] w-full border-0 bg-white"
              sandbox="allow-forms allow-popups allow-same-origin allow-scripts"
            />
          </div>
        )}
      </div>
    );
  };

  const renderUserProfileModalBody = (user: User) => {
    const displayUserChatId = user.chatId ?? user.id;
    const profileFields = [
      { labelKey: 'contacts.detail.chatId', value: displayUserChatId },
      { labelKey: 'contacts.detail.email', value: user.email },
      { labelKey: 'contacts.detail.phone', value: user.phone },
      { labelKey: 'contacts.detail.company', value: user.company },
      { labelKey: 'contacts.detail.location', value: user.location },
    ].filter((field) => Boolean(field.value));

    return (
      <div className="space-y-4">
        <div className="flex items-center gap-4 rounded-lg border border-white/5 bg-[#181818] p-4">
          <Avatar src={user.avatar} alt={user.name} className="h-16 w-16 rounded-lg bg-[#2b2b2d]" />
          <div className="min-w-0">
            <div className="truncate text-base font-semibold text-gray-100">{user.name}</div>
            <div className="mt-1 truncate text-sm text-gray-500">{user.position || t('contacts.detail.unknownPosition')}</div>
          </div>
        </div>
        <div className="rounded-lg border border-white/5 bg-[#181818] p-4">
          <div className="grid gap-3">
            {profileFields.map((field) => (
              <div key={field.labelKey} className="grid gap-1">
                <div className="text-xs text-gray-500">{t(field.labelKey)}</div>
                <div className="break-all text-sm text-gray-200">{field.value}</div>
              </div>
            ))}
          </div>
        </div>
        <div className="flex flex-wrap justify-end gap-2">
          <button
            type="button"
            onClick={() => void copyText(displayUserChatId)}
            className="flex items-center gap-2 rounded bg-white/10 px-3 py-2 text-sm text-gray-200 transition-colors hover:bg-white/15"
          >
            <Copy size={15} />
            {t('scanQr.actions.copyContent')}
          </button>
          <button
            type="button"
            onClick={() => void handleAddFriend()}
            className="flex items-center gap-2 rounded bg-[#00b42a] px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-[#009a24]"
          >
            <UserPlus size={15} />
            {t('scanQr.actions.addFriend')}
          </button>
        </div>
      </div>
    );
  };

  const renderUnknownContentBody = (payload: UnknownQrCodePayload) => (
    <div className="space-y-3">
      <pre className="max-h-[320px] overflow-auto whitespace-pre-wrap break-words rounded-lg border border-white/5 bg-black/30 p-3 text-xs leading-5 text-gray-200">
        {payload.rawContent}
      </pre>
      <div className="flex justify-end">
        <button
          type="button"
          onClick={() => void copyText(payload.rawContent)}
          className="flex items-center gap-2 rounded bg-white/10 px-3 py-2 text-sm text-gray-200 transition-colors hover:bg-white/15"
        >
          <Copy size={15} />
          {t(findQrCodeAction(payload, 'copyRawContent')?.labelKey ?? 'scanQr.actions.copyContent')}
        </button>
      </div>
    </div>
  );

  const renderUnknownResult = (payload: UnknownQrCodePayload) => {
    const viewContentAction = findQrCodeAction(payload, 'showUnknownContentModal');
    const copyAction = findQrCodeAction(payload, 'copyRawContent');
    return (
      <div className="rounded-lg border border-white/5 bg-[#181818] p-4">
        <div className="mb-3 line-clamp-3 whitespace-pre-wrap break-words text-xs leading-5 text-gray-400">
          {payload.rawContent}
        </div>
        <div className="flex flex-wrap justify-end gap-2">
          <button
            type="button"
            onClick={() => setUnknownContentModalPayload(payload)}
            className="flex items-center gap-2 rounded bg-[#00b42a] px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-[#009a24]"
          >
            <QrCode size={15} />
            {t(viewContentAction?.labelKey ?? 'scanQr.actions.viewContent')}
          </button>
          <button
            type="button"
            onClick={() => void copyText(payload.rawContent)}
            className="flex items-center gap-2 rounded bg-white/10 px-3 py-2 text-sm text-gray-200 transition-colors hover:bg-white/15"
          >
            <Copy size={15} />
            {t(copyAction?.labelKey ?? 'scanQr.actions.copyContent')}
          </button>
        </div>
      </div>
    );
  };

  const renderResult = () => {
    if (!result) {
      return null;
    }

    return (
      <div className="rounded-lg border border-white/10 bg-[#202020] p-4">
        {renderResultHeader(result.payload)}
        {result.isResolving ? (
          <div className="flex items-center gap-2 rounded-lg bg-[#181818] px-3 py-4 text-sm text-gray-400">
            <Loader2 size={16} className="animate-spin" />
            <span>{t('scanQr.state.resolving')}</span>
          </div>
        ) : (
          <>
            {result.resolveError && (
              <div className="mb-3 flex items-center gap-2 rounded-lg border border-red-500/20 bg-red-500/10 px-3 py-2 text-xs text-red-200">
                <AlertCircle size={14} />
                <span>{result.resolveError}</span>
              </div>
            )}
            {renderUserResult(result)}
            {renderGroupResult(result)}
            {renderCommunityResult(result)}
            {result.payload.kind === 'url' && renderUrlResult(result.payload)}
            {result.payload.kind === 'unknown' && renderUnknownResult(result.payload)}
          </>
        )}
      </div>
    );
  };

  return (
    <>
      <ModalWrapper isOpen={isOpen} onClose={onClose} title={t('scanQr.modal.title')} width="w-[760px]">
        <div className="space-y-4">
          {renderModeTabs()}
          {scanMode === 'upload' ? renderUploadPane() : renderCameraPane()}
          {renderStatus()}
          {renderResult()}
        </div>
      </ModalWrapper>
      <ModalWrapper
        isOpen={Boolean(unknownContentModalPayload)}
        onClose={() => setUnknownContentModalPayload(null)}
        title={t('scanQr.unknownContent.modalTitle')}
        width="w-[560px]"
      >
        {unknownContentModalPayload && renderUnknownContentBody(unknownContentModalPayload)}
      </ModalWrapper>
      <ModalWrapper
        isOpen={Boolean(userProfileModalUser)}
        onClose={() => setUserProfileModalUser(null)}
        title={t('scanQr.userProfile.modalTitle')}
        width="w-[560px]"
      >
        {userProfileModalUser && renderUserProfileModalBody(userProfileModalUser)}
      </ModalWrapper>
    </>
  );
};
