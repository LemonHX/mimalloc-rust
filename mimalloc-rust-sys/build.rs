use std::env;

fn main() {
    let mut build = cc::Build::new();

    build
        .include("./mimalloc/include")
        .include("./mimalloc/src")
        .file("./mimalloc/src/static.c")
        .define("MI_BUILD_SHARED", "0")
        .cpp(false)
        .warnings(false)
        .flag_if_supported("-w");

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("target_os not defined!");
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").expect("target_family not defined!");

    if env::var_os("CARGO_FEATURE_OVERRIDE").is_some() {
        // Overriding malloc is only available on windows in shared mode, but we
        // only ever build a static lib.
        if target_family != "windows" {
            build.define("MI_MALLOC_OVERRIDE", "0");
        }
    }

    #[cfg(feature = "secure")]
    {
        build.define("MI_SECURE", "4");
    }

    #[cfg(feature = "asm")]
    {
        build.flag_if_supported("-save-temps");
    }

    #[cfg(feature = "skip-collect-on-exit")]
    {
        build.define("MI_SKIP_COLLECT_ON_EXIT", "1");
    }

    if target_family == "unix" && target_os != "haiku" {
        #[cfg(feature = "local-dynamic-tls")]
        {
            build.flag_if_supported("-ftls-model=local-dynamic");
        }
        #[cfg(not(feature = "local-dynamic-tls"))]
        {
            build.flag_if_supported("-ftls-model=initial-exec");
        }
    }

    // Remove heavy debug assertions etc
    let profile = std::env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => build.define("MI_DEBUG_FUL", "3"),
        "release" => build.define("MI_DEBUG_FUL", "0").define("MI_DEBUG", "0"),
        _ => build.define("MI_DEBUG_FUL", "3"),
    };

    if build.get_compiler().is_like_msvc() {
        build.cpp(true);
    }

    build.compile("mimalloc");
}
