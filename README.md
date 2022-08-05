# rust-voxel-polygon-study
SNACKS vol4（2022年夏）で展示する、RustとWebAssemblyの習作。

DEMO https://novogrammer.github.io/rust-voxel-polygon-study/

## git clone
```
$ git clone git@github.com:novogrammer/rust-voxel-polygon-study.git
``

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

## Dockerを使ったビルド

### wasm

wasmフォルダは事前にbuildしておく必要がある。
```
$ cd wasm
$ docker compose build
$ docker compose up
```
### www

wasmディレクトリも使うのでプロジェクトルートでビルドする。
```
$ docker compose build
$ docker compose up
```
