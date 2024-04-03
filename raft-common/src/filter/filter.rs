pub trait Filter {
    fn do_filter(&self);
}

pub type F = Vec<Box<dyn Filter>>;

#[cfg(test)]
mod test {

    use super::*;

    struct M1;

    impl Filter for M1 {
        fn do_filter(&self) {
            println!("m1");
        }
    }

    struct M2;

    impl Filter for M2 {
        fn do_filter(&self) {
            println!("m2");
        }
    }

    #[test]
    fn test_filet() {
        let m1 = M1;
        let m2 = M2;
        let mut data = F::new();
        data.push(Box::new(m1));
        data.insert(0, Box::new(m2));
        for d in data {
            d.do_filter();
        }
    }
}
