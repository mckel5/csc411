pub fn square(x: i16) -> i32 {
    (x as i32) * (x as i32)
}

pub fn find_minimum<T: Ord>(v: Vec<T>) -> T {
    v.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::square;
    use crate::find_minimum;

    #[test]
    fn square_two() {
        assert_eq!(square(2), 4);
    }

    #[test]
    fn square_neg_three() {
        assert_eq!(square(-3), 9);
    }

    #[test]
    fn square_10k() {
        assert_eq!(square(10000), 10000*10000);
    }

    #[test]
    fn min_neg_ten() {
        assert_eq!(find_minimum(vec![-10,0,10,20]), -10);
    }

    #[test]
    fn min_fifteen() {
        assert_eq!(find_minimum(vec![100, 200, 15, 16, 15, 15, 15]), 15);
    }
}
