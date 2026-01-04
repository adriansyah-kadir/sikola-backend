-- First, create an enum type for the possible statuses
create type class_membership_status as enum (
    'pending',    -- request sent, awaiting approval
    'accepted',   -- actively enrolled
    'rejected',   -- request denied
    'withdrawn'   -- student left or was removed
);

-- Then the junction table
create table class_memberships (
    class_id   uuid not null references classes(id) on delete cascade,
    student_id uuid not null,  -- assuming users table
    status     class_membership_status not null default 'accepted',
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    primary key (class_id, student_id)
);

-- Optional: index for querying a student's pending/accepted classes quickly
create index idx_students_classes_student_status 
    on class_memberships(student_id, status);

-- Optional: index for querying a class's members by status
create index idx_students_classes_class_status 
    on class_memberships(class_id, status);
