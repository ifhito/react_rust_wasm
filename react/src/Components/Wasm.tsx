import React, { useState, useEffect } from 'react';
import styles from "../styles/Wasm.module.css";
import type * as WASM from "../../../wasm-build/wasm";
// type WasmResize = typeof WASM.resize_image;
type WasmAddFont = typeof WASM.add_font_image;
type WasmCreateGlitch = typeof WASM.create_glitch;
const Webasm= () => {
  const useInput = (initialVal:string) => {
      const [value, set] = useState(initialVal);

      return {value, onChange: (e:any) => set(e.target.value)}
  }
  const fontTextProps = useInput("表示したいテキストを入力して下さい");
  const fontLocationUpProps = useInput("テキストの上からの位置を入力して下さい(数値)");
  const fontLocationLeftProps = useInput("テキストの左からの位置を入力して下さい(数値)");
  const fontScaleProps = useInput("テキストのスケールを入力して下さい(数値)")
  const [image,setImage] = useState(new Blob());
  const [imageFileUrl, setImageFileUrl] = useState(new Blob());
  const [imageFlag, setImageFlag] = useState(false);
  const [text, setText] = useState("");
  const [WASM, setWASM]:[any, any] = useState();
  const handleClickAddFont = async () => {
    //   const url = `${process.env.PUBLIC_URL}/logo192.png`;
    //   const blob = await resizeImageWasm(imageFileUrl, 7.0,70, "png", WASM.resize_image);
      const blob = await addFontImage(imageFileUrl, parseInt(fontScaleProps.value), parseInt(fontLocationUpProps.value), parseInt(fontLocationLeftProps.value), fontTextProps.value, "png", WASM.add_font_image);
      console.log('test',blob);
      setImageFileUrl(blob);
  }
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
  const handleChangeText = (e:any) => {
      setText(e.target.value);
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
      {/* <img src={URL.createObjectURL(image)}/> */}
      
      {imageFlag?
                    <><img className={styles.flexContentsBox__images}src={URL.createObjectURL(imageFileUrl)} />
                    {[fontTextProps,fontLocationUpProps,fontLocationLeftProps,fontScaleProps].map((element, i) => {
                        return <input key={i} className={styles.flexContentsBox__input} type="text" {...element} />
                    })}
                    <button className={styles.flexContentsBox__button} onClick={() => handleClickAddFont()}>画像にフォントを追加する</button>
                    <button className={styles.flexContentsBox__button} onClick={() => handleClickCreateGlitch()}>グリッチ化する</button></>
      :<input className={styles.flexContentsBox__input} type="file" accept="image/*" onChange={processImage}></input>}
      
    </div>
  );
}

const addFontImage = async (file:Blob, scale_height:number, pixels_from_up:number, pixels_from_left:number, text:string, format: string, wasm:WasmAddFont) => {
    const arr = new Uint8Array(await file.arrayBuffer());
    const result =wasm(arr, scale_height, pixels_from_up, pixels_from_left, text, format);
    const blob = new Blob([result]);
    return blob;
    
}

const createGlitch = async (file:Blob,format:string, wasm:WasmCreateGlitch) => {
    const arr = new Uint8Array(await file.arrayBuffer());
    const result =wasm(arr, format);
    const blob = new Blob([result]);
    return blob;
}
// async function resizeImageWasm(file: Blob, width:number, height: number, format: string, wasm:WasmAddFont): Promise<Blob>{
//   console.log(`Original: ${file.size} Bytes`);
//   const arr = new Uint8Array(await file.arrayBuffer());

//   const result = wasm(arr, width, height, format);

//   const blob = new Blob([result]);
//   console.log(`Resized: ${blob.size} Bytes`);

//   return blob;
// }
export default Webasm;
