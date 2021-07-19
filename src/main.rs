#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_session;

use rustc_driver::Callbacks;
use rustc_interface::interface::Config;
use rustc_session::config::Options as RustcOptions;

use std::process::Command;

use std::{
    env,
    fmt::{self, Debug, Formatter},
};

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    let out = Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = String::from_utf8(out.stdout).unwrap();

    args.push(format!("--sysroot={}", sysroot.trim()));

    rustc_driver::RunCompiler::new(args.as_slice(), &mut Compiler)
        .run()
        .expect("Rustc failed to compile crate");
}

struct Compiler;

impl Callbacks for Compiler {
    fn config(&mut self, config: &mut Config) {
        let options = Options(&config.opts);
        eprintln!("{:#?}", options);
    }
}

struct Options<'a>(&'a RustcOptions);

macro_rules! impl_debug {
    ($( $name: ident )* ) => {
        impl Debug for Options<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.debug_struct("Options")
                $(
                    .field(stringify!($name), &self.0.$name)
                )*
                    .finish()
            }
        }
    }
}

impl_debug! {
    crate_types
    optimize
    debug_assertions
    debuginfo
    lint_opts
    lint_cap
    force_warns
    describe_lints
    output_types
    search_paths
    libs
    maybe_sysroot
    target_triple
    test
    error_format
    incremental
    // debugging_opts
    prints
    // borrowck_mode
    // externs
    // extern_dep_specs
    crate_name
    alt_std_name
    unstable_features
    actually_rustdoc
    trimmed_def_paths
    cli_forced_codegen_units
    cli_forced_thinlto_off
    remap_path_prefix
    // real_rust_source_path_dir
    edition
    json_artifact_notifications
    json_unused_externs
    pretty
}
