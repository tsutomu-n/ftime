# ftime（中文 README）

[English](../README.md) | [日本語](README-ja.md) | 中文

`ftime` 是一个只读 CLI，用来快速回答一个问题：

> 这个文件夹最近有什么变化？

- 只读
- 固定深度 1
- `Active / Today / This Week / History`
- 默认就是面向人的 bucket 视图
- hidden file 默认保留，hidden directory 默认隐藏

## 示例

```bash
ftime
ftime -a
ftime --all-history
ftime --hide-dots
ftime --files-only --ext rs,toml
ftime --plain
ftime --json | jq -r '.path'
```

## 常用参数

- `-a, --all`
- `--all-history`
- `--hide-dots`
- `--ext`
- `--files-only`
- `--plain`
- `--json`
- `--no-hints`

`ftime` 不是 `fd`、`find`、`eza` 或 `git status` 的替代品。

## 卸载

### GitHub Releases 安装

```bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | bash
curl -fsSL https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.sh | env INSTALL_DIR=/custom/bin bash
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -Command "iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing | iex"
powershell -ExecutionPolicy Bypass -Command "& ([scriptblock]::Create((iwr https://github.com/tsutomu-n/ftime/releases/latest/download/ftime-uninstall.ps1 -UseBasicParsing).Content)) -InstallDir 'C:\custom\bin'"
```

### `cargo install` / `cargo install --path .` 安装

```bash
cargo uninstall ftime
```
