extern crate redis_cluster_rs;
extern crate redis;

#[macro_use]
pub mod error;
use redis_cluster_rs::{Client, Commands};
use redis_cluster_rs::redis::RedisError;
use std::result::Result::Err;
use std::str;
use std::env;
use crate::error::GenericError;

fn main() {
    let res = execute_main();
    match res {
        Ok(_data) => println!("Ran successfully"),
        Err(err) => println!("Errored {}", err),
    };
}

fn execute_main() -> Result<(),GenericError> {
    let cluster_connection = get_cluster_connection()?;
    let mut pubsub_connection = get_simple_connection()?;
    let simple_connection = get_simple_connection()?;

    let mut pubsub = pubsub_connection.as_pubsub();
    let channels : String = env::var("CHANNELS")?;
    let split_channel = channels.split(",");
    let vec_channels = split_channel.collect::<Vec<&str>>();
    for x in &vec_channels {
        println!("Subscribing to changes for key:{}", x);
        pubsub.psubscribe(x.to_owned())?;
    }
    loop {
        let msg = pubsub.get_message()?;
        let payload : String = msg.get_payload()?;
        println!("{:?}", payload);
        let split = msg.get_channel_name().split("__:");
        let vec = split.collect::<Vec<&str>>();
        let key = String::from(vec[1]);
        println!("Key: {}", key);
        let value = get_value_for_key(&simple_connection, &key)?;
        match value {
            Some(expr) => {
                let ttl : i32 = get_ttl_for_key(&simple_connection, &key)?;
                println!("{:?}", ttl);
                let res = set_key_in_cluster(&cluster_connection, &key, expr, ttl)?;
                println!("{:?}", res);
            },
            None => println!("Value was not found while dumping"),
        };
        
    }
}

fn get_ttl_for_key(con: &redis::Connection, key: &String) -> Result<i32, RedisError> {
    let k : i32 = con.ttl(key)?;
    Ok(k)
}

fn get_value_for_key(con: &redis::Connection, key: &String) -> Result<Option<String>, RedisError> {
    let k : Option<String> = con.get(key)?;
    Ok(k)
}

fn set_key_in_cluster(con: &redis_cluster_rs::Connection, key: &String, value: String, ex: i32) -> Result<Option<String>, RedisError> {
    if ex>0 {
        let k : Option<String> = con.set_ex(key, value, ex as usize)?;
        Ok(k)
    } else {
        let k : Option<String> = con.set(key, value)?;
        Ok(k)
    }
}

fn get_cluster_connection() -> Result<redis_cluster_rs::Connection,RedisError> {
    let url : String = env::var("REDIS_CLUSTER_DESTINATION_URL").unwrap();
    let nodes = vec![&url[..]];
    let client = Client::open(nodes)?;
    let connection = client.get_connection()?;
    Ok(connection) 
}

fn get_simple_connection() -> Result<redis::Connection,RedisError> {
    let url : String = env::var("REDIS_SOURCE_URL").unwrap();
    let client = redis::Client::open(&url[..])?;
    let con = client.get_connection()?;
    Ok(con)
}