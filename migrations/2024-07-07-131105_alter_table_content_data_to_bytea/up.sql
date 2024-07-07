-- Your SQL goes here
ALTER TABLE IF EXISTS public.content
    ALTER COLUMN data TYPE bytea USING data::TEXT::BYTEA;