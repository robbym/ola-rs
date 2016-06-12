use rpc::RpcChannel;

pub struct RpcSession {
    channel: RpcChannel
}

impl RpcSession {
    pub fn new(channel: RpcChannel) -> RpcSession {
        RpcSession {
            channel: channel,
        }
    }

    pub fn channel(&self) -> &RpcChannel {
        &self.channel
    }
}