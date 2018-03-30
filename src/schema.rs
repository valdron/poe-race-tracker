table! {
    levels (id) {
        id -> Int4,
        run_id -> Int4,
        level -> Int2,
        duration_in_seconds -> Int4,
    }
}

table! {
    runs (id) {
        id -> Int4,
        duration_in_seconds -> Int4,
    }
}

table! {
    zones (id) {
        id -> Int4,
        run_id -> Int4,
        name -> Varchar,
        duration_in_seconds -> Int4,
    }
}

joinable!(levels -> runs (run_id));
joinable!(zones -> runs (run_id));

allow_tables_to_appear_in_same_query!(
    levels,
    runs,
    zones,
);
