-- Seed data for local development and testing.
-- Re-runnable: wipes existing rows, then inserts fresh examples.
-- IDs are left to all-defaults so uuidv7() generates them.

TRUNCATE TABLE tasks;

INSERT INTO tasks (name, is_done, due_date, created_at, updated_at, deleted_at)
VALUES
    -- Brand new, nothing done yet, never updated.
    (
        'Set up project skeleton',
        false,
        NULL,
        now() - interval '5 days',
        NULL,
        NULL
    ),

    -- Completed.
    (
        'Set up PostgreSQL',
        true,
        NULL,
        now() - interval '4 days',
        now() - interval '4 days',
        NULL
    ),

    -- In progress: created earlier, updated recently.
    (
        'Design REST endpoints',
        false,
        NULL,
        now() - interval '3 days',
        now() - interval '1 hour',
        NULL
    ),

    -- Not touched for a while, still open.
    (
        'Write integration tests',
        false,
        NULL,
        now() - interval '2 days',
        NULL,
        NULL
    ),

    -- With due_date in past
    (
        'Drafty sketch with due date in past!',
        true,
        now() - interval '4 days',
        now() - interval '6 days',
        now() - interval '5 days',
        NULL
    ),

    -- With due_date in future
    (
        'Drafty sketch with due date within few weeks in future',
        true,
        now() + interval '100 days',
        now() - interval '6 days',
        now() - interval '5 days',
        NULL
    ),

    -- Soft-deleted: kept for audit, but filtered out by queries.
    (
        'Draft architecture sketch',
        true,
        NULL,
        now() - interval '6 days',
        now() - interval '5 days',
        now() - interval '1 day'
    );
