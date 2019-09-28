use std::os::unix::net::UnixStream;
use std::os::unix::io::{RawFd, AsRawFd};
use std::mem;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Based on https://stackoverflow.com/a/2358843/1290530
pub fn send_fd(fd: RawFd, sock: &UnixStream) -> Result<(), std::io::Error> {
    let mut msg: libc::msghdr = unsafe { mem::zeroed() };
    let fds: [libc::c_int; 1] = [fd];
    let mut buf: [libc::c_char; ONE_FD_BUF_SIZE] = unsafe { mem::zeroed() };
    let fd_ptr: *mut libc::c_int;

    msg.msg_control = buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = mem::size_of_val(&buf);

    let mut dummy: libc::c_char = '$' as libc::c_char;
    let mut iov: [libc::iovec; 1] = unsafe { mem::zeroed() };
    iov[0].iov_base = &mut dummy as *mut _ as *mut libc::c_void;
    iov[0].iov_len = std::mem::size_of::<libc::c_char>();

    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = 1;

    let cmsg: *mut libc::cmsghdr; 
    unsafe {
        cmsg = libc::CMSG_FIRSTHDR(&msg);
        (*cmsg).cmsg_level = libc::SOL_SOCKET;
        (*cmsg).cmsg_type = libc::SCM_RIGHTS;
        (*cmsg).cmsg_len = libc::CMSG_LEN((mem::size_of::<libc::c_int>() * 1) as libc::c_uint) as libc::size_t;
        fd_ptr = libc::CMSG_DATA(cmsg) as *mut libc::c_int;
        *fd_ptr = fds[0];
        msg.msg_controllen = (*cmsg).cmsg_len;

        if libc::sendmsg(sock.as_raw_fd(), &msg, 0) < 0 {
            return Err(std::io::Error::last_os_error())
        }
        Ok(())
    }
}

// Based on https://blog.cloudflare.com/know-your-scm_rights/
// and https://gist.github.com/nazgee/2396992
pub fn receive_fd(sock: &UnixStream) -> Result<RawFd, std::io::Error> {
    let mut msg: libc::msghdr = unsafe { mem::zeroed() };
    let mut buf: [libc::c_char; ONE_FD_BUF_SIZE] = unsafe { mem::zeroed() };

    let mut dummy: libc::c_char = 0;
    let mut iov: [libc::iovec; 1] = unsafe { mem::zeroed() };
    iov[0].iov_base = &mut dummy as *mut _ as *mut libc::c_void;
    iov[0].iov_len = std::mem::size_of::<libc::c_char>();



    msg.msg_iov = iov.as_mut_ptr();
    msg.msg_iovlen = 1;
    msg.msg_control = buf.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = mem::size_of_val(&buf);
    unsafe {
        if libc::recvmsg(sock.as_raw_fd(), &mut msg as *mut libc::msghdr, 0) < 0 {
            return Err(std::io::Error::last_os_error())
        }
        let cmsg = libc::CMSG_FIRSTHDR(&msg);
        let fd = *(libc::CMSG_DATA(cmsg) as *mut libc::c_int);
        Ok(fd)
    }
}

