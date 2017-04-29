use tokio_core;

pub struct JobManager {
    tokio_core_handle: tokio_core::reactor::Handle,
}

impl JobManager {
    pub fn new() -> JobManager {
        let mut core = tokio_core::reactor::Core::new().unwrap();

        JobManager { tokio_core_handle: core.handle() }
    }
}
