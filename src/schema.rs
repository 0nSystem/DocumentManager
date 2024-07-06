// @generated automatically by Diesel CLI.

diesel::table! {
    content (id) {
        id -> Uuid,
        id_document -> Nullable<Uuid>,
        data -> Text,
        create_datetime -> Timestamptz,
        create_username -> Nullable<Varchar>,
        delete_datetime -> Nullable<Timestamptz>,
        delete_username -> Nullable<Varchar>,
    }
}

diesel::table! {
    document (id_document) {
        id_document -> Uuid,
        name -> Nullable<Varchar>,
        extension -> Nullable<Varchar>,
        application -> Nullable<Varchar>,
        create_datetime -> Timestamptz,
        create_username -> Nullable<Varchar>,
        update_datetime -> Nullable<Timestamptz>,
        update_username -> Nullable<Varchar>,
        delete_datetime -> Nullable<Timestamptz>,
        delete_username -> Nullable<Varchar>,
    }
}

diesel::joinable!(content -> document (id_document));

diesel::allow_tables_to_appear_in_same_query!(
    content,
    document,
);
