## Installation
#### 事前設定
1. Rustをインストール
2. twitter_key_templates.yamlに自分のkey, tokenを書く
3. `$ mv twitter_key_templates.yaml twitter_key.yaml`

#### Install to `~/.cargo/bin`
```bash
$ cargo install
```

#### とりあえず動かしたい場合
```bash
$ cargo run
```

#### opensslに関連するコンパイルエラー
- HomebrewのOpenSSLの場合
```bash
$ export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
$ export DIR_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
$ cargo install
```
