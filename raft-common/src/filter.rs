pub trait Filter {
    fn do_filter(&self);
}
#[derive(Default)]
pub struct FilterChain;

type F = Vec<Box<dyn Filter>>;

static mut FILTER: F = F::new();

impl FilterChain {
    pub fn init() {
        unsafe { FILTER.insert(0, Box::new(DefaultFilter::default())) };
    }

    pub fn add_filter<T>(file: T)
    where
        T: Filter + 'static,
    {
        unsafe { FILTER.push(Box::new(file)) };
    }
}
#[derive(Default)]
struct DefaultFilter;

impl Filter for DefaultFilter {
    fn do_filter(&self) {
        println!("default filter");
    }
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
        FilterChain::add_filter(m2);
        FilterChain::add_filter(m1);
        unsafe {
            for d in FILTER.iter() {
                d.do_filter();
            }
        }
    }
}
