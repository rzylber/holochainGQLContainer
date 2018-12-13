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
    entry_definition::{
        ValidatingEntryType,
        ValidatingLinkDefinition,
    },
    AGENT_ADDRESS,
};

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Person {
    pub address: Option<HashString>,
    pub name: String,
    pub gender: String,
    pub place_birth: String,
    // TODO: birthday
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Movie {
    pub address: Option<HashString>,
    pub name: String,
    pub year: String,
    pub language: String,
    // TODO: genres (array: "drama,sci-fi,thriller")
}

pub fn person_definition() -> ValidatingEntryType {
    entry!(
        name: "person",
        description: "A person",
        sharing: Sharing::Public,
        native_type: Person,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_person: Person, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            agent_people_definition(),
            person_acts_in_movie_definition(),
            person_directed_movie_definition(),
            movie_has_actor_definition(),
            movie_directed_by_definition()
        ]
    )
}

pub fn movie_definition() -> ValidatingEntryType {
    entry!(
        name: "movie",
        description: "A movie",
        sharing: Sharing::Public,
        native_type: Movie,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_movie: Movie, _ctx: hdk::ValidationData| {
            Ok(())
        },

        links: [
            agent_movie_definition()
        ]
    )
}

fn agent_people_definition() -> ValidatingLinkDefinition {
    from!(
        "%agent_id",
        tag: "people",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn agent_movie_definition() -> ValidatingLinkDefinition {
    from!(
        "%agent_id",
        tag: "movies",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn person_acts_in_movie_definition() -> ValidatingLinkDefinition {
    to!(
        "movie",
        tag: "acts_in",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn movie_has_actor_definition() -> ValidatingLinkDefinition {
    from!(
        "movie",
        tag: "has_actor",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn movie_directed_by_definition() -> ValidatingLinkDefinition {
    from!(
        "movie",
        tag: "directed_by",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn person_directed_movie_definition() -> ValidatingLinkDefinition {
    to!(
        "movie",
        tag: "directed",
        validation_package: || {
            hdk::ValidationPackageDefinition::ChainFull
        },
        validation: |_source: Address, _target: Address, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}

fn handle_create_person (
    name: String,
    gender: String,
    place_birth: String,
) -> JsonString {
    let person = Person {
        address: None,
        name,
        gender,
        place_birth,
    };

    let entry =  Entry::new(EntryType::App("person".into()), person);

    match hdk::commit_entry(&entry) {
        Ok(address) => match hdk::link_entries(&AGENT_ADDRESS, &address, "people") {
            Ok(_) => json!({ "address": address }).into(),
            Err(hdk_err) => hdk_err.into(),
        },
        Err(hdk_err) => hdk_err.into(),
    }
}

fn handle_create_movie (
    name: String,
    year: String,
    language: String,
) -> JsonString {
    let movie = Movie {
        address: None,
        name,
        year,
        language,
    };

    let entry =  Entry::new(EntryType::App("movie".into()), movie);

    match hdk::commit_entry(&entry) {
        Ok(address) => match hdk::link_entries(&AGENT_ADDRESS, &address, "movies") {
            Ok(_) => json!({ "address": address }).into(),
            Err(hdk_err) => hdk_err.into(),
        },
        Err(hdk_err) => hdk_err.into(),
    }
}

fn handle_get_people() -> JsonString {
    match hdk::get_links(&AGENT_ADDRESS, "people") {
        Ok(result) => {
            
            let mut people: Vec<Person> = Vec::new();

            for address in result.addresses().iter() {
                let result = hdk::get_entry( address.to_owned() ).unwrap(); // TODO: possible panic here!
                people.push( build_person( address.to_owned(), result.expect("Person not found!").value().to_owned() ) );
            }            

            people.into()
        },
        Err(hdk_error) => hdk_error.into(),
    }    
}

fn handle_get_movies() -> JsonString {
    match hdk::get_links(&AGENT_ADDRESS, "movies") {
        Ok(result) => {
            
            let mut movies: Vec<Movie> = Vec::new();

            for address in result.addresses().iter() {
                let result = hdk::get_entry( address.to_owned() ).unwrap(); // TODO: possible panic here!
                movies.push( build_movie( address.to_owned(), result.expect("Movie not found!").value().to_owned() ) );
            }            

            movies.into()
        },
        Err(hdk_error) => hdk_error.into(),
    }    
}

fn handle_get_movies_by_actor( actor_address: HashString ) -> JsonString {
    match hdk::get_links(&actor_address, "acts_in") {
        Ok(result) => {
            
            let mut movies: Vec<Movie> = Vec::new();

            for address in result.addresses().iter() {
                let result = hdk::get_entry( address.to_owned() ).unwrap(); // TODO: possible panic here!
                movies.push( build_movie( address.to_owned(), result.expect("Movie not found!").value().to_owned() ) );
            }            

            movies.into()
        },
        Err(hdk_error) => hdk_error.into(),
    }    
}

fn handle_get_actors_by_movie( movie_address: HashString ) -> JsonString {
    match hdk::get_links(&movie_address, "has_actor") {
        Ok(result) => {
            
            let mut people: Vec<Person> = Vec::new();

            for address in result.addresses().iter() {
                let result = hdk::get_entry( address.to_owned() ).unwrap(); // TODO: possible panic here!
                people.push( build_person( address.to_owned(), result.expect("Person not found!").value().to_owned() ) );
            }            

            people.into()
        },
        Err(hdk_error) => hdk_error.into(),
    }
}

fn handle_get_movies_by_director( director_address: HashString ) -> JsonString {
    match hdk::get_links(&director_address, "directed") {
        Ok(result) => {
            
            let mut movies: Vec<Movie> = Vec::new(); // TODO: repeated code - simplify

            for address in result.addresses().iter() {
                let result = hdk::get_entry( address.to_owned() ).unwrap(); // TODO: possible panic here!
                movies.push( build_movie( address.to_owned(), result.expect("Movie not found!").value().to_owned() ) );
            }            

            movies.into()
        },
        Err(hdk_error) => hdk_error.into(),
    }    
}

fn handle_get_director_by_movie( movie_address: HashString ) -> JsonString { // TODO: only one
    match hdk::get_links(&movie_address, "directed_by") {
        Ok(result) => {
            
            let mut people: Vec<Person> = Vec::new();

            for address in result.addresses().iter() {
                let result = hdk::get_entry( address.to_owned() ).unwrap(); // TODO: possible panic here!
                people.push( build_person( address.to_owned(), result.expect("Person not found!").value().to_owned() ) );
            }            

            people.into()
        },
        Err(hdk_error) => hdk_error.into(),
    }
}

fn handle_add_actor (
    actor_address: HashString,
    movie_address: HashString,
) -> JsonString {

    match (
        hdk::link_entries(&actor_address, &movie_address, "acts_in"), 
        hdk::link_entries(&movie_address, &actor_address, "has_actor")
    ) {
            (Ok(_),Ok(_)) => json!({ "success": true }).into(),
            // TODO: best practices
            (Err(hdk_err),Err(_)) => hdk_err.into(),
            (Err(hdk_err),Ok(_)) => hdk_err.into(),
            (Ok(_),Err(hdk_err)) => hdk_err.into(),
    }
}

fn handle_add_director ( // TODO: only one - cardinality?
    director_address: HashString,
    movie_address: HashString,
) -> JsonString {

    match (
        hdk::link_entries(&director_address, &movie_address, "directed"), 
        hdk::link_entries(&movie_address, &director_address, "directed_by")
    ) {
            (Ok(_),Ok(_)) => json!({ "success": true }).into(),
            // TODO: best practices
            (Err(hdk_err),Err(_)) => hdk_err.into(),
            (Err(hdk_err),Ok(_)) => hdk_err.into(),
            (Ok(_),Err(hdk_err)) => hdk_err.into(),
    }
}

fn handle_get_person(address: HashString) -> JsonString { // TODO: remove to_owner - understand OWNERSHIP!!
    match hdk::get_entry(address.to_owned()) {
        Ok(result) => build_person( address.to_owned(), result.expect("Person not found!").value().to_owned() ).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

fn handle_get_movie(address: HashString) -> JsonString { // TODO: remove to_owner - understand OWNERSHIP!!
    match hdk::get_entry(address.to_owned()) {
        Ok(result) => build_movie( address.to_owned(), result.expect("Movie not found!").value().to_owned() ).into(),
        Err(hdk_error) => hdk_error.into(),
    }
}

/*
fn handle_add_actor (
    actor_address: HashString,
    movie_address: HashString,
) -> JsonString {

    //match hdk::link_entries(&actor_address, &movie_address, "acts_in") {
    match hdk::link_entries(&movie_address, &actor_address, "has_actor") {    
            Ok(_) => json!({ "success": true }).into(),
            Err(hdk_err) => hdk_err.into(),
    }
}
*/

// Support functions

fn build_person( address: HashString, json_entity: JsonString ) -> Person {
    let mut person = Person::try_from( json_entity ).unwrap();
    person.address = Some(address);
    person
}
fn build_movie( address: HashString, json_entity: JsonString ) -> Movie {
    let mut movie = Movie::try_from( json_entity ).unwrap();
    movie.address = Some(address);
    movie
}

// ZOME DEFINITION

define_zome! {
    entries: [
        person_definition(),
        movie_definition()
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_person: {
                inputs: |name: String, gender: String,  place_birth: String|,
				outputs: |result: JsonString|,
				handler: handle_create_person
            }
            create_movie: {
                inputs: |name: String, year: String,  language: String|,
				outputs: |result: JsonString|,
				handler: handle_create_movie
            }
            get_people: {
                inputs: | |,
                outputs: |result: JsonString|,
                handler: handle_get_people
            }
            get_movies: {
                inputs: | |,
                outputs: |result: JsonString|,
                handler: handle_get_movies
            }
            get_person: {
                inputs: | person_address: HashString  |,
                outputs: |result: JsonString|,
                handler: handle_get_person
            }
            get_movie: {
                inputs: | movie_address: HashString  |,
                outputs: |result: JsonString|,
                handler: handle_get_movie
            }
            get_movies_by_actor: {
                inputs: | actor_address: HashString  |,
                outputs: |result: JsonString|,
                handler: handle_get_movies_by_actor
            }
            get_actors_by_movie: {
                inputs: | movie_address: HashString  |,
                outputs: |result: JsonString|,
                handler: handle_get_actors_by_movie
            }
            get_movies_by_director: {
                inputs: | director_address: HashString  |,
                outputs: |result: JsonString|,
                handler: handle_get_movies_by_director
            }
            get_director_by_movie: {
                inputs: | movie_address: HashString  |,
                outputs: |result: JsonString|,
                handler: handle_get_director_by_movie
            }
            add_actor: {
                inputs: | actor_address: HashString, movie_address: HashString  |,
                outputs: |result: JsonString|,
                handler: handle_add_actor
            }
            add_director: {
                inputs: | director_address: HashString, movie_address: HashString  |,
                outputs: |result: JsonString|,
                handler: handle_add_director
            }
        }
    }
}
