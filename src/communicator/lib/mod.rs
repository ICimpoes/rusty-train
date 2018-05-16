pub mod client;
pub mod network;


mod outermost {
    pub fn middle_function() {
        inside::inner_function()
    }

    fn middle_secret_function() {
        inside::inner_function()
    }

    pub mod inside {
        use outermost;
        pub fn inner_function() {}

        fn secret_function() {
            outermost::middle_secret_function()
        }

        pub mod inside2 {
            use outermost;
            pub fn inner_function() {
                outermost::middle_secret_function()//child
            }

        }
    }
}

pub mod a {
    fn fa(){}
    pub mod series {
        fn fb(){}
        pub mod of {
            pub fn nested_modules() {
                super::fb();
                ::a::fa();
                ::a::series::fb();
            }
        }
    }
}

pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}




fn try_me() {
    outermost::middle_function();//private+pub
//    outermost::middle_secret_function(); private+private
    outermost::inside::inner_function();//private+pub
    outermost::inside::inside2::inner_function();//private+pub+pub
//    outermost::inside::secret_function();//private+pub+private

    a::series::of::nested_modules();

    use a::series::of;
    of::nested_modules();

    use a::series::of::nested_modules;
    nested_modules()

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ::client::connect();// start from the root
        super::client::connect(); // super - parent

        use super::client::connect;
        connect()
    }
}