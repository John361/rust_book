pub struct Breakfast {
    pub eggs: bool,
    season_fruit: SeasonFruit
}

pub enum SeasonFruit {
    Peach,
    Mango
}

impl Breakfast {
    pub fn summer() -> Breakfast {
        Breakfast {
            eggs: false,
            season_fruit: SeasonFruit::Peach
        }
    }
}

fn cook_order() {
    println!("Cook order");
}

fn fix_order() {
    cook_order();
    // dining_room::service::serve_order();
}
