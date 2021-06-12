# charactors_quotes [![Rust](https://github.com/CityBear3/charactors_quotes/actions/workflows/rust.yml/badge.svg)](https://github.com/CityBear3/charactors_quotes/actions/workflows/rust.yml)

アニメやゲームなどのキャラクターの名言を登録、参照できるAPIサーバーです。
<br>
[Rustでwebサーバ開発ハンズオン！](https://zenn.dev/susiyaki/books/b927a18723da01a6066b)を参考にしました。

## 環境
- rust 1.52.1
- diesel-cli
- postgresql

## 実行方法
1. envファイルにデータベースのurlを記述する
```
DATABASE_URL=postres://"user":"password"@localhost:"port"/"database"
```
2. マイグレーションを実行する
```
diesel migration run
```
3. サーバーを起動する
```
cargo run --release
```
## 使用方法(HTTPieを想定しています)
- 登録をする
```
http localhost:8080/charactors name="hoge" title="fuga" quote="顧客が本当に必要だったもの"
```
- 参照(複数)
```
http localhost:8080/charactors
```
- 参照(一件、id検索)
```
http localhost:8080/charactors/id search:="number"
```
- 参照(一件、名前検索)
```
http localhost:8080/charactors/names search="hoge"
```
- 参照(一件、タイトル検索)
```
http localhost:8080/charactors/titles search="fuga"
```
- 更新
```
http PUT localhost:8080/charactors id:="number" name="hogehoge" title="fugafuga" quote="バグは夜更け過ぎに仕様に変わるだろう"
```
- 削除
```
http DELETE localhost:8080/charactors search:="number"
```
- 名前一覧
```
http localhost:8080/charactors/names/list
```
- タイトル一覧
```
http localhost:8080/charactors/titles/list
```
