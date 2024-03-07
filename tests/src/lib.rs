mod math;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::math::sum;
        let result = sum(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_fails() {
        panic!("This test fails")
    }
    #[test]
    fn it_assets() {
        assert!(2 + 2 == 4)
    }
}
