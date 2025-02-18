#[test]
#[should_panic]
fn test_basic() {
    assert!(1 == 1);
    panic!("oh no");
}

#[test]
fn test_equals() {
    assert_eq!(4, 2 + 2);
    assert_ne!(4, 2 + 3);
}

#[test]
fn test_return_me_two() {
    assert_eq!(2, super::for_tests::return_me_two());
}

#[test]
fn test_is_square() {
    assert!(super::for_tests::Rectangle {
        width: 2,
        height: 2
    }
    .is_square());
    assert!(!super::for_tests::Rectangle {
        width: 3,
        height: 2
    }
    .is_square());
}
