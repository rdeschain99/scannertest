use flowy_sys::prelude::*;
use std::{cell::RefCell, sync::Once};

#[allow(dead_code)]
pub fn setup_env() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        std::env::set_var("RUST_LOG", "flowy_sys=debug,debug");
        env_logger::init();
    });
}

pub async fn async_send(data: DispatchRequest<i64>) -> Result<EventResponse, SystemError> {
    EventDispatch::async_send(data).await
}

pub fn init_system<F>(module_factory: F)
where
    F: FnOnce() -> Vec<Module>,
{
    let system = EventDispatch::new(module_factory);
    EventDispatch::set_current(system);
}