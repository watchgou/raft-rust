pub trait Filter {
    fn do_filter(&self);
}

type F = Vec<Box<dyn Filter>>;

pub static mut FILTER: F = F::new();

pub fn add_filter<T>(file: T)
where
    T: Filter + 'static,
{
    unsafe { FILTER.push(Box::new(file)) };
}

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
        add_filter(m2);
        add_filter(m1);
        unsafe {
            for d in FILTER.iter() {
                d.do_filter();
            }
        }
    }
}
