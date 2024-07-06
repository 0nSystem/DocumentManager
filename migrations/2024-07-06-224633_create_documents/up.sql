-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE IF NOT EXISTS public.document
(

    id_document     uuid,
    name            varchar,
    extension       varchar,
    application     varchar,

    create_datetime timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    create_username varchar,

    update_datetime timestamptz,
    update_username varchar,

    delete_datetime timestamptz,
    delete_username varchar,
    primary key (id_document)
);

CREATE TABLE IF NOT EXISTS public.content
(
    id              uuid                 DEFAULT gen_random_uuid(),
    id_document     uuid references public.document (id_document),
    data            text        NOT NULL,

    create_datetime timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    create_username varchar,

    delete_datetime timestamptz,
    delete_username varchar,
    primary key (id)
)