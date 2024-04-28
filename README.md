# cuda_cudarc_test

## ideas
介绍cudarc：cudarc这个库是extern C 函数（在Rust中写C函数），编译为ptx，然后获取device进行运行  
介绍rust-gpu：利用CUDA builder可以把Rust函数编译为ptx（自己去写核函数）  
想法：利用rust-gpu，build.rs尝试build为ptx，cudarc直接引入ptx然后调用？  
  
把异步和多线程的框架搭起来了，框架中GPU运行的例子是，把数据从本地传到GPU，再从GPU传回来  
问题是：现在没GPU，这库装不了， 一直报错 no `find_libnvvm_bin_dir` in the root，感觉需要个GPU玩一下  
GPT:  
这个错误通常出现在使用 CUDA 相关的 crate 进行编译时，可能是由于缺少 NVVM（NVIDIA 的虚拟机管理器）的二进制文件导致的。NVVM 是用于编译 CUDA 源代码的一个组件，它包含了一些用于将 CUDA 源代码编译成 PTX（Parallel Thread Execution）的二进制文件。  
解决这个问题的方法通常是确保你的 CUDA Toolkit 已经正确安装，并且相关的路径已经添加到系统的环境变量中。如果 CUDA Toolkit 已经正确安装，但仍然出现这个问题，可能是由于相关的路径没有正确设置。  
你可以尝试手动设置 NVVM 相关的环境变量，或者查看你所使用的 crate 的文档，看是否提供了设置 NVVM 相关路径的方法。通常情况下，使用 CUDA 相关的 crate 时，它们会自动检测并使用正确的 NVVM 路径。