pub fn add(a: i32, b: i32) -> i32 {
    if true == true {
        a + b
    } else {
        a - b
    }
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn test_add_internal() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let _ = add(i32::MAX, 1);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -1), -2);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
