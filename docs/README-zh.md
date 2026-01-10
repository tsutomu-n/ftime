# ftime（中文 README）

[English](../README.md) | [日本語](README-ja.md) | 中文

`ftime` 是一个 **只读** 的小型 CLI：扫描指定目录的第一层（深度 1），按更新时间（mtime）排序，并按时间分桶展示最近修改过的文件/目录。

## 能做什么
- 按 mtime 降序列出：文件 / 目录 / 符号链接
- 4 个时间分桶：Active (<1h) / Today（今天） / This Week（7 天内） / History（其他）
- TTY：分桶展示 + 颜色 + 图标（默认 emoji，`--icons` 可切换为 Nerd Font）；History 默认折叠（用 `--all` 展开）
- 管道/重定向：输出 `TAB` 分隔的纯文本（无标题、无颜色、无图标）
- JSON Lines：`--json`（便于脚本处理）
- 过滤：`--ext`（扩展名白名单）/ ignore（见下文）

## 快速开始
```bash
ftime              # 目标为当前目录
ftime /path/to/dir # 指定目标目录
```

## 安装
要求：Rust/Cargo 1.92+（edition 2024）

### GitHub Releases（推荐）
```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.sh | bash

# Windows (PowerShell)
powershell -ExecutionPolicy Bypass -Command "iwr https://raw.githubusercontent.com/tsutomu-n/ftime/main/scripts/install.ps1 -UseBasicParsing | iex"
```

### crates.io（已发布时）
```bash
cargo install ftime
```

### 从源码安装（构建 + 全局化）
```bash
cargo install --path .
ftime --version
```

- 默认安装到 `~/.cargo/bin`（Windows 为 `%USERPROFILE%\\.cargo\\bin`）。
- 只有当上述目录在 `PATH` 中时，才能直接输入 `ftime` 运行。

### 从源码构建（只要产物）
构建有两种方式：

```bash
# 更快（自动启用 timings + sccache / 更快链接）
./scripts/build-release-fast.sh

# 标准
cargo build --release
```

仅构建并不会“全局化”（不能只输入 `ftime` 就运行），因为产物只是放在 `target/release/ftime`，不会自动加入 `PATH`。

```bash
./target/release/ftime
```

如果需要全局化，有两种方式：

```bash
# 官方推荐方式（安装到 ~/.cargo/bin）
cargo install --path .

# 或者做符号链接（Linux/macOS）
ln -s /path/to/ftime/target/release/ftime ~/bin/ftime
```

## 用法
```bash
ftime [OPTIONS] [PATH]
```

- `PATH` 省略时目标为当前目录。
- 如果传入的是文件（不是目录），会以错误码 1 退出。

### 常用选项
- `-a, --all`      ：展开 History 分桶（TTY 模式）
- `-H, --hidden`   ：包含以 `.` 开头的隐藏项
- `--ext rs,toml`  ：扩展名白名单（逗号分隔、大小写不敏感，仅对文件生效）
- `--no-ignore`    ：禁用 ignore（内置 + 用户配置）
- `--no-labels`    ：禁用标签（例如 Fresh）
- `--json`         ：JSON Lines 输出（默认构建可用；`--no-default-features` 构建则不可用）
- `-I, --icons`    ：Nerd Font 图标（需要 `--features icons` 构建；否则为无害 no-op）

### 环境变量
- `NO_COLOR`        ：禁用彩色输出
- `FTIME_FORCE_TTY` ：即使 stdout 被 pipe/redirect，也强制使用 TTY 分桶布局
- `FTIME_IGNORE`    ：覆盖全局 ignore 文件路径（默认：`~/.ftimeignore`）

### 时间分桶判定（边界）
- Active：`now - mtime < 1 小时`（即使 mtime 在未来也算 Active）
- Today：非 Active，且 mtime 位于本地时区“今天 00:00:00”之后（含）
- This Week：非 Today，且 `now - mtime < 7 天`（= 7×24 小时）
- History：以上都不满足

## ignore 规则
- 内置忽略：`.DS_Store`、`Thumbs.db`（即使使用 `--hidden` 也会忽略）
- 用户 ignore：
  - 全局：`~/.ftimeignore`（或 `FTIME_IGNORE` 指定）
  - 本地：`<PATH>/.ftimeignore`（目标目录直下）
- 使用 `--no-ignore` 可整体禁用
- ignore 文件格式：
  - 一行一个 pattern（空行和以 `#` 开头的行会忽略）
  - 仅支持 `*`（任意长度）/ `?`（单字符）通配（不支持 `**`, `[]`, `!` 等）
  - pattern 含 `/` 时匹配“相对 `PATH` 的路径”（例如：`target/*`）
  - pattern 不含 `/` 时匹配“条目名（basename）”（例如：`*.log`）

## 输出模式
### TTY（默认）
- 按分桶展示；History 默认折叠（`--all` 展开）
- 每个分桶最多显示 20 条；超过部分用 `... and N more items` 汇总

输出示例（省略颜色后的效果示意）：
```text
🔥 Active Context (< 1h)
  • src/main.rs  2 mins ago  ✨ Fresh

☕ Today's Session
  • docs/README-zh.md  3 hours ago

📅 This Week
  • target/  Yesterday
  • ftime -> target/release/ftime  3 days ago

💤 History (12 files hidden)
```

### 管道 / 重定向
- 输出 `path<TAB>relative_time` 的全量列表（无标题/颜色/图标）
- 不分桶、无 20 条上限，始终输出全部条目
- `relative_time` 为英文（例如：`just now`, `Yesterday`, `YYYY-MM-DD`）

输出示例：
```text
src/main.rs\t2 mins ago
docs/README-zh.md\t3 hours ago
```

### JSON Lines
- 一行一个 JSON。主要字段：`path`, `bucket`, `mtime`, `relative_time`, `is_dir`, `is_symlink`（按情况包含 `symlink_target`, `label`）
- `bucket` 取值：`active` / `today` / `this_week` / `history`
- `mtime` 为 RFC 3339（UTC）

输出示例：
```json
{"path":"src/main.rs","bucket":"active","mtime":"2026-01-10T05:12:20.214004873+00:00","relative_time":"just now","is_dir":false,"is_symlink":false,"label":"fresh"}
```

## 限制
- 固定深度 1（不递归）
- 只读（不会修改/删除任何文件）

## 相关文档
- 用户指南（日文）：`USER-GUIDE-ja.md`
- CLI 详细（日文）：`CLI-ja.md`
- 规格（日文）：`SPEC-ja.md`
- 架构（日文）：`ARCHITECTURE-ja.md`
- 测试计划（日文）：`TESTPLAN-ja.md`
