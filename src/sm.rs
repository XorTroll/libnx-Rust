use os;
use sys;

handle!(0 in sys::smInitialize(), sys::smExit(), {
    pub fn get_service(&self, name: &str) -> os::Result<sys::Service> {
        unsafe {
            let mut srv: sys::Service = std::mem::zeroed();
            let rc = sys::smGetService(&mut srv, name.as_ptr());
            result_final!(rc, srv)
        }
    }
});
