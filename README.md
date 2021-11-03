# fast2s

A super-fast Chinese translation tool to translate Traditional Chinese to Simplified Chinese.

Use [hashbrown](https://github.com/rust-lang/hashbrown) to build the translation state machine.

Usage:

```rust
let t = "企畫 計畫 企劃 計劃 畫圖 畫畫";
let s = fast2s::convert(k);
assert_eq!(&s, "企划 计划 企划 计划 画图 画画");
```

## Benchmark

See [simple.rs](benches/simple/benches/simple.rs) under benches directory. I compared the result with [opencc-rust](https://github.com/magiclen/opencc-rust), [simplet2s-rs](https://github.com/bosondata/simplet2s-rs), and [character_converter](https://github.com/sotch-pr35mac/character_converter). As character_converter is too slow, I have to change the sample size to 10 to not wait super long.

Test result (convert and return new string):

| tests | fast2s | simplet2s-rs | opencc-rust | character_conver |
| ----- | ------ | ------------ | ----------- | ---------------- |
| zht   | 188us  | 729us        | 5.98ms      | 1.23s            |
| zhc   | 169us  | 941us        | 6.89ms      | 2.87s            |
| en    | 69us   | 3.31ms       | 13.99ms     | 26.11s           |

Test result (mutate existing string):

| tests | fast2s | simplet2s-rs | opencc-rust | character_conver |
| ----- | ------ | ------------ | ----------- | ---------------- |
| zht   | 121us  | N/A          | N/A         | N/A              |
| zhc   | 139us  | N/A          | N/A         | N/A              |
| en    | 78us   | N/A          | N/A         | N/A              |

Note:

1. benchmark is done with rust 1.56.1.
2. zht means load "math_zht.txt" and translate, zhc means load "math_zhc.txt" (all Simplified Chinese) and translate, en means load "math_en.txt" (all English) and translate.
3. N/A means not supported.

Please do not trust the benchmark result directly, you shall run it in your local environment. See [how to run benchmark](./benches/README.md).

## Credits

[t2s.txt](src/t2s.txt) is borrored from [simplet2s](https://github.com/bosondata/simplet2s-rs/blob/master/src/t2s.txt).
