//! Build script for zcash_script.

use std::{env, fmt, fs, io::Read, path::PathBuf};

use syn::__private::ToTokens;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
enum Error {
    GenerateBindings,
    WriteBindings(std::io::Error),
    Env(std::env::VarError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GenerateBindings => write!(f, "unable to generate bindings: try running 'git submodule init' and 'git submodule update'"),
            Error::WriteBindings(source) => write!(f, "unable to write bindings: {}", source),
            Error::Env(source) => source.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

fn bindgen_headers() -> Result<()> {
    println!("cargo:rerun-if-changed=depend/zcash/src/script/zcash_script.h");

    let bindings = bindgen::Builder::default()
        .header("depend/zcash/src/script/zcash_script.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        .map_err(|_| Error::GenerateBindings)?;

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = env::var("OUT_DIR").map_err(Error::Env)?;
    let out_path = PathBuf::from(out_path);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .map_err(Error::WriteBindings)?;

    Ok(())
}

/// Use cxx_gen to generate headers and source files for FFI bindings,
/// just like zcash does (see depend/zcash/src/Makefile.am).
/// (Note that zcash uses the cxxbridge-cmd binary, while we use the
/// cxx_gen library, but the result is the same.)
///
/// We could use [`cxx_build`](https://cxx.rs/tutorial.html#compiling-the-c-code-with-cargo)
/// to do this automatically. But zcash uses the
/// [manual approach](https://cxx.rs/tutorial.html#c-generated-code) which
/// creates the headers with non-standard names and paths (e.g. "blake2b.h",
/// instead of "blake2b.rs.h" which what cxx_build would create). This would
/// requires us to rename/link files which is awkward.
///
/// Note that we must generate the files in the target dir (OUT_DIR) and not in
/// any source folder, because `cargo package` does not allow that.
/// (This is in contrast to zcash which generates in `depend/zcash/src/rust/gen/`)
fn gen_cxxbridge() -> Result<()> {
    let out_path = env::var("OUT_DIR").map_err(Error::Env)?;
    let out_path = PathBuf::from(out_path).join("gen");
    let src_out_path = PathBuf::from(&out_path).join("src");
    let header_out_path = PathBuf::from(&out_path).join("include").join("rust");

    // These must match `CXXBRIDGE_RS` in depend/zcash/src/Makefile.am
    let filenames = [
        "blake2b",
        "ed25519",
        "equihash",
        "streams",
        "bridge",
        "sapling/zip32",
    ];

    // The output folder must exist
    fs::create_dir_all(&src_out_path).unwrap();
    fs::create_dir_all(&header_out_path).unwrap();

    // Generate the generic header file
    fs::write(header_out_path.join("cxx.h"), cxx_gen::HEADER).unwrap();

    // Generate the source and header for each bridge file
    for filename in filenames {
        println!(
            "cargo:rerun-if-changed=depend/zcash/src/rust/src/{}.rs",
            filename
        );

        let mut file =
            fs::File::open(format!("depend/zcash/src/rust/src/{}.rs", filename).as_str()).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let ast = syn::parse_file(&content).unwrap();
        let token_stream = ast.to_token_stream();
        let mut opt = cxx_gen::Opt::default();
        opt.include.push(cxx_gen::Include {
            path: "rust/cxx.h".to_string(),
            kind: cxx_gen::IncludeKind::Quoted,
        });
        let output = cxx_gen::generate_header_and_cc(token_stream, &opt).unwrap_or_else(|err| {
            panic!(
                "invalid bridge file {filename}: {err}. Try updating `filenames` to match zcashd"
            )
        });

        let header_path = header_out_path.join(format!("{}.h", filename));
        // Create output dir if does not exist (since `filename` can have a subdir)
        fs::create_dir_all(header_path.parent().unwrap()).unwrap();
        fs::write(header_path, output.header).unwrap();

        let src_path = src_out_path.join(format!("{}.cpp", filename));
        // Create output dir if does not exist (since `filename` can have a subdir)
        fs::create_dir_all(src_path.parent().unwrap()).unwrap();
        fs::write(src_path, output.implementation).unwrap();
    }
    Ok(())
}

fn main() -> Result<()> {
    bindgen_headers()?;
    gen_cxxbridge()?;

    let rust_path = env::var("OUT_DIR").map_err(Error::Env)?;
    let rust_path = PathBuf::from(rust_path).join("rust");

    // We want to compile `depend/zcash/src/rust/src/sapling.rs`, which we used
    // to do in `src/sapling.rs` by just including it. However, now that it has
    // submodules, that approach doesn't work because for some reason Rust
    // searches for the submodules in `depend/zcash/src/rust/src/` instead of
    // `depend/zcash/src/rust/src/sapling/` where they are located. This can
    // be solved if `depend/zcash/src/rust/src/sapling.rs` is renamed to
    // `depend/zcash/src/rust/src/sapling/mod.rs`. But we can't do that directly
    // because we can't change the source tree inside `build.rs`. Therefore we
    // copy the required files to OUT_DIR, with a renamed sapling.rs, and include
    // the copied file instead (see src/sapling.rs).
    // See also https://stackoverflow.com/questions/77310390/how-to-include-a-source-file-that-has-modules
    fs::create_dir_all(rust_path.join("sapling")).unwrap();
    for filename in &["sapling.rs", "sapling/spec.rs", "sapling/zip32.rs"] {
        println!(
            "cargo:rerun-if-changed=depend/zcash/src/rust/src/{}.rs",
            filename
        );
    }
    fs::copy(
        "depend/zcash/src/rust/src/sapling.rs",
        rust_path.join("sapling/mod.rs"),
    )
    .unwrap();
    fs::copy(
        "depend/zcash/src/rust/src/sapling/spec.rs",
        rust_path.join("sapling/spec.rs"),
    )
    .unwrap();
    fs::copy(
        "depend/zcash/src/rust/src/sapling/zip32.rs",
        rust_path.join("sapling/zip32.rs"),
    )
    .unwrap();

    let gen_path = env::var("OUT_DIR").map_err(Error::Env)?;
    let gen_path = PathBuf::from(gen_path).join("gen");

    let target = env::var("TARGET").expect("TARGET was not set");
    let mut base_config = cc::Build::new();

    language_std(&mut base_config, "c++17");

    base_config
        .include("depend/zcash/src/")
        .include("depend/zcash/src/rust/include/")
        .include("depend/zcash/src/secp256k1/include/")
        .include("depend/expected/include/")
        .include(&gen_path.join("include"))
        .flag_if_supported("-Wno-implicit-fallthrough")
        .flag_if_supported("-Wno-catch-value")
        .flag_if_supported("-Wno-reorder")
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable")
        .flag_if_supported("-Wno-ignored-qualifiers")
        .flag_if_supported("-Wno-sign-compare")
        // when compiling using Microsoft Visual C++, ignore warnings about unused arguments
        .flag_if_supported("/wd4100")
        .define("HAVE_DECL_STRNLEN", "1")
        .define("__STDC_FORMAT_MACROS", None);

    // **Secp256k1**
    if !cfg!(feature = "external-secp") {
        build_secp256k1();
    }

    if target.contains("windows") {
        base_config.define("WIN32", "1");
    }

    base_config
        .file("depend/zcash/src/script/zcash_script.cpp")
        .file("depend/zcash/src/util/strencodings.cpp")
        .file("depend/zcash/src/amount.cpp")
        .file("depend/zcash/src/uint256.cpp")
        .file("depend/zcash/src/pubkey.cpp")
        .file("depend/zcash/src/hash.cpp")
        .file("depend/zcash/src/streams_rust.cpp")
        .file("depend/zcash/src/zip317.cpp")
        .file("depend/zcash/src/primitives/transaction.cpp")
        .file("depend/zcash/src/crypto/ripemd160.cpp")
        .file("depend/zcash/src/crypto/sha1.cpp")
        .file("depend/zcash/src/crypto/sha256.cpp")
        .file("depend/zcash/src/crypto/sha512.cpp")
        .file("depend/zcash/src/crypto/hmac_sha512.cpp")
        .file("depend/zcash/src/script/interpreter.cpp")
        .file("depend/zcash/src/script/script.cpp")
        .file("depend/zcash/src/script/script_error.cpp")
        .file("depend/zcash/src/support/cleanse.cpp")
        .file("depend/zcash/src/zcash/cache.cpp")
        // A subset of the files generated by gen_cxxbridge
        // which are required by zcash_script.
        .file(gen_path.join("src/blake2b.cpp"))
        .file(gen_path.join("src/bridge.cpp"))
        .file(gen_path.join("src/streams.cpp"))
        .compile("libzcash_script.a");

    Ok(())
}

/// Build the `secp256k1` library.
fn build_secp256k1() {
    let mut build = cc::Build::new();

    // Compile C99 code
    language_std(&mut build, "c99");

    // Define configuration constants
    build
        // This matches the #define in depend/zcash/src/secp256k1/src/secp256k1.c
        .define("SECP256K1_BUILD", "")
        .define("USE_NUM_NONE", "1")
        .define("USE_FIELD_INV_BUILTIN", "1")
        .define("USE_SCALAR_INV_BUILTIN", "1")
        .define("ECMULT_WINDOW_SIZE", "15")
        .define("ECMULT_GEN_PREC_BITS", "4")
        // Use the endomorphism optimization now that the patents have expired.
        .define("USE_ENDOMORPHISM", "1")
        // Technically libconsensus doesn't require the recovery feature, but `pubkey.cpp` does.
        .define("ENABLE_MODULE_RECOVERY", "1")
        // The source files look for headers inside an `include` sub-directory
        .include("depend/zcash/src/secp256k1")
        // Some ecmult stuff is defined but not used upstream
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-unused-parameter");

    if is_big_endian() {
        build.define("WORDS_BIGENDIAN", "1");
    }

    if is_64bit_compilation() {
        build
            .define("USE_FIELD_5X52", "1")
            .define("USE_SCALAR_4X64", "1")
            .define("HAVE___INT128", "1");
    } else {
        build
            .define("USE_FIELD_10X26", "1")
            .define("USE_SCALAR_8X32", "1");
    }

    build
        .file("depend/zcash/src/secp256k1/src/secp256k1.c")
        .file("depend/zcash/src/secp256k1/src/precomputed_ecmult.c")
        .file("depend/zcash/src/secp256k1/src/precomputed_ecmult_gen.c")
        .compile("libsecp256k1.a");
}

/// Checker whether the target architecture is big endian.
fn is_big_endian() -> bool {
    let endianess = env::var("CARGO_CFG_TARGET_ENDIAN").expect("No endian is set");

    endianess == "big"
}

/// Check whether we can use 64-bit compilation.
fn is_64bit_compilation() -> bool {
    let target_pointer_width =
        env::var("CARGO_CFG_TARGET_POINTER_WIDTH").expect("Target pointer width is not set");

    if target_pointer_width == "64" {
        let check = cc::Build::new()
            .file("depend/check_uint128_t.c")
            .cargo_metadata(false)
            .try_compile("check_uint128_t")
            .is_ok();

        if !check {
            println!(
                "cargo:warning=Compiling in 32-bit mode on a 64-bit architecture due to lack of \
                uint128_t support."
            );
        }

        check
    } else {
        false
    }
}

/// Configure the language standard used in the build.
///
/// Configures the appropriate flag based on the compiler that's used.
///
/// This will also enable or disable the `cpp` flag if the standard is for C++. The code determines
/// this based on whether `std` starts with `c++` or not.
fn language_std(build: &mut cc::Build, std: &str) {
    build.cpp(std.starts_with("c++"));

    let flag = if build.get_compiler().is_like_msvc() {
        "/std:"
    } else {
        "-std="
    };

    build.flag(&[flag, std].concat());
}
