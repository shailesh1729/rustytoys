extern crate rtoys;

#[test]
fn foo() {
    assert!(true);
}

#[test]
fn math_checks_out() {
    let result = rtoys::add_three_times_four(5i);

    assert_eq!(32i, result);
}
