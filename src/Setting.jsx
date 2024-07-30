import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default () => {
  return (
    <main className="w-full h-[100vh] px-20 py-10">
      <div className="flex w-full justify-between">
        <p>alpha</p>
        <input type="range" min={0} max={100} name="" id="" />
      </div>
    </main>
  );
}


