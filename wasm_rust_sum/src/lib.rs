use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// serde_wasm_bindgen: It allows to convert Rust data types into native JavaScript types and vice versa.

use serde::Deserialize; 
use serde::Serialize; 
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




/*
O atributo #[serde(flatten)] em Rust é uma maneira de dizer ao Serde que, ao desserializar ou serializar, ele deve tratar um campo de uma estrutura como se fosse uma extensão da estrutura, em vez de tratá-lo como um campo aninhado. Esse atributo é útil quando você deseja combinar chaves ou objetos de forma que, ao invés de um objeto complexo, o Serde "desembrulhe" ou "achate" esses campos para torná-los parte do objeto pai.

Explicação do #[serde(flatten)]
No exemplo abaixo, a estrutura Entrada contém um campo dados que é um HashMap. Sem o #[serde(flatten)], o Serde trataria o campo dados como uma estrutura aninhada dentro de Entrada. No entanto, com o #[serde(flatten)], você está dizendo ao Serde para desestruturar esse campo e tratá-lo de forma plana, ou seja, colocar diretamente as chaves e valores dentro da estrutura Entrada.
*/

#[derive(Serialize, Deserialize)]
pub struct TokenizedInputValues {
    states: Vec<String>,
    initState: Vec<String>,
    finalStates: Vec<String>,
    inAlphabet: Vec<String>,
    auxAlphabet: Vec<String>,
    initSymbol: Vec<String>,
    blankSymbol: Vec<String>,
}


#[derive(Serialize, Deserialize)]
struct Entrada {
    #[serde(flatten)]
    dados: HashMap<String, HashMap<String, DadosDetalhados>>,
}

#[wasm_bindgen]
pub struct AlgumaEstrutura {
    valores: TokenizedInputValues,
    transitions: Entrada,
}

#[derive(Serialize, Deserialize)]
struct DadosDetalhados {
    next: String,
    error: u32,
}


#[wasm_bindgen]
impl AlgumaEstrutura {
    #[wasm_bindgen(constructor)]
    pub fn new(transicoes: JsValue, data: JsValue) -> AlgumaEstrutura {
        // Converte os parâmetros recebidos
        let valores: TokenizedInputValues = from_value(data).unwrap();
        let transitions: Entrada = from_value(transicoes).unwrap();

        AlgumaEstrutura {
            valores,
            transitions,
        }
    }


    #[wasm_bindgen]
    pub fn get_valores(&self) -> JsValue {
        // Converte a estrutura de volta para JsValue para ser usada no JS
        serde_wasm_bindgen::to_value(&self.valores).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_transitions(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.transitions).unwrap()
    }



}


/*
#[wasm_bindgen]
pub fn processar_transicoes(transitions: JsValue) -> String {
    let entrada: Entrada = from_value(transitions).unwrap();
    let mut resultado = String::new();  // Criando uma string para acumular o resultado

    for (estado, transicoes) in entrada.dados {
        resultado.push_str(&format!("Estado: {}\n", estado));  // Adicionando o nome do estado
        for (simbolo, dados) in transicoes {
            resultado.push_str(&format!("  Símbolo: {}, Próximo Estado: {}, Erro: {}\n", simbolo, dados.next, dados.error));
        }
    }

    resultado  // Retornando a string acumulada
}

#[wasm_bindgen]
pub fn processar_transicoes(transitions: JsValue) -> String {
    let entrada: Entrada = from_value(transitions).unwrap();
    let mut resultado = String::new();  // Criando uma string para acumular o resultado

    for (estado, transicoes) in entrada.dados {
        resultado.push_str(&format!("Estado: {}\n", estado));  // Adicionando o nome do estado
        for (simbolo, dados) in transicoes {
            resultado.push_str(&format!("  Símbolo: {}, Próximo Estado: {}, Erro: {}\n", simbolo, dados.next, dados.error));
        }
    }

    resultado  // Retornando a string acumulada
}

*/



