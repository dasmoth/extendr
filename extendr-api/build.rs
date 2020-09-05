use std::env;

fn r_version(v: u32, p: u32, s: u32) -> u32 {
    v*65536 + p*256 + s
}

fn main() {
    println!("cargo:rustc-env=R_HOME={}", env::var("DEP_R_R_HOME").unwrap());
    let libr_version = env::var("DEP_R_R_VERSION").unwrap().parse::<u32>().unwrap();
    println!("cargo:rustc-env=R_VERSION={}", libr_version);
    if libr_version >= r_version(3,0,0) {
        println!("cargo:rustc-cfg=r_version_3_0");
    }
    if libr_version >= r_version(3,1,0) {
         println!("cargo:rustc-cfg=r_version_3_1");
    }
    if libr_version >= r_version(3,2,0) {
         println!("cargo:rustc-cfg=r_version_3_2");
    }
    if libr_version >= r_version(3,3,0) {
         println!("cargo:rustc-cfg=r_version_3_3");
    }
    if libr_version >= r_version(3,4,0) {
         println!("cargo:rustc-cfg=r_version_3_4");
    }
    if libr_version >= r_version(3,5,0) {
         println!("cargo:rustc-cfg=r_version_3_5");
    }
    if libr_version >= r_version(3,6,0) {
         println!("cargo:rustc-cfg=r_version_3_6");
    }

    if libr_version >= r_version(4,0,0) {
         println!("cargo:rustc-cfg=r_version_4_0");
    }
    if libr_version >= r_version(4,1,0) {
         println!("cargo:rustc-cfg=r_version_4_1");
    }
}
