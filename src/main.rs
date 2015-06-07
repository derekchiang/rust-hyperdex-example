#[macro_use] extern crate hyperdex;

use std::str::FromStr;

use hyperdex::*;


static COORD_ADDR: &'static str = "127.0.0.1:1982";

static SPACE_NAME: &'static str = "contacts";

static SPACE_DESC: &'static str = "
space contacts
key username
attributes first, last, int age
subspace first, last
create 2 partitions
tolerate 2 failures";

fn main() {
    let admin = Admin::new(FromStr::from_str(COORD_ADDR).unwrap()).unwrap();
    match admin.add_space(SPACE_DESC) {
        Ok(()) => (),
        Err(err) => panic!(format!("{}", err)),
    };

    let mut client = Client::new(FromStr::from_str(COORD_ADDR).unwrap()).unwrap();
    match client.put(SPACE_NAME, "derek", NewHyperObject!(
        "first", "Derek",
        "last", "Chiang",
    )) {
        Ok(()) => (),
        Err(err) => panic!(format!("{}", err)),
    }

    match client.get(SPACE_NAME, "derek") {
        Ok(obj) => {
            let first: Vec<u8> = match obj.get("first") {
                Ok(s) => s,
                Err(err) => panic!(format!("{}", err)),
            };

            let last: Vec<u8> = match obj.get("last") {
                Ok(s) => s,
                Err(err) => panic!(format!("{}", err)),
            };

            assert_eq!(first, "Derek".as_bytes());
            assert_eq!(last, "Chiang".as_bytes());
        },
        Err(err) => panic!(format!("{}", err)),
    }

    admin.remove_space(SPACE_NAME).unwrap();
    println!("Everything worked fine!");
}
