table! {
    article (id) {
        id -> Nullable<Integer>,
        title -> Varchar,
        body -> Text,
        author -> Integer,
        create_at -> Timestamp,
        update_at -> Timestamp,
    }
}

table! {
    author (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        username -> Varchar,
        resume -> Text,
        company -> Nullable<Varchar>,
        years_experience -> Nullable<Integer>,
        country -> Nullable<Varchar>,
    }
}

joinable!(article -> author (author));

allow_tables_to_appear_in_same_query!(
    article,
    author,
);
