# ftime（中文 README）

[English](../README.md) | [日本語](README-ja.md) | 中文

`ftime` 是一个只读 CLI，专门用来快速回答一个问题：

> 这个文件夹最近有什么变化？

名字来自 `files by time`。它只扫描目录的第一层，按 `mtime` 从新到旧排序，再分到 `Active / Today / This Week / History` 这些时间桶里，这样你不用递归整棵目录树，也能快速看出最近的变化。

- 只读设计：不会删除、重命名或写入文件
- 固定深度 1：只看当前文件夹，不递归整个目录树
- 时间分桶：`Active` / `Today` / `This Week` / `History`
- TTY 中显示人类可读的大小，非 TTY 可用纯文本，`--json` 可输出 JSON Lines

## 适合什么场景

- 整理 `~/Downloads`
- 检查 `./target` 这类 build 输出目录
- 快速查看日志目录或同步目录最近有没有变化
- 在几秒内回答“这里刚刚改了什么？”

## 示例

```bash
ftime
ftime ~/Downloads
ftime ./target
ftime /var/log/app
ftime --exclude-dots
ftime --json | jq -r '.path'
```

`--json` 会按每行一个 JSON 对象输出，方便接到 `jq` 或其他脚本里。

## 输出示例

```text
Active
  • Cargo.toml | 2.1 KiB | 12s ago
Today
  • README.md | 8.4 KiB | 2h ago
This Week
  • docs/ | - | 3d ago
History
  • target/ | - | 2w ago
```

目录会在 size 列显示 `-`。

## 和其他工具的区别

| 工具 | 强项 | `ftime` 的区别 |
| --- | --- | --- |
| `ls -lt` | 快速按时间排序查看 | 不会自动分到时间桶 |
| `eza` | 丰富的列表和 metadata | 没有内建时间分桶 |
| `fd` | 递归搜索和过滤 | 天生就是递归工具 |
| `bat` | 阅读文件内容 | 不是目录活动视图 |
| `ftime` | 看一个文件夹最近的活动 | 时间桶 + 大小一眼可见 |

## 安装

### GitHub Releases（推荐）

先获取 GitHub Releases 上最新的 installer，再安装最新已发布的 release，不会安装未发布的 `main`。
GitHub Releases installer 不需要 Rust。

```bash
# macOS / Linux
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-install.ps1 -UseBasicParsing | iex"
```

Windows 默认安装目录是 `%LOCALAPPDATA%\Programs\ftime\bin`。

Windows installer 目前仅覆盖 x86_64 / AMD64。

### crates.io

从 crates.io 上公开的 crate 安装。

```bash
cargo install ftime --locked
ftime --version
```

### 从源码安装

需要 Rust/Cargo 1.92+。

```bash
cargo install --path . --force
hash -r
ftime --version
```

卸载步骤写在下方的 `## 卸载`，也包含自定义安装目录的情况。

## 快速开始

```bash
ftime
ftime ~/project
ftime -a
ftime --exclude-dots
ftime --ext rs,toml
ftime --json
```

常用参数：

- `-a, --all`：在 TTY 中展开 `History`
- `-A, --absolute`：显示绝对本地时间
- `--exclude-dots`：隐藏 dotfiles
- `--json`：按每行一个 JSON 对象输出
- `--check-update`：只检查是否有更新的公开版
- `--self-update`：把当前安装位置更新到最新公开版
- `--no-ignore`：禁用 built-in / `.ftimeignore`

## 更新

```bash
ftime --check-update
ftime --self-update
```

常见输出示例：

```text
update available: 1.0.2 -> 1.0.3
ftime updated 1.0.2 -> 1.0.3 in /home/tn/.local/bin
ftime is already up to date at 1.0.2 in /home/tn/.local/bin
ftime now points to 1.0.2 (was 1.0.4) in /home/tn/.local/bin
```

如果通过 symlink 启动，`ftime --self-update` 会更新该 symlink 所在目录。

如果你当前的 binary 还没有 `--self-update`，先用最新的 GitHub Releases installer 重装一次。

## 卸载

### GitHub Releases 安装

```bash
# macOS / Linux
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | bash
```

如果你安装到了自定义目录，卸载时需要再次传入同一路径。macOS / Linux 使用 `INSTALL_DIR`，Windows 使用 `-InstallDir`。

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | env INSTALL_DIR=/custom/bin bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing | iex"
```

```powershell
powershell -ExecutionPolicy Bypass -Command "& ([scriptblock]::Create((iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\custom\bin'"
```

### `cargo install` / `cargo install --path .` 安装

```bash
cargo uninstall ftime
```

## 详细文档

- [CLI contract](CLI.md)
- [日本語文档入口](README-ja.md)
- [日本語文档导览](ftime-overview-ja.md)

如果你只需要安装和日常使用，这个 README 已经足够。需要更细的 CLI 约定时，请看 `CLI.md`。
