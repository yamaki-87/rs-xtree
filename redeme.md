# RS-XTree

`RS-XTree`は、Rust で作成されたカスタマイズ可能な CLI 版の tree コマンドです。Windows 標準の tree コマンドではできない柔軟なファイル構造表示や Git 連携が可能です。

## 特徴

- 標準のツリー表示
- JSON および Markdown 形式の出力
- 特定拡張子でのフィルター表示
- 指定したファイルやディレクトリを除外可能
- Git 差分ステータス表示

## インストール方法

```bash
cargo install --path .
```

## 使い方

```bash
rs-xtree [OPTIONS] [PATH]
```

### オプション一覧

| オプション     | 説明                                 |
| -------------- | ------------------------------------ |
| `-e, --ext`    | 特定の拡張子のファイルのみ表示       |
| `-i, --ignore` | 除外するファイルやディレクトリを指定 |
| `-d, --depth`  | ツリー表示する最大深さを指定         |
| `-j, --json`   | JSON 形式での出力                    |
| `-m, --md`     | Markdown 形式での出力                |
| `-g, --git`    | Git の差分（変更、新規など）を表示   |

### 例

```bash
# 現在のディレクトリを標準で表示
rs-xtree

# 特定拡張子だけ表示 (.rs)
rs-xtree--ext rs

# 「target」と「node_modules」フォルダを無視して表示
rs-xtree --ignore target node_modules

# JSON形式で表示
rs-xtree --json

# Markdown形式で表示
rs-xtree --md

# Gitの差分も表示
rs-xtree --git
```

## 出力例

### 標準

```
my_project/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── utils/
│   │   └── helper.rs
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
