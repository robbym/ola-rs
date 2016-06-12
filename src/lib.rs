pub struct RpcSession;
pub struct RpcController {
    session: Option<RpcSession>,
    failed: bool,
    error_str: String,
}

impl RpcController {
    pub fn new() -> RpcController {
        RpcController{
            session: None,
            failed: false,
            error_str: String::new()
        }
    }

    pub fn reset(&mut self) {

    }

    pub fn failed(&self) -> bool {
        false
    }

    pub fn error_str(&self) -> &String {
        &self.error_str
    }

    pub fn set_failed(&mut self, reason: String) {

    }

    pub fn session(&self) -> RpcSession {
        RpcSession
    }
}