// use super::*;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

fn add(left: u64, right: u64) -> u64 {
    left + right
}
