# charactors_quotes

アニメやゲームなどのキャラクターの名言を登録、参照できるAPIサーバーをです。
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
<br>
2. マイグレーションを実行する
```
diesel migration run
```
<br>
3. サーバーを起動する
```
cargo run --release
```

