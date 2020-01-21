import { greet, build } from '../../pkg/wasm_tokenizers.js';

global.alert = (str) => {
  console.log(str);
};

greet('Mik');
build("/Users/mik/dev/mikbry/tokenizers/bindings/wasm/examples/data/gpt2-vocab.json",
  "/Users/mik/dev/mikbry/tokenizers/bindings/wasm/examples/data/gpt2-merges.txt");