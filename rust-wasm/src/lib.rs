use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn saludar(nombre: &str) {
    alert(&format!("Hola, {}, ¿como estas?", nombre))   
}