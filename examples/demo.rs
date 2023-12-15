use dioxus_library_template::layer::DevtoolsLogger;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    let (devtools, layer) = DevtoolsLogger::new();
    tracing_subscriber::registry().with(layer).init();

    info!(scope_id = 1, name = "UseState", value = "123", hook_idx = 0, state = "added");

    println!("{:#?}",devtools.lock().unwrap().scopes);

    info!(scope_id = 1, name = "UseState", value = "123", hook_idx = 0, state = "removed");

    println!("{:#?}",devtools.lock().unwrap().scopes);
}

