import React, { useEffect, useState } from "react";
import init, {AlgumaEstrutura} from "./pkg/wasm_rust_sum"; // Importa o módulo WASM


export interface Transitions {
  [state: string]: {
    [symbol: string] : {
      next: string;
      error: number;
    };
  };
}

export interface TokenizedInputValues {
  states: string[];
  initState: string[];
  finalStates: string[];
  inAlphabet: string[];
  auxAlphabet: string[];
  initSymbol: string[];
  blankSymbol: string[];
}

const App = () => {
  const transitions: Transitions = {
    "q0": {
      "a": { next: "q1", error: 0 },
      "b": { next: "q2", error: 1 }
    },
    "q1": {
      "a": { next: "q0", error: 0 },
      "b": { next: "q3", error: 2 }
    },
    "q2": {
      "a": { next: "q3", error: 1 },
      "b": { next: "q0", error: 0 }
    },
    "q3": {
      "a": { next: "q2", error: 2 },
      "b": { next: "q1", error: 1 }
    }
  };

  const tokenizedInputValues: TokenizedInputValues = {
    states: ["q0", "q1", "q2", "q3"],
    initState: ["q0"],
    finalStates: ["q3"],
    inAlphabet: ["a", "b"],
    auxAlphabet: ["c", "d"],
    initSymbol: ["#"],
    blankSymbol: ["_"]
  };
  
  useEffect(() =>  {
    
    init().then(() => {
    console.log("WASM loaded and ready");
    // Agora podemos usar as funcionalidades
  })}, []);

  const handleGenerateText = () => {
    const novaEstrutura = new AlgumaEstrutura(transitions, tokenizedInputValues);
    console.log("Dados:")
    console.log(novaEstrutura.get_transitions());
    console.log(novaEstrutura.get_valores());
    console.log("--------------------")
  };

  return (
    <div>
      <h1>WASM + React</h1>
      <button onClick={handleGenerateText}>Mostrar estruturas</button>
    </div>
  );
};

export default App;
