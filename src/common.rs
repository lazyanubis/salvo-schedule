use crate::TimestampMills;

/// 默认获取当前时间戳
#[allow(unused)]
#[inline]
pub(crate) fn default_now() -> TimestampMills {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    #[allow(clippy::expect_used)] // ? checked
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    (since_the_epoch.as_millis()).into()
}
