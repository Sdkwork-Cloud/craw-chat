import React, { useState, useEffect } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";
import { ChevronLeft, User as UserIcon, Search } from "lucide-react";
import { IconButton } from "@sdkwork/im-h5-commons";
import { OrganizationService, type Organization, type OrgDepartment, type OrgMember } from "../services/OrganizationService";
import { useTranslation } from "react-i18next";
import { OrgBreadcrumbs } from "../components/OrgBreadcrumbs";
import { OrgDepartmentList } from "../components/OrgDepartmentList";
import { OrgMemberList } from "../components/OrgMemberList";
import { OrgSearchResults } from "../components/OrgSearchResults";

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
              className="text-[15px] text-primary-blue whitespace-nowrap pl-2 cursor-pointer"
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
        <OrgBreadcrumbs
          org={org}
          path={path}
          onGoToRoot={goToRoot}
          onGoToDept={goToDept}
        />
      )}

      <div className="flex-1 overflow-y-auto w-full bg-chat-other-bg pb-10">
        {loading || searching ? (
          <div className="flex justify-center p-6 mt-10">
            <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin"></div>
          </div>
        ) : isSearching && searchQuery.trim() ? (
          <OrgSearchResults searchResults={searchResults} t={t} />
        ) : (
          <div className="flex flex-col">
            {/* Departments */}
            <OrgDepartmentList
              departments={departments}
              t={t}
              onGoToDept={goToDept}
            />

            {/* Members */}
            <OrgMemberList members={members} t={t} />

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

