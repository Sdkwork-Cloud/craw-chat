import { useTranslation } from "react-i18next";
import React from "react";
import {
  cn,
  PageLayout as CommonsPageLayout,
  ListItem as CommonsListItem,
  Switch as CommonsSwitch,
} from "@sdkwork/im-h5-commons";

export const ListItem = CommonsListItem;
export const Switch = CommonsSwitch;

export const PageLayout = ({
  title,
  children,
  rightElement = null,
}: {
  title?: string;
  children: React.ReactNode;
  rightElement?: React.ReactNode;
}) => {
  const { t } = useTranslation();
return (
    <CommonsPageLayout title={title} rightElement={rightElement}>
      <div className="flex flex-col pb-12 mt-2">{children}</div>
    </CommonsPageLayout>
  );
};

export const Group = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => {
  const { t } = useTranslation();
  
  return (
  <div
    className={cn(
      "mb-2 border-y border-border-color/60 flex flex-col bg-chat-other-bg",
      className,
    )}
  >
    {children}
  </div>
);
};


export const ToggleItem = ({
  label,
  checked,
  onChange,
  hideBorder,
}: {
  label: React.ReactNode;
  checked: boolean;
  onChange: (v: boolean) => void;
  hideBorder?: boolean;
}) => (
  <ListItem
    label={label}
    hideBorder={hideBorder}
    rightElement={
      <Switch
        checked={checked}
        onChange={onChange}
        checkedColor="bg-[#00B42A]"
      />
    }
  />
);
