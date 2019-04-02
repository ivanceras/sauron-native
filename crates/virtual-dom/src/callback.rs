use std::convert::Into;

pub struct Callback<IN>(Box<dyn Fn(IN)>);

impl<IN, F: Fn(IN) + 'static> From<F> for Callback<IN> {
    fn from(func: F) -> Self {
        Callback(Box::new(func))
    }
}

impl<IN> Callback<IN> {
    /// This method calls the actual callback.
    pub fn emit<T:Into<IN>>(&self, value: T) {
        (self.0)(value.into());
    }
}

impl<IN> PartialEq for Callback<IN> {

    fn eq(&self, rhs: &Self) -> bool {
        true
    }
}
