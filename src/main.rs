use nix::libc::perror;
use nix::sys::wait::waitpid;
use nix::unistd::ForkResult;
use nix::unistd::{execvp, fork};
use std::ffi::CStr;
use std::io::{self};

// TODO: why is execvp blocking other arms? If #[allow(unreachable_code)] macro is disabled,
// it shows warning, but works correctly (?)
#[allow(unreachable_code)]
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
                let args = &input[1..];
                // let arr: Vec<&CStr> = args
                //     .iter()
                //     .map(|i| {
                //         return CStr::from_bytes_with_nul(
                //             (String::from(*i) + "\0").as_str().as_bytes(),
                //         )
                //         .expect("Failed :(");
                //     })
                //     .collect();

                // Compiler is suggesting to match if this function returns error
                // But we are taking it into account with perror
                #[allow(unused_must_use)]
                {
                    execvp(
                        CStr::from_bytes_with_nul(command.as_str().as_bytes())
                            .expect("command failed"),
                        &[CStr::from_bytes_with_nul(b"-l\0").expect("args failed")],
                    );
                }
                unsafe { perror(command.as_ptr() as *const i8) };
                std::process::exit(-1);
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
}
