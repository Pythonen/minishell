use nix::libc::perror;
use nix::sys::wait::waitpid;
use nix::unistd::ForkResult;
use nix::unistd::{execvp, fork};
use std::ffi::CStr;
use std::io::{self};

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
                let _args = &input[1..];
                // let arr: Vec<&CStr> = args
                //     .iter()
                //     .map(|i| {
                //         return CStr::from_bytes_with_nul(
                //             (String::from(*i) + "\0").as_str().as_bytes(),
                //         )
                //         .expect("Failed :(");
                //     })
                //     .collect();
                let command_cstr =
                    CStr::from_bytes_with_nul(command.as_str().as_bytes()).expect("command failed");
                let args_cstr = &[CStr::from_bytes_with_nul(b"-la\0").expect("args failed")];
                match execvp(command_cstr, args_cstr) {
                    Ok(_) => (),
                    _ => {
                        unsafe { perror(command.as_ptr() as *const i8) };
                        std::process::exit(-1);
                    }
                }
            }
            Ok(ForkResult::Parent { child }) => {
                // TODO: This could be handled some better way rather than just unwrapping.
                waitpid(child, None).unwrap();
            }
            Err(err) => panic!(
                "Could not create new child process and it caused error {}",
                err
            ),
        }
    }
}
