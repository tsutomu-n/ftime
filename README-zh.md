# ftime — 简洁的文件时间查看器

一个小巧、依赖极少的 CLI，用于按列显示文件的“修改时间”“创建时间”和“名称”。

| 列       | 含义                                                                                   |
|----------|----------------------------------------------------------------------------------------|
| mark     | 一字符标记，用于表示文件状态。`+` 表示文件在创建后被修改过。启用颜色时，此标记将以黄色显示。 |
| modified | 最后修改时间（格式：`MM-DD HH:MM`）                                                    |
| created  | 创建时间（根据新旧程度着色；若不支持则为 `-`）                                       |
| name     | 文件/目录名（启用颜色时按类型/扩展名着色）                                              |

---

## 运行要求

- Linux + GNU coreutils：`stat`、`date`
- GNU findutils（支持 `-printf`/`-print0` 的 `find`）与 GNU `sort`（支持 `-z`）
- Bash shell（`#!/usr/bin/env bash`）

---

## 安装（推荐：克隆仓库）

通过在 `~/.local/bin` 中创建符号链接，您可以将其作为 `ftime` 命令来使用。

```bash
git clone https://github.com/tsutomu-n/ftime.git
cd ftime
chmod +x ftime-list.sh
mkdir -p ~/.local/bin
ln -sf "$PWD/ftime-list.sh" ~/.local/bin/ftime
hash -r
ftime --help
```

### 卸载

```bash
rm ~/.local/bin/ftime
```

---

## 单行命令安装（仅下载脚本）

```bash
mkdir -p ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/ftime-list.sh \
  -o ~/.local/bin/ftime
chmod +x ~/.local/bin/ftime
hash -r
ftime --help
```

> 如果 shell 仍找不到 `ftime`，请打开新终端或 `source ~/.zshrc`。

---

## 用法

### 快速开始

```bash
ftime               # 列出当前目录
ftime --help        # 详细帮助
ftime --help-short  # 简短帮助
ftime --version     # 显示版本
```

### 语法

```bash
ftime [DIR] [PATTERN ...]
```

- DIR（可选）：指定要扫描的目录。默认为当前目录。
- PATTERN（可选，可提供多个，为 OR 关系）：
  - 含 `*` 或 `?` → 原样作为 glob 使用
  - 以 `.` 开头 → 前置 `*`（如 `.log` → `*.log`）
  - 其他 → 视作扩展名（如 `md` → `*.md`）

### 选项

- `-h, --help`：显示完整帮助
- `--help-short`：显示简短帮助
- `-V, --version`：显示版本
- `-a, --age`：使用相对时间替代绝对时间戳（例如：`5m`、`3h`）

说明：
- 优先级：命令行选项 > 环境变量 > 默认值
- 为兼容保留 `FTL_RELATIVE`，但推荐使用 `-a/--age`

### 示例

```bash
ftime                 # 全部
ftime md              # 仅 *.md
ftime py              # 仅 *.py
ftime .log            # 仅 *.log
ftime docs md         # ./docs 下的 *.md
ftime '*.test.*'      # 显式 glob
```

---

## 时区

- 默认：使用您计算机的本地时区。
- 覆盖：可通过环境变量 `FTL_TZ` 进行覆盖（例如：`FTL_TZ=Asia/Tokyo ftime md`）。

## 颜色

- 在终端（TTY）中运行时，颜色功能会自动启用。
- 在管道（pipe）或分页器（pager）中使用时，可通过 `FTL_FORCE_COLOR=1` 强制启用颜色（例如：`ftime | less -R`）。
- 关闭所有颜色：设置 `NO_COLOR=1` 或 `FTL_NO_COLOR=1`。

### 着色范围
- `modified` 和 `created` 列：根据文件的“新旧程度”进行着色。
- `name` 列：根据文件类型或扩展名进行着色。
- `mark` 列：如果文件在创建后被修改，则以黄色显示 `+`。

### 基于时间的着色（可配置）
- **活跃**（默认4小时内）：亮绿色
- **最近**（默认24小时内）：默认颜色（不额外着色）
- **较旧**（7天及以上）：灰色
- 仅禁用基于时间的着色：`FTL_NO_TIME_COLOR=1`
- 调整时间阈值：`FTL_ACTIVE_HOURS=4 FTL_RECENT_HOURS=24`

### 如何使用环境变量（示例）

在 `ftime` 命令前加上变量，可以为该次命令设置一次性的临时配置。此设置不是永久性的。您也可以组合使用多个变量。

```bash
# 将时区更改为纽约
FTL_TZ=America/New_York ftime

# 将“活跃”状态的阈值更改为1小时
FTL_ACTIVE_HOURS=1 ftime

# 组合使用多个变量
FTL_TZ=UTC FTL_RECENT_HOURS=48 ftime

# 使用相对时间替代绝对时间戳
FTL_RELATIVE=1 ftime
# 通过选项启用（短/长）
ftime -a
ftime --age
```

### 环境变量（参考）
- `FTL_TZ`：覆盖时区（例如 `Asia/Tokyo`）。
- `FTL_FORCE_COLOR`：在管道（pipe）中也强制启用颜色。
- `NO_COLOR` / `FTL_NO_COLOR`：关闭所有颜色功能。
- `FTL_NO_TIME_COLOR`：仅关闭基于时间的着色功能。
- `FTL_ACTIVE_HOURS`、`FTL_RECENT_HOURS`：设置“活跃”和“最近”状态的时间阈值（以小时为单位）。

---

## 安全 / 限制

- 创建时间取决于文件系统/内核/工具，可能显示为 `-`。
- 文件名可能包含控制字符。将带 ANSI 颜色的输出粘贴到会解释 ANSI 的位置时请注意。
- 仅支持 Linux/GNU。macOS/BSD 的 `stat`/`date` 选项不同。

---

## 许可证

本项目采用 MIT 许可证（详见 `LICENSE` 文件）。
