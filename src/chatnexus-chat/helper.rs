use rand::{thread_rng, Rng, distributions::Alphanumeric};


pub fn system_print(message: &str){
    let prefix = "SYSTEM:";
    println!("{} {}", prefix, message);
}

pub fn gen_string(len: usize) -> String {
    let rng = thread_rng();
    let result: String = rng.sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect();
    result
}