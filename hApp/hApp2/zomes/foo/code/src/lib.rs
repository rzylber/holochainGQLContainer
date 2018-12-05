#![feature(try_from)]

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    holochain_core_types::{
        json::JsonString,
        // entry::Entry,
        // entry::entry_type::EntryType,

        // DefaultJson (derive) @see https://github.com/holochain/holochain-rust/blob/develop/core_types_derive/src/lib.rs
        error::HolochainError,
    },
};

// see https://developer.holochain.org/api/0.0.2/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
struct TestResponse {
    greeting: String
}

fn handle_test(name: String)-> JsonString {
    TestResponse{greeting: format!("{} {}", String::from("HI"), name)}.into()
}

define_zome! {
    entries: []

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            test: {
                inputs: |name: String|,
                // it must return JsonString!
                // But it can change @see https://github.com/holochain/holochain-rust/issues/588
                outputs: |result: JsonString|,
                handler: handle_test
            }
        }
    }
}



// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = handle_test( String::from("HOLO") );
        println!("RESULT {:#?}", result);

        assert_eq!(result, TestResponse{greeting: String::from("HI HOLO")}.into());      
    }
}
