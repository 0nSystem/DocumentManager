-- This file should undo anything in `up.sql`
DROP EXTENSION IF EXISTS "uuid-ossp";

drop table if exists public.document;
drop table if exists public.content;