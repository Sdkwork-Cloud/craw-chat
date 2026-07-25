import React, { useState, useEffect } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { ChevronLeft, Folder, User as UserIcon, Search } from "lucide-react";
import { IconButton, Avatar } from "@sdkwork/im-h5-commons";
import { OrganizationService, type Organization, type OrgDepartment, type OrgMember } from "../services/OrganizationService";
import { useTranslation } from "react-i18next";

export const OrganizationDetail: React.FC = () => {
  const { t } = useTranslation();
const navigate = useNavigate();
  const { id: orgId } = useParams<{ id: string }>();
  const [searchParams] = useSearchParams();
  const deptId = searchParams.get("deptId");

  const [org, setOrg] = useState<Organization | null>(null);
  const [departments, setDepartments] = useState<OrgDepartment[]>([]);
  const [members, setMembers] = useState<OrgMember[]>([]);
  const [path, setPath] = useState<OrgDepartment[]>([]);
  const [loading, setLoading] = useState(true);
  const [isSearching, setIsSearching] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchResults, setSearchResults] = useState<OrgMember[]>([]);
  const [searching, setSearching] = useState(false);

  useEffect(() => {
    if (!orgId) return;
    setLoading(true);

    Promise.all([
      OrganizationService.getOrganizations(),
      OrganizationService.getDepartments(orgId, deptId || null),
      deptId ? OrganizationService.getMembers(orgId, deptId) : Promise.resolve([]),
      deptId ? OrganizationService.getDepartmentPath(deptId) : Promise.resolve([]),
    ]).then(([orgs, depts, mems, pth]) => {
      setOrg(orgs.find((o) => o.id === orgId) || null);
      setDepartments(depts);
      setMembers(mems);
      setPath(pth);
      setLoading(false);
    });
  }, [orgId, deptId]);

  useEffect(() => {
    if (!isSearching || !searchQuery.trim() || !orgId) {
      setSearchResults([]);
      return;
    }
    setSearching(true);
    const timer = setTimeout(() => {
      OrganizationService.searchMembers(orgId, searchQuery).then((results) => {
        setSearchResults(results);
        setSearching(false);
      });
    }, 500);
    return () => clearTimeout(timer);
  }, [searchQuery, isSearching, orgId]);

  const goToDept = (id: string) => {
  navigate(`/contacts/org/${orgId}?deptId=${id}`);
  };

  const goToRoot = () => {
  navigate(`/contacts/org/${orgId}`);
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="h-[52px] flex items-center justify-between px-2 bg-bg-color/90 backdrop-blur-md sticky top-0 z-20 shrink-0 pt-safe">
        {isSearching ? (
          <div className="flex items-center w-full px-2 gap-2 h-full">
            <div className="flex-1 bg-chat-other-bg h-9 rounded-md flex items-center px-3">
              <Search className="w-4 h-4 text-text-sub shrink-0" />
              <input
                type="text"
                autoFocus
                className="flex-1 bg-transparent border-none outline-none text-[15px] ml-2 text-text-main"
                placeholder={t('common.search')}
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
              />
            </div>
            <span
              className="text-[15px] text-primary-blue whitespace-nowrap pl-2"
              onClick={() => {
                setIsSearching(false);
                setSearchQuery("");
              }}
            >
              {t('common.cancel')}
            </span>
          </div>
        ) : (
          <>
            <div className="flex items-center z-10 min-w-[80px]">
              <IconButton
                icon={<ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />}
                onClick={() => navigate(-1)}
              />
            </div>
            <div className="flex-1 min-w-0 pr-12 text-center pointer-events-none flex flex-col items-center">
              <span className="font-semibold text-[17px] text-text-main truncate max-w-full">
                {org ? org.name : t('contacts.org_structure')}
              </span>
              {deptId && path.length > 0 && (
                <span className="text-[11px] text-text-sub truncate max-w-full">
                  {path[path.length - 1].name}
                </span>
              )}
            </div>
            <div className="flex items-center justify-end z-10 w-[80px] gap-1 pr-2 absolute right-0">
              <IconButton
                icon={<Search className="w-5 h-5 text-text-main" />}
                onClick={() => setIsSearching(true)}
              />
            </div>
          </>
        )}
      </header>

      {/* Breadcrumbs */}
      {!isSearching && (deptId !== null) && (
        <div className="px-4 py-3 bg-bg-color border-b border-border-color overflow-x-auto whitespace-nowrap no-scrollbar flex items-center gap-1.5 shrink-0">
          <span
            className="text-[14px] text-primary-blue cursor-pointer"
            onClick={goToRoot}
          >
            {org?.name}
          </span>
          {path.map((p, idx) => (
            <React.Fragment key={p.id}>
              <span className="text-text-sub/50 text-[12px]">/</span>
              <span
                className={`text-[14px] cursor-pointer ${idx === path.length - 1 ? "text-text-main font-medium" : "text-primary-blue"}`}
                onClick={() => goToDept(p.id)}
              >
                {p.name}
              </span>
            </React.Fragment>
          ))}
        </div>
      )}

      <div className="flex-1 overflow-y-auto w-full bg-chat-other-bg pb-10">
        {loading || searching ? (
          <div className="flex justify-center p-6 mt-10">
            <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin"></div>
          </div>
        ) : isSearching && searchQuery.trim() ? (
          <div className="flex flex-col bg-bg-color">
            {searchResults.length > 0 ? (
              searchResults.map((member, index) => (
                <div key={member.id} className="relative">
                  <div className="flex items-center gap-3 px-4 py-3 cursor-pointer active:bg-black/5 dark:active:bg-white/5">
                    <Avatar
                      src={member.avatar || ""}
                      alt={member.name}
                      fallback={member.name}
                      size="md"
                      className="rounded"
                    />
                    <div className="flex flex-col flex-1 min-w-0 justify-center">
                      <span className="text-[16px] text-text-main truncate font-medium">{member.name}</span>
                      {member.jobTitle && (
                        <span className="text-[13px] text-text-sub truncate mt-0.5">{member.jobTitle}</span>
                      )}
                    </div>
                  </div>
                  {index < searchResults.length - 1 && (
                    <div className="ml-16 border-b border-border-color" />
                  )}
                </div>
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub gap-2">
                <Search className="w-12 h-12 text-text-sub/30" />
                <span className="text-[14px]">{t('contacts.no_search_results', { defaultValue: 'No results found' })}</span>
              </div>
            )}
          </div>
        ) : (
          <div className="flex flex-col">
            {/* Departments */}
            {departments.length > 0 && (
              <div className="bg-bg-color mt-2">
                {departments.map((dept, index) => (
                  <div key={dept.id} className="relative">
                    <div
                      className="flex items-center justify-between px-4 py-3.5 cursor-pointer active:bg-black/5 dark:active:bg-white/5"
                      onClick={() => goToDept(dept.id)}
                    >
                      <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded bg-[#4395F5]/10 flex items-center justify-center shrink-0">
                          <Folder className="w-5 h-5 text-[#4395F5] fill-[#4395F5]" />
                        </div>
                        <span className="text-[16px] text-text-main">{dept.name}</span>
                      </div>
                      <div className="flex items-center gap-2">
                        {dept.count > 0 && (
                          <span className="text-[14px] text-text-sub">{t('contacts.people_count', { count: dept.count })}</span>
                        )}
                        <ChevronLeft className="w-5 h-5 text-text-sub/40 rotate-180" />
                      </div>
                    </div>
                    {index < departments.length - 1 && (
                      <div className="ml-16 border-b border-border-color" />
                    )}
                  </div>
                ))}
              </div>
            )}

            {/* Members */}
            {members.length > 0 && (
              <div className="bg-bg-color mt-2">
                <div className="px-4 py-2 border-b border-border-color">
                  <span className="text-[13px] text-text-sub">{t('contacts.org_members')} ({members.length})</span>
                </div>
                {members.map((member, index) => (
                  <div key={member.id} className="relative">
                    <div className="flex items-center gap-3 px-4 py-3 cursor-pointer active:bg-black/5 dark:active:bg-white/5">
                      <Avatar
                        src={member.avatar || ""}
                        alt={member.name}
                        fallback={member.name}
                        size="md"
                        className="rounded"
                      />
                      <div className="flex flex-col flex-1 min-w-0 justify-center">
                        <span className="text-[16px] text-text-main truncate font-medium">{member.name}</span>
                        {member.jobTitle && (
                          <span className="text-[13px] text-text-sub truncate mt-0.5">{member.jobTitle}</span>
                        )}
                      </div>
                    </div>
                    {index < members.length - 1 && (
                      <div className="ml-16 border-b border-border-color" />
                    )}
                  </div>
                ))}
              </div>
            )}

            {!loading && departments.length === 0 && members.length === 0 && (
               <div className="flex flex-col items-center justify-center py-20 text-text-sub gap-2">
                 <UserIcon className="w-12 h-12 text-text-sub/30" />
                 <span className="text-[14px]">{t('contacts.no_org_data')}</span>
               </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};
