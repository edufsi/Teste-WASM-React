import React, { useEffect, useState } from "react";
import init, { WordCollection } from "./pkg/wasm_rust_sum"; // Importa o módulo WASM

const App = () => {
  const [words, setWords] = useState<string[]>([]);
  const [displayText, setDisplayText] = useState<string>("");
  
  useEffect(() =>  {
    
    init().then(() => {
    console.log("WASM loaded and ready");
    // Agora podemos usar as funcionalidades
  })}, []);

  const handleAddWord = (event: React.FormEvent) => {
    event.preventDefault();
    const input = (event.target as HTMLFormElement).word.value;
    if (input.trim() !== "") {
      setWords([...words, input.trim()]);
    }
    (event.target as HTMLFormElement).reset();
  };

  const handleGenerateText = () => {
    const wordCollection = new WordCollection(words);
    setDisplayText(wordCollection.display_words());
  };

  return (
    <div>
      <h1>WASM + React</h1>
      <form onSubmit={handleAddWord}>
        <input type="text" name="word" placeholder="Digite uma palavra" />
        <button type="submit">Adicionar</button>
      </form>
      <button onClick={handleGenerateText}>Mostrar palavras</button>
      <p>Palavras: {displayText}</p>
    </div>
  );
};

export default App;
