use::restaurant::constants::PI;
use:: restaurant::back_of_house::take_care_trash;
use:: restaurant::front_of_house::hosting::add_to_wait_list;
use:: restaurant::front_of_house::hosting::seat_at_table;


fn main() {
    add_to_wait_list();
    take_care_trash();
    seat_at_table();
    println!("The value o PI is: {}",PI);
}
