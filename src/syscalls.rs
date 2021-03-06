use libc;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Syscall {
    read                = libc::SYS_read                as isize,
    write               = libc::SYS_write               as isize,
    #[cfg(not(target_arch = "aarch64"))]
    open                = libc::SYS_open                as isize,
    close               = libc::SYS_close               as isize,
    #[cfg(not(target_arch = "aarch64"))]
    stat                = libc::SYS_stat                as isize,
    #[cfg(target_arch = "arm")]
    stat64              = libc::SYS_stat64              as isize,
    fstat               = libc::SYS_fstat               as isize,
    #[cfg(target_arch = "arm")]
    fstat64             = libc::SYS_fstat64             as isize,
    #[cfg(not(target_arch = "aarch64"))]
    lstat               = libc::SYS_lstat               as isize,
    #[cfg(target_arch = "arm")]
    lstat64             = libc::SYS_lstat64             as isize,
    #[cfg(not(target_arch = "aarch64"))]
    poll                = libc::SYS_poll                as isize,
    #[cfg(target_arch = "aarch64")]
    ppoll               = libc::SYS_ppoll               as isize,
    lseek               = libc::SYS_lseek               as isize,
    #[cfg(not(target_arch = "arm"))]
    mmap                = libc::SYS_mmap                as isize,
    #[cfg(target_arch = "arm")]
    mmap2               = libc::SYS_mmap2               as isize,
    mprotect            = libc::SYS_mprotect            as isize,
    munmap              = libc::SYS_munmap              as isize,
    rt_sigprocmask      = libc::SYS_rt_sigprocmask      as isize,
    ioctl               = libc::SYS_ioctl               as isize,
    readv               = libc::SYS_readv               as isize,
    socket              = libc::SYS_socket              as isize,
    connect             = libc::SYS_connect             as isize,
    #[cfg(target_arch = "arm")]
    send                = libc::SYS_send                as isize,
    sendto              = libc::SYS_sendto              as isize,
    sendmsg             = libc::SYS_sendmsg             as isize,
    #[cfg(target_arch = "arm")]
    recv                = libc::SYS_recv                as isize,
    recvfrom            = libc::SYS_recvfrom            as isize,
    recvmsg             = libc::SYS_recvmsg             as isize,
    bind                = libc::SYS_bind                as isize,
    getsockname         = libc::SYS_getsockname         as isize,
    setsockopt          = libc::SYS_setsockopt          as isize,
    getsockopt          = libc::SYS_getsockopt          as isize,
    clone               = libc::SYS_clone               as isize,
    uname               = libc::SYS_uname               as isize,
    fcntl               = libc::SYS_fcntl               as isize,
    #[cfg(target_arch = "arm")]
    fcntl64             = libc::SYS_fcntl64             as isize,
    #[cfg(not(target_arch = "aarch64"))]
    getdents            = libc::SYS_getdents            as isize,
    chdir               = libc::SYS_chdir               as isize,
    getuid              = libc::SYS_getuid              as isize,
    #[cfg(target_arch = "arm")]
    getuid32            = libc::SYS_getuid32              as isize,
    getgid              = libc::SYS_getgid              as isize,
    #[cfg(target_arch = "arm")]
    getgid32            = libc::SYS_getgid32              as isize,
    geteuid             = libc::SYS_geteuid             as isize,
    #[cfg(target_arch = "arm")]
    geteuid32           = libc::SYS_geteuid32           as isize,
    getegid             = libc::SYS_getegid             as isize,
    #[cfg(target_arch = "arm")]
    getegid32           = libc::SYS_getegid32           as isize,
    setuid              = libc::SYS_setuid              as isize,
    #[cfg(target_arch = "arm")]
    setuid32            = libc::SYS_setuid32            as isize,
    setgid              = libc::SYS_setgid              as isize,
    #[cfg(target_arch = "arm")]
    setgid32            = libc::SYS_setgid32            as isize,
    getgroups           = libc::SYS_getgroups           as isize,
    #[cfg(target_arch = "arm")]
    getgroups32         = libc::SYS_getgroups32         as isize,
    setgroups           = libc::SYS_setgroups           as isize,
    getresuid           = libc::SYS_getresuid           as isize,
    #[cfg(target_arch = "arm")]
    getresuid32         = libc::SYS_getresuid32         as isize,
    getresgid           = libc::SYS_getresgid           as isize,
    #[cfg(target_arch = "arm")]
    getresgid32         = libc::SYS_getresgid32         as isize,
    #[cfg(target_arch = "arm")]
    setgroups32         = libc::SYS_setgroups32         as isize,
    sigaltstack         = libc::SYS_sigaltstack         as isize,
    prctl               = libc::SYS_prctl               as isize,
    chroot              = libc::SYS_chroot              as isize,
    futex               = libc::SYS_futex               as isize,
    sched_getaffinity   = libc::SYS_sched_getaffinity   as isize,
    sched_yield         = libc::SYS_sched_yield         as isize,
    clock_getres        = libc::SYS_clock_getres        as isize,
    exit_group          = libc::SYS_exit_group          as isize,
    set_robust_list     = libc::SYS_set_robust_list     as isize,
    openat              = libc::SYS_openat              as isize,
    #[cfg(not(target_arch = "arm"))]
    newfstatat          = libc::SYS_newfstatat          as isize,
    seccomp             = libc::SYS_seccomp             as isize,
    getrandom           = libc::SYS_getrandom           as isize,
    #[cfg(not(target_arch = "aarch64"))]
    pipe                = libc::SYS_pipe                as isize,
    wait4               = libc::SYS_wait4               as isize,
    clock_gettime       = libc::SYS_clock_gettime       as isize,
    gettimeofday        = libc::SYS_gettimeofday        as isize,
    socketpair          = libc::SYS_socketpair          as isize,
    epoll_create1       = libc::SYS_epoll_create1       as isize,
    pipe2               = libc::SYS_pipe2               as isize,
    epoll_ctl           = libc::SYS_epoll_ctl           as isize,
    listen              = libc::SYS_listen              as isize,
    epoll_pwait         = libc::SYS_epoll_pwait         as isize,
    accept4             = libc::SYS_accept4             as isize,
    getdents64          = libc::SYS_getdents64          as isize,
    getpid              = libc::SYS_getpid              as isize,
    eventfd2            = libc::SYS_eventfd2            as isize,
    sched_getparam      = libc::SYS_sched_getparam      as isize,
    sched_getscheduler  = libc::SYS_sched_getscheduler  as isize,
    sched_setscheduler  = libc::SYS_sched_setscheduler  as isize,
    getpeername         = libc::SYS_getpeername         as isize,
    #[cfg(not(target_arch = "aarch64"))]
    readlink            = libc::SYS_readlink            as isize,
    readlinkat          = libc::SYS_readlinkat          as isize,
    #[cfg(not(target_arch = "aarch64"))]
    mkdir               = libc::SYS_mkdir               as isize,
    mkdirat             = libc::SYS_mkdirat             as isize,
    #[cfg(not(target_arch = "aarch64"))]
    unlink              = libc::SYS_unlink              as isize,
    unlinkat            = libc::SYS_unlinkat            as isize,
    #[cfg(not(target_arch = "aarch64"))]
    symlink             = libc::SYS_symlink             as isize,
    symlinkat           = libc::SYS_symlinkat           as isize,
    #[cfg(not(target_arch = "aarch64"))]
    epoll_wait          = libc::SYS_epoll_wait          as isize,
    #[cfg(not(target_arch = "aarch64"))]
    chmod               = libc::SYS_chmod               as isize,
    fchmodat            = libc::SYS_fchmodat            as isize,
    shutdown            = libc::SYS_shutdown            as isize,
    nanosleep           = libc::SYS_nanosleep           as isize,
    madvise             = libc::SYS_madvise             as isize,
    exit                = libc::SYS_exit                as isize,
    capget              = libc::SYS_capget              as isize,
    capset              = libc::SYS_capset              as isize,
    brk                 = libc::SYS_brk                 as isize,
    gettid              = libc::SYS_gettid              as isize,
    tgkill              = libc::SYS_tgkill              as isize,
    #[cfg(not(target_arch = "aarch64"))]
    chown               = libc::SYS_chown               as isize,
    fchown              = libc::SYS_fchown              as isize,
}

impl Syscall {
    #[inline]
    pub fn into_i32(self) -> i32 {
        self as i32
    }
}
