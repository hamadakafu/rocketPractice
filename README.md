# Usage
## /othello/create/room/<room_name>
path: /othello/create/room/test_room_name  
response: true or 40x

## /othello/delete/room/<room_name>
path: /othello/delete/room/test_room_name  
response: true or 40x

## /othello/get/room
path: /othello/get/room  
response: \[(ROOM,)*\] or 40x

## /othello/set/<room_name>/\<color\>/\<x\>/\<y\>
path: /othello/set/test_room_name/(white|black)/3/5  
response: othello or 40x

# Memo
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
- k8s/productionをいちいち暗号化するのが手間

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

## cargo 
project name は絶対にsnake_caseにしろ
'-'が'_'に自動的に変換されるため
rust-api -> rust_api
予期せぬ自体になる

## minikube 
これをしないとlocalのimageを使わない
```eval $(minikube docker-env)```

## google cloud build
- localのimageをつかってくれない

## DNS
### フルサービスリソルバー・DNSキャッシュサーバ
カンニング野郎
### 権威DNSサーバ・DNSコンテンツサーバ
手持ちのゾーンファイルをみる
### NSレコード
[domain] IN NS [server]

## kubernetes
- deployment は spec.template が変更されないと変わらないので，  
- set image をやる必要があるかも
- patch で解決できるが
- 動的にyaml生成できるようにしたい
- minikubeを使っているときにimage pull policy が Always担っていると  
- 外部からとってきてしまいlocalを使わないので注意
- Neverにしておけば良さそう

## failure
### 実装パターン
#### String -> Error
- failure::err_msg
- format_err!
        
## Todo
- mongodbの永続化をあとでやる
- 動的にyaml生成できるようにしたい
