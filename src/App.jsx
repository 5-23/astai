import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from "@tauri-apps/api/window";
import "./index.css";
import forms from '@tailwindcss/forms';

function App() {
  const [name, setName] = useState("");
  const [imagePath, setImagePath] = useState("");

  async function getImage() {
    return await invoke("get_image");
  }
  // await getImage();
  return (
    <>
    <button className="text-red-600" onClick={async () => {console.log(await getImage())}}>asdf</button>
    </>
  );
}

export default App;