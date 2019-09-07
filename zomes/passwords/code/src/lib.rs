#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::{JsonString},
};


// see https://developer.holochain.org/api/0.0.25-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Credentials {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Domain {
    domainname: String,
}

pub fn handle_store_password(domainname: String, username: String, password: String) -> ZomeApiResult<Address> {
    //call create domain, create credentials and link entries
    let domainname_struct = Domain {domainname:domainname};
    let domain_adress = handle_create_domain(domainname_struct)?;
    let cred =Credentials {username: username, password:password};
    let credentials_adress = handle_create_credentials(cred)?;
    handle_link_credential_to_domain(domain_adress, credentials_adress)
}


pub fn handle_create_domain(entry: Domain) -> ZomeApiResult<Address> {
    let entry = Entry::App("domain".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_create_credentials(entry: Credentials) -> ZomeApiResult<Address> {
    let entry = Entry::App("credentials".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_link_credential_to_domain(base: Address,target : Address) -> ZomeApiResult<Address> {
    hdk::link_entries(&base, &target, "has_credentials", "")
}


pub fn handle_get_credentials_for_domain(domainname: String) -> ZomeApiResult<Vec<ZomeApiResult<Entry>>> {
    let domain_address = Entry::App("domain".into(), Domain {domainname}.into());
    let domain_hash = hdk::entry_address(&domain_address)?;
    hdk::get_links_and_load(
        &domain_hash,
        LinkMatch::Exactly("has_credentials"),
        LinkMatch::Any
        )

}

fn credentials_definition() -> ValidatingEntryType {
    entry!(
        name: "credentials",
        description: "a username and password for a domain",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Credentials>| {
            Ok(())
        },
        links: [
            from!(
                "domain",
                link_type: "has_credentials",
                validation_package: || {
                    hdk::ValidationPackageDefinition::ChainFull
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )]
    )
}

fn domains_definition() -> ValidatingEntryType {
    entry!(
        name: "domain",
        description: "a website domain name",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<Domain>| {
            Ok(())
        }
    )
}

define_zome! {
    entries: [
       credentials_definition(),
       domains_definition()
    ]

    init: || { Ok(()) }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
        store_password: {
            inputs: |domainname: String, username: String, password: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_store_password
        }
        create_credentials: {
            inputs: |entry: Credentials|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_credentials
        }
        create_domain: {
            inputs: |entry: Domain|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_domain
        }
        get_credentials_for_domain: {
            inputs: |domainname: String|,
            outputs: |result: ZomeApiResult<Vec<ZomeApiResult<Entry>>>|,
            handler: handle_get_credentials_for_domain
        }
        link_credential_to_domain: {
            inputs: |base : Address,target: Address|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_link_credential_to_domain
        }
    ]

    traits: {
        hc_public [store_password,get_credentials_for_domain]
    }
}
