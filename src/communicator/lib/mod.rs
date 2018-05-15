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
                outermost::middle_secret_function()
            }

        }
    }

}

fn try_me() {
    outermost::middle_function();
//    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::inside2::inner_function();
//    outermost::inside::secret_function();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}