"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Home() {
  const [options, setOptions] = useState<string[]>([]);
  const [selectedOption, setSelectedOption] = useState<string | null>(null);

  // get options from rust api fetch_options
  useEffect(() => {
    async function fetchOptions() {
      const response: string[] = await invoke("fetch_options");
      console.log(response);
      setOptions(response);
    }
    fetchOptions();
  }, []);

  return (
    <div>
      <div>
        <select onChange={(e) => setSelectedOption(e.target.value)}>
          {options.map((option) => (
            <option key={option} value={option}>
              {option}
            </option>
          ))}
        </select>
      </div>
      <div>
        <button
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full"
          onClick={() => {
            if (selectedOption) {
              console.log(selectedOption);
              invoke("kick_back", { select: selectedOption });
            }
          }}
        >
          Kick Back
        </button>
      </div>
    </div>
  );
}
