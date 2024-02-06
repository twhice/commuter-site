#![feature(async_closure)]
use leptos::*;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://154.8.150.125:12000";
// const BASE_URL: &str = "http://localhost:12000";

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    name: String,
    passwd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RegisterResult {
    Exist,
    Success,
    Unknown,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SignInRequest {
    name: String,
    passwd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SignInResult {
    IncorrectPassword,
    UserDoesNotExist,
    Success,
    Unknown,
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (passwd, set_password) = create_signal(String::new());
    let (info, set_info) = create_signal(String::new());

    let clear_input = move || {
        set_name(String::new());
        set_password(String::new())
    };

    // let res = create_resource()

    let register = move |_ev| {
        if name().is_empty() || passwd().is_empty() {
            return;
        }

        spawn_local(async move {
            let requert = RegisterRequest {
                name: name(),
                passwd: passwd(),
            };
            let result = reqwest::Client::new()
                .post(format!("{BASE_URL}/register"))
                .json(&requert)
                .send()
                .await
                .unwrap()
                .json::<RegisterResult>()
                .await
                .unwrap();

            let info = match result {
                RegisterResult::Success => "注册成功！",
                RegisterResult::Exist => {
                    clear_input();
                    "用户已存在"
                }
                RegisterResult::Unknown => {
                    clear_input();
                    "肮脏的Hecker！"
                }
            };
            set_info(info.into());
        })
    };

    let sigh_in = move |_ev| {
        if name().is_empty() || passwd().is_empty() {
            return;
        }

        spawn_local(async move {
            let request = SignInRequest {
                name: name(),
                passwd: passwd(),
            };
            let result = reqwest::Client::new()
                .post(format!("{BASE_URL}/sign_in"))
                .json(&request)
                .send()
                .await
                .unwrap()
                .json::<SignInResult>()
                .await
                .unwrap();
            match result {
                SignInResult::Success => {
                    set_info("打卡成功！".into());
                }
                SignInResult::IncorrectPassword => {
                    set_password("".into());
                    set_info("密码错误！".into());
                }
                SignInResult::UserDoesNotExist => {
                    clear_input();
                    set_info("用户不存在！".into());
                }
                SignInResult::Unknown => {
                    set_info("肮脏的Hecker！".into());
                }
            }
        })
    };

    view! {
        <div>
            <h1>幻海星河 <br/> 员工打卡系统</h1>
            <div>
                <input
                    type="text"
                    on:input=move |ev| {
                        set_name(event_target_value(&ev));
                    }

                    prop:value=name
                />
            </div>
            <div>
                <input
                    type="password"
                    on:input=move |ev| {
                        set_password(event_target_value(&ev));
                    }

                    prop:value=passwd
                />
            </div>
            <div>
                <button on:click=register>注册</button>
                <button on:click=sigh_in>打卡</button>
            </div>
            <p>{info}</p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}
