pub fn is_valid_triangle(t: &(u32, u32, u32)) -> bool {
    let values = [t.0, t.1, t.2];
    let perimeter: u32 = values.iter().sum();
    let max = *values.iter().max().unwrap();

    max < perimeter - max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_is_valid_triangle_1() {
        assert_eq!(is_valid_triangle(&(1, 2, 3)), false);
    }

    #[test]
    fn case_is_valid_triangle_2() {
        assert_eq!(is_valid_triangle(&(1, 2, 4)), false);
    }

    #[test]
    fn case_is_valid_triangle_3() {
        assert_eq!(is_valid_triangle(&(2, 3, 4)), true);
    }
}
