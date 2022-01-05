table! {
    track_base (id) {
        id -> Int4,
        category -> Nullable<Int2>,
        description -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        timespan -> Timestamp,
        create_user -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
    }
}
