create type class_status as enum (
    'active',    -- open for enrollment, ongoing
    'archived',  -- finished, no longer joinable, kept for history
    'draft',     -- teacher is still preparing it, not visible to students yet
    'cancelled'  -- class was cancelled
);

create table classes (
    id          uuid default uuidv7() primary key,
    name        text not null,
    slug        text unique,                     -- optional: human-readable URL slug
    description text,
    teacher_id  uuid not null,
    
    status      class_status not null default 'draft',
    
    max_students integer,                        -- optional: enrollment limit
    starts_at   timestamptz,                     -- optional: when the class begins
    ends_at     timestamptz,                       -- optional: when it ends
    
    created_at  timestamptz not null default now(),
    updated_at  timestamptz not null default now(),

    -- Ensure name is unique only among active/draft classes if you want
    -- Or keep it fully unique if class names must never repeat
    unique(name)
);
