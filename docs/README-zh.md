# ftime（中文 README）

[English](../README.md) | [日本語](README-ja.md) | 中文

`ftime` 是一个只读 CLI：只扫描目录的第一层，按 `mtime` 从新到旧排序，并按时间分桶展示最近改动过的文件或目录。

- 固定深度 1，不递归
- `Active / Today / This Week / History` 四个时间桶
- 终端输出适合人看，plain text / JSON 适合脚本处理

## 安装

### GitHub Releases（推荐）
安装最新已发布的 release，不会安装未发布的 `main`。

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/install.ps1 -UseBasicParsing | iex"
```

### crates.io（已发布时）

```bash
cargo install ftime
```

### 从源码安装
需要 Rust/Cargo 1.92+。

```bash
cargo install --path . --force
hash -r
ftime --version
```

Windows installer 目前仅覆盖 x86_64 / AMD64。

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
- `--no-ignore`：禁用 built-in / `.ftimeignore`

## 卸载

### GitHub Releases 安装

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/uninstall.sh | bash
```

如果你安装到了自定义目录，卸载时需要再次传入同一路径。macOS / Linux 使用 `INSTALL_DIR`，Windows 使用 `-InstallDir`。

```bash
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/uninstall.sh | env INSTALL_DIR=/custom/bin bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/uninstall.ps1 -UseBasicParsing | iex"
```

```powershell
powershell -ExecutionPolicy Bypass -Command "& ([scriptblock]::Create((iwr https://raw.githubusercontent.com/tsutomu-n/ftime/v1.0.0/scripts/uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\custom\bin'"
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
