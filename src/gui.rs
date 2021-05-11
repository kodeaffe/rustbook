trait Draw {
    fn draw(&self);
}


struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        println!("Drawing components ...");
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
        println!("Drawing button: {}, {}, {}", self.width, self.height, &self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing select box: {}, {}, {:?}", self.width, self.height, &self.options);
    }
}


pub fn run() {
    let screen = Screen {
        components: vec![
            Box::new(Button { width: 2, height: 3, label: "save".to_string() }),
            Box::new(Button { width: 1, height: 1, label: "cancel".to_string() }),
            Box::new(SelectBox {
                width: 5,
                height: 7,
                options: vec!("do a".to_string(), "do b".to_string(), "do c".to_string()),
            }),
        ],
    };
    screen.run();
}