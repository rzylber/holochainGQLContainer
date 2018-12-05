#![feature(try_from)]
use std::convert::TryFrom;

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate boolinator;

use boolinator::Boolinator; // ok_or_else
use hdk::{
    holochain_core_types::{
        dna::zome::entry_types::Sharing,
        hash::HashString,
        json::JsonString,
        entry::Entry,
        entry::entry_type::EntryType,
        cas::content::Address,

        // DefaultJson (derive) @see https://github.com/holochain/holochain-rust/blob/develop/core_types_derive/src/lib.rs
        error::HolochainError,
    },
    AGENT_ADDRESS,
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
struct Post {
    text: String,
}

fn handle_send_post(text: String)-> JsonString {
    let post_entry = Entry::new(EntryType::App("post".into()), Post {
        text,
    });
    match hdk::commit_entry(&post_entry) {
        Ok(address) => match hdk::link_entries(&AGENT_ADDRESS, &address, "posts") {
            Ok(_) => json!({ "address": address }).into(),
            Err(hdk_err) => hdk_err.into(),
        },
        Err(hdk_error) => hdk_error.into(),
    }
}

fn handle_get_post(address: HashString) -> JsonString {
    match hdk::get_entry(address) {
        // Ok(result) => result.and_then(|entry| Some( Post::try_from(entry.value()).unwrap() )).unwrap().into(),
        Ok(result) => Post::try_from(result.expect("Post not found!").value()).unwrap().into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

fn handle_get_posts() -> JsonString {
    match hdk::get_links(&AGENT_ADDRESS, "posts") {
        Ok(result) => {
            
            let mut posts: Vec<Post> = Vec::new();

            for address in result.addresses().iter() {
                let result = hdk::get_entry( address.to_owned() ).unwrap(); // TODO: possible panic here!
                posts.push( Post::try_from(result.expect("Post not found!").value()).unwrap() );
            }            

            posts.into()
        },
        Err(hdk_error) => hdk_error.into(),
    }    
}

// hdk::debug( format!( "DEBUG: {:#?}",  posts ) ).ok();

// ESTUDAR:
// TODO: onde armazena? memory?
// TODO: opções para armazenamento (ainda não)
// TODO: persiste entre inicializações (Container)? Parece que não!? Seár que o padrão é CAS Memória?
// TODO: quem é o agente?
// TODO: consigo compartilhar as mensagens entre agentes (já que é uma entry pública)?
// TODO: qual a hashfunction (CAS?)


// TODO: comments
define_zome! {
    entries: [
        entry!(
            name: "post",
            description: "A simple public post",
            sharing: Sharing::Public,
            native_type: Post,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |post: Post, _ctx: hdk::ValidationData| {
                (post.text.len() >= 2)
                    .ok_or_else(|| String::from("Text must be at least 2 characters"))
            },
            links: [
                from!(
                    "%agent_id",
                    tag: "posts",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::ChainFull
                    },
                    validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            send_post: {
                inputs: |text: String|,
                outputs: |result: JsonString|,
                handler: handle_send_post
            }
            get_post: {
                inputs: |address: HashString|,
                outputs: |result: JsonString|,
                handler: handle_get_post
            }
            get_posts: {
                inputs: | |,
                outputs: |result: JsonString|,
                handler: handle_get_posts
            }
        }
    }
}