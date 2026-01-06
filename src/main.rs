use serde::Deserialize;
use std::{
    fs, sync::{Arc, atomic::{AtomicBool, Ordering}}, thread, time::Duration
};
use windows::{
    core::*,
    Win32::UI::{
        Input::KeyboardAndMouse::*,
        WindowsAndMessaging::*,
    },
};

/// 配置结构体，包含点击点信息和延迟设置
#[derive(Debug, Deserialize, Clone)]
struct Config {
    /// 点击点数组，每个点包含 [x坐标, y坐标, 点击前延迟(ms), 点击后延迟(ms)]
    points: Vec<[i32; 4]>,
    /// 每轮点击前的延迟时间(毫秒)
    pre_round_delay: u64,
    /// 每轮点击后的延迟时间(毫秒)
    post_round_delay: u64,
}

impl Config {
    /// 从JSON文件加载配置
    fn from_file(filename: &str) -> Result<Self> {
        // 读取配置文件内容
        let content = fs::read_to_string(filename)
            .map_err(|_| Error::new(HRESULT(0), "无法读取配置文件"))?;

        // 直接解析为配置结构体
        serde_json::from_str(&content)
            .map_err(|_| Error::new(HRESULT(0), "配置文件格式错误"))
    }
}

/// 高精度睡眠函数，用于精确控制延迟时间
fn high_precision_sleep(milliseconds: u64) {
    if milliseconds == 0 {
        return;
    }

    // 将毫秒转换为纳秒，避免浮点数精度问题
    let nanos = milliseconds.saturating_mul(1_000_000);
    std::thread::sleep(Duration::from_nanos(nanos));
}

/// 设置DPI感知
fn set_dpi_awareness() -> Result<()> {
    unsafe {
        // 设置系统DPI感知
        if SetProcessDPIAware().as_bool() {
            return Ok(());
        }

        Err(Error::new(HRESULT(0), "无法设置DPI感知"))
    }
}

/// 模拟鼠标左键点击
fn simulate_click(x: i32, y: i32) -> Result<()> {
    unsafe {
        // 先使用SetCursorPos设置鼠标位置
        SetCursorPos(x, y).map_err(|_| Error::new(HRESULT(0), "设置鼠标位置失败"))?;

        let mut inputs = [
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_LEFTDOWN,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
            INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 {
                    mi: MOUSEINPUT {
                        dx: 0,
                        dy: 0,
                        mouseData: 0,
                        dwFlags: MOUSEEVENTF_LEFTUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];

        // 发送鼠标按下和松开事件
        let sent = SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);
        if sent != inputs.len() as u32 {
            return Err(Error::new(HRESULT(0), "鼠标点击模拟失败"));
        }

        Ok(())
    }
}

/// 检查鼠标侧键1是否被按下
fn is_side_button1_pressed() -> bool {
    unsafe {
        // 检查侧键1按下状态
        (GetAsyncKeyState(VK_XBUTTON1.0 as i32) as u16 & 0x8000) != 0
    }
}

/// 检查ESC键是否被按下
fn is_escape_pressed() -> bool {
    unsafe {
        // 检查ESC键按下状态
        (GetAsyncKeyState(VK_ESCAPE.0 as i32) as u16 & 0x8000) != 0
    }
}

/// 执行点击序列（在子线程中运行）
fn execute_click_sequence(config: &Config, should_stop: Arc<AtomicBool>) {
    // 持续循环执行点击序列，直到收到停止信号
    loop {
        // 每次循环开始时检查是否应该停止
        if should_stop.load(Ordering::Relaxed) {
            break;
        }

        // 每轮开始前的延迟
        if config.pre_round_delay > 0 {
            let mut remaining_delay = config.pre_round_delay;
            while remaining_delay > 0 && !should_stop.load(Ordering::Relaxed) {
                let sleep_time = if remaining_delay > 10 { 10 } else { remaining_delay };
                high_precision_sleep(sleep_time);
                remaining_delay -= sleep_time;
            }
        }

        // 如果已经收到停止信号，直接返回
        if should_stop.load(Ordering::Relaxed) {
            break;
        }

        // 执行一轮点击序列
        for point in &config.points {
            // 检查是否应该停止
            if should_stop.load(Ordering::Relaxed) {
                break;
            }

            let [x, y, pre_delay, post_delay] = *point;

            // 点击前延迟 - 分段检查停止信号
            if pre_delay > 0 {
                let mut remaining_delay = pre_delay as u64;
                while remaining_delay > 0 && !should_stop.load(Ordering::Relaxed) {
                    let sleep_time = if remaining_delay > 10 { 10 } else { remaining_delay };
                    high_precision_sleep(sleep_time);
                    remaining_delay -= sleep_time;
                }
            }

            // 再次检查停止信号
            if should_stop.load(Ordering::Relaxed) {
                break;
            }

            // 执行点击
            let _ = simulate_click(x, y);

            // 点击后延迟 - 分段检查停止信号
            if post_delay > 0 {
                let mut remaining_delay = post_delay as u64;
                while remaining_delay > 0 && !should_stop.load(Ordering::Relaxed) {
                    let sleep_time = if remaining_delay > 10 { 10 } else { remaining_delay };
                    high_precision_sleep(sleep_time);
                    remaining_delay -= sleep_time;
                }
            }
        }

        // 每轮结束后的延迟
        if config.post_round_delay > 0 {
            let mut remaining_delay = config.post_round_delay;
            while remaining_delay > 0 && !should_stop.load(Ordering::Relaxed) {
                let sleep_time = if remaining_delay > 10 { 10 } else { remaining_delay };
                high_precision_sleep(sleep_time);
                remaining_delay -= sleep_time;
            }
        }
    }
}

fn main() -> Result<()> {
    // 设置DPI感知，确保坐标精度
    let _ = set_dpi_awareness();

    // 从配置文件加载设置
    let config = Config::from_file("config.json")
        .map_err(|_| Error::new(HRESULT(0), "配置文件不存在或格式错误"))?;

    // 创建共享的停止标志
    let should_stop = Arc::new(AtomicBool::new(false));

    // 跟踪是否有正在运行的点击线程
    let mut click_thread_handle: Option<thread::JoinHandle<()>> = None;

    // 主循环：监听鼠标侧键1控制自动点击
    loop {
        // 检查ESC键退出程序
        if is_escape_pressed() {
            // 设置停止标志，让正在运行的点击线程停止
            should_stop.store(true, Ordering::Relaxed);

            // 等待点击线程结束
            if let Some(handle) = click_thread_handle.take() {
                let _ = handle.join();
            }
            break;
        }

        let side_button1_pressed = is_side_button1_pressed();

        // 如果侧键1被按下且当前没有点击线程在运行
        if side_button1_pressed && click_thread_handle.is_none() {
            // 重置停止标志
            should_stop.store(false, Ordering::Relaxed);

            // 克隆配置和共享状态
            let config_clone = config.clone();
            let should_stop_clone = Arc::clone(&should_stop);

            // 启动点击线程
            click_thread_handle = Some(thread::spawn(move || {
                execute_click_sequence(&config_clone, should_stop_clone);
            }));
        }
        // 如果侧键1松开但有点击线程在运行
        else if !side_button1_pressed && click_thread_handle.is_some() {
            // 设置停止标志
            should_stop.store(true, Ordering::Relaxed);

            // 等待点击线程结束
            if let Some(handle) = click_thread_handle.take() {
                let _ = handle.join();
            }
        }
        // 如果没有点击线程在运行，降低CPU占用率
        else if click_thread_handle.is_none() {
            high_precision_sleep(10);
        }

        // 如果有点击线程在运行，短暂延迟后继续检查按键状态
        if click_thread_handle.is_some() {
            high_precision_sleep(1);
        }
    }

    Ok(())
}