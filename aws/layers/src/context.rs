use rootly_lib::prelude::{Rootly};
use crate::efs_storage::EfsStorage;

pub fn get_rootly() -> Rootly {
    let storage = Box::new(EfsStorage::new());
    let rootly = Rootly::init(None, storage);
    rootly
}