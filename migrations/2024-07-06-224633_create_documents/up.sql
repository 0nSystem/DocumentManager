-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE IF NOT EXISTS public.document
(

    id_document     uuid,
    name            varchar     NOT NULL,
    extension       varchar     NOT NULL,
    application     varchar     NOT NULL,

    create_datetime timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    create_username varchar     NOT NULL,

    update_datetime timestamptz,
    update_username varchar,

    delete_datetime timestamptz,
    delete_username varchar,
    primary key (id_document)
);

CREATE TABLE IF NOT EXISTS public.content
(
    id              uuid        NOT NULL DEFAULT gen_random_uuid(),
    id_document     uuid        NOT NULL references public.document (id_document),
    data            text        NOT NULL,

    create_datetime timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    create_username varchar     NOT NULL,

    delete_datetime timestamptz,
    delete_username varchar,
    primary key (id)
)