
#[cfg(test)]
mod tests {
    use crate::calculate_distance;

    #[test]
    fn calculate_distance_test() {
        assert_eq!(calculate_distance(7, 0), 0);
        assert_eq!(calculate_distance(7, 1), 6);
        assert_eq!(calculate_distance(7, 2), 10);
        assert_eq!(calculate_distance(7, 3), 12);
        assert_eq!(calculate_distance(7, 4), 12);
        assert_eq!(calculate_distance(7, 5), 10);
        assert_eq!(calculate_distance(7, 6), 6);
        assert_eq!(calculate_distance(7, 7), 0);
    }
}