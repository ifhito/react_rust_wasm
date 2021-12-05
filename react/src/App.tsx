import React, { useState, useEffect } from 'react';
import "./styles/global.css";
import Webasm from './Components/Wasm';
const App = () => {

  return (
    <div id="flexContents">
      <Webasm />
    </div>
  );
}

export default App;
