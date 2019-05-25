table! {
    sections (id) {
        id -> Int4,
        module -> Varchar,
        href -> Varchar,
        section_type -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        account_name -> Varchar,
        password_hash -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    sections,
    users,
);
