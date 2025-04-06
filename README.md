# RS-XTree

`RS-XTree`は、Rust で作成されたカスタマイズ可能な CLI 版の tree コマンドです。Windows 標準の tree コマンドではできない柔軟なファイル構造表示や Git 連携が可能です。

## 特徴

- 標準のツリー表示
- JSON および Markdown 形式の出力
- 特定拡張子でのフィルター表示
- 指定したファイルやディレクトリを除外可能
- Git 差分ステータス表示
- ファイルサイズを表示
- ファイルやディレクトリの詳細な情報を表示
- サイズ、名前、作成日時順で sort
- **非同期・同期モードの切り替えによる柔軟なパフォーマンスチューニング**

## インストール方法

```bash
cargo install --path .
```

## 使い方

```bash
rs-xtree [OPTIONS] [PATH]
```

### オプション一覧

| オプション     | 説明                                                                                 |
| -------------- | ------------------------------------------------------------------------------------ |
| `-e, --ext`    | 特定の拡張子のファイルのみ表示                                                       |
| `-i, --ignore` | 除外するファイルやディレクトリを指定                                                 |
| `-d, --depth`  | ツリー表示する最大深さを指定                                                         |
| `-j, --json`   | JSON 形式での出力                                                                    |
| `-m, --md`     | Markdown 形式での出力                                                                |
| `-g, --git`    | Git の差分（変更、新規など）を表示                                                   |
| `-s, --size`   | ファイルサイズ、ディレクトリサイズを `b` (バイトサイズ) / `h` (読みやすい単位)で表示 |
| `-l, --long`   | ファイル、ディレクトリの詳細な情報を表示( `-s` (サイズ表示フラグ) と併用は不可)      |
| `-S, --sort`   | tree を `s` (サイズ) / `n` (名前) / `t` (作成日時順)でソート                         |
| `--mode`       | ツリー構築処理の方式を選択：`sync`（同期）/ `async`（非同期）                        |

### 例

```bash
# 現在のディレクトリを標準で表示
rs-xtree

# 特定拡張子だけ表示 (.rs)
rs-xtree --ext rs

# 「target」と「node_modules」フォルダを無視して表示
rs-xtree --ignore target node_modules

# JSON形式で表示
rs-xtree --json

# Markdown形式で表示
rs-xtree --md

# Gitの差分も表示
rs-xtree --git

# ファイルサイズ、ディレクトリサイズをバイト形式で表示
rs-xtree -s b

# ファイルサイズ、ディレクトリサイズを読みやすい形式で表示
rs-xtree -s h

# ファイルやディレクトリの詳細な情報を表示
rs-xtree -l

# ファイルやディレクトリを作成日時でsortします
rs-xtree -S t

# ファイルやディレクトリを名前でsortします
rs-xtree -S n

# ファイルやディレクトリをサイズでsortします
rs-xtree -S s

# 非同期でツリーを構築（ファイル数が多い場合に高速）
rs-xtree --mode async

# 同期（デフォルト動作）
rs-xtree --mode sync
```

## 出力例

### 標準

```
my_project
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── utils
│        └── helper.rs
└── Cargo.toml
```

### Markdown

```markdown
- my_project/
  - src/
    - main.rs
    - utils/
      - file.rs
  - Cargo.toml
```

### JSON

```json
{
  "name": "my_project",
  "children": [
    {
      "name": "src",
      "children": [
        { "name": "main.rs", "children": null },
        {
          "name": "utils",
          "children": [{ "name": "file.rs", "children": null }]
        }
      ]
    },
    { "name": "Cargo.toml", "children": null }
  ]
}
```

## ライセンス

MIT ライセンス
