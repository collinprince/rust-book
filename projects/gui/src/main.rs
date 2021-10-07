use gui::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "SelectBox with width: {}, height: {}, & options: {:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    let my_screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 10,
                height: 20,
                options: vec![
                    String::from("hello"),
                    String::from("goodbye"),
                    String::from("mahalo"),
                ],
            }),
            Box::new(Button {
                width: 30,
                height: 10,
                label: String::from("click me"),
            }),
        ],
    };

    my_screen.run();
}
