#[cfg(any(target_os = "linux", target_os = "android"))]
mod epoll;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::epoll::{Events, Selector};

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd",
    target_os = "dragonfly", target_os = "netbsd",))]
mod kqueue;

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd",
    target_os = "dragonfly", target_os = "netbsd",))]
pub use self::kqueue::{Events, Selector};

mod awakener;
mod eventedfd;
mod io;
mod net;
mod socket;
mod tcp;
mod udp;
mod uds;

pub use self::awakener::Awakener;
pub use self::eventedfd::EventedFd;
pub use self::io::Io;
pub use self::socket::Socket;
pub use self::tcp::{TcpStream, TcpListener};
pub use self::udp::UdpSocket;
pub use self::uds::UnixSocket;

pub fn pipe() -> ::io::Result<(Io, Io)> {
    // Create pipe without flags
    let (rd, wr) = nix::pipe().map_err(from_nix_error)?;

    // Set O_CLOEXEC and O_NONBLOCK manually
    for &fd in &[rd, wr] {
        let flags = nix::OFlag::from_bits_truncate(nix::fcntl(fd, nix::FcntlArg::F_GETFL).map_err(from_nix_error)?);
        nix::fcntl(fd, nix::FcntlArg::F_SETFL(flags | nix::OFlag::O_CLOEXEC | nix::OFlag::O_NONBLOCK)).map_err(from_nix_error)?;
    }

    Ok((Io::from_raw_fd(rd), Io::from_raw_fd(wr)))
}

pub fn from_nix_error(err: ::nix::Error) -> ::io::Error {
    match err {
        nix::Error::Sys(errno) => {
            ::io::Error::from_raw_os_error(errno as i32)
        },
        _ => {
            ::io::Error::new(::io::ErrorKind::Other, err)
        }
    }
}

mod nix {
    pub use nix::Error;
    pub use nix::libc::{c_int, linger};
    pub use nix::fcntl::{fcntl, FcntlArg, OFlag};
    pub use nix::sys::socket::MsgFlags;
    pub use nix::errno::Errno::EINPROGRESS;
    pub use nix::sys::socket::{
        sockopt,
        AddressFamily,
        SockAddr,
        SockFlag,
        SockType,
        InetAddr,
        IpMembershipRequest,
        Ipv6MembershipRequest,
        Ipv4Addr,
        Ipv6Addr,
        ControlMessage,
        CmsgSpace,
        accept4,
        bind,
        connect,
        getsockname,
        getsockopt,
        listen,
        recvfrom,
        recvmsg,
        sendto,
        sendmsg,
        setsockopt,
        socket,
    };
    pub use nix::sys::time::TimeVal;
    pub use nix::sys::uio::IoVec;
    pub use nix::unistd::{dup, pipe};
}
