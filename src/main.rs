#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;

mod dashboard;

fn main() {
    launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "Hello, world!" }
        h2 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }

        button {
            onclick: move |_| async move {
                let _ = print_users_counter(count()).await;
            },
            "Print counter on the server"
        }

        dashboard::Dashboard {}
    }
}

// Server Function only runs on the server
// Server Functions need to be async and return a Result
#[server]
async fn print_users_counter(value: i32) -> Result<(), ServerFnError> {
    println!("Your value is: {}", value);
    Ok(())
}
