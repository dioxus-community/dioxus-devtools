#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::{Arc, Mutex}, thread};
use crossterm::{
    terminal::{
        enable_raw_mode, EnterAlternateScreen,
    },
    ExecutableCommand,
};
use rand::Rng;
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::stdout;
use dioxus_library_template::{use_cool_state::use_cool_state, layer::DevtoolsLogger};
use freya::prelude::*;
use freya_core::plugins::{FreyaPlugin, PluginEvent};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;


struct DevtoolsPlugin(Arc<Mutex<DevtoolsLogger>>);

impl FreyaPlugin for DevtoolsPlugin {
    fn on_event(&mut self, event: &PluginEvent) {
        if let PluginEvent::WindowCreated(_) = event {
            let devtools = self.0.clone();
            thread::spawn(move || {
                stdout().execute(EnterAlternateScreen).unwrap();
                enable_raw_mode().unwrap();
                let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
                terminal.clear().unwrap();
                loop {
                    terminal.draw(|frame| {
                        let mut i = 0;
                        for (id, scope) in devtools.lock().unwrap().scopes.iter() {
                            
                            let area = ratatui::prelude::Rect::new(0, i, frame.size().width, 1);
                            frame.render_widget(
                                Paragraph::new(format!("Scope {}: {}", id, scope.len()))
                                    .white(),
                                area,
                            );
                            i+=1;
                            for hook in scope.values() {
                                let area = ratatui::prelude::Rect::new(0, i, frame.size().width, 1);
                                frame.render_widget(
                                    Paragraph::new(format!(" > {hook:?}"))
                                        .white(),
                                    area,
                                );
                                i+=1;
                            }
                        }
                    }).unwrap();
                    
                }
                //stdout().execute(LeaveAlternateScreen).unwrap();
                //disable_raw_mode().unwrap();
            });
        }
    }
}

fn main() {
    let (devtools, layer) = DevtoolsLogger::new();
    tracing_subscriber::registry().with(layer).init();

    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_plugin(DevtoolsPlugin(devtools))
            .build(),
    )
}

fn app(cx: Scope) -> Element {
    let elements = use_ref(cx, || Vec::new());

    render!(
        Button {
            onclick: |_| {
                let mut rng = rand::thread_rng();
                elements.write().push(rng.gen());
            },
            label {
                "Add"
            }
        }
        Button {
            onclick: |_| {
                elements.write().pop();
            },
            label {
                "Remove"
            }
        }
        elements.read().iter().map(|e: &usize| rsx!(
            Something {
                key: "{e}",
                id: *e
            }
        ))
    )
}

#[allow(non_snake_case)]
#[component]
fn Something(cx: Scope, id: usize) -> Element {
    let _x = use_cool_state(cx, || 2);
    render!(label {
        "Wathever -> {id}"
    })
}