create table students_classes (
  class_id uuid not null references classes(id),
  student_id uuid not null,
  is_active boolean not null default true,
  created_at timestamptz not null default now(),
  primary key (class_id, student_id)
);
