// use std::{
//     collections::HashMap,
//     ops::{Deref, DerefMut},
// };

// use futures::{lock::Mutex, Future};
// use tokio::sync::RwLock;

// struct Tasks(HashMap<ScheduledTaskId, ScheduledTask>);

// impl Deref for Tasks {
//     type Target = HashMap<ScheduledTaskId, ScheduledTask>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl DerefMut for Tasks {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl Tasks {
//     fn new() -> Self {
//         Self(HashMap::new())
//     }
// }

// pub struct ScheduledTasks {
//     created: TimestampMills, // 任务创建时间
//     next_id: usize,          // 每次生成任务返回一个唯一的 id，用以移除
//     checked: TimestampMills, // 上次检查时间，用以计算下次任务时间
//     Tasks: RwLock<Tasks>,      // 所有的任务 id

//     executor: Mutex<Option<tokio::task::JoinHandle<()>>>, // 当前任务的执行器
// }

// impl ScheduledTasks {
//     pub fn new() -> Self {
//         Self {
//             created: now(),
//             next_id: 0,
//             checked: TimestampMills(0),
//             Tasks: RwLock::new(Tasks::new()),

//             executor: Mutex::new(None),
//         }
//     }

//     pub async fn push(&mut self, scheduled: Scheduled, Task: ScheduledTaskFunction) -> ScheduledTaskId {
//         let Task_id = ScheduledTaskId(self.next_id);
//         self.next_id += 1;

//         self.Tasks
//             .write()
//             .await
//             .insert(Task_id, ScheduledTask { Task_id, scheduled, Task });

//         self.execute().await;

//         Task_id
//     }

//     pub async fn remove(&mut self, Task_id: ScheduledTaskId) {
//         self.Tasks.write().await.remove(&Task_id);
//     }

//     async fn execute(&mut self) {
//         if let Some(join_handler) = self.executor.lock().await.as_ref() {
//             if !join_handler.is_finished() {
//                 return;
//             }
//         }

//         let
//         // 新启动一个任务
//         let join_handler = tokio::spawn(async move {
//             loop {
//                 // 任务内容
//                 // 1. 检查是否有到当前时间的，执行任务
//                 let now = now();
//                 // 2. 检查下一个最近的任务时间点
//                 let Tasks = self.Tasks.write().await;

//                 // 3. 睡眠等待相差的时间
//             }
//         });

//         *self.executor.lock().await = Some(join_handler);
//     }
// }

// impl ScheduledCron {
//     pub fn new(initial_delay: Option<TimestampMills>, cron: impl Into<String>, zone: Option<String>) -> Self {
//         Self {
//             initial_delay,
//             cron: cron.into(),
//             zone,

//             next: None,
//         }
//     }
// }

// impl ScheduledFixedDelay {
//     pub fn new(initial_delay: Option<TimestampMills>, fixed_delay: TimestampMills) -> Self {
//         Self {
//             initial_delay,
//             fixed_delay,
//             last: None,
//             next: None,
//         }
//     }
// }

// impl ScheduledFixedRate {
//     pub fn new(initial_delay: Option<TimestampMills>, fixed_rate: TimestampMills) -> Self {
//         Self {
//             initial_delay,
//             fixed_rate,
//             next: None,
//         }
//     }
// }

// impl ScheduledCalculate {
//     pub fn new(
//         initial_delay: Option<TimestampMills>,
//         calculate: fn(last: Option<TimestampMills>, index: usize) -> Option<TimestampMills>,
//     ) -> Self {
//         Self {
//             initial_delay,
//             calculate,

//             last: None,
//             index: 0,
//             next: None,
//         }
//     }
// }
