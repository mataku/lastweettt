## Installation
### 事前設定
1. Rustをインストール
2. `$ cp twitter_key_templates.yaml twitter_key.yaml`
3. twitter_key.yaml に取得したkey, tokenを書く

### Install to `~/.cargo/bin`
```bash
$ cargo install
```

### とりあえず動かしたい場合
```bash
$ cargo run
```

### opensslに関連するコンパイルエラー
- HomebrewのOpenSSLの場合
```bash
$ export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
$ export DIR_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
$ cargo install
```
