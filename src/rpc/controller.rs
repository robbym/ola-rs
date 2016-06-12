use rpc::RpcSession;

pub struct RpcController {
    session: RpcSession,
    failed: bool,
    error: String,
}

impl RpcController {
    pub fn new(session: RpcSession) -> RpcController {
        RpcController {
            session: session,
            failed: false,
            error: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.failed = false;
        self.error.clear();
    }

    pub fn failed(&self) -> bool {
        self.failed
    }

    pub fn error(&self) -> &String {
        &self.error
    }

    pub fn set_failed(&mut self, reason: String) {
        self.failed = true;
        self.error = reason;
    }

    pub fn session(&self) -> &RpcSession {
        &self.session
    }
}