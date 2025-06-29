use once_cell::sync::Lazy;
use std::any::Any;
use std::sync::Mutex;

pub trait Filter: Send + Sync {
    fn accept(&self, request: &dyn Any);
}

#[derive(Default)]
pub struct FilterManager;

type F = Lazy<Mutex<Vec<Box<dyn Filter>>>>;

static FILTER: F = F::new(|| Mutex::new(Vec::new()));

impl FilterManager {
    pub fn add_filter<T>(file: T)
    where
        T: Filter + 'static,
    {
        let mut filters = FILTER.lock().unwrap();

        filters.push(Box::new(file));
    }

    pub fn clear_filters() {
        let mut filters = FILTER.lock().unwrap();

        filters.clear();
    }

    pub fn apply_filters(&self, request: &dyn Any) {
        let filters = FILTER.lock().unwrap();
        for filter in filters.iter() {
            filter.accept(request);
        }
    }

    pub fn new() -> Self {
        Self
    }
}
