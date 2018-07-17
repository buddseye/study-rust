#!/bin/bash

# インストールは下記コマンドだけ
curl https://sh.rustup.rs -sSf | sh

# 環境変数はログイン時に読み込まれるため、すぐに使用する場合は下記を実行
source $HOME/.cargo/env

# 最新版への更新
rustup update

# アンインストール
#rustup self uninstall
