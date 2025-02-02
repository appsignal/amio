use {io};
use std::net::SocketAddr;
use std::os::unix::io::{AsRawFd, RawFd};
use sys::unix::{nix, Io};

pub fn socket(family: nix::AddressFamily, ty: nix::SockType, nonblock: bool) -> io::Result<RawFd> {
    // Create the socket
    let sock_fd = nix::socket(family, ty, nix::SockFlag::empty(), None)
        .map_err(super::from_nix_error)?;

    if nonblock {
        // Set the socket to nonblocking mode using fcntl
        let flags = nix::OFlag::from_bits_truncate(
            nix::fcntl(sock_fd, nix::FcntlArg::F_GETFL).map_err(super::from_nix_error)?
        );
        nix::fcntl(sock_fd, nix:: FcntlArg::F_SETFL(flags | nix::OFlag::O_NONBLOCK))
            .map_err(super::from_nix_error)?;
    }

    Ok(sock_fd)
}

pub fn connect(io: &Io, addr: &nix::SockAddr) -> io::Result<bool> {
    match nix::connect(io.as_raw_fd(), addr) {
        Ok(_) => Ok(true),
        Err(e) => {
            match e {
                nix::Error::Sys(nix::EINPROGRESS) => Ok(false),
                _ => Err(super::from_nix_error(e))
            }
        }
    }
}

pub fn bind(io: &Io, addr: &nix::SockAddr) -> io::Result<()> {
    nix::bind(io.as_raw_fd(), addr)
        .map_err(super::from_nix_error)
}

pub fn listen(io: &Io, backlog: usize) -> io::Result<()> {
    nix::listen(io.as_raw_fd(), backlog)
        .map_err(super::from_nix_error)
}

pub fn accept(io: &Io, nonblock: bool) -> io::Result<RawFd> {
    let sock_fd = nix::accept4(io.as_raw_fd(), nix::SockFlag::empty())
        .map_err(super::from_nix_error)?;

    if nonblock {
        // Set the socket to nonblocking mode using fcntl
        let flags = nix::OFlag::from_bits_truncate(
            nix::fcntl(sock_fd, nix::FcntlArg::F_GETFL).map_err(super::from_nix_error)?
        );
        nix::fcntl(sock_fd, nix:: FcntlArg::F_SETFL(flags | nix::OFlag::O_NONBLOCK))
            .map_err(super::from_nix_error)?;
    }

    Ok(sock_fd)
}

// UDP & UDS
#[inline]
pub fn recvfrom(io: &Io, buf: &mut [u8]) -> io::Result<(usize, nix::SockAddr)> {
    nix::recvfrom(io.as_raw_fd(), buf)
        .map_err(super::from_nix_error)
}

// UDP & UDS
#[inline]
pub fn sendto(io: &Io, buf: &[u8], target: &nix::SockAddr) -> io::Result<usize> {
    nix::sendto(io.as_raw_fd(), buf, target, nix::MsgFlags::MSG_DONTWAIT)
        .map_err(super::from_nix_error)
}

pub fn getsockname(io: &Io) -> io::Result<nix::SockAddr> {
    nix::getsockname(io.as_raw_fd())
        .map_err(super::from_nix_error)
}

#[inline]
pub fn dup(io: &Io) -> io::Result<Io> {
    nix::dup(io.as_raw_fd())
        .map_err(super::from_nix_error)
        .map(|fd| Io::from_raw_fd(fd))
}

/*
 *
 * ===== Helpers =====
 *
 */

pub fn to_nix_addr(addr: &SocketAddr) -> nix::SockAddr {
    nix::SockAddr::Inet(nix::InetAddr::from_std(addr))
}

pub fn to_std_addr(addr: nix::SockAddr) -> SocketAddr {
    match addr {
        nix::SockAddr::Inet(ref addr) => addr.to_std(),
        _ => panic!("unexpected unix socket address"),
    }
}
