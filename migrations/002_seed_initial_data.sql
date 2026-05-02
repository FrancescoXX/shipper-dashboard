INSERT INTO dashboard_stats (
    total_members,
    current_tier,
    claimed_spots,
    spots_left,
    estimated_revenue_cents
)
VALUES (116, 'Legend', 16, 84, 1196400)
ON CONFLICT DO NOTHING;

INSERT INTO revenue_points (date, revenue_cents)
VALUES
    ('2026-04-27', 640000),
    ('2026-04-28', 720000),
    ('2026-04-29', 810000),
    ('2026-04-30', 900000),
    ('2026-05-01', 990000),
    ('2026-05-02', 1196400)
ON CONFLICT (date) DO UPDATE
SET revenue_cents = EXCLUDED.revenue_cents;
