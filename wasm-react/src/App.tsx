import React, { useEffect, useState } from "react";
import init, {AlgumaEstrutura} from "./pkg/wasm_rust_sum"; // Importa o módulo WASM

const estruturaArray = [
  { chave1: "chave1_1", chave2: "chave2_1", valor1: "valor1_1", valor2: "valor2_1" },
  { chave1: "chave1_1", chave2: "chave2_2", valor1: "valor1_2", valor2: "valor2_2" },
  { chave1: "chave1_2", chave2: "chave2_1", valor1: "valor1_3", valor2: "valor2_3" },
];


const App = () => {
  const [displayText, setDisplayText] = useState<string>("");
  
  useEffect(() =>  {
    
    init().then(() => {
    console.log("WASM loaded and ready");
    // Agora podemos usar as funcionalidades
  })}, []);

  const handleShowStruct = () => {
    const struct = new AlgumaEstrutura(estruturaArray);
    setDisplayText(struct.mostrar_dados());
  };

  const handleClean = () => {
    setDisplayText("");
  }

  return (
    <div>
      <h1>WASM + React</h1>

      <button onClick={handleShowStruct}>Mostrar estrutura</button>
      <button onClick={handleClean}>Limpar</button>
      <p>Estrutura: {displayText}</p>
    </div>
  );
};

export default App;
