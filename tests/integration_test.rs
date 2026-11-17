//! Integration tests for SigmaOS-systemd-firstboot.

use sigmaos_systemd_firstboot_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
