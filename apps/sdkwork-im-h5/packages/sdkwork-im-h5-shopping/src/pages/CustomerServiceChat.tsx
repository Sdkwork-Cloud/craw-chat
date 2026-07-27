import { useTranslation } from "react-i18next";
import React, { useState, useEffect, useRef } from "react";
import { useNavigate, useParams } from "react-router";
import {
  ChevronLeft,
  MoreHorizontal,
  Phone,
  Image as ImageIcon,
  Send,
  Smile,
  PlusCircle,
  Bot,
  User,
} from "lucide-react";
import { IconButton, cn, showToast } from "@sdkwork/im-h5-commons";
import { ProductService } from "../services/ProductService";
import { Shop, CustomerServiceMessage } from "../types";

export const CustomerServiceChat = () => {
  const { t } = useTranslation();
const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [shop, setShop] = useState<Shop | null>(null);
  const [messages, setMessages] = useState<CustomerServiceMessage[]>([]);
  const [inputValue, setInputValue] = useState("");
  const [isAiMode, setIsAiMode] = useState(true);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (id) {
      ProductService.getShopById(id).then(setShop);
      ProductService.getCustomerServiceMessages(id).then(msgs => setMessages([...msgs]));
    }
  }, [id]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSend = () => {
  if (!inputValue.trim() || !id) return;
    const userMsg: CustomerServiceMessage = {
      id: `msg_${Date.now()}`,
      content: inputValue,
      senderId: "user_1", // Current user
      senderType: "user",
      timestamp: Date.now(),
    };
    ProductService.sendCustomMessage(id, userMsg);
    setMessages((prev) => [...prev, userMsg]);
    setInputValue("");

    // Mock reply
    setTimeout(() => {
      const replyMsg: CustomerServiceMessage = {
        id: `msg_${Date.now() + 1}`,
        content: isAiMode
          ? `[智能客服] 您好，我已经收到您的问题：“${userMsg.content}”。我是AI助手，正在为您查询相关信息，请稍候...`
          : `[人工客服] 您好，请问有什么问题需要人工介入处理？`,
        senderId: isAiMode ? "agent" : "human_agent",
        senderType: isAiMode ? "agent" : "human",
        timestamp: Date.now() + 100,
      };
      ProductService.sendCustomMessage(id, replyMsg);
      setMessages((prev) => [...prev, replyMsg]);
    }, 1000);
  };

  if (!shop) {
    return (
      <div className="flex flex-col h-full bg-[#EDEDED] dark:bg-bg-color items-center justify-center pt-safe">
        <div className="w-8 h-8 border-4 border-primary-blue/30 border-t-primary-blue rounded-full animate-spin mb-4" />
        <span className="text-text-sub text-[14px]">{t('shopping.auto_1f8acd28', '连接中...')}</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#EDEDED] dark:bg-bg-color">
      {/* Header */}
      <header className="flex items-center px-2 pt-safe h-[56px] bg-bg-color border-b border-border-color shrink-0 sticky top-0 z-10 transition-colors">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <div
          className="flex-1 flex flex-col items-center justify-center min-w-0 px-2 cursor-pointer"
          onClick={() => navigate(`/shop/${shop.id}`)}
        >
          <div className="flex items-center gap-1.5 w-full justify-center">
            <span className="text-[17px] font-medium text-text-main truncate max-w-[150px]">{t('shopping.auto_65977f80', '{shop.name}客服')}</span>
            {isAiMode ? (
              <span className="bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400 text-[10px] px-1.5 py-0.5 rounded-sm shrink-0 flex items-center gap-1">
                <Bot className="w-3 h-3" /> AI
              </span>
            ) : (
              <span className="bg-green-100 text-green-600 dark:bg-green-900/30 dark:text-green-400 text-[10px] px-1.5 py-0.5 rounded-sm shrink-0 flex items-center gap-1">
                <User className="w-3 h-3" />{t('shopping.auto_9e66b', '人工')}</span>
            )}
          </div>
        </div>
        <IconButton
          icon={
            isAiMode ? (
              <User className="w-5 h-5 text-text-main" />
            ) : (
              <Bot className="w-5 h-5 text-text-main" />
            )
          }
          onClick={() => setIsAiMode(!isAiMode)}
        />
      </header>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto px-4 py-4 flex flex-col gap-4">
        {messages.map((msg) => {
          const isMe = msg.senderType === "user";
          return (
            <div
              key={msg.id}
              className={cn(
                "flex w-full gap-3",
                isMe ? "justify-end" : "justify-start",
              )}
            >
              {!isMe && (
                <img
                  src={shop.logo}
                  className="w-10 h-10 rounded-full object-cover shrink-0 border border-border-color/20"
                />
              )}
              <div
                className={cn(
                  "max-w-[70%] rounded-xl px-4 py-2.5 text-[16px] break-words leading-relaxed",
                  isMe
                    ? "bg-chat-me-bg text-[#111111] dark:text-white rounded-tr-sm shadow-sm"
                    : "bg-chat-other-bg text-text-main rounded-tl-sm shadow-sm",
                )}
              >
                {msg.content}
              </div>
            </div>
          );
        })}
        <div ref={messagesEndRef} />
      </div>

      {/* Bottom Input */}
      <div className="bg-bg-color border-t border-border-color pb-safe shrink-0">
        <div className="flex items-center px-3 py-2 min-h-[56px] gap-2">
          <IconButton
            icon={<Phone className="w-[26px] h-[26px] text-text-main" />}
            onClick={() => navigate(`/call/voice/${shop.id}`)}
          />
          <div className="flex-1 bg-chat-other-bg rounded-lg min-h-[40px] flex items-center px-1">
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-[16px] px-2 text-text-main h-full w-full placeholder:text-text-sub/50"
              value={inputValue}
              onChange={(e) => setInputValue(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSend()}
            />
          </div>
          {inputValue ? (
            <button
              className="bg-[#07C160] text-white px-4 h-9 rounded text-[14px] font-medium active:scale-95 transition-transform shrink-0"
              onClick={handleSend}
            >{t('shopping.auto_ab650', '发送')}</button>
          ) : (
            <>
              <IconButton
                icon={<Smile className="w-[26px] h-[26px] text-text-main" />}
              />
              <IconButton
                icon={
                  <PlusCircle className="w-[26px] h-[26px] text-text-main" />
                }
              />
            </>
          )}
        </div>
      </div>
    </div>
  );
};
