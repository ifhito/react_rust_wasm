import React, { useState, useEffect } from 'react';
import styles from "../styles/Wasm.module.css";
import type * as WASM from "../../../wasm-build/wasm";
type WasmCreateGlitch = typeof WASM.create_glitch;
const Webasm= () => {
  const [imageFileUrl, setImageFileUrl] = useState(new Blob());
  const [imageFlag, setImageFlag] = useState(false);
  const [WASM, setWASM]:[any, any] = useState();
  const handleClickCreateGlitch = async () => {
        const blob = await createGlitch(imageFileUrl, "png", WASM.create_glitch);
        console.log('test',blob);
        setImageFileUrl(blob);
  }
  const processImage = async (event:any) => {
    const imageFile = event.target.files[0];
    const imageUrl = URL.createObjectURL(imageFile);
    const resp = await fetch(imageUrl);
    const b = await resp.blob();
    setImageFileUrl(b);
    setImageFlag(true);
  }
  useEffect(() => {
      const getWasm = async () => {
          const wasm = await import("wasm");
          setWASM(wasm);
      }
      getWasm();
  },[]);
  
  return (
    <div className={styles.flexContentsBox}>
      {imageFlag?
      <>
        <img className={styles.flexContentsBox__images}src={URL.createObjectURL(imageFileUrl)} />
        <button className={styles.flexContentsBox__button} onClick={() => handleClickCreateGlitch()}>グリッチ化する</button>
      </>
      :<input className={styles.flexContentsBox__input} type="file" accept="image/*" onChange={processImage}></input>}
    </div>
  );
}

const createGlitch = async (file:Blob,format:string, wasm:WasmCreateGlitch) => {
    const arr = new Uint8Array(await file.arrayBuffer());
    const result =wasm(arr, format);
    const blob = new Blob([result]);
    return blob;
}
export default Webasm;
