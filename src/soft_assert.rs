#[macro_export]
macro_rules! soft_assert {
    ($condition:expr, $msg:literal $(,)*) => {{
        use std::sync::atomic::{AtomicBool, Ordering};

        static TRIPPED: AtomicBool = AtomicBool::new(false);
        if !$condition && !TRIPPED.swap(true, Ordering::Relaxed) {
            eprintln!("Soft assert violated; msg={}; file={}; line={}", $msg, file!(), line!());
        }
    }};

    ($condition:expr $(,)*) => {{
        use std::sync::atomic::{AtomicBool, Ordering};

        static TRIPPED: AtomicBool = AtomicBool::new(false);
        if !$condition && !TRIPPED.swap(true, Ordering::Relaxed) {
            eprintln!("Soft assert violated; file={}; line={}", file!(), line!());
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn soft_assert() {
        soft_assert!(true, "msg0");
        soft_assert!(false, "msg1");
        soft_assert!(true);
        soft_assert!(false);
        soft_assert!(true, "msg0",);
        soft_assert!(false, "msg1",);
        soft_assert!(true,);
        soft_assert!(false,);
    }
}
