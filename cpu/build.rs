use cuda_builder::CudaBuilder;

fn main() {
    CudaBuilder::new("../cuda_cudarc_gpu")
        .copy_to("resources/cuda_cudarc.ptx")
        .build()
        .unwrap();
}