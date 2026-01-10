# ftime（中文 README）

`ftime` 是一个 **只读** 的小型 CLI：扫描指定目录的第一层（深度 1），按更新时间（mtime）排序，并按时间分桶展示最近修改过的文件/目录。

## 能做什么
- 按 mtime 降序列出：文件 / 目录 / 符号链接
- 4 个时间分桶：Active (<1h) / Today（今天） / This Week（7 天内） / History（其他）
- TTY：分桶展示 + 颜色 +（可选）图标；History 默认折叠（用 `--all` 展开）
- 管道/重定向：输出 `TAB` 分隔的纯文本（无标题、无颜色、无图标）
- JSON Lines：`--json`（便于脚本处理）
- 过滤：`--ext`（扩展名白名单）/ ignore（见下文）

## 快速开始
```bash
ftime              # 目标为当前目录
ftime /path/to/dir # 指定目标目录
```

## 安装
要求：Rust/Cargo 1.85+（edition 2024）

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

## ignore 规则
- 内置忽略：`.DS_Store`、`Thumbs.db`（即使使用 `--hidden` 也会忽略）
- 用户 ignore：
  - 全局：`~/.ftimeignore`（或 `FTIME_IGNORE` 指定）
  - 本地：`<PATH>/.ftimeignore`（目标目录直下）
- 使用 `--no-ignore` 可整体禁用

## 输出模式
### TTY（默认）
- 按分桶展示；History 默认折叠（`--all` 展开）

### 管道 / 重定向
- 输出 `path<TAB>relative_time` 的全量列表（无标题/颜色/图标）

### JSON Lines
- 一行一个 JSON。主要字段：`path`, `bucket`, `mtime`, `relative_time`, `is_dir`, `is_symlink`（按情况包含 `symlink_target`, `label`）

## 限制
- 固定深度 1（不递归）
- 只读（不会修改/删除任何文件）

## 相关文档
- 用户指南（日文）：`docs/USER-GUIDE-ja.md`
- CLI 详细（日文）：`docs/CLI-ja.md`
- 规格（日文）：`docs/SPEC-ja.md`
- 架构（日文）：`docs/ARCHITECTURE-ja.md`
- 测试计划（日文）：`docs/TESTPLAN-ja.md`
