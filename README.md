# cargo-io-lib-template

This is tweaked `cargo init --lib` for FOSS. It contains recommendations from [cargo book].

## What this template provide?:
- Follow [Rust API Guidelines]
  * license MIT OR APACHE v2.0
- Contains default `README.tpl` that help you generate README.md with [cargo-readme]
- Contains `CHANGELOG.md` that follow [keepchangelog]
- Quick start [CI workflow](https://github.com/actions-rs/meta/blob/master/recipes/quickstart.md) on stable rust:
  * `cargo check`
  * `cargo test`
  * `cargo fmt`
  * `cargo clippy -- -D warnings`

## pre-requires:
Installed:
- [cargo-generate] `cargo install --git https://github.com/ashleygwilliams/cargo-generate` [(see issue #1)](https://github.com/xoac/crates-io-lib-template/issues/1)
- [cargo-readme] `cargo install cargo-readme`

## How to use:
You need to do four simple steps:
### 1. Use `cargo generate` to clone this template
```
cargo generate --git https://github.com/xoac/crates-io-lib-template.git --name my-project
cd my-project
```
[Learn more about `cargo generate` here.][cargo-generate]

### 2. Update CHANGELOG.md and README.tpl Cargo.toml
You need to replace `GITHUB_ORG_PATH` with your organization path. For example for this project the `GITHUB_ORG_PATH` would be `https://github.com/xoac/` (remember about last `/`).

You can do this with `sed` or with your IDE.
```
sed -i 's/GITHUB_ORG_PATH/https:\/\/github\.com\/xoac\//g' README.tpl CHANGELOG.md Cargo.toml
```

This is limitation because of this [issue](https://github.com/ashleygwilliams/cargo-generate/issues/17).

### 3. Update Cargo.toml
Edit `Cargo.toml` there are some basic information you should provide (look for `TODO`).

[Learn more about Cargo.toml here.](https://doc.rust-lang.org/cargo/reference/manifest.html)

This is limitation because of this [issue](https://github.com/ashleygwilliams/cargo-generate/issues/17).

### 4. Replace this README.md
Add documentation at top of `src/lib.rs` and generate `README.md` from that with:
```
cargo readme > README.md
```
[Lern more about `cargo readme` here.][cargo-readme]


[Rust API Guidelines]:https://rust-lang.github.io/api-guidelines/about.html
[cargo-readme]:https://github.com/livioribeiro/cargo-readme
[cargo-generate]:https://github.com/ashleygwilliams/cargo-generate
[keepchangelog]:https://keepachangelog.com
[cargo book]:https://doc.rust-lang.org/cargo
