use leptos::*;

#[component]
pub fn StatCard(
    label: &'static str,
    value: String,
    detail: &'static str,
    emphasis: bool,
) -> impl IntoView {
    let class = if emphasis {
        "relative min-h-36 overflow-hidden rounded-lg border border-zinc-950 bg-zinc-950 p-4 text-white shadow-[0_18px_60px_rgb(10_10_10_/_0.14)] after:absolute after:inset-x-0 after:bottom-0 after:h-1 after:origin-left after:scale-x-75 after:bg-white dark:border-white dark:bg-white dark:text-black dark:shadow-[0_22px_70px_rgb(0_0_0_/_0.44)]"
    } else {
        "relative min-h-36 overflow-hidden rounded-lg border border-zinc-200 bg-white p-4 text-zinc-950 shadow-[0_18px_60px_rgb(10_10_10_/_0.08)] after:absolute after:inset-x-0 after:bottom-0 after:h-1 after:origin-left after:scale-x-[.35] after:bg-zinc-950 dark:border-zinc-800 dark:bg-zinc-950 dark:text-zinc-50 dark:shadow-[0_22px_70px_rgb(0_0_0_/_0.44)] dark:after:bg-white"
    };
    let muted_class = if emphasis {
        "text-[0.78rem] font-extrabold uppercase text-white/65 dark:text-black/60"
    } else {
        "text-[0.78rem] font-extrabold uppercase text-zinc-500 dark:text-zinc-400"
    };
    let detail_class = if emphasis {
        "mt-3 block text-[0.82rem] font-semibold text-white/65 dark:text-black/60"
    } else {
        "mt-3 block text-[0.82rem] font-semibold text-zinc-500 dark:text-zinc-400"
    };

    view! {
        <article class=class>
            <div class="mb-6 flex items-center justify-between">
                <span class=muted_class>{label}</span>
                <i class="block h-2.5 w-2.5 rounded-full border border-current opacity-50"></i>
            </div>
            <strong class="block overflow-hidden break-words text-[clamp(1.75rem,3vw,2.6rem)] font-black leading-none tracking-normal">{value}</strong>
            <small class=detail_class>{detail}</small>
        </article>
    }
}
