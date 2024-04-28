use std::{
    pin::Pin, sync::{Arc, Mutex}, task::{Context, Poll, Waker}, thread
};

use cudarc::{cublas::result, driver::{CudaDevice, CudaSlice, DeviceRepr, DriverError}};

pub struct Dev<T: Clone + Default + DeviceRepr + Unpin> {
    dev: Arc<CudaDevice>,
    data: Vec<T>,
    shared_state: Arc<Mutex<SharedState>>,
}
//let a: CudaSlice<f64> = dev.alloc_zeros::<f64>(10)?;这个可以传数据

struct SharedState { //记录共享状态啊的信息
    /// 定时(睡眠)是否结束
    completed: bool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}

impl<T> Future for Dev<T> 
where 
    T:Clone + Default + DeviceRepr + Unpin,
{
    type Output = Vec<T>; 

    fn poll(self:Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output> { 
        //自己引入的传统的future是Pin的，而CUDA要求是Unpin的类型,这里可能会有点问题？？？
        //如 self: Pin<&mut Self>,
        // let buffer = self.dev.sync_reclaim(self.a.clone());
        // if let Ok(data) = buffer {
        //     Poll::Ready(data)
        // }
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl Dev<T: Clone + Default + DeviceRepr + Unpin> {
    //创建任务
    pub fn new() -> Self {
        //创建设备
        let dev = CudaDevice::new(0)?;
        //创建Cuda的数据区域，并分配到GPU上
        let mut data = dev.alloc_zeros::<f64>(10)?;
        //创建metadata,用于记录共享状态信息
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        //创建线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            //分配到GPU
            dev.htod_sync_copy_into(&[3.0; 10], &mut data)?;
            //
            //此处省略如何调用函数计算
            //
            //取回结果
            let result = dev.dtoh_sync_copy(&data)?;
            
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake() //任务做完了就唤醒
            }
        });

        Self {
            dev,
            data:result,
            shared_state,
        }
    }
}