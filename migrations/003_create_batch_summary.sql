CREATE TABLE IF NOT EXISTS batch_summary (
    id BIGSERIAL PRIMARY KEY,
    batch_number BIGINT NOT NULL,
    total_batches BIGINT NOT NULL,
    claimed_spots BIGINT NOT NULL,
    total_spots BIGINT NOT NULL,
    spots_left BIGINT NOT NULL,
    tier_name TEXT NOT NULL,
    founder_name TEXT NOT NULL,
    founder_avatar_url TEXT NOT NULL,
    cta_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO batch_summary (
    batch_number,
    total_batches,
    claimed_spots,
    total_spots,
    spots_left,
    tier_name,
    founder_name,
    founder_avatar_url,
    cta_url
)
VALUES (
    2,
    10,
    16,
    100,
    84,
    'Legend',
    'OrcDev',
    'https://www.shipper.club/builders/orcdev.jpeg',
    'https://www.shipper.club/'
)
ON CONFLICT DO NOTHING;
