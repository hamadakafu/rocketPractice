## git
- branch nameには'/'と'#'を使わない
  - docker のtagにできないため
## wither_derive
### Model
すべての Model struct は 
```
#[serde(rename="_id", skip_serializing_if="Option::is_none")]
pub id: Option<ObjectId>,
```
を持っている必要がある

## Google Cloud Key Management Service
秘密にしたいファイルを暗号化する

## docker
- --cache-fromでは中間イメージを指定しなければ意味ない
- multi stage build 時には中間image の cacheが効かない

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

### cargo 
project name は絶対にsnake_caseにしろ
'-'が'_'に自動的に変換されるため
rust-api -> rust_api
予期せぬ自体になる

## DNS
### フルサービスリソルバー・DNSキャッシュサーバ
カンニング野郎
### 権威DNSサーバ・DNSコンテンツサーバ
手持ちのゾーンファイルをみる
### NSレコード
[domain] IN NS [server]

## Todo
- mongodbの永続化をあとでやる
