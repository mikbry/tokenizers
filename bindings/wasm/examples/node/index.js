import fs from 'fs';
import { start, bench } from '../../pkg/wasm_tokenizers.js';

const fsp = fs.promises;

global.alert = (str) => {
  console.log(str);
};

(async () => {
  start('Mik');
  const vocab = await fsp.readFile("../data/gpt2-vocab.json");
  const merges = await fsp.readFile("../data/gpt2-merges.txt");
  const inputs = await fsp.readFile("../data/big.txt");
  bench(vocab, merges, inputs);
})();
