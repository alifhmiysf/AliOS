use std::env;
use std::path::PathBuf;

fn main() {
    let kernel = PathBuf::from(
        env::var_os("CARGO_BIN_FILE_KERNEL_alios-kernel")
            .expect("kernel artifact not found")
    );

    let out_dir = PathBuf::from(
        env::var_os("OUT_DIR")
            .expect("OUT_DIR not found")
    );

    let bios_path = out_dir.join("alios-bios.img");

    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .expect("failed to create BIOS disk image");

    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}