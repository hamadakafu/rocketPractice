## wither_derive
### Model
すべての Model struct は 
```
#[serde(rename="_id", skip_serializing_if="Option::is_none")]
pub id: Option<ObjectId>,
```
を持っている必要がある

## skaffold
```bash
minikube start
skaffold run
```
minikube に kubectl が向いているときは，  
image名は何でも良さそう
### skaffold dev 
毎秒ごとにファイルの変更を見てくれる
### skaffold run
一回だけやってくれる

## DNS
### フルサービスリソルバー・DNSキャッシュサーバ
カンニング野郎
### 権威DNSサーバ・DNSコンテンツサーバ
手持ちのゾーンファイルをみる
### NSレコード
[domain] IN NS [server]

## Todo
- mongodbの永続化をあとでやる
