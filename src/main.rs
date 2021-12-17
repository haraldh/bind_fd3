use libc::{close, dup2};
use std::net::TcpListener;
use std::os::unix::io::AsRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let fd = listener.as_raw_fd();

    let mut args = std::env::args().into_iter().skip(1);
    let exe = args.next().unwrap();

    let _status = unsafe {
        Command::new(exe).args(args).pre_exec({
            move || {
                dup2(fd, 5);
                dup2(5, 3);
                close(5);
                Ok(())
            }
        })
    }
    .status();

    Ok(())
}
