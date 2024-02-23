use std::{
    collections::{HashMap, VecDeque},
    future::Future,
};

/// 时间戳 毫秒 ms
#[derive(Debug)]
pub struct TimestampMills(u128);

impl From<u128> for TimestampMills {
    fn from(value: u128) -> Self {
        TimestampMills(value)
    }
}

/// 任务 id
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ScheduledTaskId(usize);

/// 定时 - 只执行一次
#[derive(Debug)]
pub struct ScheduledOneShot {
    initial_delay: Option<TimestampMills>, // 延迟启动时间
}

/// 定时 - 每次完成任务后延迟固定时间
#[derive(Debug)]
pub struct ScheduledFixedDelay {
    initial_delay: Option<TimestampMills>, // 延迟启动时间
    fixed_delay: TimestampMills,           // 每次执行完毕后，等待多长时间
}

/// 定时 - 固定频率启动任务
#[derive(Debug)]
pub struct ScheduledFixedRate {
    initial_delay: Option<TimestampMills>, // 延迟启动时间
    fixed_rate: TimestampMills,            // 执行间隔
}

/// 定时 - CRON 表达式
#[derive(Debug)]
pub struct ScheduledCron {
    initial_delay: Option<TimestampMills>, // 延迟启动时间
    cron: String,                          // cron 表达式
    zone: Option<String>,                  // 指定时区
}

/// 定时 - 自定义触发时间
#[derive(Debug)]
pub struct ScheduledCustom {
    initial_delay: Option<TimestampMills>, // 延迟启动时间
    custom: fn(
        count: usize,                      // 该任务总执行次数
        last: Option<TimestampMills>,      // 该任务上次执行时间
        last_done: Option<TimestampMills>, // 该任务上次执行完成时间
    ) -> Option<TimestampMills>, // 计算下次的时间
}

/// 定时
#[derive(Debug)]
pub enum Scheduled {
    None,
    OneShot(ScheduledOneShot),
    FixedDelay(ScheduledFixedDelay),
    FixedRate(ScheduledFixedRate),
    Cron(ScheduledCron),
    Custom(ScheduledCustom),
}

pub struct ScheduledTaskRunning {
    start: TimestampMills,       // 开始时间
    end: Option<TimestampMills>, // 结束时间
    handler: Option<u8>,         // TODO 任务句柄
}

pub enum ScheduledState {
    Initial {
        tick: TimestampMills, // 下次触发时间，不代表启动时间
    }, // 刚启动，尚未到启动时间
    Running {
        // 统计数据
        count: usize,                                // 总执行次数
        all_cost: TimestampMills,                    // 总执行时长
        latest_cost: VecDeque<ScheduledTaskRunning>, // 记录最近 x 次的记录时长

        tick: TimestampMills,         // 下次触发时间，不代表启动时间
        next: Option<TimestampMills>, // 下次执行时间

        running: HashMap<usize, ScheduledTaskRunning>, // 当前正在执行的任务
    }, // 未到时间
    Finish, // 已经开始
}

pub struct ScheduledData {
    scheduled: Scheduled,
    state: ScheduledState,
}

pub enum ScheduledTaskResult {
    None,
    Scheduled(Scheduled), // 修改自身定时器
    Removed,              // 移除该任务
}

type ScheduledTaskFunction = fn(
    scheduled: Scheduled,
    state: ScheduledState,
) -> Box<dyn Future<Output = Option<ScheduledTaskResult>>>;

pub struct ScheduledTask {
    task_id: ScheduledTaskId,
    scheduled: Scheduled,
    state: ScheduledState,
    task: ScheduledTaskFunction,
}
