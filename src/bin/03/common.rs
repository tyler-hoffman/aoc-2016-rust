pub fn solve(triangles: Vec<(u32, u32, u32)>) -> usize {
    triangles.iter().filter(|t| is_valid_triangle(*t)).count()
}

fn is_valid_triangle(t: &(u32, u32, u32)) -> bool {
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

    #[test]
    fn solve_case_0() {
        assert_eq!(solve(vec![(1, 1, 3)]), 0);
    }

    #[test]
    fn solve_case_1() {
        assert_eq!(solve(vec![(2, 3, 4)]), 1);
    }

    #[test]
    fn solve_case_2() {
        assert_eq!(solve(vec![(2, 3, 4), (1, 1, 1), (1, 1, 2)]), 2);
    }
}
