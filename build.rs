use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn compile_lwip() {
    cc::Build::new()
        .file("src/lwip/src/core/init.c")
        .file("src/lwip/src/core/def.c")
        // .file("src/lwip/src/core/dns.c")
        .file("src/lwip/src/core/inet_chksum.c")
        .file("src/lwip/src/core/ip.c")
        .file("src/lwip/src/core/mem.c")
        .file("src/lwip/src/core/memp.c")
        .file("src/lwip/src/core/netif.c")
        .file("src/lwip/src/core/pbuf.c")
        .file("src/lwip/src/core/raw.c")
        // .file("src/lwip/src/core/stats.c")
        // .file("src/lwip/src/core/sys.c")
        .file("src/lwip/src/core/tcp.c")
        .file("src/lwip/src/core/tcp_in.c")
        .file("src/lwip/src/core/tcp_out.c")
        .file("src/lwip/src/core/timeouts.c")
        .file("src/lwip/src/core/udp.c")
        // .file("src/lwip/src/core/ipv4/autoip.c")
        // .file("src/lwip/src/core/ipv4/dhcp.c")
        // .file("src/lwip/src/core/ipv4/etharp.c")
        .file("src/lwip/src/core/ipv4/icmp.c")
        // .file("src/lwip/src/core/ipv4/igmp.c")
        .file("src/lwip/src/core/ipv4/ip4_frag.c")
        .file("src/lwip/src/core/ipv4/ip4.c")
        .file("src/lwip/src/core/ipv4/ip4_addr.c")
        // .file("src/lwip/src/core/ipv6/dhcp6.c")
        // .file("src/lwip/src/core/ipv6/ethip6.c")
        .file("src/lwip/src/core/ipv6/icmp6.c")
        // .file("src/lwip/src/core/ipv6/inet6.c")
        .file("src/lwip/src/core/ipv6/ip6.c")
        .file("src/lwip/src/core/ipv6/ip6_addr.c")
        .file("src/lwip/src/core/ipv6/ip6_frag.c")
        // .file("src/lwip/src/core/ipv6/mld6.c")
        .file("src/lwip/src/core/ipv6/nd6.c")
        // .file("src/lwip/src/custom/sys_arch.c")
        .include("src/lwip/src/include")
        .include("src/lwip/contrib/ports/unix/lib")
        .include("src/lwip/contrib/ports/unix/port/include")
        .warnings(false)
        .flag_if_supported("-Wno-everything")
        .compile("liblwip.a");
}

fn generate_lwip_bindings() {
    println!("cargo:rustc-link-lib=lwip");
    // println!("cargo:rerun-if-changed=src/proxy/tun/netstack/wrapper.h");
    println!("cargo:include=src/lwip/src/include");

    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let bindings = bindgen::Builder::default()
        // .header("src/proxy/tun/netstack/wrapper.h")
        .header("src/lwip/src/include/lwip/init.h")
        .header("src/lwip/src/include/lwip/timeouts.h")
        .header("src/lwip/src/include/lwip/netif.h")
        .header("src/lwip/src/include/lwip/tcp.h")
        .header("src/lwip/src/include/lwip/udp.h")
        .header("src/lwip/src/include/lwip/ip_addr.h")
        // .header("src/lwip/contrib/ports/unix/lib/lwipopts.h")
        // .header("src/lwip/contrib/ports/unix/port/include/arch/sys_arch.h")
        .clang_arg("-I./src/lwip/src/include")
        .clang_arg("-I./src/lwip/contrib/ports/unix/lib")
        .clang_arg("-I./src/lwip/contrib/ports/unix/port/include")
        // .clang_arg("-I./src/lwip/src/custom")
        .clang_arg("-Wno-everything")
        .layout_tests(false)
        .clang_arg(if arch == "aarch64" && os == "ios" {
            // https://github.com/rust-lang/rust-bindgen/issues/1211
            "--target=arm64-apple-ios"
        } else {
            ""
        })
        .clang_arg(if arch == "aarch64" && os == "ios" {
            // sdk path find by `xcrun --sdk iphoneos --show-sdk-path`
            let output = Command::new("xcrun")
                .arg("--sdk")
                .arg("iphoneos")
                .arg("--show-sdk-path")
                .output()
                .expect("failed to execute xcrun");
            let inc_path =
                Path::new(String::from_utf8_lossy(&output.stdout).trim()).join("usr/include");
            format!("-I{}", inc_path.to_str().expect("invalid include path"))
        } else {
            "".to_string()
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let mut out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    out_path = out_path.join("src/tun/netstack");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    compile_lwip();
    generate_lwip_bindings();
}
