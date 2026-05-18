use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    badge: String,
    badge_class: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                badge: user_badge(u),
                                badge_class: user_badge_class(u).into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex min-h-screen w-screen bg-[#f7fbf5] text-slate-800">
                <div class="flex-none w-64 border-r border-emerald-200/70 bg-white/70">
                    <div class="flex items-center justify-between px-4 py-4">
                        <div>
                            <div class="text-xs uppercase tracking-[0.35em] text-emerald-600">{"Neighbors"}</div>
                            <div class="font-display text-lg">{"In the garden"}</div>
                        </div>
                        <div class="flex h-8 w-8 items-center justify-center rounded-full border border-emerald-200 text-xs">{"☀️"}</div>
                    </div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="mx-3 mb-3 flex items-center gap-3 rounded-2xl border border-emerald-200/70 bg-white p-3">
                                    <div class={classes!("flex", "h-12", "w-12", "items-center", "justify-center", "rounded-full", "text-sm", "font-semibold", u.badge_class.clone())}>
                                        {u.badge.clone()}
                                    </div>
                                    <div>
                                        <div class="text-sm font-semibold">{u.name.clone()}</div>
                                        <div class="text-xs text-emerald-600/80">{"Here and listening"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                <div class="flex h-screen grow flex-col">
                    <div class="flex items-center justify-between border-b border-emerald-200/70 px-6 py-4">
                        <div>
                            <div class="text-xs uppercase tracking-[0.35em] text-emerald-600">{"Channel"}</div>
                            <div class="font-display text-xl">{"Sunlit Chat"}</div>
                        </div>
                        <div class="flex items-center gap-3 text-xs text-emerald-700/80">
                            <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
                            {"Online"}
                        </div>
                    </div>
                    <div class="w-full grow overflow-auto px-6 py-4">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html!{
                                    <div class="mb-6 flex max-w-3xl items-end gap-4 rounded-[28px] border border-emerald-200/70 bg-white p-4 shadow-sm shadow-emerald-100/60">
                                        <div class={classes!("flex", "h-10", "w-10", "items-center", "justify-center", "rounded-full", "text-xs", "font-semibold", user.badge_class.clone())}>
                                            {user.badge.clone()}
                                        </div>
                                        <div>
                                            <div class="text-sm font-semibold">{m.from.clone()}</div>
                                            <div class="text-sm text-slate-600">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-3 rounded-2xl" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }

                    </div>
                    <div class="flex items-center gap-3 border-t border-emerald-200/70 px-6 py-4">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Share something gentle"
                            class="block w-full rounded-2xl border border-emerald-200 bg-white px-4 py-3 text-sm text-slate-800 placeholder:text-emerald-400 focus:border-emerald-400 focus:outline-none"
                            name="message"
                            required=true
                        />
                        <button onclick={submit} class="flex h-11 w-11 items-center justify-center rounded-2xl bg-emerald-500 text-white">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 fill-current">
                                <path d="M0 0h24v24H0z" fill="none"></path>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

fn user_badge(name: &str) -> String {
    name.chars()
        .next()
        .map(|c| c.to_ascii_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string())
}

fn user_badge_class(name: &str) -> &'static str {
    let sum: usize = name.bytes().map(|b| b as usize).sum();
    match sum % 4 {
        0 => "bg-emerald-200 text-emerald-800",
        1 => "bg-lime-200 text-lime-800",
        2 => "bg-teal-200 text-teal-800",
        _ => "bg-amber-200 text-amber-800",
    }
}
