import { useRef, useState } from "react";
// import reactLogo from "../assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import "./App.css";

function App() {
  const [name, setName] = useState("");
  const [imagePath, setImagePath] = useState("");
  
  const folderPath = useRef();
  async function getImages() {
    return await invoke("get_images");
  }
  async function getImage() {
    return await invoke("get_image");
  }
  return (
    <div className="bg-red-400">aaa</div>
    // <main className="flex flex-col justify-between p-8 h-[100dvh]">
    //   <article>
    //     <img src="" className="block bg-gray-200 min-h-100 min-w-230"/>
    //   </article>

    //   <article>
    //     <div className="flex gap-4 justify-between w-full">
    //       {/* <input value="불러오기" type="file" accept="png" ref={folderPath} className="flex-1 duration-300 bg-violet-600/80 hover:bg-violet-600 text-white py-3 rounded-md text-xm"/> */}
    //       <input value="불러오기" type="button" className="flex-1 duration-300 bg-violet-600/80 hover:bg-violet-600 text-white py-3 rounded-md text-xm" onClick={async () => {console.log(await getImages())}} />
    //     </div>
    //   </article>
    // </main>
  );
}

export default App;

