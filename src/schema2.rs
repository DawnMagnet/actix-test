use chrono::NaiveDateTime;
table! {
    version_message (id) {
        id -> Integer,
        bid -> Nullable<Char>,
        version_bid -> Nullable<Char>,
        user_bid -> Nullable<Char>,
        operation -> Nullable<Integer>,
        content -> Nullable<Varchar>,
        create_time -> Nullable<Datetime>,
    }
}
#[derive(Insertable)]
#[table_name = "version_message"]
pub struct VersionMessage {
    pub bid: String,
    pub version_bid: String,
    pub user_bid: String,
    pub operation: i32,
    pub content: String,
    pub create_time: NaiveDateTime,
}
