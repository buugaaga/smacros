#[macro_export]
macro_rules! s {
    () => { String::new() };

    ($e:expr) => { $e.to_string() };

    ($e:expr, $($rest:tt)*) => {
        $e.to_string() + &s!($($rest)*)
    };
}
