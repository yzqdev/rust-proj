#![windows_subsystem = "windows"]
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
            div {
                id: "hero",
                img { src: HEADER_SVG, id: "header" }

                button {
                    onclick: async move |_|{
                        let client = reqwest::Client::new();
                        let res = client.post("http://httpbin.org/post")
                        .body("the exact body that is sent")
                        .send()
                        .await;
                        match res {
                             Ok(data) => {
                             println!("{:?}",data.text().await.unwrap())
              },
        Err(_) => todo!(),
    }


                    },
                    "点击我"
                }
            }
        }
}
