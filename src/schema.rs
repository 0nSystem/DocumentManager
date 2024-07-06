// @generated automatically by Diesel CLI.

use diesel::table;

table! {
    document (id_document) {
        id_document -> Uuid,
        name -> VarChar,
        extension -> VarChar,
        application -> VarChar,

        create_datetime -> Timestamptz,
        create_username -> VarChar,

        update_datetime -> Timestamptz,
        update_username -> VarChar,

        delete_datetime -> Timestamptz,
        delete_username -> VarChar
    }
}