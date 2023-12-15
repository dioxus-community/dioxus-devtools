use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tracing_subscriber::Layer;

#[derive(Debug, Clone, PartialEq)]
pub enum HookState {
    Added,
    Removed,
}

impl HookState {
    pub fn from_str(txt: &str) -> Option<Self> {
        match txt {
            "added" => Some(Self::Added),
            "removed" => Some(Self::Removed),
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Hook {
    UseState { value: String },
}

#[derive(Default, Debug)]
pub struct DevtoolsLogger {
    pub scopes: HashMap<usize, HashMap<usize, Hook>>,
}

impl DevtoolsLogger {
    pub fn new() -> (Arc<Mutex<Self>>, CustomLayer) {
        let devtools = Arc::new(Mutex::new(DevtoolsLogger::default()));
        (devtools.clone(), CustomLayer(devtools))
    }
}

pub struct CustomLayer(pub Arc<Mutex<DevtoolsLogger>>);

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut builder = HookBuilder::default();
        let mut visitor = EventVisitor(&mut builder);
        event.record(&mut visitor);

        let scope_id = builder.scope_id.clone();
        let hook_idx = builder.hook_idx.clone();
        let state = builder.state.clone();

        let val: Result<Hook, _> = builder.try_into();
        if let Ok(ev) = val {
            if let Some((scope_id, hook_idx)) = scope_id.zip(hook_idx) {
                let mut devtools = self.0.lock().unwrap();
                let scope_active = devtools.scopes.entry(scope_id).or_default();

                if state == Some(HookState::Removed) {
                    scope_active.remove(&hook_idx);

                    if scope_active.is_empty() {
                        devtools.scopes.remove(&scope_id);
                    }
                } else {
                    scope_active.insert(hook_idx, ev);
                }
            }
        }
    }
}

#[derive(Default)]
struct HookBuilder {
    scope_id: Option<usize>,
    name: Option<String>,
    value: Option<String>,
    state: Option<HookState>,
    hook_idx: Option<usize>,
}

impl TryFrom<HookBuilder> for Hook {
    type Error = ();

    fn try_from(value: HookBuilder) -> Result<Self, Self::Error> {
        match value.name.as_ref() {
            Some(v) if v == "UseState" => Ok(Self::UseState {
                value: value.value.unwrap(),
            }),
            _ => Err(()),
        }
    }
}

struct EventVisitor<'a>(&'a mut HookBuilder);

impl<'a> tracing::field::Visit for EventVisitor<'a> {
    fn record_f64(&mut self, _field: &tracing::field::Field, _value: f64) {
        //println!("  field={} value={}", field.name(), value)
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        match field.name() {
            "hook_idx" => self.0.hook_idx = Some(value as usize),
            "scope_id" => self.0.scope_id = Some(value as usize),
            _ => {}
        }
    }

    fn record_u64(&mut self, _field: &tracing::field::Field, _value: u64) {
        ////println!("  field={} value={}", field.name(), value)
    }

    fn record_bool(&mut self, _field: &tracing::field::Field, _value: bool) {
        ////println!("  field={} value={}", field.name(), value)
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "name" => self.0.name = Some(value.to_string()),
            "value" => self.0.value = Some(value.to_string()),
            "state" => self.0.state = HookState::from_str(value),
            _ => {}
        }
    }

    fn record_error(
        &mut self,
        _field: &tracing::field::Field,
        _value: &(dyn std::error::Error + 'static),
    ) {
        ////println!("  field={} value={}", field.name(), value)
    }

    fn record_debug(&mut self, _field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {
        ////println!("  field={} value={:?}", field.name(), value)
    }
}
