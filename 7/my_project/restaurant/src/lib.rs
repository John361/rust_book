// use std::io;
// use std::io::Write;
// -> use std::io::{self, Write};

use dining_room::home as dining_room_home;
pub use dining_room::service as dining_root_service;

pub mod dining_room;
mod kitchen;


pub fn eat_at_restaurant() {
    dining_room_home::add_to_wait_list();

    let breakfast = kitchen::Breakfast::summer();
}

