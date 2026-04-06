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
ftime --check-update
```

## 命令速查

下面按“相对默认 `ftime` 有什么变化”来比较公开命令。

| 命令 | 适合什么时候用 | 变化 |
| --- | --- | --- |
| `ftime` | 用默认 human view 看当前目录 | 显示 bucket / size / time / suffix |
| `ftime [PATH]` | 查看别的目录 | 输出形状不变，只是目标目录不同 |
| `ftime -a` | 也想看 hidden directory | 保留 hidden file，同时把 hidden directory 加进来 |
| `ftime --hide-dots` | 想完全隐藏 dot 项 | 隐藏 hidden file / dir / symlink |
| `ftime --no-ignore` | 想看被 ignore 的条目 | 关闭 built-in ignore 和 `.ftimeignore` |
| `ftime --ext rs,toml` | 只关注某些扩展名 | 只过滤 regular file，保留 dir / symlink |
| `ftime --files-only` | 只看文件 | 去掉 directory 和 symlink |
| `ftime --all-history` | 展开全部 `History` | 移除 `History` 的预览上限 |
| `ftime -A` | 想看精确时间 | 把相对时间换成绝对时间 |
| `ftime --no-hints` | 不想看 `[child: ...]` | 去掉 directory 的 child hint suffix |
| `ftime --plain` | 给脚本用 | 输出 `path<TAB>bucket<TAB>time` |
| `ftime --json` | 想要结构化结果 | 输出 JSON Lines |
| `ftime --check-update` | 检查是否有新发布版 | 只打印更新状态 |
| `ftime --self-update` | 更新当前安装的二进制 | 安装最新的已发布版本 |
| `ftime --help` | 查看完整帮助 | 打印 usage 和参数约束 |
| `ftime --version` | 查看当前版本 | 打印版本号 |

hidden 项比较：

```text
$ ftime
Today (2)
  .env       312 B   2h
  README.md  8.4 KiB 3h

This Week (1)
  src/           —   2d [child: today]
```

```text
$ ftime -a
Today (2)
  .env       312 B   2h
  README.md  8.4 KiB 3h

This Week (3)
  .git/          —   1d [child: active]
  .cache/        —   1d
  src/           —   2d [child: today]
```

```text
$ ftime --hide-dots
Today (1)
  README.md  8.4 KiB 3h

This Week (1)
  src/           —   2d [child: today]
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
- `--check-update`
- `--self-update`

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
