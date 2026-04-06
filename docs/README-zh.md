# ftime（中文 README）

[English](../README.md) | [日本語](README-ja.md) | 中文

`ftime` 是一个只读 CLI，用来快速回答一个问题：

> 这个文件夹最近有什么变化？

- 只读
- 固定深度 1
- `Active / Today / This Week / History`
- 默认就是面向人的 bucket 视图
- hidden file 默认保留，hidden directory 默认隐藏

## 常见示例

```bash
ftime
ftime -a
ftime --all-history
ftime --hide-dots
ftime --files-only --ext rs,toml
ftime --plain
ftime --json | jq -r '.path'
ftime --no-hints
ftime --check-update
```

`ftime` 不是 `fd`、`find`、`eza` 或 `git status` 的替代品。

代表参数：`--all-history`、`--hide-dots`、`--plain`、`--files-only`、`--no-hints`

## 继续阅读

- [Japanese user guide](USER-GUIDE-ja.md)
- [Japanese CLI reference](CLI-ja.md)
- [详细命令比较（English）](COMMANDS.md)
- [Install / Update / Uninstall（English）](INSTALL.md)
