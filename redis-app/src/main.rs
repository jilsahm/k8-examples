use redis::Commands;

#[macro_use]
extern crate log;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let address = "redis://172.22.146.199:30010/";

    let client = redis::Client::open(address).unwrap();
    //let client = redis::cluster::ClusterClient::open(vec!["redis://172.31.144.73:30001/"]).unwrap();
    info!("Redis client open");

    let mut con = client.get_connection().unwrap();
    info!("Redis connection open");

    let _: () = con.set("my_key", 42).unwrap();
    info!("Saved key 'my_key' with value 42");
    
    let i: isize = con.get("my_key").unwrap();
    info!("Obtained value {} from 'my_key'", i);
}
