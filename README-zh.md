# ftime — 简洁的文件时间查看器

一个小巧、依赖极少的 CLI，用于按列显示文件的“修改时间”“创建时间”和“名称”。

<p align="left">
  <img src="./media/basic.gif"   alt="ftime: 一眼查看 modified/created/name" width="600" />
  
</p>

面向初学者和非英语母语者进行友好设计，提供更清晰的错误信息和上手友好的帮助系统。

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

## 安装（单行：仅下载） – 推荐

无需克隆仓库。只需下载脚本并赋予可执行权限。

```bash
mkdir -p ~/.local/bin
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/ftime-list.sh \
  -o ~/.local/bin/ftime
chmod +x ~/.local/bin/ftime

# test
hash -r
ftime --help
```

### 卸载

```bash
rm ~/.local/bin/ftime
```

---

<details>
  <summary><strong>从已克隆的仓库安装 – 可选</strong></summary>

使用 `~/.local/bin` 中的符号链接，将其作为 `ftime` 命令使用。

1) 克隆到任意位置

```bash
git clone https://github.com/tsutomu-n/ftime.git
cd ftime   # 进入仓库根目录
```

2) 赋予可执行权限

```bash
chmod +x ftime-list.sh
```

3) 确保 `~/.local/bin` 在 PATH 中（自动检测 zsh/bash；若无 rc 文件则创建）

```bash
if [ -n "$ZSH_VERSION" ]; then
  rc="${ZDOTDIR:-$HOME}/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
  rc="$HOME/.bashrc"
else
  rc="$HOME/.profile"
fi
mkdir -p "$(dirname "$rc")"
grep -q '\\.local/bin' "$rc" 2>/dev/null || \
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
. "$rc"
```

4) 创建 `ftime` 命令

```bash
mkdir -p ~/.local/bin
ln -sf "$PWD/ftime-list.sh" ~/.local/bin/ftime
```

5) 刷新并测试

```bash
hash -r
ftime --help
```

</details>

**Notes**

- 如果 shell 仍找不到 `ftime`，请打开新终端或执行 `source ~/.zshrc`。
- 本工具需要 Linux 上的 GNU `stat`/`date` 和 Bash。

---

## 用法

### 快速开始

```bash
ftime               # 列出当前目录
ftime -a            # 使用相对时间
ftime -s time       # 按修改时间排序（新到旧）
ftime -R -d 2 md    # 递归到深度2，并仅列出 *.md
ftime --help        # 详细帮助
ftime --help-short  # 简短帮助（3行）
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

- `-a, --age`：使用相对时间替代绝对时间戳（例如：`5m`、`3h`）
- `-s, --sort time|name`：排序键（默认：`name`；`time` 为修改时间）
- `-r, --reverse`：反转排序顺序
- `-R, --recursive`：递归进入子目录
- `-d, --max-depth N`：将递归深度限制为 N（需要 `-R`）
- `-h, --help`：显示完整帮助
- `--help-short`：显示简短帮助
- `-V, --version`：显示版本

注记：
- 为兼容保留 `FTL_RELATIVE`，但推荐使用 `-a/--age`

### 示例（组合）

```bash
# 递归整个目录树（可能结果较大）
ftime -R

# 递归深度 3
ftime -R -d 3

# 在 docs/ 下递归深度 2，且仅 *.md
ftime -R -d 2 docs md

# 按修改时间排序，并仅递归 1 层
ftime -s time -R -d 1

# 按修改时间升序（较旧优先）遍历整个目录树
ftime -s time -r -R
```

### 常见陷阱

- `-d` 需要跟数字
  ```bash
  ftime -d          # Error: --max-depth expects a positive integer
  ftime -d -R       # Error: --max-depth expects a positive integer
  ftime -R -d 3     # OK
  ftime -d 3 -R     # OK（选项顺序无关）
  ```

- 使用 `-d` 时需要同时加上 `-R`
  ```bash
  ftime -d 3        # Error: --max-depth requires --recursive (-R)
  ftime -R -d 3     # OK
  ```

- 为避免 shell 展开，请对模式加引号
  ```bash
  ftime '*.md'      # OK：模式由 ftime 作为过滤器处理
  ftime *.md        # 可能被 shell 展开为文件名，导致行为与预期不符
  ```

- 深度以起始 DIR 为基准
  ```bash
  ftime -R -d 1 docs   # 仅 docs/ 及其直接子项（不包含孙级）
  ```

---

**Notes**
- 优先级：命令行选项 > 环境变量 > 默认值

时区：默认使用本机本地时区。可用环境变量 `FTL_TZ` 覆盖（例如：`FTL_TZ=Asia/Tokyo ftime md`）。

<details>
  <summary><strong>显示自定义（可选）</strong></summary>

## 颜色

- 在 TTY 上自动启用
- 在管道/分页器中强制启用：`FTL_FORCE_COLOR=1 ftime | less -R`
- 关闭所有颜色：`NO_COLOR=1` 或 `FTL_NO_COLOR=1`

### 着色内容
- `modified` 与 `created` 列按新旧程度着色
- `name` 列按类型/扩展名着色
- `mark` 列：创建后有修改则以黄色显示 `+`（否则留空）

### 基于时间的着色（可配置）
- 活跃（默认 4h）：亮绿色
- 最近（默认 24h）：默认色
- 较旧（7d+）：灰色
- 仅关闭时间着色：`FTL_NO_TIME_COLOR=1`
- 调整阈值：`FTL_ACTIVE_HOURS=4 FTL_RECENT_HOURS=24`

</details>

<details>
  <summary><strong>环境变量（可选）</strong></summary>

### 用法示例

在命令前临时添加变量即可，亦可组合多个变量。

```bash
# 将时区改为纽约
FTL_TZ=America/New_York ftime

# 将“活跃”阈值改为 1 小时
FTL_ACTIVE_HOURS=1 ftime

# 组合多个变量
FTL_TZ=UTC FTL_RECENT_HOURS=48 ftime

# 启用相对时间显示
FTL_RELATIVE=1 ftime
# 也可用选项
ftime -a
ftime --age
```

### 参考
- `FTL_TZ`：覆盖时区（例如 `Asia/Tokyo`）
- `FTL_FORCE_COLOR`：在管道中强制启用颜色
- `NO_COLOR` / `FTL_NO_COLOR`：关闭所有颜色
- `FTL_NO_TIME_COLOR`：仅关闭基于时间的着色
- `FTL_ACTIVE_HOURS`、`FTL_RECENT_HOURS`：新旧阈值（小时）
- `FTL_RELATIVE`：显示相对时间（例如 `5m`, `3h`）

</details>

---

## 安全 / 限制

- 创建时间取决于文件系统/内核/工具，可能显示为 `-`。
- 文件名可能包含控制字符。将带 ANSI 颜色的输出粘贴到会解释 ANSI 的位置时请注意。
- 仅支持 Linux/GNU。macOS/BSD 的 `stat`/`date` 选项不同。

---

## 许可证

本项目采用 MIT 许可证（详见 `LICENSE` 文件）。
