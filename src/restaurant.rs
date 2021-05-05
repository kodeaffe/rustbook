pub mod front_of_house;
pub mod back_of_house;

use front_of_house::{hosting, serving};
use back_of_house::kitchen;

pub fn run() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    kitchen::cook_order();
    serving::serve_order();
    serving::fix_incorrect_order();
    serving::take_payment();
    hosting::say_goodbye();
}