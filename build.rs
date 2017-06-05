extern crate gcc;
extern crate glob;
extern crate tempdir;

use glob::glob;

fn compile_static(libshort: &str, src_dir: &str, blacklist: Option<&[&str]>) {
    let td = tempdir::TempDir::new_in("/tmp", "").unwrap();
    let libname = format!("lib{}.a", libshort);
    let out_dir = std::env::var_os("OUT_DIR")
        .map(std::path::PathBuf::from)
        .unwrap();
    let lib_dest = out_dir.join(&libname);
    let lib_src = td.path().join(&libname);

    let mut config = gcc::Config::new();
    let src_glob = format!("LAPACKE/{}/*.c", src_dir);
    if let Some(blacklist) = blacklist {
        let blacklist = blacklist
            .iter()
            .map(|f| std::path::PathBuf::from(f))
            .collect::<Vec<std::path::PathBuf>>();
        for file in glob(&src_glob).unwrap() {
            let fname = file.unwrap();
            if !blacklist.contains(&fname) {
                config.file(fname);
            }
        }
    } else {
        for file in glob(&src_glob).unwrap() {
            config.file(file.unwrap());
        }
    }
    config.include("LAPACKE/include");
    config.out_dir(td); // keep filenames short
    config.compile(&libname);

    std::fs::copy(&lib_src, &lib_dest).unwrap();
    println!("cargo:rustc-link-lib=static={}", libshort);
}

fn main() {
    compile_static("lapacke-utils", "utils", None);
    // These files redefine a function.
    let blacklist = ["LAPACKE/src/lapacke_cgelq.c",
                     "LAPACKE/src/lapacke_zgelq.c",
                     "LAPACKE/src/lapacke_sgelq.c",
                     "LAPACKE/src/lapacke_dgelq.c"];
    compile_static("lapacke-src", "src", Some(&blacklist));
}
