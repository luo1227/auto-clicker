# Auto Clicker (自动点击器)

一个使用Rust编写的Windows自动点击器程序，通过鼠标侧键控制自动点击序列。

## 项目简介

该项目实现了一个高效的自动点击器，主要用于游戏、重复性任务自动化等场景。通过鼠标侧键1(XButton1)控制点击序列的开始和停止，使用ESC键退出程序。

## 主要特性

- **高精度点击**: 使用Windows API实现精确的鼠标坐标定位和点击
- **可配置延迟**: 支持每个点击点的独立延迟设置，以及每轮的全局延迟
- **实时控制**: 通过鼠标侧键1实时控制点击序列的开始/停止
- **线程安全**: 使用原子操作和Arc实现安全的并发控制
- **低CPU占用**: 优化了休眠机制，减少CPU占用
- **DPI感知**: 自动设置DPI感知，确保在高DPI显示器上正常工作

## 技术栈

- **编程语言**: Rust
- **GUI框架**: Windows API (Win32)
- **序列化**: Serde JSON
- **构建工具**: Cargo

## 项目结构

```
auto-click/
├── src/
│   └── main.rs              # 主程序文件
├── Cargo.toml               # Rust项目配置
├── config.json             # 配置文件
├── 使用说明.txt            # 用户使用说明
├── auto_clicker.exe        # 编译后的可执行文件
└── README.md               # 项目说明文档
```

## 配置文件格式

配置文件`config.json`使用JSON数组格式：

```json
[
  [[x1, y1, pre_delay1, post_delay1], [x2, y2, pre_delay2, post_delay2], ...],
  pre_round_delay,
  post_round_delay
]
```

### 参数说明

- **坐标点列表**: 二维数组，每个子数组包含4个数字 `[x, y, pre_delay, post_delay]`
  - `x, y`: 点击坐标（像素）
  - `pre_delay`: 点击前延迟（毫秒）
  - `post_delay`: 点击后延迟（毫秒）
- **pre_round_delay**: 每轮点击开始前的延迟（毫秒）
- **post_round_delay**: 每轮点击结束后的延迟（毫秒）

### 配置示例

```json
[
  [[100, 200, 50, 100], [300, 400, 0, 200], [500, 600, 100, 150]],
  1000,
  2000
]
```

此配置表示：
- 依次点击(100,200)、(300,400)、(500,600)三个位置
- 每个点击有独立的预处理和后处理延迟
- 每轮开始前等待1000ms
- 每轮结束后等待2000ms

## 控制方式

- **启动点击**: 按住鼠标侧键1(XButton1)
- **停止点击**: 松开鼠标侧键1
- **退出程序**: 按ESC键

## 开发环境搭建

### 环境要求

- Rust 1.70+
- Windows 10/11
- Visual Studio Build Tools (用于Windows API)

### 快捷命令

项目提供了几个npm脚本命令来简化开发和分发流程：

- **`npm run build`**: 编译release版本并将可执行文件复制到项目根目录
- **`npm run dist`**: 执行完整的分发流程，包括构建和打包

分发命令会自动：
1. 构建最新的release程序
2. 将exe文件复制到根目录
3. 创建`dist`目录（如果不存在）
4. 将`auto_clicker.exe`、`config.json`和`使用说明.txt`压缩成zip包
5. 输出到`dist/auto-clicker-v{版本号}.zip`

### 编译运行

1. 克隆项目
```bash
git clone https://github.com/luo1227/auto-clicker.git
cd auto-clicker
```

2. 编译项目
```bash
cargo build --release
```

3. 配置点击点
编辑`config.json`文件设置点击坐标和延迟

4. 运行程序
```bash
cargo run --release
# 或者直接运行编译后的exe
./target/release/auto_clicker.exe
```

## 核心实现原理

### 鼠标点击模拟

使用Windows `SendInput` API实现精确的鼠标点击：

```rust
fn simulate_click(x: i32, y: i32) -> Result<()> {
    // 设置鼠标位置
    SetCursorPos(x, y)?;

    // 发送鼠标按下和松开事件
    let inputs = [
        INPUT { /* MOUSEEVENTF_LEFTDOWN */ },
        INPUT { /* MOUSEEVENTF_LEFTUP */ }
    ];

    SendInput(&mut inputs, size_of::<INPUT>() as i32);
    Ok(())
}
```

### 并发控制

使用原子布尔值和Arc实现安全的线程间通信：

```rust
let should_stop = Arc::new(AtomicBool::new(false));
let should_stop_clone = Arc::clone(&should_stop);

// 在主线程中
should_stop.store(true, Ordering::Relaxed);

// 在子线程中检查
if should_stop.load(Ordering::Relaxed) {
    break;
}
```

### 高精度延迟

使用纳秒级精度的`Duration::from_nanos()`实现精确延迟控制：

```rust
fn high_precision_sleep(milliseconds: u64) {
    let nanos = milliseconds.saturating_mul(1_000_000);
    std::thread::sleep(Duration::from_nanos(nanos));
}
```

## 性能优化

1. **分段休眠**: 将长延迟分成多个短延迟，在每个短延迟间检查停止信号
2. **自适应休眠**: 根据运行状态调整休眠时间，减少CPU占用
3. **原子操作**: 使用Relaxed内存序优化性能

## 注意事项

1. **权限要求**: 需要管理员权限运行（某些游戏可能需要）
2. **坐标精度**: 坐标基于屏幕像素，建议使用屏幕坐标拾取工具获取准确坐标
3. **游戏兼容性**: 部分游戏有反作弊检测，请谨慎使用
4. **系统兼容性**: 仅支持Windows平台

## 许可证

MIT License

## 贡献

欢迎提交Issue和Pull Request来改进这个项目！

## 作者

luo1227