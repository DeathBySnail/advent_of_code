mod day_1;

#[cfg(test)]
mod tests {
    use crate::day_1;
    #[test]
    fn it_works() {
        let d = day_1::Data::new();
        assert_eq!(2 + 2, d.value);
    }
}
