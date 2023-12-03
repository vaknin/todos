diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Varchar,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}