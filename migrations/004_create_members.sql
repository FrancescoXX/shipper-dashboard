CREATE TABLE IF NOT EXISTS members (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    avatar_url TEXT,
    tier TEXT NOT NULL,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO members (name, avatar_url, tier, joined_at)
VALUES
    ('OrcDev',    'https://www.shipper.club/builders/orcdev.jpeg', 'Legend',  '2026-04-15T10:00:00Z'),
    ('Alice',     NULL, 'Legend',  '2026-04-16T12:00:00Z'),
    ('Bob',       NULL, 'Legend',  '2026-04-17T09:30:00Z'),
    ('Charlie',   NULL, 'Legend',  '2026-04-18T14:00:00Z'),
    ('Diana',     NULL, 'Legend',  '2026-04-19T11:15:00Z'),
    ('Eve',       NULL, 'Builder', '2026-04-20T16:45:00Z'),
    ('Frank',     NULL, 'Builder', '2026-04-21T08:00:00Z'),
    ('Grace',     NULL, 'Builder', '2026-04-22T13:20:00Z'),
    ('Hank',      NULL, 'Builder', '2026-04-23T10:50:00Z'),
    ('Ivy',       NULL, 'Builder', '2026-04-24T15:00:00Z'),
    ('Jack',      NULL, 'Starter', '2026-04-25T09:00:00Z'),
    ('Karen',     NULL, 'Starter', '2026-04-26T12:30:00Z'),
    ('Leo',       NULL, 'Starter', '2026-04-27T14:10:00Z'),
    ('Mona',      NULL, 'Starter', '2026-04-28T11:00:00Z'),
    ('Nick',      NULL, 'Starter', '2026-04-29T16:00:00Z'),
    ('Olivia',    NULL, 'Starter', '2026-04-30T08:45:00Z')
ON CONFLICT DO NOTHING;
