use cmake::Config;
use std::env;

fn main(){

    let mut dst = Config::new("c_src/mimalloc");

    let mut out_name = "mimalloc";
    if env::var_os("CARGO_FEATURE_DEBUG").is_some()
        || (env::var_os("CARGO_FEATURE_DEBUG_IN_DEBUG").is_some() && cfg!(debug_assertions))
    {
        dst.define("CMAKE_BUILD_TYPE", "Release");
    } else {
        // Remove heavy debug assertions etc
        dst.define("CMAKE_BUILD_TYPE", "Debug");
        out_name = "mimalloc-debug"
    }

    let dst = dst.build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    println!("cargo:rustc-link-lib={}", out_name);
}
