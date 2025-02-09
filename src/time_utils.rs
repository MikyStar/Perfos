use std::time::Duration;

use humanize_duration::{prelude::DurationExt, Truncate};

////////////////////////////////////////

pub fn nano_to_hr(time: Duration) -> String {
    time.human(Truncate::Nano).to_string()
}

pub fn seconds_to_hr(time: Duration) -> String {
    time.human(Truncate::Second).to_string()
}

////////////////////////////////////////

#[macro_export]
macro_rules! time {
    ($func:expr) => {{
        let start = Instant::now();
        let result = $func();
        let duration = start.elapsed();
        duration
    }};
}
