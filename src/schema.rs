table! {
    version (id) {
        id -> Integer,
        bid -> Char,
        #[sql_name = "version"]
        _version -> Integer,
        md5 -> Nullable<Varchar>,
        download_url -> Nullable<Varchar>,
        max_os_api -> Integer,
        min_os_api -> Integer,
        max_version -> Integer,
        min_version -> Integer,
        cpu_arch -> Varchar,
        channel -> Nullable<Varchar>,
        platform -> Nullable<Varchar>,
        pop_count -> Nullable<Integer>,
        install_count -> Nullable<Integer>,
        aid -> Nullable<Integer>,
        create_time -> Datetime,
        delete_time -> Nullable<Datetime>,
        is_delete -> Bool,
        is_release -> Bool,
    }
}

