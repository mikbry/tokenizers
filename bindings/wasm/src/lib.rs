mod utils;
use rayon::prelude::*;
use std::collections::HashMap;
use tokenizers::models::bpe::BPE;
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::{AddedToken, EncodeInput, Tokenizer};
use wasm_bindgen::prelude::*;

// Not public in Tokenizers
type Pair = (u32, u32);

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
  alert(&format!("Hello, {}!", name));
}

fn create_gpt2_tokenizer(bpe: BPE) -> Tokenizer {
  let mut tokenizer = Tokenizer::new(Box::new(bpe));
  tokenizer.with_pre_tokenizer(Box::new(ByteLevel::new(true)));
  tokenizer.with_decoder(Box::new(ByteLevel::new(false)));
  tokenizer.add_tokens(&[
    AddedToken {
      content: String::from("ing"),
      single_word: false,
    },
    AddedToken {
      content: String::from("[ENT]"),
      single_word: true,
    },
  ]);
  tokenizer
}

#[wasm_bindgen]
pub fn build(vocab_src: &str, merge_src: &str) {
  alert("Build starting !");
  let mut vocab = HashMap::<String, u32>::new();
  vocab.insert("!".to_string(), 0);
	vocab.insert("\"".to_string(), 1);
	vocab.insert("#".to_string(), 2);
	vocab.insert("$".to_string(), 3);
	vocab.insert("%".to_string(), 4);
	vocab.insert("&".to_string(), 5);
	vocab.insert("'".to_string(), 6);
	vocab.insert("(".to_string(), 7);
  let mut merge = HashMap::<Pair, (u32, u32)>::new();
  // let bpe = BPE::from_files(vocab, merge).unwrap().build().unwrap();
  let bpe = BPE::new(vocab, merge);
  let tokenizer = create_gpt2_tokenizer(bpe);
  /* builder
  .dropout(0.1)
  .unk_token("[UNK]".into())
  .build(); */

  // let mut tokenizer = Tokenizer::new(Box::new(builder));

  // let encoding = tokenizer.encode(EncodeInput::Single("Hey there!".into()));
  // alert(encoding.get_tokens());
  alert("Build done !");
}
