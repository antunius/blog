table! {
    article (id) {
        id -> Bigint,
        title -> Varchar,
        body -> Text,
        author -> Bigint,
        create_at -> Nullable<Timestamp>,
        update_at -> Nullable<Timestamp>,
    }
}

table! {
    author (id) {
        id -> Bigint,
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
