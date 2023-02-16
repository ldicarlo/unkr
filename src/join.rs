pub fn join_seed(strs: Vec<String>, _: u64) -> Vec<String> {
    join(strs)
}

pub fn join(strs: Vec<String>) -> Vec<String> {
    vec![strs.join("")]
}
