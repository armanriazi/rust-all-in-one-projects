#![allow(dead_code, unused_variables)]


/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-all-in-one```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --workspace```
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// ```rust,compile_fail,ignore
/// #[allow(dead_code)]
/// fn m(){
///     core::panicking::panic!("in crash");
///     unimplemented!()
/// }
/// ```

#[cfg(panic = "unwind")]
#[cfg(target_family = "unix")]
fn get_platform() -> String {
    "UNIX".into()
}

#[cfg(target_family = "windows")]
fn get_platform() -> String {
    "Windows".into()
}

fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

/// We have two get_platform and selected by conditional_features
fn main() {
    greet_world();
    println!("This code is running on a {} family OS", get_platform());
    if cfg!(target_feature = "avx2") {
        println!("avx2 is enabled");
    } else {
        println!("avx2 is not enabled");
    }
    if cfg!(not(any(target_arch = "x86", target_arch = "x86_64"))) {
        println!("This code is running on a non-Intel CPU");
    }

    finish();
}

fn finish() -> impl std::process::Termination {
    let machine_kind = if cfg!(unix) {
        println!("I was running on a {} machine!", "unix");
        std::process::ExitCode::SUCCESS
    } else if cfg!(windows) {
        println!("I was running on a {} machine!", "windows");
        std::process::ExitCode::SUCCESS
    } else {
        println!("I was running on a {} machine!", "unknown");
        std::process::ExitCode::FAILURE
    };
}
