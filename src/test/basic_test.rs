#[cfg(test)]

#[test]
fn it_works() {
    let result = 1;
    assert_eq!(2 + result, 3);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 4, 6);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
}

#[test]
fn it_does_not_work() {
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
    assert_eq!(2 + 2, 4);
}