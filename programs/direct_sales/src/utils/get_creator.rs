use anchor_lang::prelude::*;
use mpl_token_metadata::types::Creator;
use mpl_token_metadata::accounts::{Metadata};
use crate::models::*;

fn _get_creator(mint_account_info: &AccountInfo) -> Result<Creator> {
    let metadata = Metadata::try_from(mint_account_info).unwrap();
    let creators = metadata.creators.unwrap();
    let developer = creators.get(1); //get the second item

    return match developer {
        None => {
            Err(NoCreator.into())
        }
        Some(developer) => {
            Ok(developer.clone())
        }
    };
}