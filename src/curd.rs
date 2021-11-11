use crate::prelude::*;

pub fn query_release_state_by_bid(input_bid: &str, db: &MysqlConnection) -> QueryResult<bool> {
    pub use crate::prelude::version::dsl::*;
    version
        .select(is_release)
        .filter(bid.eq(input_bid.to_string()))
        .first::<bool>(db)
}

pub fn update_release_state_by_bid(input_bid: &str, new_state: bool, db: &MysqlConnection) -> bool {
    pub use crate::prelude::version::dsl::*;
    diesel::update(version.filter(bid.eq(input_bid.to_string())))
        .set(is_release.eq(new_state))
        .execute(db).is_ok()
}

pub fn update_delete_state_by_bid(input_bid: &str, db: &MysqlConnection) -> bool {
    pub use crate::prelude::version::dsl::*;
    diesel::update(version.filter(bid.eq(input_bid.to_string())))
        .set(is_delete.eq(true))
        .execute(db).is_ok()
}

pub fn create_record_version() -> bool {
    true
}


pub fn create_record_version_message(data: &VersionMessage, db: &MysqlConnection) -> bool {
    pub use crate::prelude::version_message::dsl::*;
    diesel::insert_into(version_message)
        .values(data)
        .execute(db)
        .is_ok()
}
