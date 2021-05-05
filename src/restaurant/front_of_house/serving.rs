use super::super::back_of_house;

pub fn take_order() { println!("Take order"); }
pub fn serve_order() { println!("Serve order"); }
pub fn take_payment() { println!("Take payment"); }

pub fn fix_incorrect_order() {
        println!("Fix incorrect order");
        back_of_house::kitchen::cook_order();
        serve_order();
    }
