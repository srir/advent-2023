pub(crate) fn parse_numbers(s: &str) -> Result<Vec<usize>, ()> {
    s.split_whitespace().map(|n| n.parse()).collect::<Result<_, _>>().map_err(|_| ())
}

