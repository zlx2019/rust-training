/// 什么是执行单元？
///     Node(分布式集群节点) -> Process(进程) -> Thread(线程) -> Goroutine/Promise/Future...(异步任务)
///   这些都可以视认为一个执行单元，不同之处在于它们的运行成本以及粒度，逐渐变的更加轻量级.
/// 
/// ```
/// pub trait Future{
///     type Output;
///    
///     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
/// }
/// 
/// pub enum Poll<T>{
///     Ready(T),
///     Pending,
/// }
/// ```
/// 
/// 什么是Future？什么是Poll？
///   1. Future 是Rust标准库提供的一个标准，表示一个异步任务。
///   2. Future必须要实现 poll 方法，该方法返回一个 Poll<T> 枚举, 这个枚举表示了异步任务当前是否完成的状态。
///   3. Poll取值有两个: Ready(T) 表示任务已完成，并且获取到结果T; Peding 表示任务还未完成。
/// 
///   Rust只提供了异步编程的架子与约束，并未提供完整的运行时，所以需要使用一些第三方提供的异步运行时。但是它们都遵循于标准库的约束。
/// 最主流的就是Rust社区提供的 Tokio。
fn main(){

}