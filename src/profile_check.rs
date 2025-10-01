use wasm_bindgen::prelude::*;

// Expose sum_fields to JS/WASM runtime
#[wasm_bindgen]
pub fn sum_fields(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn run_warmup_a(iterations: i32) -> i32 {
    let mut total = 0;
    for _ in 0..iterations {
        // "make_a" equivalent: always same shape
        let a = 1;
        let b = 2;
        total += sum_fields(a, b);
    }
    total
}

#[wasm_bindgen]
pub fn run_warmup_b(iterations: i32) -> i32 {
    let mut total = 0;
    for _ in 0..iterations {
        // "make_b" equivalent: here we *could* add an extra field in JS,
        // but Rust’s struct layout is fixed. So we just ignore "c".
        let a = 1;
        let b = 2;
        let _c = 3;
        total += sum_fields(a, b);
    }
    total
}
