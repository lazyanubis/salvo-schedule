// #![deny(clippy::unwrap_used)]
// #![deny(clippy::expect_used)]
// #![deny(clippy::panic)]

// use salvo::catcher::Catcher;
// use salvo::cors::Cors;
// use salvo::http::Method;
// use salvo::prelude::*;

// #[tokio::main]
// async fn main() {
//     // 定时任务
//     let mut scheduled = scheduled::ScheduledTasks::new();
//     scheduled
//         .push(scheduled::Scheduled::None, scheduled_test_none)
//         .await;
//     scheduled
//         .push(
//             scheduled::Scheduled::FixedRate(scheduled::ScheduledFixedRate::new(None, 5000.into())),
//             scheduled_test_none,
//         )
//         .await;
//     scheduled
//         .push(
//             scheduled::Scheduled::FixedDelay(scheduled::ScheduledFixedDelay::new(
//                 None,
//                 5000.into(),
//             )),
//             |scheduled| {
//                 Box::new(async move {
//                     tracing::info!("scheduled: {:?}", scheduled);

//                     None
//                 })
//             },
//         )
//         .await;
//     scheduled
//         .push(
//             scheduled::Scheduled::Cron(scheduled::ScheduledCron::new(None, "0/5 * * * * *", None)),
//             |scheduled| {
//                 Box::new(async move {
//                     tracing::info!("scheduled: {:?}", scheduled);

//                     None
//                 })
//             },
//         )
//         .await;
//     let Task_id = scheduled
//         .push(
//             scheduled::Scheduled::Calculate(scheduled::ScheduledCalculate::new(
//                 None,
//                 |last, next| None,
//             )),
//             |scheduled| {
//                 Box::new(async move {
//                     tracing::info!("scheduled: {:?}", scheduled);

//                     None
//                 })
//             },
//         )
//         .await;
//     scheduled.remove(Task_id).await;

//     server.serve(service).await;
// }

// fn scheduled_test_none(
//     scheduled: scheduled::Scheduled,
// ) -> Box<dyn futures::Future<Output = Option<scheduled::ScheduledTaskResult>>> {
//     Box::new(async move {
//         tracing::info!("scheduled: {:?}", scheduled);

//         None
//     })
// }
