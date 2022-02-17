mod front_of_house {
   pub mod hosting {
        pub fn add_to_waitlist() {

        }

        fn seat_at_table() {
 
        }
    }

    mod serving {
        fn take_order() {

        }

        fn serve_order() {

        }

        fn take_payment() {

        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use self::back_of_house::Breakfast;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer("Bimbo");
    meal.toast = String::from("Wheat");
}