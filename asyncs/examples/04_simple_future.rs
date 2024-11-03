/// 本节我们手动实现一个简易版的 Future 借此来更深入理解 Future
///   考虑一个需要从 socket 读取数据的场景：如果有数据，可以直接读取数据并返回 Poll::Ready(data)， 
/// 但如果没有数据，Future 会被阻塞且不会再继续执行，此时它会注册一个 wake 函数，当 socket 数据准备好时，
/// 该函数将被调用以通知执行器：我们的 Future 已经准备好了，可以继续执行。

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}


/// 表示从Socket 中读取一次数据的异步任务
pub struct SocketRead<'a> {
    socket: &'a Socket,
}


/// 为 SocketRead 实现 SimpleFuture 特征
impl SimpleFuture for SocketRead<'_> {
    // 异步任务返回值，读取到的字节序列
    type Output = Vec<u8>;

    // 该任务的具体逻辑，尝试从Socket中读取数据
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_read() {
            // socket 中有数据可读, 读取数据, 放入Poll::Ready(bytes) 返回, 表示任务完成
            Poll::Ready(self.socket.read_buf())
        }else {
            // 当前 socket 没有可读数据
            // 注册一个 wake 函数, 当有数据可读时，该函数会被调用, 然后调度器会再次执行该Future的poll函数, 周而复始...
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}


fn main(){
    
}