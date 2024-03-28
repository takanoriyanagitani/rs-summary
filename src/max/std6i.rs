pub fn max6i(s6i: &[i64], alt: i64) -> i64 {
    s6i.iter().max().copied().unwrap_or(alt)
}
