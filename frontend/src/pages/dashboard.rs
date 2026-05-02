use crate::{
    api::client::{fetch_revenue, fetch_stats},
    components::{simple_chart::SimpleChart, stat_card::StatCard},
};
use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let stats = create_resource(|| (), |_| async { fetch_stats().await });
    let revenue = create_resource(|| (), |_| async { fetch_revenue().await });
    let (dark_mode, set_dark_mode) = create_signal(true);

    view! {
        <style>{include_str!("../styles.css")}</style>
        <section class=move || {
            if dark_mode.get() {
                "dark min-h-screen bg-[linear-gradient(180deg,rgb(5_5_5_/_0.96),rgb(14_14_12_/_0.98)),radial-gradient(circle_at_top_right,rgb(255_255_255_/_0.10),transparent_32%)] font-sans text-zinc-50"
            } else {
                "min-h-screen bg-[linear-gradient(180deg,rgb(255_255_255_/_0.86),rgb(247_247_244_/_0.94)),radial-gradient(circle_at_top_right,rgb(0_0_0_/_0.08),transparent_34%)] font-sans text-zinc-950"
            }
        }>
            <div class="mx-auto w-[min(1160px,calc(100%_-_32px))] py-8 max-sm:w-[calc(100%_-_24px)] max-sm:py-6">
                <header class="mb-6 flex items-start justify-between gap-5 max-sm:flex-col">
                    <div>
                        <div class="mb-3 flex items-center gap-2.5">
                            <img
                                class="block h-10 w-10 rounded-lg border border-zinc-200 bg-white p-1 object-cover shadow-[0_8px_24px_rgb(0_0_0_/_0.12)] dark:border-zinc-800 dark:bg-zinc-950"
                                src="https://www.shipper.club/favicon.ico"
                                alt="Shipper Club icon"
                                width="32"
                                height="32"
                            />
                            <p class="m-0 text-xs font-extrabold uppercase tracking-normal text-zinc-500 dark:text-zinc-400">"Shipper Club"</p>
                        </div>
                        <h1 class="m-0 max-w-[900px] text-[clamp(2.45rem,6vw,6.4rem)] font-black leading-[0.88] tracking-normal">"The place that sharpens your taste."</h1>
                    </div>
                    <div class="flex shrink-0 items-center gap-2.5 max-sm:w-full max-sm:justify-between">
                        <a
                            class="inline-flex min-h-9 items-center justify-center whitespace-nowrap rounded-full border border-red-400 bg-shipper-red px-4 text-[0.82rem] font-extrabold text-white shadow-[0_14px_34px_rgb(225_29_29_/_0.26)] hover:bg-red-500"
                            href="https://www.shipper.club/"
                            target="_blank"
                            rel="noreferrer"
                        >
                            "Claim your spot"
                        </a>
                        <div class="flex rounded-full border border-zinc-200 bg-white/80 p-1 dark:border-zinc-800 dark:bg-zinc-950/80" role="group" aria-label="Theme">
                            <button
                                type="button"
                                class=move || {
                                    if dark_mode.get() {
                                        "min-w-14 cursor-pointer rounded-full px-2.5 py-1.5 text-[0.8rem] font-bold text-zinc-500 dark:text-zinc-400"
                                    } else {
                                        "min-w-14 cursor-pointer rounded-full bg-zinc-950 px-2.5 py-1.5 text-[0.8rem] font-bold text-white"
                                    }
                                }
                                on:click=move |_| set_dark_mode.set(false)
                            >
                                "Light"
                            </button>
                            <button
                                type="button"
                                class=move || {
                                    if dark_mode.get() {
                                        "min-w-14 cursor-pointer rounded-full bg-white px-2.5 py-1.5 text-[0.8rem] font-bold text-black"
                                    } else {
                                        "min-w-14 cursor-pointer rounded-full px-2.5 py-1.5 text-[0.8rem] font-bold text-zinc-500"
                                    }
                                }
                                on:click=move |_| set_dark_mode.set(true)
                            >
                                "Dark"
                            </button>
                        </div>
                        <span class="rounded-full border border-zinc-200 px-3 py-2 text-[0.78rem] font-extrabold text-zinc-500 dark:border-zinc-800 dark:text-zinc-400">"MVP"</span>
                    </div>
                </header>

                <div class="mb-3.5 grid grid-cols-[1fr_auto_auto] items-center gap-3 rounded-lg bg-zinc-950 p-5 text-white shadow-[0_18px_60px_rgb(10_10_10_/_0.12)] dark:bg-white dark:text-black dark:shadow-[0_22px_70px_rgb(0_0_0_/_0.44)] max-lg:grid-cols-1 max-lg:items-start">
                    <div>
                        <p class="m-0 mb-1 text-[0.88rem] font-bold text-white/70 dark:text-black/65">"Current batch - become a Legend"</p>
                        <span class="text-[0.88rem] font-bold text-white/70 dark:text-black/65">"Batch 2 of 10 · 16/100 claimed"</span>
                    </div>
                    <strong class="text-[clamp(1.2rem,3vw,2.35rem)] font-black uppercase leading-none tracking-normal">"84 spots left"</strong>
                    <div class="flex items-center gap-2.5 rounded-full border border-white/25 py-1.5 pl-1.5 pr-3 dark:border-black/20">
                        <img
                            class="block h-[42px] w-[42px] rounded-full border border-white/45 object-cover dark:border-black/35"
                            src="https://www.shipper.club/builders/orcdev.jpeg"
                            alt="OrcDev avatar"
                            width="42"
                            height="42"
                        />
                        <div class="grid gap-px">
                            <span class="text-[0.68rem] font-bold uppercase text-white/70 dark:text-black/60">"Founded by"</span>
                            <b class="text-[0.92rem] leading-none">"OrcDev"</b>
                        </div>
                    </div>
                </div>

                <Suspense fallback=move || view! { <p class="text-zinc-500 dark:text-zinc-400">"Loading dashboard..."</p> }>
                    {move || {
                        stats.get().map(|result| match result {
                            Ok(stats) => view! {
                                <div class="mb-3.5 grid grid-cols-5 gap-3 max-lg:grid-cols-2 max-sm:grid-cols-1">
                                    <StatCard label="Total members" value=stats.total_members.to_string() detail="Builders inside" emphasis=false />
                                    <StatCard label="Current tier" value=stats.current_tier detail="Open now" emphasis=true />
                                    <StatCard label="Claimed spots" value=stats.claimed_spots.to_string() detail="This batch" emphasis=false />
                                    <StatCard label="Spots left" value=stats.spots_left.to_string() detail="Until price jump" emphasis=false />
                                    <StatCard label="Estimated revenue" value=format_cents(stats.estimated_revenue_cents) detail="Seeded gross" emphasis=true />
                                </div>
                            }.into_view(),
                            Err(error) => view! { <p class="text-red-600">{error}</p> }.into_view(),
                        })
                    }}
                </Suspense>

                <section class="rounded-lg border border-zinc-200 bg-white/95 p-5 shadow-[0_18px_60px_rgb(10_10_10_/_0.08)] dark:border-zinc-800 dark:bg-zinc-950/95 dark:shadow-[0_22px_70px_rgb(0_0_0_/_0.44)]">
                    <div class="mb-5 flex items-start justify-between gap-3 max-sm:flex-col">
                        <div>
                            <p class="m-0 mb-2 text-xs font-extrabold uppercase tracking-normal text-zinc-500 dark:text-zinc-400">"Signal"</p>
                            <h2 class="m-0 text-[1.15rem] font-black tracking-normal">"Revenue over time"</h2>
                        </div>
                        <p class="m-0 text-zinc-500 dark:text-zinc-400">"Seeded MVP data"</p>
                    </div>
                    <Suspense fallback=move || view! { <p class="text-zinc-500 dark:text-zinc-400">"Loading revenue..."</p> }>
                        {move || {
                            revenue.get().map(|result| match result {
                                Ok(points) => view! { <SimpleChart points=points /> }.into_view(),
                                Err(error) => view! { <p class="text-red-600">{error}</p> }.into_view(),
                            })
                        }}
                    </Suspense>
                </section>
            </div>
        </section>
    }
}

fn format_cents(cents: i64) -> String {
    format!("${:.2}", cents as f64 / 100.0)
}
