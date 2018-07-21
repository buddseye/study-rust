#!/bin/bash -x

# インストールの確認
cargo --version

# プロジェクトの作成
# `--bin` は実行可能なアプリケーションの指定
# hello_cargoディレクトリが作成される
cargo new hello_cargo --bin

cd hello_cargo
# ビルド
cargo build
# 実行
cargo run
# 実行ファイル
./target/debug/hello_cargo

# リリース用ビルド
cargo build --release
./target/release/hello_cargo
