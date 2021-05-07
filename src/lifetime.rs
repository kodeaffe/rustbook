fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//fn longest(x: & str, y: & str) -> & str {
//    let result = String::from("really long string");
//    result.as_str()
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
     */
}
