extern crate nix;
extern crate drm as libdrm;

use nix::fcntl;
use nix::sys::stat;
use std::path::Path;
use std::os::unix::io;

use libdrm::{drm, drm_mode};

fn print_crtcs(fd: &io::RawFd, resources: &drm_mode::Resources) {
    println!("  -> count ctrcs: {}", resources.get_count_crtcs());
    for id in resources.get_crtcs() {
        match drm_mode::get_crtc(fd, id) {
            Some(crtc) => println!("    - {:?}", crtc),
            None => println!("    - failed to get info"),
        }
    }
}

fn print_encoders(fd: &io::RawFd, resources: &drm_mode::Resources) {
    println!("  -> count encoders: {}", resources.get_count_encoders());
    for id in resources.get_encoders() {
        match drm_mode::get_encoder(fd, id) {
            Some(encoder) => println!("    - {:?}", encoder),
            None => println!("    - failed to get info"),
        }
    }
}

fn print_connectors(fd: &io::RawFd, resources: &drm_mode::Resources) {
    println!("  -> count connectors: {}", resources.get_count_connectors());
    for id in resources.get_connectors() {
        match drm_mode::get_connector(fd, id) {
            Some(connector) => println!("    - {:?}", connector),
            None => println!("    - failed to get info"),
        }
    }
}

fn main() {
    let mut i = 0;

    loop {
        let node = format!("/dev/dri/card{}", i);
        let path = Path::new(&node);
        let fd = match fcntl::open(path, fcntl::O_RDONLY, stat::Mode::empty()) {
            Ok(fd) => fd,
            Err(_) => break,
        };

        println!("{}", node);
        println!(" => Capabilities:");

        let db_cap = drm::get_cap(&fd, drm::Capability::DumbBuffer);
        match db_cap {
            Ok(v) => println!("  -> has dumb buffers ({})", v),
            Err(e) => println!("  -> does not have dumb buffers ({})", e),
        }

        let prime_cap = drm::get_cap(&fd, drm::Capability::Prime);
        match prime_cap {
            Ok(v) => println!("  -> has PRIME ({})", v),
            Err(e) => println!("  -> does not have PRIME ({})", e),
        }

        println!(" => Resources:");
        match drm_mode::get_resources(&fd) {
            Some(resources) => {
                println!("  -> count fbs: {}", resources.get_count_fbs());
                print_crtcs(&fd, &resources);
                print_encoders(&fd, &resources);
                print_connectors(&fd, &resources);
            }
            None => println!("  -> No resources"),
        }

        println!("");
        i += 1;
    }

    if i == 0 {
        println!("No graphic card found...");
    }
}
