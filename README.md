# rust-voxel-polygon-study

SNACKS vol4（2022 年夏）で展示する、Rust と WebAssembly の習作。

DEMO https://novogrammer.github.io/rust-voxel-polygon-study/

## git clone

```
$ git clone git@github.com:novogrammer/rust-voxel-polygon-study.git
```

## setup

### Rust

`Rust`のインストール https://www.rust-lang.org/tools/install

`wasm-pack`のインストール https://rustwasm.github.io/wasm-pack/installer/

### Node.js

```
$ cd www
$ npm i
```

## ビルド

### wasm

```
$ cd wasm
$ wasm-pack build
```

### www

```
$ cd www
$ npm run build
```

## 開発サーバー

```
$ cd www
$ npm run start
```

## Docker を使ったビルド

### wasm

（Mac では）うまくコンパイルできないので事前に Cargo.toml を変更しておく

```
[package.metadata.wasm-pack.profile.release]
# wasm-opt = ['-O4', '-g']
wasm-opt = false
```

wasm フォルダは事前に build しておく必要がある。

```
$ cd wasm
$ docker compose build
$ docker compose up
```

### www

wasm ディレクトリも使うのでプロジェクトルートでビルドする。

```
$ docker compose build
$ docker compose up
```
