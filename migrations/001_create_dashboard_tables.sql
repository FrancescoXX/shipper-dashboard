CREATE TABLE IF NOT EXISTS dashboard_stats (
    id BIGSERIAL PRIMARY KEY,
    total_members BIGINT NOT NULL,
    current_tier TEXT NOT NULL,
    claimed_spots BIGINT NOT NULL,
    spots_left BIGINT NOT NULL,
    estimated_revenue_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS revenue_points (
    id BIGSERIAL PRIMARY KEY,
    date DATE NOT NULL UNIQUE,
    revenue_cents BIGINT NOT NULL
);
