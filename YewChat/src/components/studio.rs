use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Studio)]
pub fn studio() -> Html {
    html! {
        <div class="min-h-screen w-screen bg-[#f7fbf5] text-slate-800">
            <div class="mx-auto max-w-6xl px-6 py-12">
                <div class="flex flex-wrap items-center justify-between gap-6">
                    <div>
                        <div class="flex items-center gap-3 text-xs uppercase tracking-[0.4em] text-emerald-600">
                            <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
                            {"Studio Garden"}
                        </div>
                        <h1 class="mt-4 font-display text-4xl md:text-6xl">{"Notes for a kinder chat"}</h1>
                        <p class="mt-4 max-w-xl text-sm md:text-base text-slate-600">
                            {"A gentle prep space: pick your vibe, choose a tiny ritual, and bring the warmest version of your voice."}
                        </p>
                    </div>
                    <Link<Route>
                        to={Route::Login}
                        classes="rounded-full border border-emerald-200 bg-white/80 px-6 py-2 text-xs uppercase tracking-[0.25em] hover:border-emerald-400 hover:text-emerald-600 transition"
                    >
                        {"Back to Login"}
                    </Link<Route>>
                </div>

                <div class="mt-12 grid gap-6 md:grid-cols-3">
                    <div class="rounded-3xl border border-emerald-200/70 bg-white/80 p-6 shadow-sm shadow-emerald-100/60">
                        <div class="text-3xl">{"🪴"}</div>
                        <div class="mt-4 font-display text-xl">{"Welcome Ritual"}</div>
                        <p class="mt-2 text-sm text-emerald-700/70">
                            {"Say hello with one small win. The first message sets the tone."}
                        </p>
                    </div>
                    <div class="rounded-3xl border border-emerald-200/70 bg-white/80 p-6 shadow-sm shadow-emerald-100/60">
                        <div class="text-3xl">{"🍃"}</div>
                        <div class="mt-4 font-display text-xl">{"Breeze Mode"}</div>
                        <p class="mt-2 text-sm text-emerald-700/70">
                            {"Short replies, soft timing, and a friendly pace."}
                        </p>
                    </div>
                    <div class="rounded-3xl border border-emerald-200/70 bg-white/80 p-6 shadow-sm shadow-emerald-100/60">
                        <div class="text-3xl">{"🧩"}</div>
                        <div class="mt-4 font-display text-xl">{"Prompt Pebbles"}</div>
                        <p class="mt-2 text-sm text-emerald-700/70">
                            {"Tiny prompts that keep the room playful and easy."}
                        </p>
                    </div>
                </div>

                <div class="mt-12 grid gap-6 lg:grid-cols-[1.4fr_1fr]">
                    <div class="rounded-[32px] border border-emerald-200/70 bg-white/90 p-8 shadow-sm shadow-emerald-100/60">
                        <h2 class="font-display text-2xl">{"Little calm checklist"}</h2>
                        <ul class="mt-6 space-y-3 text-sm text-emerald-700/80">
                            <li>{"1. Choose a name that feels welcoming."}</li>
                            <li>{"2. Share one small good thing from today."}</li>
                            <li>{"3. Ask a gentle question to open the room."}</li>
                        </ul>
                        <div class="mt-8 flex flex-wrap gap-3 text-xs">
                            <span class="rounded-full border border-emerald-200 bg-white px-3 py-1">{"gentle"}</span>
                            <span class="rounded-full border border-emerald-200 bg-white px-3 py-1">{"bright"}</span>
                            <span class="rounded-full border border-emerald-200 bg-white px-3 py-1">{"cozy"}</span>
                            <span class="rounded-full border border-emerald-200 bg-white px-3 py-1">{"fresh"}</span>
                        </div>
                    </div>
                    <div class="rounded-[32px] border border-emerald-200/70 bg-white/80 p-8 shadow-sm shadow-emerald-100/60">
                        <h2 class="font-display text-2xl">{"Icons of the day"}</h2>
                        <div class="mt-6 grid gap-4">
                            <div class="flex items-center gap-4 rounded-2xl border border-emerald-200/70 bg-white p-4">
                                <div class="text-2xl">{"🌼"}</div>
                                <div>
                                    <div class="text-sm font-semibold">{"Daisy Ping"}</div>
                                    <div class="text-xs text-emerald-700/70">{"Use for warm greetings and quick hellos."}</div>
                                </div>
                            </div>
                            <div class="flex items-center gap-4 rounded-2xl border border-emerald-200/70 bg-white p-4">
                                <div class="text-2xl">{"🍯"}</div>
                                <div>
                                    <div class="text-sm font-semibold">{"Honey Note"}</div>
                                    <div class="text-xs text-emerald-700/70">{"Best for compliments and gratitude."}</div>
                                </div>
                            </div>
                            <div class="flex items-center gap-4 rounded-2xl border border-emerald-200/70 bg-white p-4">
                                <div class="text-2xl">{"🫧"}</div>
                                <div>
                                    <div class="text-sm font-semibold">{"Bubble Pause"}</div>
                                    <div class="text-xs text-emerald-700/70">{"Pair with thoughtful, slower messages."}</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
