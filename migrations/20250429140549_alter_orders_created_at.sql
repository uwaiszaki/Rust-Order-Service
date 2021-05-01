-- Add migration script here
ALTER TABLE orders
    ALTER COLUMN created_at SET NOT NULL;