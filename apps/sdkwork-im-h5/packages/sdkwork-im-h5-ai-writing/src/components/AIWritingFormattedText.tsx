import React from "react";

interface AIWritingFormattedTextProps {
  content: string;
}

export const AIWritingFormattedText: React.FC<AIWritingFormattedTextProps> = ({
  content,
}) => {
  if (!content) return null;

  return (
    <div className="text-[15px] text-text-main leading-relaxed font-sans flex flex-col gap-2 relative whitespace-pre-wrap break-words">
      {content.split("\n").map((line, i) => {
        const renderInlineText = (text: string) => {
          const parts = text.split(/(\*\*.*?\*\*|\*.*?\*)/g);
          return parts.map((part, idx) => {
            if (part.startsWith("**") && part.endsWith("**")) {
              return <strong key={idx}>{part.slice(2, -2)}</strong>;
            } else if (part.startsWith("*") && part.endsWith("*")) {
              return <em key={idx}>{part.slice(1, -1)}</em>;
            }
            return (
              <span
                key={idx}
                dangerouslySetInnerHTML={{ __html: part }}
              />
            );
          });
        };

        if (line.startsWith("# ")) {
          return (
            <h1 key={i} className="text-xl font-bold mt-4 mb-2">
              {renderInlineText(line.replace("# ", ""))}
            </h1>
          );
        } else if (line.startsWith("## ")) {
          return (
            <h2 key={i} className="text-lg font-bold mt-3 mb-1">
              {renderInlineText(line.replace("## ", ""))}
            </h2>
          );
        } else if (line.startsWith("### ")) {
          return (
            <h3 key={i} className="text-base font-bold mt-2 mb-1">
              {renderInlineText(line.replace("### ", ""))}
            </h3>
          );
        } else if (line.startsWith("*Conclusion*")) {
          return (
            <strong key={i} className="italic block mt-3 mb-1">
              Conclusion
            </strong>
          );
        } else if (line.match(/^(\d+\.|-)\s/)) {
          return (
            <p
              key={i}
              className="pl-4 relative before:content-['•'] before:absolute before:left-0 before:text-text-sub my-1"
            >
              {renderInlineText(line.replace(/^(\d+\.|-)\s/, ""))}
            </p>
          );
        } else if (line.trim() === "") {
          return <br key={i} />;
        }
        return (
          <p key={i} className="min-h-[1em] mb-2">
            {renderInlineText(line)}
          </p>
        );
      })}
    </div>
  );
};
