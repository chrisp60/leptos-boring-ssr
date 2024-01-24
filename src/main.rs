use axum::{extract::State, response::Html, routing::get, Router};
use leptos::*;

#[derive(Clone, Debug, Default, axum::extract::FromRef)]
struct MyState {
    important_things: std::sync::Arc<std::sync::Mutex<String>>,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_leptos_component))
        .with_state(MyState::default());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_leptos_component(State(state): State<MyState>) -> Html<String> {
    state.important_things.lock().unwrap().push_str("here");
    leptos::ssr::render_to_string(MyPage).to_string().into()
}

#[component]
fn MyPage() -> impl IntoView {
    let on_click = move |_| leptos::logging::error!("Don't expect this to work");
    // No event handlers or reactivity
    view! {
      <p>"hello world!"</p>
      <button on:click=on_click>"Yo!"</button>
    }
}
