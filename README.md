# Auto Clicker (自动点击器)

一个使用 Rust 编写的 Windows 自动点击器，通过鼠标侧键控制点击序列。

## 配置说明

编辑 `config.json` 文件设置点击坐标和延迟：

```json
[
  [[x1, y1, pre_delay, post_delay], [x2, y2, pre_delay, post_delay], ...],
  round_start_delay,
  round_end_delay
]
```

- `[x, y, pre_delay, post_delay]`: 点击坐标(x,y)和延迟时间(毫秒)
- `round_start_delay`: 每轮开始前等待时间(毫秒)
- `round_end_delay`: 每轮结束后等待时间(毫秒)

## 使用方法

1. **配置点击点**: 编辑 `config.json` 设置要点击的坐标位置
2. **运行程序**: 双击 `auto_clicker.exe` 或运行 `cargo run --release`
3. **控制点击**:
   - **开始**: 按住鼠标侧键 1 (XButton1)
   - **停止**: 松开鼠标侧键 1
   - **退出**: 按 ESC 键

**注意**: 程序需要管理员权限运行

## 快速开始

```bash
# 克隆项目
git clone https://github.com/luo1227/auto-clicker.git
cd auto-clicker

# 编译程序
cargo build --release

# 运行程序
cargo run --release
```

### 开发命令

- `npm run build`: 编译并复制可执行文件到根目录
- `npm run dist`: 构建并创建分发包

## 注意事项

- 需要管理员权限运行
- 仅支持 Windows 平台
- 游戏使用时请注意反作弊机制

## 许可证

MIT License - luo1227
