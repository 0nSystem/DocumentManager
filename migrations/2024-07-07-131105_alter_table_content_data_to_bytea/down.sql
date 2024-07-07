-- This file should undo anything in `up.sql`
ALTER TABLE IF EXISTS public.content
    ALTER COLUMN data TYPE text;