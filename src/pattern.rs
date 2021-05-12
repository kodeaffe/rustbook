#![allow(dead_code)]

fn color() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    //let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn setting() {
    let mut setting_value = Some(5);
    //let mut setting_value = None;
    let new_setting_value = None;
    //let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            println!("Overriding setting");
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
}


fn remaining() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
        /*
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
         */
    }
}

fn guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = true;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn binding() {
    enum Message {
        Hello { id: i32 },
    }
    use Message::Hello;

    let msg = Hello { id: 6 };

    match msg {
        Hello { id: 3..=5, } => println!("Found an id in range"),
        Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Hello { id } => println!("Found some other id: {}", id),
    }
}

pub fn run() {
    //color();
    //foo(3, 4);
    //setting();
    //remaining();
    //guard();
    binding();
}