use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::Deserialize; // Habilita a conversão de dados serializados (como JSON) em estruturas Rust.
use serde_wasm_bindgen::from_value; // Função que converte um JsValue (valor recebido do JavaScript) para uma estrutura Rust, desde que essa estrutura implemente Deserialize.

// Isso faz com que serde consiga automaticamente converter JSON para essa struct.
#[derive(Deserialize)]
struct Entrada {
    chave1: String,
    chave2: String,
    valor1: String,
    valor2: String,
}

#[wasm_bindgen]
pub struct AlgumaEstrutura {
    dados: HashMap<String, HashMap<String, (String, String)>>,
}

#[wasm_bindgen]
impl AlgumaEstrutura {
    #[wasm_bindgen(constructor)]
    pub fn new(data: JsValue) -> AlgumaEstrutura {
        // Converte JsValue para um vetor de structs Entrada
        let array: Vec<Entrada> = from_value(data).unwrap();

        let mut dados: HashMap<String, HashMap<String, (String, String)>> = HashMap::new();

        for item in array {
            let entry = dados.entry(item.chave1.clone()).or_insert_with(HashMap::new);
            entry.insert(item.chave2, (item.valor1, item.valor2));
        }

        AlgumaEstrutura { dados }
    }

    pub fn mostrar_dados(&self) -> String {
        let mut resultado = String::new();
        
        for (chave1, chaves2) in &self.dados {
            resultado.push_str(&format!("Chave1: {}\n", chave1));
            for (chave2, (valor1, valor2)) in chaves2 {
                resultado.push_str(&format!("  Chave2: {}, Valor1: {}, Valor2: {}\n", chave2, valor1, valor2));
            }
        }
    
        resultado
    }
}
