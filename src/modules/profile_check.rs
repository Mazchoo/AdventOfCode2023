use wasm_bindgen::prelude::*;

// A struct similar to a JS object {a, b, c?}
#[wasm_bindgen]
pub struct Obj {
    pub a: i32,
    pub b: i32,
    pub c: Option<i32>, // optional field
}

#[wasm_bindgen]
impl Obj {
    #[wasm_bindgen(constructor)]
    pub fn new(a: i32, b: i32, c: Option<i32>) -> Obj {
        Obj { a, b, c }
    }
}

#[wasm_bindgen]
pub fn sum_fields(o: &Obj) -> i32 {
    // Equivalent to "o.a + o.b"
    o.a + o.b
}

#[wasm_bindgen]
pub fn run_warmup_a(iterations: i32) -> i32 {
    let mut total = 0;
    for _ in 0..iterations {
        let obj = Obj {
            a: 1,
            b: 2,
            c: None,
        }; // shape A: no "c"
        total += sum_fields(&obj);
    }
    total
}

#[wasm_bindgen]
pub fn run_warmup_b(iterations: i32) -> i32 {
    let mut total = 0;
    for _ in 0..iterations {
        let obj = Obj {
            a: 1,
            b: 2,
            c: Some(3),
        }; // shape B: has "c"
        total += sum_fields(&obj);
    }
    total
}
