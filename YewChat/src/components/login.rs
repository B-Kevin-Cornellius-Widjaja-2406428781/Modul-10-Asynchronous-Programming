use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="min-h-screen w-screen bg-[#f7fbf5] text-slate-800">
            <div class="mx-auto flex min-h-screen max-w-6xl flex-col items-center justify-center gap-10 px-6 py-12 lg:flex-row">
                <div class="max-w-xl">
                    <div class="inline-flex items-center gap-3 rounded-full border border-emerald-200/60 bg-white/70 px-4 py-2 text-xs uppercase tracking-[0.35em] text-emerald-700">
                        <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
                        {"Yewchat Garden"}
                    </div>
                    <h1 class="mt-6 font-display text-4xl md:text-6xl">{"Say hello in a softer place."}</h1>
                    <p class="mt-4 text-sm md:text-base text-slate-600">
                        {"A calm chat room that feels like a meadow: welcoming, bright, and friendly to every new voice."}
                    </p>
                    <div class="mt-8 flex items-center gap-3 text-sm text-emerald-700">
                        <span class="text-2xl">{"🌿"}</span>
                        {"Pick a name that feels friendly."}
                    </div>
                </div>

                <div class="w-full max-w-md rounded-[32px] border border-emerald-200/70 bg-white p-8 shadow-sm shadow-emerald-100/70">
                    <div class="text-sm uppercase tracking-[0.3em] text-emerald-600">{"Join the room"}</div>
                    <div class="mt-2 font-display text-2xl">{"Choose a nickname"}</div>
                    <form class="mt-6 flex flex-col gap-4">
                        <input
                            {oninput}
                            class="rounded-2xl border border-emerald-200 bg-white px-4 py-3 text-sm text-slate-800 placeholder:text-emerald-400 focus:border-emerald-400 focus:outline-none"
                            placeholder="SunnyTurtle"
                        />
                        <Link<Route> to={Route::Chat}>
                            <button
                                {onclick}
                                disabled={username.len() < 1}
                                class="w-full rounded-2xl bg-emerald-500 px-6 py-3 text-xs font-semibold uppercase tracking-[0.25em] text-white transition hover:translate-y-[-1px] hover:bg-emerald-400 disabled:cursor-not-allowed disabled:bg-emerald-200"
                            >
                                {"Enter the Garden"}
                            </button>
                        </Link<Route>>
                    </form>
                    <div class="mt-6 flex items-center justify-between text-xs text-emerald-700/70">
                        <span>{"New tonight"}</span>
                        <Link<Route> to={Route::Studio} classes="underline decoration-emerald-400/60 underline-offset-4">
                            {"Visit the Studio"}
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </div>
    }
}
