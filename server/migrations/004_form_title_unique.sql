-- Form titles should be unique across all form records (all versions share the same title).
-- This allows the admin check-title endpoint to enforce uniqueness before creation.
create unique index form_title_unique_idx on form.form (lower(trim(title)));
