-- As a style choice, we prefer to avoid plurals in table names, mainly because it makes queries read better.
-- Tables get an integer auto-incrementing `id` and a `{table}_id` uuid column used throughout
-- the application. The table `id` column is database-only, primarily for indexes. Foreign keys
-- and joins use the uuid column for application usage.

-- Keep form-domain tables together instead of adding them directly to `public`.
-- This makes ownership clearer as the questionnaire/submission model grows.
create schema form;

create table form.form
(
    -- Database-only surrogate key. Application code should use `form_id`.
    id integer generated always as identity primary key,

    -- Stable public identifier for a form across all versions.
    form_id uuid not null default uuid_generate_v1mc(),

    -- Incremented whenever the form definition changes. Submissions point to a specific
    -- `(form_id, version)` pair so historical submissions always reference the exact
    -- questionnaire the signer saw.
    version integer not null,

    -- Top-level display fields for the form. These are columns because they are commonly
    -- shown in lists, admin views, and logs without needing to inspect the JSON document.
    title text not null,
    description_markdown text,

    -- Controls whether admins can create new share tokens for this form version.
    -- Existing unexpired tokens can still be honored or rejected by application policy.
    active boolean not null default true,

    -- JSON data for the actual questionnaire: Vec<Section>, including questions and options.
    --
    -- This intentionally keeps the nested form-definition model as a document:
    --   Form.sections -> Section.questions -> Question.kind -> QuestionOption[]
    --
    -- That lets the Rust model evolve without requiring a large relational migration for
    -- each question-kind-specific field. Use JSONB rather than JSON so Postgres can index
    -- and query into this document later if we need reporting or admin search.
    form_section jsonb not null,

    -- Flattened FormMeta fields. `created_at` and `updated_at` are database timestamps;
    -- `created_by` and `updated_by` identify the application/admin actor responsible.
    created_at timestamptz not null default now(),
    created_by text not null,
    updated_at timestamptz,
    updated_by text,

    -- Allows multiple versions of the same form while preventing duplicate version rows.
    unique (form_id, version)
);

-- Keep `updated_at` current whenever any column changes.
select trigger_updated_at('form.form');

create table form.share_token
(
    -- Database-only surrogate key. Application code should use `share_token_id`.
    id integer generated always as identity primary key,

    -- Stable public identifier for admin views and audit trails. This is not the secret
    -- token value sent to respondents.
    share_token_id uuid not null default uuid_generate_v1mc() unique,

    -- Store only a hash of the respondent-facing token. The raw token should be generated
    -- once, placed in the share URL, and never persisted in plaintext.
    token_hash text not null unique,

    -- Optional non-secret prefix for support/admin lookup. For example, this could be the
    -- first few characters of the raw token shown in logs or admin tables.
    token_prefix text,

    -- The exact form version this token grants access to. Pinning the version prevents a
    -- respondent from seeing a different questionnaire if the form changes after the token
    -- is generated.
    form_id uuid not null,
    form_version integer not null,

    -- Single-use state. A token is usable only when active is true, used_at is null, and
    -- expires_at is either null or in the future.
    active boolean not null default true,
    expires_at timestamptz,
    used_at timestamptz,

    -- Free-text admin context for identifying the intended recipient or purpose.
    notes text,

    -- Admin/audit metadata for token lifecycle.
    created_at timestamptz not null default now(),
    created_by text not null,
    updated_at timestamptz,
    updated_by text,

    foreign key (form_id, form_version)
        references form.form (form_id, version),

    -- Enables submissions to prove that their token belongs to the same form version
    -- they are recording.
    unique (share_token_id, form_id, form_version),

    check (used_at is null or active = false)
);

-- Common admin view: all tokens generated for a specific form version.
create index share_token_form_idx on form.share_token (form_id, form_version);

-- Keep `updated_at` current whenever any column changes.
select trigger_updated_at('form.share_token');

create table form.completed_form
(
    -- Database-only surrogate key. Application code should use `completed_form_id`.
    id integer generated always as identity primary key,

    -- Stable public identifier for a completed/submitted form.
    completed_form_id uuid not null default uuid_generate_v1mc() unique,

    -- The exact form version this submission answered. This is intentionally not just
    -- `form_id`; otherwise a later form edit could make an old submission ambiguous.
    form_id uuid not null,
    form_version integer not null,

    -- The single-use share token that authorized this respondent. This creates an audit
    -- trail from token generation through final submission.
    share_token_id uuid not null unique,

    -- Submission metadata captured at signing time.
    company_name text not null,
    signer_name text not null,
    signer_title text not null,
    submitted_at timestamptz not null,

    -- Database audit timestamps for the completed-form record itself.
    created_at timestamptz not null default now(),
    updated_at timestamptz,

    -- Pin each submission to a valid form definition version.
    foreign key (form_id, form_version)
        references form.form (form_id, version),

    -- Ensure the submission's token belongs to the same form version being submitted.
    foreign key (share_token_id, form_id, form_version)
        references form.share_token (share_token_id, form_id, form_version)
);

-- Keep `updated_at` current whenever any column changes.
select trigger_updated_at('form.completed_form');

create table form.question_response
(
    -- Database-only surrogate key. Application code should use `question_response_id`.
    id integer generated always as identity primary key,

    -- Stable public identifier for this single answer.
    question_response_id uuid not null default uuid_generate_v1mc() unique,

    -- Parent completed form. Deleting a completed form deletes its answers.
    completed_form_id uuid not null,

    -- JSON data for a submitted answer. Keeping this flexible matches
    -- form.form_section being JSONB.
    --
    -- Expected shape should include the form question id plus the Rust `Response` enum
    -- payload, for example:
    --   {
    --     "question_id": "preferred-contact",
    --     "response": {
    --       "type": "choice",
    --       "selected_option_id": "email",
    --       "comment": null
    --     }
    --   }
    --
    -- Question and option ids are validated in application code if/when the API chooses
    -- to understand the form document. The database stores the submitted answer document.
    response jsonb not null,

    -- Optional per-answer timestamp from the client/form flow.
    answered_at timestamptz,

    -- Database audit timestamps for the answer row itself.
    created_at timestamptz not null default now(),
    updated_at timestamptz,

    foreign key (completed_form_id)
        references form.completed_form (completed_form_id)
        on delete cascade
);

-- Keep `updated_at` current whenever any column changes.
select trigger_updated_at('form.question_response');

create table form.viewer_token
(
    -- Database-only surrogate key. Application code should use `viewer_token_id`.
    id integer generated always as identity primary key,

    -- Stable public identifier for admin views and audit trails. This is not the secret
    -- token value sent to viewers.
    viewer_token_id uuid not null default uuid_generate_v1mc() unique,

    -- Store only a hash of the viewer-facing token. The raw token should be generated
    -- once, placed in the viewer URL, and never persisted in plaintext.
    token_hash text not null unique,

    -- Optional non-secret prefix for support/admin lookup.
    token_prefix text,

    -- The completed form this token can read.
    completed_form_id uuid not null,

    -- Viewer tokens are read-only access grants. Unlike share tokens, they are not
    -- single-use; disable them with `active = false` or time-limit them with `expires_at`.
    active boolean not null default true,
    expires_at timestamptz,

    -- Admin/audit metadata for viewer-token lifecycle.
    created_at timestamptz not null default now(),
    created_by text not null,
    updated_at timestamptz,
    updated_by text,

    foreign key (completed_form_id)
        references form.completed_form (completed_form_id)
        on delete cascade
);

-- Keep `updated_at` current whenever any column changes.
select trigger_updated_at('form.viewer_token');
