# elinor-cli

```sh
cargo run --release -p elinor-cli --bin elinor-evaluate -- \
    --gold-jsonl test-data/toy/gold.jsonl \
    --pred-jsonl test-data/toy/pred_1.jsonl \
    --metrics precision@3 ap rr ndcg@3
```

```
precision@3     0.2667
ap      0.3500
rr      0.5000
ndcg@3  0.4480
```

## Licensing

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
