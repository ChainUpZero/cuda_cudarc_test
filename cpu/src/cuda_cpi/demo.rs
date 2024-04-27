use cudarc::driver::LaunchAsync;
use cudarc::driver::LaunchConfig;
use anyhow::Result;

pub fn demo() -> Result<()> {
    {
        let dev = cudarc::driver::CudaDevice::new(0)?;
        let ptx = cudarc::nvrtc::compile_ptx(PTX_SRC)?;
        dev.load_ptx(ptx, "hello_cuda_from_gpu", &["hello_cuda_from_gpu"])?;
        let hello_cuda_from_gpu = dev.get_func("hello_cuda_from_gpu", "hello_cuda_from_gpu").unwrap();
        let cfg = LaunchConfig {
            block_dim: (1,1,9),
            grid_dim: (1,1,2),
            shared_mem_bytes: 0,
        };
        unsafe { hello_cuda_from_gpu.launch(cfg, (0usize,)) }?;
    }
}