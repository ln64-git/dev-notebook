import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function ChatFooter(): JSX.Element {
  const [inputValue, setInputValue] = useState<string>("");

  const handleInputChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInputValue(event.target.value);
  };

  const handleEnterSubmit = (
    event: React.KeyboardEvent<HTMLTextAreaElement>
  ) => {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      submitInput();
      setInputValue("");
    }
  };

  useEffect(() => {
    const textarea = document.getElementById("textarea") as HTMLTextAreaElement;
    if (!textarea) return;

    const adjustTextareaHeight = () => {
      textarea.style.height = "auto";
      textarea.style.height = textarea.scrollHeight + "px";
    };

    textarea.addEventListener("input", adjustTextareaHeight);
    textarea.dispatchEvent(new Event("input"));

    return () => {
      textarea.removeEventListener("input", adjustTextareaHeight);
    };
  }, []);

  const submitInput = () => {
    invoke("process_input_from_frontend", { text: inputValue });
  };

  return (
    <div className="w-full bg-opacity-60 px-1">
      <div className="max-w-[460px] mx-auto pt-1.5 pb-2  flex  ">
        <div className="max-w-[420px] mx-auto  opacity-60 rounded-md w-full flex justify-between">
          <textarea
            style={{
              resize: "none",
              overflow: "hidden",
              outline: "none",
              flex: "1",
            }}
            className="p-2.5 w-full text-sm rounded-lg bg-zinc-950 "
            rows={1}
            spellCheck={true}
            value={inputValue}
            onChange={handleInputChange}
            onKeyDown={handleEnterSubmit}
          ></textarea>
          <div className="flex items-center justify-center pr-2">
            <div className="transition duration-300 ease-in-out hover:bg-zinc-900 rounded-md cursor-pointer flex "></div>
          </div>
        </div>
      </div>
    </div>
  );
}
