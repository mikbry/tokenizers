WIP : WASM implementation of Tokenizer

## Todo
- [ ] Minimal binding
- [ ] All functions implementations
- [ ] Tests
- [ ] Bench
- [ ] Node.js examples
- [ ] Browser examples
- [ ] Documentation

## Notes

Due to dependencies to console (which not build with wasm-pack), we need to make some changes in rust source code :
- Need to copy ../../tokenizers to ./tkz
- Remove file
./tkz/src/cli.rs
- In Cargo.toml :
- - 1 Remove `[[bin]]` and `[[bench]]` entries
- - 2 Remove clap and indicatif
- Update ./tkz/src/tokenizer/mod.rs to remove dependencies to indicatif
- Update ./tkz/src/models/bpe/mod.rs to remove dependencies to indicatif

