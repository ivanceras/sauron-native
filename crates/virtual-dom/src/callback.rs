
pub struct Callback<IN>(Box<dyn Fn(IN)>);

impl<IN, F: Fn(IN) + 'static> From<F> for Callback<IN> {
    fn from(func: F) -> Self {
        Callback(Box::new(func))
    }
}

impl<IN> Callback<IN> {
    /// This method calls the actual callback.
    pub fn emit(&self, value: IN) {
        (self.0)(value);
    }
}
