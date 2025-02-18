import React, { useEffect, useState } from "react";
import init, {processar_entrada} from "./pkg/wasm_rust_sum"; // Importa o módulo WASM


export interface Transitions {
  [state: string]: {
    [symbol: string] : {
      next: string;
      error: number;
    };
  };
}

const transitions:Transitions = {q0:{"@":{next:"", error:0}, "-":{next:"", error:0}}}

const App = () => {
  
  useEffect(() =>  {
    
    init().then(() => {
    console.log("WASM loaded and ready");
    // Agora podemos usar as funcionalidades
  })}, []);

  const handleGenerateText = () => {
    console.log(processar_entrada(transitions));
  };

  return (
    <div>
      <h1>WASM + React</h1>
      <button onClick={handleGenerateText}>Mostrar palavras</button>
    </div>
  );
};

export default App;
