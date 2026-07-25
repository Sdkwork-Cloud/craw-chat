import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, X, Video, PenTool } from "lucide-react";
import { IconButton, cn, showToast, ActionSheet } from "@sdkwork/im-h5-commons";
import { NotaryFullPageEditor } from "../components/NotaryFullPageEditor";
import { NotaryBottomPicker } from "../components/NotaryBottomPicker";
import { IdentityVerificationSection } from "../components/IdentityVerificationSection";
import { BasicInfoSection } from "../components/BasicInfoSection";
import { AccessoriesRemarksSection } from "../components/AccessoriesRemarksSection";
import { NotaryFullscreenImageOverlay } from "../components/NotaryFullscreenImageOverlay";
import { useTranslation } from "react-i18next";

export const NotaryPartyParams = {
  editData: null as any,
  isReadonly: false,
  onAdd: (party: any) => {},
  onEdit: (party: any) => {},
};

export const NotaryAddParty: React.FC = () => {
  const { t } = useTranslation();
  
const navigate = useNavigate();
  

  const [formData, setFormData] = useState(() => {
    if (NotaryPartyParams.editData) {
      return {
        name: NotaryPartyParams.editData.name || "",
        idCard: NotaryPartyParams.editData.idCard || "",
        gender: NotaryPartyParams.editData.gender || "男",
        dob: NotaryPartyParams.editData.dob || "",
        idStartDate: NotaryPartyParams.editData.idStartDate || "",
        idEndDate: NotaryPartyParams.editData.idEndDate || "",
        phone: NotaryPartyParams.editData.phone || "",
        address: NotaryPartyParams.editData.address || "",
        remarks: NotaryPartyParams.editData.remarks || "",
      };
    }
    return {
      name: "",
      idCard: "",
      gender: "男",
      dob: "",
      idStartDate: "",
      idEndDate: "",
      phone: "",
      address: "",
      remarks: "",
    };
  });

  const [faceScore, setFaceScore] = useState<number | null>(() => {
    if (NotaryPartyParams.editData?.faceScore) {
      return parseFloat(NotaryPartyParams.editData.faceScore);
    }
    return null;
  });
  const [isScanning, setIsScanning] = useState(false);
  const [pickerType, setPickerType] = useState<
    "gender" | "dob" | "idStartDate" | "idEndDate" | null
  >(null);
  const [tempDate, setTempDate] = useState({ year: 1990, month: 1, day: 1 });

  const [fullPageEditor, setFullPageEditor] = useState<{
    field: keyof typeof formData;
    title: string;
    placeholder: string;
    value: string;
    isTextArea?: boolean;
    inputType?: string;
  } | null>(null);

  const idFrontRef = useRef<HTMLInputElement>(null);
  const idBackRef = useRef<HTMLInputElement>(null);
  const attachmentRef = useRef<HTMLInputElement>(null);
  const faceRef = useRef<HTMLInputElement>(null);

  const [idFrontPreview, setIdFrontPreview] = useState<string | null>(
    NotaryPartyParams.editData?.idFrontPreview || null,
  );
  const [idBackPreview, setIdBackPreview] = useState<string | null>(
    NotaryPartyParams.editData?.idBackPreview || null,
  );
  const [facePreview, setFacePreview] = useState<string | null>(
    NotaryPartyParams.editData?.facePreview || null,
  );
  const [attachments, setAttachments] = useState<
    { name: string; url: string }[]
  >(NotaryPartyParams.editData?.attachments || []);
  const [fullscreenImage, setFullscreenImage] = useState<string | null>(null);
  const [showVideoActionSheet, setShowVideoActionSheet] = useState(false);

  const videoOptions = [
    {
      label: t("notary.add_party.call_now"),
      onClick: () => {
        setShowVideoActionSheet(false);
        navigate(`/call/video-notary/${NotaryPartyParams.editData?.id || 'party'}`);
      }
    },
    {
      label: t("notary.add_party.video_qr"),
      onClick: () => {
        setShowVideoActionSheet(false);
        navigate(`/notary/party-video-qr/${NotaryPartyParams.editData?.id || 'party'}`); 
      }
    }
  ];

  // Cleanup object URLs on unmount
  useEffect(() => {
    return () => {};
  }, []);

  const handleFileChange = (
    e: React.ChangeEvent<HTMLInputElement>,
    setter: React.Dispatch<React.SetStateAction<string | null>>,
    existingUrl: string | null,
    side?: "front" | "back",
  ) => {
  if (NotaryPartyParams.isReadonly) return;
    const file = e.target.files?.[0];
    if (file) {
      setter(URL.createObjectURL(file));

      showToast(t("notary.add_party.recognizing"));
      setTimeout(() => {
        if (side === "front") {
          setFormData((prev) => ({
            ...prev,
            name: "李小明",
            idCard: "11010519900101234X",
            gender: "男",
            dob: "1990-01-01",
            address: "北京市朝阳区建国路88号",
          }));
          showToast(t("notary.add_party.front_success"));
        } else if (side === "back") {
          setFormData((prev) => ({
            ...prev,
            idStartDate: "2020-01-01",
            idEndDate: "2040-01-01",
          }));
          showToast(t("notary.add_party.back_success"));
        }
      }, 1000);
    }
  };

  const handleAttachmentsChange = (e: React.ChangeEvent<HTMLInputElement>) => {
  if (NotaryPartyParams.isReadonly) return;
    const files = e.target.files;
    if (files && files.length > 0) {
      const newAttachments = Array.from(files).map((file: File) => ({
        name: file.name,
        url: URL.createObjectURL(file), // Will generate preview if it's an image
      }));
      setAttachments((prev) => [...prev, ...newAttachments]);
    }
  };

  const handleFaceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
  if (NotaryPartyParams.isReadonly) return;
    const file = e.target.files?.[0];
    if (file) {
      setFacePreview(URL.createObjectURL(file));
      setFaceScore(null);
    }
  };

  const handleStartComparison = () => {
  if (NotaryPartyParams.isReadonly || !facePreview) return;
    setIsScanning(true);
    setTimeout(() => {
      setFaceScore(98.5);
      setIsScanning(false);
    }, 1500);
  };

  const handleSave = () => {
  if (!idFrontPreview) return showToast(t("notary.add_party.err_front"));
    if (!idBackPreview) return showToast(t("notary.add_party.err_back"));
    if (!facePreview) return showToast(t("notary.add_party.err_face"));
    if (!formData.name || formData.name.trim().length < 2)
      return showToast(t("notary.add_party.err_name"));
    if (
      !formData.idCard ||
      !/(^\d{15}$)|(^\d{18}$)|(^\d{17}(\d|X|x)$)/.test(formData.idCard)
    )
      return showToast(t("notary.add_party.err_id"));
    if (!formData.phone || !/^1\d{10}$/.test(formData.phone))
      return showToast(t("notary.add_party.err_phone"));

    if (!formData.idStartDate || !formData.idEndDate)
      return showToast(t("notary.add_party.err_id_date"));

    const partyData = {
      id: NotaryPartyParams.editData
        ? NotaryPartyParams.editData.id
        : Date.now().toString(),
      name: formData.name,
      idCard: formData.idCard,
      gender: formData.gender,
      dob: formData.dob,
      idStartDate: formData.idStartDate,
      idEndDate: formData.idEndDate,
      phone: formData.phone,
      address: formData.address,
      remarks: formData.remarks,
      faceScore: faceScore ? faceScore.toFixed(2) : null,
      attachmentsCount: attachments.length,
      attachments: attachments,
      idFrontPreview,
      idBackPreview,
      facePreview,
    };

    if (NotaryPartyParams.editData && NotaryPartyParams.onEdit) {
      NotaryPartyParams.onEdit(partyData);
      navigate(-1);
    } else if (NotaryPartyParams.onAdd) {
      NotaryPartyParams.onAdd(partyData);
      navigate(`/notary/party-signature/${partyData.id}`, { replace: true });
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black text-text-main fixed inset-0 z-[100] animate-in slide-in-from-right">
      <header className="h-[44px] flex items-center justify-between sticky top-0 shrink-0 pt-safe px-1 z-20 bg-bg-color border-b border-border-color">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="flex items-center justify-center font-bold text-[17px] pointer-events-none">
          {NotaryPartyParams.isReadonly ? t("notary.add_party.detail") : (NotaryPartyParams.editData ? t("notary.add_party.edit") : t("notary.add_party.add"))}
        </div>
        <div className="flex justify-end items-center gap-3 z-10 flex-1 pr-4">
          {NotaryPartyParams.isReadonly && (
            <>
              <div 
                className="flex items-center gap-1 text-text-sub cursor-pointer active:opacity-70"
                onClick={() => navigate(`/notary/party-signature/${NotaryPartyParams.editData?.id || 'party'}`)}
              >
                 <PenTool className="w-5 h-5" />
              </div>
              <Video
                className="w-6 h-6 text-primary-blue cursor-pointer"
                onClick={() => setShowVideoActionSheet(true)}
              />
            </>
          )}
        </div>
      </header>

      <div className="flex-1 overflow-y-auto pb-24 relative z-0">
        <div className="flex flex-col gap-2">
          {/* Section 1: Identity Verification */}
          <IdentityVerificationSection
            idFrontRef={idFrontRef}
            idBackRef={idBackRef}
            faceRef={faceRef}
            idFrontPreview={idFrontPreview}
            idBackPreview={idBackPreview}
            facePreview={facePreview}
            faceScore={faceScore}
            isScanning={isScanning}
            setIdFrontPreview={setIdFrontPreview}
            setIdBackPreview={setIdBackPreview}
            setFacePreview={setFacePreview}
            setFaceScore={setFaceScore}
            setFullscreenImage={setFullscreenImage}
            handleFileChange={handleFileChange}
            handleFaceChange={handleFaceChange}
            handleStartComparison={handleStartComparison}
          />

          {/* Section 2: Basic Info (Cell Layout) */}
          <div className={cn(NotaryPartyParams.isReadonly && "pointer-events-none cursor-default")}>
            <BasicInfoSection
              formData={formData}
              setFullPageEditor={setFullPageEditor}
              setTempDate={setTempDate}
              setPickerType={setPickerType}
            />
          </div>

          {/* Section 3: Accessories & Remarks */}
          <div className={cn(NotaryPartyParams.isReadonly && "pointer-events-none cursor-default")}>
            <AccessoriesRemarksSection
              formData={formData}
              attachments={attachments}
              setFullPageEditor={setFullPageEditor}
              setAttachments={setAttachments}
              attachmentRef={attachmentRef}
              handleAttachmentsChange={handleAttachmentsChange}
            />
          </div>
        </div>
      </div>

      {/* Fixed Bottom Operations */}
      <div className="fixed bottom-0 left-0 right-0 p-3 bg-bg-color border-t border-border-color pb-safe z-20 flex gap-3 shadow-[0_-4px_20px_rgba(0,0,0,0.03)] dark:shadow-none">
        {NotaryPartyParams.isReadonly ? (
          <button
            onClick={() => navigate(-1)}
            className="w-full h-12 rounded-xl font-bold text-[15px] flex items-center justify-center transition-opacity shadow-sm bg-primary-blue text-white active:scale-[0.98]"
          >
            {t("notary.add_party.back")}
          </button>
        ) : (
          <>
            <button
              onClick={() => navigate(-1)}
              className="flex-[1] h-12 rounded-xl font-bold text-[15px] flex items-center justify-center bg-active-bg text-text-main active:opacity-70 transition-opacity"
            >
              {t("notary.add_party.cancel")}
            </button>
            <button
              onClick={handleSave}
              className="flex-[2] h-12 rounded-xl font-bold text-[15px] flex items-center justify-center transition-opacity shadow-sm bg-primary-blue text-white active:scale-[0.98]"
            >
              {t("notary.add_party.save")}
            </button>
          </>
        )}
      </div>

      {/* Fullscreen Image Preview Overlay */}
      <NotaryFullscreenImageOverlay
        imageUrl={fullscreenImage}
        onClose={() => setFullscreenImage(null)}
      />

      {/* Full Page Editor Overlay */}
      {fullPageEditor && (
        <NotaryFullPageEditor
          field={fullPageEditor.field}
          title={fullPageEditor.title}
          placeholder={fullPageEditor.placeholder}
          value={fullPageEditor.value}
          isTextArea={fullPageEditor.isTextArea}
          inputType={fullPageEditor.inputType}
          onChange={(val) =>
            setFullPageEditor({ ...fullPageEditor, value: val })
          }
          onSave={() => {
            setFormData((prev) => ({
              ...prev,
              [fullPageEditor.field]: fullPageEditor.value,
            }));
            setFullPageEditor(null);
          }}
          onClose={() => setFullPageEditor(null)}
        />
      )}

      {/* Pickers Overlay */}
      <NotaryBottomPicker
        pickerType={pickerType}
        formData={formData}
        setFormData={setFormData}
        setPickerType={setPickerType}
      />

      {/* Video Call Action Sheet */}
      <ActionSheet 
        isOpen={showVideoActionSheet}
        options={videoOptions}
        onClose={() => setShowVideoActionSheet(false)}
        title={t("notary.add_party.video_call")}
      />
    </div>
  );
};
