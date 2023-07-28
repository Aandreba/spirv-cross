use bindgen::EnumVariation;
use std::{env, path::Path};

fn main() {
    let out_path = env::var_os("OUT_DIR").unwrap();

    let target_vendor = std::env::var("CARGO_CFG_TARGET_VENDOR");
    let is_apple = target_vendor.is_ok() && target_vendor.unwrap() == "apple";

    let target_os = std::env::var("CARGO_CFG_TARGET_OS");
    let is_ios = target_os.is_ok() && target_os.unwrap() == "ios";

    let mut build = cc::Build::new();
    build.cpp(true);

    let compiler = build.try_get_compiler();
    let is_clang = compiler.is_ok() && compiler.unwrap().is_like_clang();

    if is_apple && (is_clang || is_ios) {
        build.flag("-std=c++14").cpp_set_stdlib("c++");
    } else {
        build.flag_if_supported("-std=c++14");
    }

    build
        .file("SPIRV-Cross/spirv_cfg.cpp")
        .file("SPIRV-Cross/spirv_cross.cpp")
        .file("SPIRV-Cross/spirv_cross_c.cpp")
        .file("SPIRV-Cross/spirv_cross_parsed_ir.cpp")
        .file("SPIRV-Cross/spirv_parser.cpp")
        .file("SPIRV-Cross/spirv_cross_util.cpp");

    // Ideally the GLSL compiler would be omitted here, but the HLSL and MSL compiler
    // currently inherit from it. So it's necessary to unconditionally include it here.
    #[cfg(feature = "glsl")]
    build
        .file("SPIRV-Cross/spirv_glsl.cpp")
        .define("SPIRV_CROSS_C_API_GLSL", None);

    #[cfg(feature = "hlsl")]
    build
        .file("SPIRV-Cross/spirv_hlsl.cpp")
        .define("SPIRV_CROSS_C_API_HLSL", None);

    #[cfg(feature = "msl")]
    build
        .file("SPIRV-Cross/spirv_msl.cpp")
        .define("SPIRV_CROSS_C_API_MSL", None);

    build.compile("spirv-cross-rust-wrapper");
    generate_bindings(out_path.as_ref());
}

fn generate_bindings(out_path: &Path) {
    // For native targets, include all types and functions
    bindgen::Builder::default()
        .header("SPIRV-Cross/spirv_cross_c.h")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
