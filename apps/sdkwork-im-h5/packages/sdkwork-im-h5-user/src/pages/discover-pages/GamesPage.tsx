import { useTranslation } from "react-i18next";
import React from "react";
import { PageLayout } from "../../components/SettingsCommons";
import { Gamepad2, ChevronRight, Trophy, Flame, PlayCircle, Star, Search } from "lucide-react";
import { cn } from "@sdkwork/im-h5-commons";

export const GamesPage = () => {
  const { t } = useTranslation();
const GAMES = [
    {
      title: "跳一跳",
      icon: "https://picsum.photos/seed/g1/100",
      players: "1.2亿",
      tags: ["休闲", "竞技"],
      color: "text-[#F53F3F]",
      desc: "指尖悦动，挑战高分",
    },
    {
      title: "欢乐斗地主",
      icon: "https://picsum.photos/seed/g2/100",
      players: "5000万",
      tags: ["棋牌", "休闲"],
      color: "text-[#FF7D00]",
      desc: "全民经典，智力对决",
    },
    {
      title: "羊了个羊",
      icon: "https://picsum.photos/seed/g3/100",
      players: "1000万",
      tags: ["益智", "挑战"],
      color: "text-[#FABC14]",
      desc: "魔性消除，越玩越上头",
    },
    {
      title: "王者荣耀",
      icon: "https://picsum.photos/seed/g4/100",
      players: "2亿",
      tags: ["竞技", "动作"],
      color: "text-text-sub",
      desc: "5v5英雄公平对战",
    },
    {
      title: "和平精英",
      icon: "https://picsum.photos/seed/g5/100",
      players: "1.5亿",
      tags: ["射击", "生存"],
      color: "text-text-sub",
      desc: "反恐军事竞赛体验",
    },
  ];

  return (
    <PageLayout title={t('user.auto_prop_3394384d', '游戏中心')}>
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c] overflow-y-auto">
        <div className="p-4 bg-white dark:bg-[#2c2d2e] sticky top-0 z-10">
          <div className="bg-black/5 dark:bg-white/5 rounded-full flex items-center px-4 py-2">
            <Search className="w-4 h-4 text-text-sub" />
            <input 
              className="bg-transparent border-none outline-none ml-2 text-[14px] flex-1 text-text-main"
              placeholder={t('user.auto_prop_7075fb36', '搜索热门游戏...')}
            />
          </div>
        </div>

        <div className="p-4 flex flex-col gap-4">
          <div className="w-full h-36 bg-gradient-to-br from-indigo-500 via-purple-500 to-pink-500 rounded-xl flex flex-col items-start justify-center p-6 text-white relative overflow-hidden shadow-sm">
            <div className="absolute top-0 right-0 p-2 opacity-20 transform translate-x-4">
              <Gamepad2 className="w-40 h-40" />
            </div>
            <span className="px-2 py-0.5 bg-white/20 text-white rounded text-[11px] font-medium mb-2 relative z-10 backdrop-blur-sm">{t('user.auto_35a8c8c5', '独家首发')}</span>
            <h2 className="text-[20px] font-bold mb-1 relative z-10">{t('user.auto_2b5b246', '极品飞车：街头狂飙')}</h2>
            <p className="text-[13px] opacity-90 relative z-10 mb-3">{t('user.auto_48493b5e', '次世代画质，体验极致竞速')}</p>
            <button className="px-4 py-1.5 bg-white text-purple-600 rounded-full text-[13px] font-bold relative z-10 active:scale-95 transition-transform">{t('user.auto_3916c4b1', '立即开玩')}</button>
          </div>

          <div className="grid grid-cols-4 gap-3 bg-white dark:bg-[#2c2d2e] p-4 rounded-xl shadow-sm">
             {[
               { icon: <Flame className="w-6 h-6 text-red-500"/>, label: "热游榜" },
               { icon: <Trophy className="w-6 h-6 text-yellow-500"/>, label: "新游榜" },
               { icon: <Star className="w-6 h-6 text-orange-500"/>, label: "必玩推荐" },
               { icon: <PlayCircle className="w-6 h-6 text-blue-500"/>, label: "全部游戏" },
             ].map((menu, i) => (
                <div key={i} className="flex flex-col items-center gap-2 cursor-pointer active:scale-95 transition-transform">
                   <div className="w-12 h-12 rounded-full border border-black/5 dark:border-white/5 bg-black/5 dark:bg-white/5 flex items-center justify-center">
                     {menu.icon}
                   </div>
                   <span className="text-[12px] font-medium text-text-main">{menu.label}</span>
                </div>
             ))}
          </div>

          <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 shadow-sm">
            <div className="flex justify-between items-center mb-5">
              <h3 className="font-bold text-text-main text-[16px] flex items-center">
                <Flame className="w-5 h-5 mr-1 text-red-500" />{t('user.auto_3c919192', '热玩小游戏榜')}</h3>
              <span className="text-[13px] text-text-sub flex items-center cursor-pointer">{t('user.auto_310602ec', '查看更多')}<ChevronRight className="w-4 h-4 ml-0.5" />
              </span>
            </div>

            <div className="flex flex-col gap-5">
              {GAMES.map((game, i) => (
                <div
                  key={i}
                  className="flex items-center gap-3 active:scale-95 transition-transform cursor-pointer relative"
                >
                  <span
                    className={cn(
                      "font-bold text-[18px] w-5 text-center shrink-0 italic",
                      game.color
                    )}
                  >
                    {i + 1}
                  </span>
                  <img
                    src={game.icon}
                    className="w-14 h-14 rounded-xl border border-black/5 dark:border-white/5"
                    alt={game.title}
                  />
                  <div className="flex-1 flex flex-col justify-center overflow-hidden">
                    <h4 className="font-bold text-text-main text-[15px] mb-0.5 truncate">
                      {game.title}
                    </h4>
                    <span className="text-[12px] text-text-sub mb-1 truncate">{game.desc}</span>
                    <div className="flex items-center gap-2 text-[11px] text-text-sub">
                      <span>{t('user.auto_58da3735', '{game.players} 在玩')}</span>
                    </div>
                  </div>
                  <button className="px-4 py-1.5 bg-blue-50 dark:bg-blue-500/10 text-primary-blue font-bold rounded-full text-[13px] shrink-0 border border-blue-100 dark:border-blue-500/20 shadow-sm active:scale-95">{t('user.auto_1bc1312', '玩一玩')}</button>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </PageLayout>
  );
};
