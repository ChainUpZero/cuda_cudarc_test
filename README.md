# cuda_cudarc_test

## ideas
介绍cudarc：cudarc这个库是extern C 函数（在Rust中写C函数），编译为ptx，然后获取device进行运行  
介绍rust-gpu：利用CUDA builder可以把Rust函数编译为ptx（自己去写核函数）  
想法：利用rust-gpu，build.rs尝试build为ptx，cudarc直接引入ptx然后调用？  
  
把异步和多线程的框架搭起来了，框架中GPU运行的例子是，把数据从本地传到GPU，再从GPU传回来  
问题是：现在没GPU，这库装不了， 一直报错 no `find_libnvvm_bin_dir` in the root，感觉需要个GPU玩一下