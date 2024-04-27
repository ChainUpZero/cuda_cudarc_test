# cuda_cudarc_test

## ideas
介绍cudarc：cudarc这个库是extern C 函数（在Rust中写C函数），编译为ptx，然后获取device进行运行  
介绍rust-gpu：利用CUDA builder可以把Rust函数编译为ptx（自己去写核函数）  
想法：利用rust-gpu，build.rs尝试build为ptx，cudarc直接引入ptx然后调用？
