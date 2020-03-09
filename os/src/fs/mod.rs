mod device;
pub mod stdio;
pub mod file;

use alloc::{sync::Arc, vec::Vec};
use lazy_static::*;
use rcore_fs::vfs::*;
use rcore_fs_sfs::SimpleFileSystem;

lazy_static! {
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        let device = {
            Arc::new(unsafe { device::MemBuf::new(0, 0) })
        };
        let sfs = SimpleFileSystem::open(device).expect("failed to open SFS");
        sfs.root_inode()
    };
}

pub trait INodeExt {
    fn read_as_vec(&self) -> Result<Vec<u8>>;
}

impl INodeExt for dyn INode {
    fn read_as_vec(&self) -> Result<Vec<u8>> {
        let size = self.metadata()?.size;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        self.read_at(0, buf.as_mut_slice())?;
        Ok(buf)
    }
}

pub fn init() {
    println!("available programs in rust/ are:");
    let mut id = 0;
    let mut rust_dir = ROOT_INODE.lookup("rust").unwrap();
    while let Ok(name) = rust_dir.get_entry(id) {
        id += 1;
        println!("  {}", name);
    }
    println!("++++ setup fs!        ++++")
}
