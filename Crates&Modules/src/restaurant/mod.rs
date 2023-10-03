mod ramen_order {
    pub struct Ramen {
        pub noodles: String,
        pub broth: String,
        pub toppings: String,
    }
    impl Ramen {
        pub fn lunch(toppings: &str) -> Ramen {
            Ramen {
                noodles: String::from("soba noodles"),
                broth: String::from("shoyu broth"),
                toppings: String::from(toppings),
            }
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }
        pub fn take_order() {
            seat_at_table();
            let cust_ramen: super::Ramen = super::Ramen::lunch("egg");
            serve_customer(cust_ramen);
        }
        fn serve_customer(cust_ramen: super::Ramen) {
            println!(
                "The customer is served a ramen bowl with {}",
                cust_ramen.toppings
            );
        }
    }
}

pub fn order_food() {
    crate::restaurant::ramen_order::help_customer::take_order();
}
