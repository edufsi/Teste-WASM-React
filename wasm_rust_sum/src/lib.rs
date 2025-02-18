use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// serde_wasm_bindgen: It allows to convert Rust data types into native JavaScript types and vice versa.

use serde::Deserialize; 
use serde_wasm_bindgen::from_value;


/*
O que é Serialização e Desserialização?
Serialização é o processo de converter um objeto em memória (como um HashMap, uma struct, um Vec<T>, etc.) para um formato que possa ser armazenado ou transmitido, como JSON, XML, BSON, etc.
Desserialização é o processo inverso: pegar um dado serializado e reconstruir um objeto na memória.

No caso do serde_wasm_bindgen, estaremos desserializando um JsValue e serializando as estruturas de Rust para JsValue

O que é um JsValue?
é um tipo genérico que representa qualquer valor JavaScript dentro do Rust quando usamos WASM (WebAssembly) com a wasm-bindgen. O JsValue é um wrapper para valores do JavaScript no Rust.
Ele não é um JSON, mas pode conter um JSON dentro dele, pois JSON é um subconjunto dos objetos JavaScript.


*/

#[derive(Deserialize)]
struct Entrada {
    #[serde(flatten)]
    dados: HashMap<String, HashMap<String, DadosDetalhados>>,
}

#[derive(Deserialize)]
struct DadosDetalhados {
    next: String,
    error: u32,
}

#[wasm_bindgen]
pub fn processar_entrada(data: JsValue) -> String {
    let entrada: Entrada = from_value(data).unwrap();
    let mut resultado = String::new();  // Criando uma string para acumular o resultado

    for (estado, transicoes) in entrada.dados {
        resultado.push_str(&format!("Estado: {}\n", estado));  // Adicionando o nome do estado
        for (simbolo, dados) in transicoes {
            resultado.push_str(&format!("  Símbolo: {}, Próximo Estado: {}, Erro: {}\n", simbolo, dados.next, dados.error));
        }
    }

    resultado  // Retornando a string acumulada
}
