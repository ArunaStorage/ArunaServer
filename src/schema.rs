table! {
    collection_labels (collection_id, label_id) {
        collection_id -> Uuid,
        label_id -> Uuid,
    }
}

table! {
    collections (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

table! {
    labels (id) {
        id -> Uuid,
        key -> Text,
        value -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    collection_labels,
    collections,
    labels,
);
