extern crate ola;
use ola::rpc::{RpcController, RpcSession};

#[test]
fn rpc_controller_set_failed() {
    let mut ctrl = RpcController::new(RpcSession);
    ctrl.set_failed(String::from("Yo!"));
    assert_eq!(ctrl.error(), "Yo!");
}