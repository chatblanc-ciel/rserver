# Practice Rust Web server
## 目的
CRUDが実行できるWebアプリケーションを作る。

## 条件
- Actix-webのようなWebフレームワークを使わず、オリジナルのフレームワークで。
- 勉強も兼ねてクレートにできるだけ頼らない実装。
- CRUDのRestAPIを作成。
- フロントエンドのデザインは、Create/Read/Update/Deleteができるボタンとレスポンスを表示するだけ。
- DB（MySQL or PostgreSQL)の使い方を学ぶ。
- Dockerを利用した開発環境構築。

## 考慮しなくて良いこと
- セキュリティ面は無視してもらって良いです。
- パフォーマンスも気にしなくて良いです。

# コンセプト
制限時間があるため、始めから欲張ってフルスクラッチしない。
以下のクレートは使用し、時間があれば書き直す。
あと`std`は使い倒すことにする。使わなくてもCをFFI経由で
バインドするかOSコールを多用して、マルチプラットフォーム性が
損なわれるのが目に見えるため。
コアすぎる車輪の再開発禁止。

1. diesel

データベースのコントローラー。
HTTPレスポンスの処理が作れたら、
次はこれを処理する。

2. Tera

HTMLのテンプレートジェネレーター。
これをフルスクラッチはコスパ悪いので、最後に作業する。

3. 並列処理

本当なら非同期処理も行うべきだが、
時間の都合で見送り。できたらスレッドプールくらいで。



 
