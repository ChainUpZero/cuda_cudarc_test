pub use future_cuda;
pub use exec;

pub mod future_cuda;
pub mod exec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (executor, spawner) = new_executor_and_spawner();
    
        // 生成一个任务
        spawner.spawn(async {
            println!("begin!");
            // 创建定时器Future，并等待它完成
            Dev::new().await;
            println!("done!");
        });
    
        // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
        drop(spawner);
    
        // 运行执行器直到任务队列为空
        // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
        executor.run();
    }
}