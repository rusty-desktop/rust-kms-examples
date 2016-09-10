extern crate nix;
extern crate drm as libdrm;

use nix::fcntl;
use nix::sys::stat;
use std::path::Path;

use libdrm::{drm, drm_mode};

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
                println!("  -> count ctrcs: {}", resources.get_count_crtcs());
                println!("  -> count connectors: {}", resources.get_count_connectors());
                println!("  -> count encoders: {}", resources.get_count_encoders());
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
