pub fn add(left: u64, right: u64) -> u64 {
    left.checked_add(right).expect("Integer overflow in addition calculation")
}
