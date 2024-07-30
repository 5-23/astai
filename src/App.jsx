import { useRef, useState } from "react";
// import reactLogo from "../assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./styles/App.css"

export default () => {
  // const [name, setName] = useState("");
  const [folderPath, setFolderPath] = useState("");
  const [imgUrl, setImgUrl] = useState("");
  const [classNames, setClassNames] = useState([]);



  async function getImages() {
    return await invoke("get_images");
  }
  async function getImage(name) {
    return await invoke("get_image", { name });
  }
  async function getFolder() {
    return await invoke("get_folder");
  }
  async function getClass() {
    return await invoke("get_class");
  }


  const reload = async () => {
    let imgs = await getImages();
    let base64_img = await getImage(imgs[0])
    setImgUrl(base64_img)

    console.log(await getClass())
  }

  return (
    <main className="flex flex-col justify-between p-8 h-[100dvh]">

      <article className="bg-gray-800/45 text-white h-full  ">
        {folderPath}
        <section className="display-flex h-fit w-full">

          <img src={imgUrl} className="h-full object-fill " />
          <div className="w-[100px] bg-blue-300">
            {classNames.map((name, idx) => <div>
              <input id={`cls-name-{idx}`} value={name} />
              <input type="button" id={`remove-name-{idx}`} value={x} />
            </div>)}

          </div>
        </section>
      </article>

      <article>
        {folderPath ? <></> : <div className="absolute top-2/4 left-2/4 -translate-x-2/4  -translate-y-2/4">
          <input value="불러오기" type="button" className="px-2 py-1 cursor-pointer duration-300 bg-blue-500/80 hover:bg-blue-500 text-white rounded-md text-xm" onClick={async () => { setFolderPath(await getFolder()); await reload() }} />
        </div>
        }
      </article>
    </main>
  );
}
