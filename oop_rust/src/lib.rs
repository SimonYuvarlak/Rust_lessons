// Diger OOP dillerde olan class mantiginin Rust taki implementasyon ornegi
// Class gbi struct imizi tanimliyoruz ama icinde sadece field lar var
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// Normal OOP konseptinde bir class icinde bulunur field lar da fonksiyonlar da 
// Burada ise field lar struct la ayrilmisken, fonksiyonlar impl ile ayriliyor
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Trait Objects
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

pub trait Draw {
    fn draw(&self);
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

