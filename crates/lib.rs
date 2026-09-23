pub fn compute_exposure(val1: u64, val2: u64) -> u64 {
    val1.wrapping_sub(val2)
}
