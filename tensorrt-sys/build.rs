use bindgen::builder;
use cmake::Config;

fn cuda_configuration() {
    let cudadir = match option_env!("CUDA_INSTALL_DIR") {
        Some(cuda_dir) => cuda_dir,
        None => "/usr/local/cuda",
    };

    println!("cargo:rustc-link-search={}/lib64", cudadir);
    println!("cargo:rustc-link-lib=dylib=cudart");
}

fn tensorrt_configuration() {
    match option_env!("TRT_INSTALL_DIR") {
        Some(trt_lib_dir) => {
            println!("cargo:rustc-link-search={}/lib", trt_lib_dir);
        }
        None => (),
    }
    println!("cargo:rustc-link-lib=dylib=nvinfer");
    println!("cargo:rustc-link-lib=dylib=nvonnxparser");
    println!("cargo:rustc-link-lib=dylib=nvparsers");
    println!("cargo:rustc-link-lib=dylib=nvinfer_plugin");
}

// Not sure if I love this solution but I think it's relatively robust enough for now on Unix systems.
// Still have to thoroughly test what happens with a TRT library installed that's not done by the
// dpkg. It's possible that we'll just have to fall back to only supporting one system library and assuming that
// the user has the correct library installed and is viewable via ldconfig.
//
// Hopefully something like this will work for Windows installs as well, not having a default library
// install location will make that significantly harder.
//
fn main() -> Result<(), ()> {
    let mut cfg = Config::new("trt-sys");

    #[cfg(feature = "trt-7")]
    {
        println!("Setting Config to TRT7");
        cfg.define("TRT7", "");
        let bindings = builder()
        .clang_args(&[
            "-x", "c++",
            "-I/usr/include/c++/11", 
            "-I/usr/include/c++/v1", 
            "-I/usr/include/aarch64-linux-gnu/c++/11", 
            "-I/usr/include/aarch64-unknown-linux-gnu/c++/v1/",
            "-I/usr/local/cuda-12.2/targets/aarch64-linux/include", // CUDA include path
            "-I/path/to/tensorrt/include", // Add this line for TensorRT headers
        ])
        .header("trt-sys/tensorrt_api.h")
        .size_t_is_usize(true)
        .generate()?;

        bindings.write_to_file("src/bindings.rs").unwrap();
    }

    let dst = cfg.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=trt-sys");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    tensorrt_configuration();
    cuda_configuration();

    Ok(())
}
