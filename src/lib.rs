use std::os::unix::net::UnixDatagram;
use std::os::unix::io::RawFd;
use std::mem;

// Based on https://stackoverflow.com/a/2358843/1290530
/*fn send_fd(fd: RawFd, sock: &UnixDatagram) {
    let dummy: libc::c_char = '$';
    let iov = libc::iovec {
        iov_base: &dummy,
        iov_len: std::mem::size_of_val(&dummy);
    };

    let msg = libc::msghdr {
        msg_name: 0,
        msg_namelen: 0,
        msg_iov: &iov,
        msg_iovlen: 1,
        msg_control: 
    };
}*/
