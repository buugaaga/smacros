/// Creates a `String` from any value that implements`ToString`.
///
/// Concatenates multiple values into a single `String`.
///
/// This macro takes any number of expressions and concatenates their string representations
/// into a single `String`.
///
/// Accept strings, numbers, boolean
/// # Examples
/// ```
/// use s_macro::s;
///
/// let str = s!("hello");
/// let num = s!(42);
/// let concats = s!("hi", " ", 42, "now");
/// ```
#[macro_export]
macro_rules! s {
    () => { String::new() };

    ($e:expr) => { $e.to_string() };

    ($e:expr, $($rest:tt)*) => {
        $e.to_string() + &s!($($rest)*)
    };
}
