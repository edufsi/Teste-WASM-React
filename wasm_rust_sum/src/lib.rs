use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WordCollection {
    words: Vec<String>,
}

#[wasm_bindgen]
impl WordCollection {
    #[wasm_bindgen(constructor)]
    pub fn new(words: Vec<String>) -> WordCollection {
        WordCollection { words }
    }

    pub fn display_words(&self) -> String {
        self.words.join(", ")
    }
}