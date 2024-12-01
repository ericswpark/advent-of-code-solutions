use helpers::get_input as helpers_get_input;

pub(crate) fn get_input(path: &str) -> Vec<Vec<char>> {
    helpers_get_input(path)
        .iter()
        .map(|s| s.chars().collect())
        .collect()
}
