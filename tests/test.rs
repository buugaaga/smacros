use smacros::s;

#[test]
fn test_empty() {
    let empty_str = s!();
    assert!(empty_str.is_empty());
    assert_eq!(empty_str, "");
}

#[test]
fn test_string_literal() {
    assert_eq!(s!("hi"), "hi");
}

#[test]
fn test_number() {
    let test_num = s!(42);
    assert_eq!(test_num, "42");
}

#[test]
fn test_concatination() {
    let str_literal = s!("hello", "hello");

    assert_eq!(str_literal, "hellohello");
}

#[test]
fn test_nested() {
    assert_eq!(s!(s!("inner")), "inner");
}

#[test]
fn test_special_chars() {
    assert_eq!(s!("Привет", ", ", "мир!"), "Привет, мир!");
}

#[test]
fn test_mixed_types() {
    assert_eq!(s!("id:", 123, true), "id:123true");
}

#[test]
fn test_many_args() {
    let many = s!("1", "2", 3, true);
    assert_eq!(many, "123true");
}
