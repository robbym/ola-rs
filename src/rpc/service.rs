use protobuf::descriptor::MethodDescriptorProto;
use protobuf::core::Message;

use rpc::RpcController;

pub type CompletionCallback = Fn();

pub trait RpcService {
    fn descriptor<'a>() -> &'a MethodDescriptorProto;
    fn call_method(method: &MethodDescriptorProto, controller: &mut RpcController, request: &Message, response: &mut Message, done: CompletionCallback);
    fn request_proto<'a>(method: &MethodDescriptorProto) -> &'a Message;
    fn response_proto<'a>(method: &MethodDescriptorProto) -> &'a Message;
}