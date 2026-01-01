create table classes (
  id uuid default uuidv7() primary key,
  name text not null unique,
  description text,
  teacher_id uuid not null,
  is_active boolean not null default true,
  created_at timestamptz not null default now()
);
