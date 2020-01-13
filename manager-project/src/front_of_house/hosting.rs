pub mod wait_list {
    use rand::Rng;

    pub fn add_to_wait_list() {
        let wait_seconds = rand::thread_rng().gen_range(1, 300);
        println!("add to wait list succesed! need to wait {} seconds.", wait_seconds);
    }
}
