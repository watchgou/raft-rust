use std::any::Any;

pub trait Filter {
    fn do_filter(&self, request: Box<dyn Any>);
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
    fn do_filter(&self, _: Box<dyn Any>) {
        println!("default filter");
    }
}
