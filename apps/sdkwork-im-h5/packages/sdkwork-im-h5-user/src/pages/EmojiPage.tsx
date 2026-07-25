import { useTranslation } from "react-i18next";
import React from "react";
import { showToast } from "@sdkwork/im-h5-commons";
import { PageLayout } from "../components/PageLayout";

export const EmojiPage = () => {
  const { t } = useTranslation();
const PACKS = [
    {
      title: "打工人的日常",
      author: "小李打工记",
      img: "https://picsum.photos/seed/e1/100",
    },
    {
      title: "萌宠猫咪大赏",
      author: "喵星人俱乐部",
      img: "https://picsum.photos/seed/e2/100",
    },
    {
      title: "社交悍匪专用包",
      author: "社牛局",
      img: "https://picsum.photos/seed/e3/100",
    },
  ];
  return (
    <PageLayout title={t('user.auto_prop_10e55d', '表情')}>
      <div className="flex border-b border-border-color sticky top-0 bg-bg-color z-10 w-full">
        <div className="flex-1 py-3 text-center text-primary-blue border-b-2 border-primary-blue font-medium text-[15px]">{t('user.auto_3ae1f9a8', '精选表情')}</div>
        <div className="flex-1 py-3 text-center text-text-main text-[15px] opacity-70">{t('user.auto_302c2483', '更多表情')}</div>
      </div>
      <div className="flex-1 overflow-y-auto w-full p-4">
        <h3 className="font-bold text-text-main text-lg mb-4">{t('user.auto_35949c83', '热门推荐')}</h3>
        <div className="flex flex-col gap-5 w-full">
          {PACKS.map((pack, i) => (
            <div
              key={i}
              className="flex items-center gap-3 active:scale-95 transition-transform cursor-pointer"
              onClick={() => showToast(`已添加：${pack.title}`)}
            >
              <img
                src={pack.img}
                className="w-16 h-16 rounded-xl border border-border-color object-cover"
              />
              <div className="flex-1">
                <h4 className="font-bold text-text-main text-base mb-1">
                  {pack.title}
                </h4>
                <p className="text-[12px] text-text-sub">@{pack.author}</p>
              </div>
              <button className="px-4 py-1.5 h-8 bg-black/5 dark:bg-white/10 text-primary-blue font-medium rounded-full text-[13px] shrink-0 border border-primary-blue/20">{t('user.auto_da405', '添加')}</button>
            </div>
          ))}
        </div>
      </div>
    </PageLayout>
  );
};
