#[derive(Debug)]
pub enum Msg {
    Click,
}

#[derive(Default)]
pub struct Store {
    click_count: u32,
    listeners: Vec<Box<Fn()>>,
}

impl Store {
    pub fn new(count: u32) -> Self {
        Store {
            click_count: count,
            ..Default::default()
        }
    }

    pub fn msg(&mut self, msg: &Msg) {
        match msg {
            Msg::Click => self.increment_click(),
        };

        // Whenever we update state we'll let all of our state listeners know that state was
        // updated
        for callback in self.listeners.iter() {
            callback();
        }
    }

    pub fn click_count(&self) -> u32 {
        self.click_count
    }

    fn increment_click(&mut self) {
        self.click_count += 1;
    }

    pub fn subscribe(&mut self, callback: Box<Fn()>) {
        self.listeners.push(callback);
    }
}
