use leptos::*;
use shared::RevenuePoint;

#[component]
pub fn SimpleChart(points: Vec<RevenuePoint>) -> impl IntoView {
    let max_revenue = points
        .iter()
        .map(|point| point.revenue_cents)
        .max()
        .unwrap_or(1)
        .max(1);

    view! {
        <div
            class="grid h-[280px] grid-cols-[repeat(auto-fit,minmax(58px,1fr))] items-end gap-3 border-b border-zinc-300 pt-4 dark:border-zinc-700"
            role="img"
            aria-label="Revenue over time bar chart"
        >
            {points.into_iter().map(|point| {
                let height = ((point.revenue_cents as f64 / max_revenue as f64) * 100.0).max(8.0);
                let revenue = format!("${:.0}", point.revenue_cents as f64 / 100.0);
                let date_label = point.date.format("%b %-d").to_string();
                let bar_label = format!("{date_label}: {revenue}");
                view! {
                    <div class="flex h-full min-w-0 flex-col items-center justify-end" aria-label=bar_label>
                        <span class="mb-2 text-[0.72rem] font-extrabold text-zinc-500 dark:text-zinc-400">{revenue}</span>
                        <div
                            class="min-h-2 w-full rounded-t-lg bg-zinc-950 dark:bg-zinc-50"
                            style=format!("height: {height:.1}%")
                        ></div>
                        <span class="mt-2 min-h-[18px] text-[0.76rem] font-bold text-zinc-500 dark:text-zinc-400">{date_label}</span>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
