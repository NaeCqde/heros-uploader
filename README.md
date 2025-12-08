# Heros Uploader

## 機能

・[Catbox](https://catbox.moe/) へ画像をアップロードする

・[GoFile](https://gofile.io/) へファイルをアップロードする

## ※注意

GoFile へのアップロード機能では、ファイル名は、提出された URL の最下層がそのまま採用され、解釈に失敗したら`video.mp4`にフォールバックします。

例：[https://nae.quest/discord/`runa.png`?fake=true](https://nae.quest/discord/runa.png) → `runa.png`

## 使い方 & デモサーバー

### [Web UI：https://heros.nae.quest/](https://heros.nae.quest/)

## Web API：

・`thumbnail` → Catbox

・`src` → GoFile

### リクエスト

### GET [`/upload`](https://heros.nae.quest/upload)

URL クエリパラメーター：

片方のクエリのみでも OK

```js
// javascript
`?thumbnail=${encodeURIComponent(画像直リンク)}&src=${encodeURIComponent(
    ファイル直リンク
)}`;
```

### POST [`/upload`](https://heros.nae.quest/upload)

必要なヘッダー：`content-type: application/json`

JSON：

片方のキーのみでも OK

```js
// javascript
JSON.stringify({
    thumbnail: encodeURIComponent(画像直リンク),
    src: encodeURIComponent(ファイル直リンク),
});
```

### レスポンス

GETとPOSTレスポンスは共通です

レスポンスヘッダー：`content-type: application/json`

### 200 OK：

片方しか指定しなかった場合は、指定したキーのみになります

両方指定した場合は、片方でも失敗したら 200 になりません

```
{
    "thumbnail": "https://files.catbox.moe/<id>.<ext>",
    "src": "https://gofile.io/d/<id>"
}
```

### 400 Bad Request：

ファイルを取得できなかった

存在しないドメインの指定などのネットワークエラーは 500 になります

```
{
    "message": "The provided urls are not accessible"
}
```

### 500 Internal Server Error：

エラーの原因は、セキュリティ上の理由によりクライアントに開示されません

サーバー管理者であれば、stderr からエラーが読めます

```
{
    "message": "Internal server error"
}
```
