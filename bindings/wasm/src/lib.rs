use serde_json::Value;
use std::error;
use std::fmt;
use std::{
  collections::HashMap,
  io,
  io::BufRead,
};
use tokenizers::models::bpe::BPE;
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::{AddedToken, EncodeInput, Tokenizer};
use wasm_bindgen::prelude::*;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  /// An error encountered while reading files mainly.
  Io(std::io::Error),
  JsonError(serde_json::Error),
  BuildError,
}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
      Error::Io(error)
  }
}

impl From<serde_json::Error> for Error {
  fn from(error: serde_json::Error) -> Self {
      Error::JsonError(error)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "build error")
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match self {
      Error::Io(e) => Some(e),
      Error::JsonError(e) => Some(e),
      _ => None,
    }
  }
}

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
pub fn start(name: &str) {
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

fn line_to_input(line: io::Result<String>) -> EncodeInput {
  EncodeInput::Single(line.unwrap())
}

fn build(vocab_src: &[u8], merges_src: &[u8]) -> Result<Tokenizer> {
  let mut vocab = HashMap::<String, u32>::new();
  let json: Value = serde_json::from_slice(&vocab_src).unwrap();
  match json {
    Value::Object(m) => {
      for (token, id) in m {
        if let Value::Number(id) = id {
          let id = id.as_u64().ok_or(Error::BuildError)? as u32;
          vocab.insert(token, id);
        }
      }
    }
    _ => return Err(Error::BuildError),
  };
  alert(&format!("vocab.len={}", vocab.len()));

  let mut merges = HashMap::<Pair, (u32, u32)>::new();
  //let merges_src = BufReader::new(merges_src);
  for (rank, line) in merges_src.lines().enumerate() {
    let line = line?;
    if line.starts_with("#version") {
      // Skip line with: #version
      continue;
    }

    let parts = line.split(' ').collect::<Vec<_>>();
    if parts.len() != 2 {
      return Err(Error::BuildError);
    }

    let a = vocab.get(parts[0]).ok_or_else(|| Error::BuildError)?;
    let b = vocab.get(parts[1]).ok_or_else(|| Error::BuildError)?;
    let pair = (*a, *b);
    let new_token = format!("{}{}", parts[0], parts[1]);
    let new_id = vocab.get(&new_token).ok_or(Error::BuildError)?;

    merges.insert(pair, (rank as u32, *new_id));
  }
  alert(&format!("merges.len={}", merges.len()));

  let bpe = BPE::new(vocab, merges);
  let tokenizer = create_gpt2_tokenizer(bpe);
  Ok(tokenizer)
}

#[wasm_bindgen]
pub fn bench(vocab: &[u8], merges: &[u8], _inputs: &[u8]) {
  alert("Bench starting !");
  let tokenizer = build(vocab, merges).unwrap();

  let encoding = tokenizer.encode(EncodeInput::Single("Hey there!".into())).unwrap();
  alert(&format!("tokens: {:?}", encoding.get_tokens()));
  alert("Bench done !");
}
