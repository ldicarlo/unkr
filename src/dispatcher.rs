pub fn dispatch(thread_count: usize, combination_count: usize) -> Vec<usize> {
    internal_dispatch(vec![], thread_count, combination_count)
}

pub fn internal_dispatch(
    mut acc: Vec<usize>,
    thread_count: usize,
    combination_count: usize,
) -> Vec<usize> {
    if thread_count == 0 {
        return acc;
    }

    let average = combination_count / thread_count;

    acc.push(average);

    internal_dispatch(acc, thread_count - 1, combination_count - average)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works_1() {
        assert_eq!(dispatch(2, 10), vec![5, 5]);
        assert_eq!(dispatch(5, 6), vec![1, 1, 1, 1, 2]);
        assert_eq!(dispatch(4, 2), vec![0, 0, 1, 1]);
    }
}
