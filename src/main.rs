use nix::sys::wait::waitpid;
use nix::unistd::ForkResult;
use nix::unistd::{execvp, fork};
use std::ffi::CStr;

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            execvp(
                CStr::from_bytes_with_nul(b"ls\0").expect("command failed"),
                &[CStr::from_bytes_with_nul(b"-l\0").expect("args failed")],
            )
            .expect("exec failed");
        }
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).unwrap();
        }
        Err(err) => panic!(
            "Could not create new child process and it caused error {}",
            err
        ),
    }
}
