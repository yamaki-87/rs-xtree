[日本語はこちら 🇯🇵](./README.ja.md)

# RS-XTree

**RS-XTree** is a customizable CLI tree viewer written in Rust.  
It offers more flexible and powerful file structure display compared to the built-in `tree` command on Windows, and includes Git integration, filtering, and asynchronous execution.

---

## Features

- Standard directory tree view
- Output in **JSON** and **Markdown**
- Filter by file extension
- Ignore specific files or directories
- Display Git diff status
- Show file and directory sizes
- Display detailed file metadata
- Sort by size, name, or creation time
- **Choose execution mode: sync / parallel**
- Display file extension statistics

---

## Installation

```bash
cargo install --path .
```

## Usage

```bash
rs-xtree [OPTIONS] [PATH]
```

### オプション一覧

| オプション     | 説明                                                          |
| -------------- | ------------------------------------------------------------- |
| `-e, --ext`    | Show only files with the specified extension                  |
| `-i, --ignore` | Exclude specific files or directories                         |
| `-d, --depth`  | Limit the maximum depth of the tree                           |
| `-j, --json`   | Output as JSON                                                |
| `-m, --md`     | Output as Markdown                                            |
| `-g, --git`    | Display Git status (modified, new, etc.)                      |
| `-s, --size`   | Show sizes: b (bytes) or h (human readable)                   |
| `-l, --long`   | Show detailed file info (not combinable with -s)              |
| `-S, --sort`   | Sort by: s (size), n (name), t (timestamp)                    |
| `--mode`       | Execution mode: sync (default), parallel                      |
| `--stats`      | Aggregate and display file counts and total size by extension |
| `-a, --all`    | Show hidden files and directories (those starting with `.`)   |

### Examples

```bash
rs-xtree                      # Default tree output

rs-xtree --ext rs            # Show only .rs files

rs-xtree --ignore target     # Ignore `target` directory

rs-xtree --json              # Output as JSON

rs-xtree --md                # Output as Markdown

rs-xtree --git               # Show Git status

rs-xtree -s b                # Show sizes in bytes

rs-xtree -s h                # Show sizes in human-readable format

rs-xtree -l                  # Show detailed file/directory info

rs-xtree -S t                # Sort by creation time

rs-xtree -S n                # Sort by name

rs-xtree -S s                # Sort by size

rs-xtree --mode p        # Use parallel tree building

rs-xtree --stats             # Show file extension statistics

rs-xtree -a                 # Show hidden files and folders (e.g., .git, .env)
```

## Output Examples

### Tree

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

### STATS

```
Extension    Count    Total Size
--------------------------------
md           1        4156 Bytes
(no ext)     165      1274923835 Bytes
sample       13       23513 Bytes
txt          1        11809 Bytes
lock         1        29174 Bytes
toml         1        400 Bytes
rs           15       36587 Bytes
```

## ライセンス

MIT License
