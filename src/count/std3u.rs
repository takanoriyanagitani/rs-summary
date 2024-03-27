pub fn countif3u_ge(s: &[u8], lbi: u8) -> i32 {
    let i = s.iter();
    let filtered = i.filter(|&u: &&u8| lbi <= *u);
    filtered.count().try_into().ok().unwrap_or(-1)
}
