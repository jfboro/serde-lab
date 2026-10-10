# Serde-Lab

The purpose of this repository is featuring several use cases of the `serde` crate.

It accompanies my [article](https://jfboro.dev/articles/rust_json_and_serde_robust_deserialization_patterns) about JSON deserialization using Rust in order to illustrate concepts discussed there.

Run
```bash
cargo run --example <example-name>
```
to see the serialization/deserialization output for each scenario.

Or run
```bash
./run-all-examples.sh --all
```
to execute all at once. (Omit the `--all` option to pause the execution at the end of each example, pressing `Enter` to go to the next one.)

## Examples
- Structure wise :
    - [01_json_object_into_rust_struct](https://github.com/jfboro/serde-lab/blob/main/examples/01_json_object_into_rust_struct.rs)
    - [02_externally_tagged_enums](https://github.com/jfboro/serde-lab/blob/main/examples/02_externally_tagged_enums.rs)
    - [03_internally_tagged_enums](https://github.com/jfboro/serde-lab/blob/main/examples/03_internally_tagged_enums.rs)
    - [04_adjacently_tagged_enums](https://github.com/jfboro/serde-lab/blob/main/examples/04_adjacently_tagged_enums.rs)
    - [05_untagged_enums](https://github.com/jfboro/serde-lab/blob/main/examples/05_untagged_enums.rs)
    - [06_flattened_structs](https://github.com/jfboro/serde-lab/blob/main/examples/06_flattened_structs.rs)
    - [07_flattened_structs_and_enums](https://github.com/jfboro/serde-lab/blob/main/examples/07_flattened_structs_and_enums.rs)
- Content wise:
    - [08_cursed_strings_01](https://github.com/jfboro/serde-lab/blob/main/examples/08_cursed_strings_01.rs)
    - [08_cursed_strings_02](https://github.com/jfboro/serde-lab/blob/main/examples/08_cursed_strings_02.rs)
    - [08_cursed_strings_03](https://github.com/jfboro/serde-lab/blob/main/examples/08_cursed_strings_03.rs)
- Performance wise:
    - [09_zero_copy_deserialization](https://github.com/jfboro/serde-lab/blob/main/examples/09_zero_copy_deserialization.rs)