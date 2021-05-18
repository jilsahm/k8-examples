use redis::Commands;

#[macro_use]
extern crate log;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let address = "redis://172.30.66.90:30001/";

    let client = redis::Client::open(address).unwrap();
    info!("Redis client open");

    let mut con = client.get_connection().unwrap();
    info!("Redis connection open");

    let _: () = con.set("my_key", 42).unwrap();
    info!("Saved key 'my_key' with value 42");
    
    let i: isize = con.get("my_key").unwrap();
    info!("Obtained value {} from 'my_key'", i);
}
