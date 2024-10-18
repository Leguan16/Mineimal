use egui::{TextBuffer, Ui, Window};
use graph_rs_sdk::http::HttpResponseExt;
use graph_rs_sdk::identity::PublicClientApplication;

pub trait Auth {
    fn add_account();
    async fn poll_device_code(ui: &Ui) -> anyhow::Result<Session>;
}

pub struct FetchCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: usize,
    interval: usize,
    message: String,
}

pub struct Session {
    token: String,
    refresh_token: String,
}

impl Session {}

impl Auth for Session {
    fn add_account() {}

    async fn poll_device_code(ui: &Ui) -> anyhow::Result<Session> {
        let client = PublicClientApplication::builder("7527a3cf-2408-4077-8112-23ef3bc7b489")
            .with_device_code_executor()
            .with_scope(Vec::from(["user.read", "openid", "profile"]))
            .poll()?;

        while let Ok(response) = client.recv() {
            let window = egui::Window::new("Add Account");

            if !response.body().clone()?["access_token"].clone().as_str().is_none() {
                return Ok(Session {token: response.body().clone()?["access_token"].clone().as_str().unwrap().parse()?, refresh_token: response.body().clone()?["refresh_token"].clone().as_str().unwrap().parse()? })
            }
            
            if !response.body().clone()?["message"].clone().as_str().is_none() {
                println!("{}", response.body().clone().unwrap()["message"].as_str().unwrap());
                window.show(ui.ctx(), |ui| {
                        ui.label(response.body().clone().unwrap()["message"].as_str().unwrap())
                    });
            }
        }

        Err(anyhow::Error::msg("Process Aborted"))
    }
}
