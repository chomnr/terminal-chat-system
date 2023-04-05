use rand::{thread_rng, Rng};


pub fn system_print(message: &str){
    let prefix = "SYSTEM:";
    println!("{} {}", prefix, message);
}