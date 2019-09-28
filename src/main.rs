use std::os::unix::net::UnixStream;

fn main() {
    let stdout_fd = 2;
    let first_msg = "Hello with first fd!\n";
    let second_msg = "Hello with new fd!\n";
    unsafe { libc::write(stdout_fd, first_msg.as_ptr() as *const libc::c_void, first_msg.len()); }

    let (first, second) = UnixStream::pair().unwrap();
    println!("Sending first fd...");
    spawn_pidfd::send_fd(stdout_fd, &first).unwrap();
    println!("Sent!");

    let new_stdout = spawn_pidfd::receive_fd(&second).unwrap();
    println!("Got second fd!");
    unsafe { libc::write(new_stdout, second_msg.as_ptr() as *const libc::c_void, second_msg.len()); }
}
