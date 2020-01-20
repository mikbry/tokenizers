mod utils;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;
use tokenizers::tokenizer::{Tokenizer, EncodeInput};
use tokenizers::models::bpe::BPE;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn build(vocab: &str) {
  let bpe_builder = BPE::new("./path/to/vocab.json");
  let bpe = bpe_builder
  .dropout(0.1)
  .unk_token("[UNK]".into())
  .build()?;

  let mut tokenizer = Tokenizer::new(Box::new(bpe));

  let encoding = tokenizer.encode(EncodeInput::Single("Hey there!".into()))?;
  alert(encoding.get_tokens());
}