use nix::libc::perror;
use nix::sys::wait::waitpid;
use nix::unistd::ForkResult;
use nix::unistd::{execvp, fork};
use std::ffi::{CStr, CString};
use std::io;

fn main() {
    loop {
        // User input should be a command of several arguments,
        // like "ls -la"
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        // The input string is separated to individual words vector
        // like "ls" "-la"
        // to simplify further processing
        let input: Vec<&str> = input.split_whitespace().collect();

        // Creating a new child process according to => https://man7.org/linux/man-pages/man2/fork.2.html
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                // Hacky.. some better way to append to a str would be nice
                let command = String::from(input[0]) + "\0";
                // collecting args for the command above
                let args = &input[1..];
                let arr: Vec<_> = args
                    .into_iter()
                    .map(|i| {
                        CString::new((String::from(*i)).as_str().as_bytes()).expect("Failed :(")
                    })
                    .collect();
                let command_cstr =
                    CStr::from_bytes_with_nul(command.as_str().as_bytes()).expect("command failed");
                match execvp(command_cstr, &arr) {
                    Ok(_) => (),
                    _ => {
                        unsafe { perror(command.as_ptr() as *const i8) };
                        std::process::exit(-1);
                    }
                }
            }
            Ok(ForkResult::Parent { child }) => match waitpid(child, None) {
                Ok(_) => (),
                Err(err) => panic!("Could not wait for a process to change status {}", err),
            },
            Err(err) => panic!(
                "Could not create new child process and it caused error {}",
                err
            ),
        }
    }
}
