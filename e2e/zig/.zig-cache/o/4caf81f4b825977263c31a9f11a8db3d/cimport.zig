const __root = @This();
pub const __builtin = @import("std").zig.c_translation.builtins;
pub const __helpers = @import("std").zig.c_translation.helpers;
pub const __builtin_va_list = [*c]u8;
pub const va_list = __builtin_va_list;
pub const __gnuc_va_list = __builtin_va_list;
pub const int_least8_t = i8;
pub const int_least16_t = i16;
pub const int_least32_t = i32;
pub const int_least64_t = i64;
pub const uint_least8_t = u8;
pub const uint_least16_t = u16;
pub const uint_least32_t = u32;
pub const uint_least64_t = u64;
pub const int_fast8_t = i8;
pub const int_fast16_t = i16;
pub const int_fast32_t = i32;
pub const int_fast64_t = i64;
pub const uint_fast8_t = u8;
pub const uint_fast16_t = u16;
pub const uint_fast32_t = u32;
pub const uint_fast64_t = u64;
pub const __int8_t = i8;
pub const __uint8_t = u8;
pub const __int16_t = c_short;
pub const __uint16_t = c_ushort;
pub const __int32_t = c_int;
pub const __uint32_t = c_uint;
pub const __int64_t = c_longlong;
pub const __uint64_t = c_ulonglong;
pub const __darwin_intptr_t = c_long;
pub const __darwin_natural_t = c_uint;
pub const __darwin_ct_rune_t = c_int;
pub const __mbstate_t = extern union {
    __mbstate8: [128]u8,
    _mbstateL: c_longlong,
};
pub const __darwin_mbstate_t = __mbstate_t;
pub const __darwin_ptrdiff_t = c_long;
pub const __darwin_size_t = c_ulong;
pub const __darwin_va_list = __builtin_va_list;
pub const __darwin_wchar_t = c_int;
pub const __darwin_rune_t = __darwin_wchar_t;
pub const __darwin_wint_t = c_int;
pub const __darwin_clock_t = c_ulong;
pub const __darwin_socklen_t = __uint32_t;
pub const __darwin_ssize_t = c_long;
pub const __darwin_time_t = c_long;
pub const __darwin_blkcnt_t = __int64_t;
pub const __darwin_blksize_t = __int32_t;
pub const __darwin_dev_t = __int32_t;
pub const __darwin_fsblkcnt_t = c_uint;
pub const __darwin_fsfilcnt_t = c_uint;
pub const __darwin_gid_t = __uint32_t;
pub const __darwin_id_t = __uint32_t;
pub const __darwin_ino64_t = __uint64_t;
pub const __darwin_ino_t = __darwin_ino64_t;
pub const __darwin_mach_port_name_t = __darwin_natural_t;
pub const __darwin_mach_port_t = __darwin_mach_port_name_t;
pub const __darwin_mode_t = __uint16_t;
pub const __darwin_off_t = __int64_t;
pub const __darwin_pid_t = __int32_t;
pub const __darwin_sigset_t = __uint32_t;
pub const __darwin_suseconds_t = __int32_t;
pub const __darwin_uid_t = __uint32_t;
pub const __darwin_useconds_t = __uint32_t;
pub const __darwin_uuid_t = [16]u8;
pub const __darwin_uuid_string_t = [37]u8;
pub const struct___darwin_pthread_handler_rec = extern struct {
    __routine: ?*const fn (?*anyopaque) callconv(.c) void = null,
    __arg: ?*anyopaque = null,
    __next: [*c]struct___darwin_pthread_handler_rec = null,
};
pub const struct__opaque_pthread_attr_t = extern struct {
    __sig: c_long = 0,
    __opaque: [56]u8 = @import("std").mem.zeroes([56]u8),
};
pub const struct__opaque_pthread_cond_t = extern struct {
    __sig: c_long = 0,
    __opaque: [40]u8 = @import("std").mem.zeroes([40]u8),
};
pub const struct__opaque_pthread_condattr_t = extern struct {
    __sig: c_long = 0,
    __opaque: [8]u8 = @import("std").mem.zeroes([8]u8),
};
pub const struct__opaque_pthread_mutex_t = extern struct {
    __sig: c_long = 0,
    __opaque: [56]u8 = @import("std").mem.zeroes([56]u8),
};
pub const struct__opaque_pthread_mutexattr_t = extern struct {
    __sig: c_long = 0,
    __opaque: [8]u8 = @import("std").mem.zeroes([8]u8),
};
pub const struct__opaque_pthread_once_t = extern struct {
    __sig: c_long = 0,
    __opaque: [8]u8 = @import("std").mem.zeroes([8]u8),
};
pub const struct__opaque_pthread_rwlock_t = extern struct {
    __sig: c_long = 0,
    __opaque: [192]u8 = @import("std").mem.zeroes([192]u8),
};
pub const struct__opaque_pthread_rwlockattr_t = extern struct {
    __sig: c_long = 0,
    __opaque: [16]u8 = @import("std").mem.zeroes([16]u8),
};
pub const struct__opaque_pthread_t = extern struct {
    __sig: c_long = 0,
    __cleanup_stack: [*c]struct___darwin_pthread_handler_rec = null,
    __opaque: [8176]u8 = @import("std").mem.zeroes([8176]u8),
};
pub const __darwin_pthread_attr_t = struct__opaque_pthread_attr_t;
pub const __darwin_pthread_cond_t = struct__opaque_pthread_cond_t;
pub const __darwin_pthread_condattr_t = struct__opaque_pthread_condattr_t;
pub const __darwin_pthread_key_t = c_ulong;
pub const __darwin_pthread_mutex_t = struct__opaque_pthread_mutex_t;
pub const __darwin_pthread_mutexattr_t = struct__opaque_pthread_mutexattr_t;
pub const __darwin_pthread_once_t = struct__opaque_pthread_once_t;
pub const __darwin_pthread_rwlock_t = struct__opaque_pthread_rwlock_t;
pub const __darwin_pthread_rwlockattr_t = struct__opaque_pthread_rwlockattr_t;
pub const __darwin_pthread_t = [*c]struct__opaque_pthread_t;
pub const intmax_t = c_long;
pub const uintmax_t = c_ulong;
pub const __darwin_nl_item = c_int;
pub const __darwin_wctrans_t = c_int;
pub const __darwin_wctype_t = __uint32_t;
pub const P_ALL: c_int = 0;
pub const P_PID: c_int = 1;
pub const P_PGID: c_int = 2;
pub const idtype_t = c_uint;
pub const pid_t = __darwin_pid_t;
pub const id_t = __darwin_id_t;
pub const sig_atomic_t = c_int;
pub const u_int8_t = u8;
pub const u_int16_t = c_ushort;
pub const u_int32_t = c_uint;
pub const u_int64_t = c_ulonglong;
pub const register_t = i64;
pub const user_addr_t = u_int64_t;
pub const user_size_t = u_int64_t;
pub const user_ssize_t = i64;
pub const user_long_t = i64;
pub const user_ulong_t = u_int64_t;
pub const user_time_t = i64;
pub const user_off_t = i64;
pub const syscall_arg_t = u_int64_t;
pub const struct___darwin_arm_exception_state = extern struct {
    __exception: __uint32_t = 0,
    __fsr: __uint32_t = 0,
    __far: __uint32_t = 0,
};
pub const struct___darwin_arm_exception_state64 = extern struct {
    __far: __uint64_t = 0,
    __esr: __uint32_t = 0,
    __exception: __uint32_t = 0,
};
pub const struct___darwin_arm_exception_state64_v2 = extern struct {
    __far: __uint64_t = 0,
    __esr: __uint64_t = 0,
};
pub const struct___darwin_arm_thread_state = extern struct {
    __r: [13]__uint32_t = @import("std").mem.zeroes([13]__uint32_t),
    __sp: __uint32_t = 0,
    __lr: __uint32_t = 0,
    __pc: __uint32_t = 0,
    __cpsr: __uint32_t = 0,
};
pub const struct___darwin_arm_thread_state64 = extern struct {
    __x: [29]__uint64_t = @import("std").mem.zeroes([29]__uint64_t),
    __fp: __uint64_t = 0,
    __lr: __uint64_t = 0,
    __sp: __uint64_t = 0,
    __pc: __uint64_t = 0,
    __cpsr: __uint32_t = 0,
    __pad: __uint32_t = 0,
};
pub const struct___darwin_arm_vfp_state = extern struct {
    __r: [64]__uint32_t = @import("std").mem.zeroes([64]__uint32_t),
    __fpscr: __uint32_t = 0,
};
pub const __uint128_t = u128;
pub const struct___darwin_arm_neon_state64 = extern struct {
    __v: [32]__uint128_t = @import("std").mem.zeroes([32]__uint128_t),
    __fpsr: __uint32_t = 0,
    __fpcr: __uint32_t = 0,
};
pub const struct___darwin_arm_neon_state = extern struct {
    __v: [16]__uint128_t = @import("std").mem.zeroes([16]__uint128_t),
    __fpsr: __uint32_t = 0,
    __fpcr: __uint32_t = 0,
};
pub const struct___arm_pagein_state = extern struct {
    __pagein_error: c_int = 0,
};
pub const struct___darwin_arm_sme_state = extern struct {
    __svcr: __uint64_t = 0,
    __tpidr2_el0: __uint64_t = 0,
    __svl_b: __uint16_t = 0,
};
pub const struct___darwin_arm_sve_z_state = extern struct {
    __z: [16][256]u8 align(4) = @import("std").mem.zeroes([16][256]u8),
};
pub const struct___darwin_arm_sve_p_state = extern struct {
    __p: [16][32]u8 align(4) = @import("std").mem.zeroes([16][32]u8),
};
pub const struct___darwin_arm_sme_za_state = extern struct {
    __za: [4096]u8 align(4) = @import("std").mem.zeroes([4096]u8),
};
pub const struct___darwin_arm_sme2_state = extern struct {
    __zt0: [64]u8 align(4) = @import("std").mem.zeroes([64]u8),
};
pub const struct___arm_legacy_debug_state = extern struct {
    __bvr: [16]__uint32_t = @import("std").mem.zeroes([16]__uint32_t),
    __bcr: [16]__uint32_t = @import("std").mem.zeroes([16]__uint32_t),
    __wvr: [16]__uint32_t = @import("std").mem.zeroes([16]__uint32_t),
    __wcr: [16]__uint32_t = @import("std").mem.zeroes([16]__uint32_t),
};
pub const struct___darwin_arm_debug_state32 = extern struct {
    __bvr: [16]__uint32_t = @import("std").mem.zeroes([16]__uint32_t),
    __bcr: [16]__uint32_t = @import("std").mem.zeroes([16]__uint32_t),
    __wvr: [16]__uint32_t = @import("std").mem.zeroes([16]__uint32_t),
    __wcr: [16]__uint32_t = @import("std").mem.zeroes([16]__uint32_t),
    __mdscr_el1: __uint64_t = 0,
};
pub const struct___darwin_arm_debug_state64 = extern struct {
    __bvr: [16]__uint64_t = @import("std").mem.zeroes([16]__uint64_t),
    __bcr: [16]__uint64_t = @import("std").mem.zeroes([16]__uint64_t),
    __wvr: [16]__uint64_t = @import("std").mem.zeroes([16]__uint64_t),
    __wcr: [16]__uint64_t = @import("std").mem.zeroes([16]__uint64_t),
    __mdscr_el1: __uint64_t = 0,
};
pub const struct___darwin_arm_cpmu_state64 = extern struct {
    __ctrs: [16]__uint64_t = @import("std").mem.zeroes([16]__uint64_t),
};
pub const struct___darwin_mcontext32 = extern struct {
    __es: struct___darwin_arm_exception_state = @import("std").mem.zeroes(struct___darwin_arm_exception_state),
    __ss: struct___darwin_arm_thread_state = @import("std").mem.zeroes(struct___darwin_arm_thread_state),
    __fs: struct___darwin_arm_vfp_state = @import("std").mem.zeroes(struct___darwin_arm_vfp_state),
};
pub const struct___darwin_mcontext64 = extern struct {
    __es: struct___darwin_arm_exception_state64 = @import("std").mem.zeroes(struct___darwin_arm_exception_state64),
    __ss: struct___darwin_arm_thread_state64 = @import("std").mem.zeroes(struct___darwin_arm_thread_state64),
    __ns: struct___darwin_arm_neon_state64 = @import("std").mem.zeroes(struct___darwin_arm_neon_state64),
};
pub const mcontext_t = [*c]struct___darwin_mcontext64;
pub const pthread_attr_t = __darwin_pthread_attr_t;
pub const struct___darwin_sigaltstack = extern struct {
    ss_sp: ?*anyopaque = null,
    ss_size: __darwin_size_t = 0,
    ss_flags: c_int = 0,
};
pub const stack_t = struct___darwin_sigaltstack;
pub const struct___darwin_ucontext = extern struct {
    uc_onstack: c_int = 0,
    uc_sigmask: __darwin_sigset_t = 0,
    uc_stack: struct___darwin_sigaltstack = @import("std").mem.zeroes(struct___darwin_sigaltstack),
    uc_link: [*c]struct___darwin_ucontext = null,
    uc_mcsize: __darwin_size_t = 0,
    uc_mcontext: [*c]struct___darwin_mcontext64 = null,
};
pub const ucontext_t = struct___darwin_ucontext;
pub const sigset_t = __darwin_sigset_t;
pub const uid_t = __darwin_uid_t;
pub const union_sigval = extern union {
    sival_int: c_int,
    sival_ptr: ?*anyopaque,
};
pub const struct_sigevent = extern struct {
    sigev_notify: c_int = 0,
    sigev_signo: c_int = 0,
    sigev_value: union_sigval = @import("std").mem.zeroes(union_sigval),
    sigev_notify_function: ?*const fn (union_sigval) callconv(.c) void = null,
    sigev_notify_attributes: [*c]pthread_attr_t = null,
};
pub const struct___siginfo = extern struct {
    si_signo: c_int = 0,
    si_errno: c_int = 0,
    si_code: c_int = 0,
    si_pid: pid_t = 0,
    si_uid: uid_t = 0,
    si_status: c_int = 0,
    si_addr: ?*anyopaque = null,
    si_value: union_sigval = @import("std").mem.zeroes(union_sigval),
    si_band: c_long = 0,
    __pad: [7]c_ulong = @import("std").mem.zeroes([7]c_ulong),
};
pub const siginfo_t = struct___siginfo;
pub const union___sigaction_u = extern union {
    __sa_handler: ?*const fn (c_int) callconv(.c) void,
    __sa_sigaction: ?*const fn (c_int, [*c]struct___siginfo, ?*anyopaque) callconv(.c) void,
};
pub const struct___sigaction = extern struct {
    __sigaction_u: union___sigaction_u = @import("std").mem.zeroes(union___sigaction_u),
    sa_tramp: ?*const fn (?*anyopaque, c_int, c_int, [*c]siginfo_t, ?*anyopaque) callconv(.c) void = null,
    sa_mask: sigset_t = 0,
    sa_flags: c_int = 0,
};
pub const struct_sigaction = extern struct {
    __sigaction_u: union___sigaction_u = @import("std").mem.zeroes(union___sigaction_u),
    sa_mask: sigset_t = 0,
    sa_flags: c_int = 0,
};
pub const sig_t = ?*const fn (c_int) callconv(.c) void;
pub const struct_sigvec = extern struct {
    sv_handler: ?*const fn (c_int) callconv(.c) void = null,
    sv_mask: c_int = 0,
    sv_flags: c_int = 0,
};
pub const struct_sigstack = extern struct {
    ss_sp: [*c]u8 = null,
    ss_onstack: c_int = 0,
};
pub extern fn signal(c_int, ?*const fn (c_int) callconv(.c) void) ?*const fn (c_int) callconv(.c) void;
pub const struct_timeval = extern struct {
    tv_sec: __darwin_time_t = 0,
    tv_usec: __darwin_suseconds_t = 0,
};
pub const rlim_t = __uint64_t;
pub const struct_rusage = extern struct {
    ru_utime: struct_timeval = @import("std").mem.zeroes(struct_timeval),
    ru_stime: struct_timeval = @import("std").mem.zeroes(struct_timeval),
    ru_maxrss: c_long = 0,
    ru_ixrss: c_long = 0,
    ru_idrss: c_long = 0,
    ru_isrss: c_long = 0,
    ru_minflt: c_long = 0,
    ru_majflt: c_long = 0,
    ru_nswap: c_long = 0,
    ru_inblock: c_long = 0,
    ru_oublock: c_long = 0,
    ru_msgsnd: c_long = 0,
    ru_msgrcv: c_long = 0,
    ru_nsignals: c_long = 0,
    ru_nvcsw: c_long = 0,
    ru_nivcsw: c_long = 0,
};
pub const rusage_info_t = ?*anyopaque;
pub const struct_rusage_info_v0 = extern struct {
    ri_uuid: [16]u8 = @import("std").mem.zeroes([16]u8),
    ri_user_time: u64 = 0,
    ri_system_time: u64 = 0,
    ri_pkg_idle_wkups: u64 = 0,
    ri_interrupt_wkups: u64 = 0,
    ri_pageins: u64 = 0,
    ri_wired_size: u64 = 0,
    ri_resident_size: u64 = 0,
    ri_phys_footprint: u64 = 0,
    ri_proc_start_abstime: u64 = 0,
    ri_proc_exit_abstime: u64 = 0,
};
pub const struct_rusage_info_v1 = extern struct {
    ri_uuid: [16]u8 = @import("std").mem.zeroes([16]u8),
    ri_user_time: u64 = 0,
    ri_system_time: u64 = 0,
    ri_pkg_idle_wkups: u64 = 0,
    ri_interrupt_wkups: u64 = 0,
    ri_pageins: u64 = 0,
    ri_wired_size: u64 = 0,
    ri_resident_size: u64 = 0,
    ri_phys_footprint: u64 = 0,
    ri_proc_start_abstime: u64 = 0,
    ri_proc_exit_abstime: u64 = 0,
    ri_child_user_time: u64 = 0,
    ri_child_system_time: u64 = 0,
    ri_child_pkg_idle_wkups: u64 = 0,
    ri_child_interrupt_wkups: u64 = 0,
    ri_child_pageins: u64 = 0,
    ri_child_elapsed_abstime: u64 = 0,
};
pub const struct_rusage_info_v2 = extern struct {
    ri_uuid: [16]u8 = @import("std").mem.zeroes([16]u8),
    ri_user_time: u64 = 0,
    ri_system_time: u64 = 0,
    ri_pkg_idle_wkups: u64 = 0,
    ri_interrupt_wkups: u64 = 0,
    ri_pageins: u64 = 0,
    ri_wired_size: u64 = 0,
    ri_resident_size: u64 = 0,
    ri_phys_footprint: u64 = 0,
    ri_proc_start_abstime: u64 = 0,
    ri_proc_exit_abstime: u64 = 0,
    ri_child_user_time: u64 = 0,
    ri_child_system_time: u64 = 0,
    ri_child_pkg_idle_wkups: u64 = 0,
    ri_child_interrupt_wkups: u64 = 0,
    ri_child_pageins: u64 = 0,
    ri_child_elapsed_abstime: u64 = 0,
    ri_diskio_bytesread: u64 = 0,
    ri_diskio_byteswritten: u64 = 0,
};
pub const struct_rusage_info_v3 = extern struct {
    ri_uuid: [16]u8 = @import("std").mem.zeroes([16]u8),
    ri_user_time: u64 = 0,
    ri_system_time: u64 = 0,
    ri_pkg_idle_wkups: u64 = 0,
    ri_interrupt_wkups: u64 = 0,
    ri_pageins: u64 = 0,
    ri_wired_size: u64 = 0,
    ri_resident_size: u64 = 0,
    ri_phys_footprint: u64 = 0,
    ri_proc_start_abstime: u64 = 0,
    ri_proc_exit_abstime: u64 = 0,
    ri_child_user_time: u64 = 0,
    ri_child_system_time: u64 = 0,
    ri_child_pkg_idle_wkups: u64 = 0,
    ri_child_interrupt_wkups: u64 = 0,
    ri_child_pageins: u64 = 0,
    ri_child_elapsed_abstime: u64 = 0,
    ri_diskio_bytesread: u64 = 0,
    ri_diskio_byteswritten: u64 = 0,
    ri_cpu_time_qos_default: u64 = 0,
    ri_cpu_time_qos_maintenance: u64 = 0,
    ri_cpu_time_qos_background: u64 = 0,
    ri_cpu_time_qos_utility: u64 = 0,
    ri_cpu_time_qos_legacy: u64 = 0,
    ri_cpu_time_qos_user_initiated: u64 = 0,
    ri_cpu_time_qos_user_interactive: u64 = 0,
    ri_billed_system_time: u64 = 0,
    ri_serviced_system_time: u64 = 0,
};
pub const struct_rusage_info_v4 = extern struct {
    ri_uuid: [16]u8 = @import("std").mem.zeroes([16]u8),
    ri_user_time: u64 = 0,
    ri_system_time: u64 = 0,
    ri_pkg_idle_wkups: u64 = 0,
    ri_interrupt_wkups: u64 = 0,
    ri_pageins: u64 = 0,
    ri_wired_size: u64 = 0,
    ri_resident_size: u64 = 0,
    ri_phys_footprint: u64 = 0,
    ri_proc_start_abstime: u64 = 0,
    ri_proc_exit_abstime: u64 = 0,
    ri_child_user_time: u64 = 0,
    ri_child_system_time: u64 = 0,
    ri_child_pkg_idle_wkups: u64 = 0,
    ri_child_interrupt_wkups: u64 = 0,
    ri_child_pageins: u64 = 0,
    ri_child_elapsed_abstime: u64 = 0,
    ri_diskio_bytesread: u64 = 0,
    ri_diskio_byteswritten: u64 = 0,
    ri_cpu_time_qos_default: u64 = 0,
    ri_cpu_time_qos_maintenance: u64 = 0,
    ri_cpu_time_qos_background: u64 = 0,
    ri_cpu_time_qos_utility: u64 = 0,
    ri_cpu_time_qos_legacy: u64 = 0,
    ri_cpu_time_qos_user_initiated: u64 = 0,
    ri_cpu_time_qos_user_interactive: u64 = 0,
    ri_billed_system_time: u64 = 0,
    ri_serviced_system_time: u64 = 0,
    ri_logical_writes: u64 = 0,
    ri_lifetime_max_phys_footprint: u64 = 0,
    ri_instructions: u64 = 0,
    ri_cycles: u64 = 0,
    ri_billed_energy: u64 = 0,
    ri_serviced_energy: u64 = 0,
    ri_interval_max_phys_footprint: u64 = 0,
    ri_runnable_time: u64 = 0,
};
pub const struct_rusage_info_v5 = extern struct {
    ri_uuid: [16]u8 = @import("std").mem.zeroes([16]u8),
    ri_user_time: u64 = 0,
    ri_system_time: u64 = 0,
    ri_pkg_idle_wkups: u64 = 0,
    ri_interrupt_wkups: u64 = 0,
    ri_pageins: u64 = 0,
    ri_wired_size: u64 = 0,
    ri_resident_size: u64 = 0,
    ri_phys_footprint: u64 = 0,
    ri_proc_start_abstime: u64 = 0,
    ri_proc_exit_abstime: u64 = 0,
    ri_child_user_time: u64 = 0,
    ri_child_system_time: u64 = 0,
    ri_child_pkg_idle_wkups: u64 = 0,
    ri_child_interrupt_wkups: u64 = 0,
    ri_child_pageins: u64 = 0,
    ri_child_elapsed_abstime: u64 = 0,
    ri_diskio_bytesread: u64 = 0,
    ri_diskio_byteswritten: u64 = 0,
    ri_cpu_time_qos_default: u64 = 0,
    ri_cpu_time_qos_maintenance: u64 = 0,
    ri_cpu_time_qos_background: u64 = 0,
    ri_cpu_time_qos_utility: u64 = 0,
    ri_cpu_time_qos_legacy: u64 = 0,
    ri_cpu_time_qos_user_initiated: u64 = 0,
    ri_cpu_time_qos_user_interactive: u64 = 0,
    ri_billed_system_time: u64 = 0,
    ri_serviced_system_time: u64 = 0,
    ri_logical_writes: u64 = 0,
    ri_lifetime_max_phys_footprint: u64 = 0,
    ri_instructions: u64 = 0,
    ri_cycles: u64 = 0,
    ri_billed_energy: u64 = 0,
    ri_serviced_energy: u64 = 0,
    ri_interval_max_phys_footprint: u64 = 0,
    ri_runnable_time: u64 = 0,
    ri_flags: u64 = 0,
};
pub const struct_rusage_info_v6 = extern struct {
    ri_uuid: [16]u8 = @import("std").mem.zeroes([16]u8),
    ri_user_time: u64 = 0,
    ri_system_time: u64 = 0,
    ri_pkg_idle_wkups: u64 = 0,
    ri_interrupt_wkups: u64 = 0,
    ri_pageins: u64 = 0,
    ri_wired_size: u64 = 0,
    ri_resident_size: u64 = 0,
    ri_phys_footprint: u64 = 0,
    ri_proc_start_abstime: u64 = 0,
    ri_proc_exit_abstime: u64 = 0,
    ri_child_user_time: u64 = 0,
    ri_child_system_time: u64 = 0,
    ri_child_pkg_idle_wkups: u64 = 0,
    ri_child_interrupt_wkups: u64 = 0,
    ri_child_pageins: u64 = 0,
    ri_child_elapsed_abstime: u64 = 0,
    ri_diskio_bytesread: u64 = 0,
    ri_diskio_byteswritten: u64 = 0,
    ri_cpu_time_qos_default: u64 = 0,
    ri_cpu_time_qos_maintenance: u64 = 0,
    ri_cpu_time_qos_background: u64 = 0,
    ri_cpu_time_qos_utility: u64 = 0,
    ri_cpu_time_qos_legacy: u64 = 0,
    ri_cpu_time_qos_user_initiated: u64 = 0,
    ri_cpu_time_qos_user_interactive: u64 = 0,
    ri_billed_system_time: u64 = 0,
    ri_serviced_system_time: u64 = 0,
    ri_logical_writes: u64 = 0,
    ri_lifetime_max_phys_footprint: u64 = 0,
    ri_instructions: u64 = 0,
    ri_cycles: u64 = 0,
    ri_billed_energy: u64 = 0,
    ri_serviced_energy: u64 = 0,
    ri_interval_max_phys_footprint: u64 = 0,
    ri_runnable_time: u64 = 0,
    ri_flags: u64 = 0,
    ri_user_ptime: u64 = 0,
    ri_system_ptime: u64 = 0,
    ri_pinstructions: u64 = 0,
    ri_pcycles: u64 = 0,
    ri_energy_nj: u64 = 0,
    ri_penergy_nj: u64 = 0,
    ri_secure_time_in_system: u64 = 0,
    ri_secure_ptime_in_system: u64 = 0,
    ri_neural_footprint: u64 = 0,
    ri_lifetime_max_neural_footprint: u64 = 0,
    ri_interval_max_neural_footprint: u64 = 0,
    ri_reserved: [9]u64 = @import("std").mem.zeroes([9]u64),
};
pub const rusage_info_current = struct_rusage_info_v6;
pub const struct_rlimit = extern struct {
    rlim_cur: rlim_t = 0,
    rlim_max: rlim_t = 0,
};
pub const struct_proc_rlimit_control_wakeupmon = extern struct {
    wm_flags: u32 = 0,
    wm_rate: i32 = 0,
};
pub extern fn getpriority(c_int, id_t) c_int;
pub extern fn getiopolicy_np(c_int, c_int) c_int;
pub extern fn getrlimit(c_int, [*c]struct_rlimit) c_int;
pub extern fn getrusage(c_int, [*c]struct_rusage) c_int;
pub extern fn setpriority(c_int, id_t, c_int) c_int;
pub extern fn setiopolicy_np(c_int, c_int, c_int) c_int;
pub extern fn setrlimit(c_int, [*c]const struct_rlimit) c_int;
pub fn _OSSwapInt16(arg__data: __uint16_t) callconv(.c) __uint16_t {
    var _data = arg__data;
    _ = &_data;
    return @bitCast(@as(c_short, @truncate((@as(c_int, _data) << @intCast(@as(c_int, 8))) | (@as(c_int, _data) >> @intCast(@as(c_int, 8))))));
}
pub fn _OSSwapInt32(arg__data: __uint32_t) callconv(.c) __uint32_t {
    var _data = arg__data;
    _ = &_data;
    _data = (((_data ^ ((_data >> @intCast(@as(__uint32_t, 16))) | (_data << @intCast(@as(__uint32_t, 16))))) & @as(c_uint, 4278255615)) >> @intCast(@as(__uint32_t, 8))) ^ ((_data >> @intCast(@as(__uint32_t, 8))) | (_data << @intCast(@as(__uint32_t, 24))));
    return _data;
}
pub fn _OSSwapInt64(arg__data: __uint64_t) callconv(.c) __uint64_t {
    var _data = arg__data;
    _ = &_data;
    const union_unnamed_1 = extern union {
        _ull: __uint64_t,
        _ul: [2]__uint32_t,
    };
    _ = &union_unnamed_1;
    var _u: union_unnamed_1 = undefined;
    _ = &_u;
    _u._ul[@as(c_int, 0)] = @truncate(_data >> @intCast(@as(__uint64_t, 32)));
    _u._ul[@as(c_int, 1)] = @truncate(_data & @as(__uint64_t, 4294967295));
    _u._ul[@as(c_int, 0)] = _OSSwapInt32(_u._ul[@as(c_int, 0)]);
    _u._ul[@as(c_int, 1)] = _OSSwapInt32(_u._ul[@as(c_int, 1)]);
    return _u._ull;
} // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/wait.h:201:19: warning: struct demoted to opaque type - has bitfield
const struct_unnamed_2 = opaque {}; // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/wait.h:211:4: warning: union demoted to opaque type - has opaque field
pub const union_wait = opaque {};
pub extern fn wait([*c]c_int) pid_t;
pub extern fn waitpid(pid_t, [*c]c_int, c_int) pid_t;
pub extern fn waitid(idtype_t, id_t, [*c]siginfo_t, c_int) c_int;
pub extern fn wait3([*c]c_int, c_int, [*c]struct_rusage) pid_t;
pub extern fn wait4(pid_t, [*c]c_int, c_int, [*c]struct_rusage) pid_t;
pub extern fn alloca(__size: usize) ?*anyopaque;
pub const ct_rune_t = __darwin_ct_rune_t;
pub const rune_t = __darwin_rune_t;
pub const wchar_t = __darwin_wchar_t;
pub const div_t = extern struct {
    quot: c_int = 0,
    rem: c_int = 0,
};
pub const ldiv_t = extern struct {
    quot: c_long = 0,
    rem: c_long = 0,
};
pub const lldiv_t = extern struct {
    quot: c_longlong = 0,
    rem: c_longlong = 0,
};
pub extern var __mb_cur_max: c_int;
pub const malloc_type_id_t = c_ulonglong;
pub extern fn malloc_type_malloc(size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_calloc(count: usize, size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_free(ptr: ?*anyopaque, type_id: malloc_type_id_t) void;
pub extern fn malloc_type_realloc(ptr: ?*anyopaque, size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_valloc(size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_aligned_alloc(alignment: usize, size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_posix_memalign(memptr: [*c]?*anyopaque, alignment: usize, size: usize, type_id: malloc_type_id_t) c_int;
pub const struct__malloc_zone_t = opaque {
    pub const malloc_type_zone_malloc = __root.malloc_type_zone_malloc;
    pub const malloc_type_zone_calloc = __root.malloc_type_zone_calloc;
    pub const malloc_type_zone_free = __root.malloc_type_zone_free;
    pub const malloc_type_zone_realloc = __root.malloc_type_zone_realloc;
    pub const malloc_type_zone_valloc = __root.malloc_type_zone_valloc;
    pub const malloc_type_zone_memalign = __root.malloc_type_zone_memalign;
    pub const memalign = __root.malloc_type_zone_memalign;
};
pub const malloc_zone_t = struct__malloc_zone_t;
pub extern fn malloc_type_zone_malloc(zone: ?*malloc_zone_t, size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_zone_calloc(zone: ?*malloc_zone_t, count: usize, size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_zone_free(zone: ?*malloc_zone_t, ptr: ?*anyopaque, type_id: malloc_type_id_t) void;
pub extern fn malloc_type_zone_realloc(zone: ?*malloc_zone_t, ptr: ?*anyopaque, size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_zone_valloc(zone: ?*malloc_zone_t, size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc_type_zone_memalign(zone: ?*malloc_zone_t, alignment: usize, size: usize, type_id: malloc_type_id_t) ?*anyopaque;
pub extern fn malloc(__size: usize) ?*anyopaque;
pub extern fn calloc(__count: usize, __size: usize) ?*anyopaque;
pub extern fn free(?*anyopaque) void;
pub extern fn realloc(__ptr: ?*anyopaque, __size: usize) ?*anyopaque;
pub extern fn reallocf(__ptr: ?*anyopaque, __size: usize) ?*anyopaque;
pub extern fn valloc(__size: usize) ?*anyopaque;
pub extern fn aligned_alloc(__alignment: usize, __size: usize) ?*anyopaque;
pub extern fn posix_memalign(__memptr: [*c]?*anyopaque, __alignment: usize, __size: usize) c_int;
pub extern fn abort() noreturn;
pub extern fn abs(c_int) c_int;
pub extern fn atexit(?*const fn () callconv(.c) void) c_int;
pub extern fn at_quick_exit(?*const fn () callconv(.c) void) c_int;
pub extern fn atof([*c]const u8) f64;
pub extern fn atoi([*c]const u8) c_int;
pub extern fn atol([*c]const u8) c_long;
pub extern fn atoll([*c]const u8) c_longlong;
pub extern fn bsearch(__key: ?*const anyopaque, __base: ?*const anyopaque, __nel: usize, __width: usize, __compar: ?*const fn (?*const anyopaque, ?*const anyopaque) callconv(.c) c_int) ?*anyopaque;
pub extern fn div(c_int, c_int) div_t;
pub extern fn exit(c_int) noreturn;
pub extern fn getenv([*c]const u8) [*c]u8;
pub extern fn labs(c_long) c_long;
pub extern fn ldiv(c_long, c_long) ldiv_t;
pub extern fn llabs(c_longlong) c_longlong;
pub extern fn lldiv(c_longlong, c_longlong) lldiv_t;
pub extern fn mblen(__s: [*c]const u8, __n: usize) c_int;
pub extern fn mbstowcs(noalias [*c]wchar_t, noalias [*c]const u8, __n: usize) usize;
pub extern fn mbtowc(noalias [*c]wchar_t, noalias [*c]const u8, __n: usize) c_int;
pub extern fn qsort(__base: ?*anyopaque, __nel: usize, __width: usize, __compar: ?*const fn (?*const anyopaque, ?*const anyopaque) callconv(.c) c_int) void;
pub extern fn quick_exit(c_int) noreturn;
pub extern fn rand() c_int;
pub extern fn srand(c_uint) void;
pub extern fn strtod([*c]const u8, [*c][*c]u8) f64;
pub extern fn strtof([*c]const u8, [*c][*c]u8) f32;
pub extern fn strtol(__str: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_long;
pub extern fn strtold([*c]const u8, [*c][*c]u8) c_longdouble;
pub extern fn strtoll(__str: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_longlong;
pub extern fn strtoul(__str: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_ulong;
pub extern fn strtoull(__str: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_ulonglong;
pub extern fn system([*c]const u8) c_int;
pub extern fn wcstombs(noalias [*c]u8, noalias [*c]const wchar_t, __n: usize) usize;
pub extern fn wctomb([*c]u8, wchar_t) c_int;
pub extern fn _Exit(c_int) noreturn;
pub extern fn a64l([*c]const u8) c_long;
pub extern fn drand48() f64;
pub extern fn ecvt(f64, c_int, noalias [*c]c_int, noalias [*c]c_int) [*c]u8;
pub extern fn erand48([*c]c_ushort) f64;
pub extern fn fcvt(f64, c_int, noalias [*c]c_int, noalias [*c]c_int) [*c]u8;
pub extern fn gcvt(f64, c_int, [*c]u8) [*c]u8;
pub extern fn getsubopt([*c][*c]u8, [*c]const [*c]u8, [*c][*c]u8) c_int;
pub extern fn grantpt(c_int) c_int;
pub extern fn initstate(c_uint, [*c]u8, __size: usize) [*c]u8;
pub extern fn jrand48([*c]c_ushort) c_long;
pub extern fn l64a(c_long) [*c]u8;
pub extern fn lcong48([*c]c_ushort) void;
pub extern fn lrand48() c_long;
pub extern fn mktemp([*c]u8) [*c]u8;
pub extern fn mkstemp([*c]u8) c_int;
pub extern fn mrand48() c_long;
pub extern fn nrand48([*c]c_ushort) c_long;
pub extern fn posix_openpt(c_int) c_int;
pub extern fn ptsname(c_int) [*c]u8;
pub extern fn ptsname_r(fildes: c_int, buffer: [*c]u8, buflen: usize) c_int;
pub extern fn putenv([*c]u8) c_int;
pub extern fn random() c_long;
pub extern fn rand_r([*c]c_uint) c_int;
pub extern fn realpath(noalias [*c]const u8, noalias [*c]u8) [*c]u8;
pub extern fn seed48([*c]c_ushort) [*c]c_ushort;
pub extern fn setenv(__name: [*c]const u8, __value: [*c]const u8, __overwrite: c_int) c_int;
pub extern fn setkey([*c]const u8) void;
pub extern fn setstate([*c]const u8) [*c]u8;
pub extern fn srand48(c_long) void;
pub extern fn srandom(c_uint) void;
pub extern fn unlockpt(c_int) c_int;
pub extern fn unsetenv([*c]const u8) c_int;
pub const dev_t = __darwin_dev_t;
pub const mode_t = __darwin_mode_t;
pub extern fn arc4random() u32;
pub extern fn arc4random_addrandom([*c]u8, __datlen: c_int) void;
pub extern fn arc4random_buf(__buf: ?*anyopaque, __nbytes: usize) void;
pub extern fn arc4random_stir() void;
pub extern fn arc4random_uniform(__upper_bound: u32) u32;
pub extern fn cgetcap([*c]u8, [*c]const u8, c_int) [*c]u8;
pub extern fn cgetclose() c_int;
pub extern fn cgetent([*c][*c]u8, [*c][*c]u8, [*c]const u8) c_int;
pub extern fn cgetfirst([*c][*c]u8, [*c][*c]u8) c_int;
pub extern fn cgetmatch([*c]const u8, [*c]const u8) c_int;
pub extern fn cgetnext([*c][*c]u8, [*c][*c]u8) c_int;
pub extern fn cgetnum([*c]u8, [*c]const u8, [*c]c_long) c_int;
pub extern fn cgetset([*c]const u8) c_int;
pub extern fn cgetstr([*c]u8, [*c]const u8, [*c][*c]u8) c_int;
pub extern fn cgetustr([*c]u8, [*c]const u8, [*c][*c]u8) c_int;
pub extern fn daemon(c_int, c_int) c_int;
pub extern fn devname(dev_t, mode_t) [*c]u8;
pub extern fn devname_r(dev_t, mode_t, buf: [*c]u8, len: c_int) [*c]u8;
pub extern fn getbsize([*c]c_int, [*c]c_long) [*c]u8;
pub extern fn getloadavg([*c]f64, __nelem: c_int) c_int;
pub extern fn getprogname() [*c]const u8;
pub extern fn setprogname([*c]const u8) void;
pub extern fn heapsort(__base: ?*anyopaque, __nel: usize, __width: usize, __compar: ?*const fn (?*const anyopaque, ?*const anyopaque) callconv(.c) c_int) c_int;
pub extern fn mergesort(__base: ?*anyopaque, __nel: usize, __width: usize, __compar: ?*const fn (?*const anyopaque, ?*const anyopaque) callconv(.c) c_int) c_int;
pub extern fn psort(__base: ?*anyopaque, __nel: usize, __width: usize, __compar: ?*const fn (?*const anyopaque, ?*const anyopaque) callconv(.c) c_int) void;
pub extern fn psort_r(__base: ?*anyopaque, __nel: usize, __width: usize, ?*anyopaque, __compar: ?*const fn (?*anyopaque, ?*const anyopaque, ?*const anyopaque) callconv(.c) c_int) void;
pub extern fn qsort_r(__base: ?*anyopaque, __nel: usize, __width: usize, ?*anyopaque, __compar: ?*const fn (?*anyopaque, ?*const anyopaque, ?*const anyopaque) callconv(.c) c_int) void;
pub extern fn radixsort(__base: [*c][*c]const u8, __nel: c_int, __table: [*c]const u8, __endbyte: c_uint) c_int;
pub extern fn rpmatch([*c]const u8) c_int;
pub extern fn sradixsort(__base: [*c][*c]const u8, __nel: c_int, __table: [*c]const u8, __endbyte: c_uint) c_int;
pub extern fn sranddev() void;
pub extern fn srandomdev() void;
pub extern fn strtonum(__numstr: [*c]const u8, __minval: c_longlong, __maxval: c_longlong, __errstrp: [*c][*c]const u8) c_longlong;
pub extern fn strtoq(__str: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_longlong;
pub extern fn strtouq(__str: [*c]const u8, __endptr: [*c][*c]u8, __base: c_int) c_ulonglong;
pub extern var suboptarg: [*c]u8;
pub const struct_KREUZBERGAccelerationConfig = opaque {
    pub const kreuzberg_acceleration_config_to_json = __root.kreuzberg_acceleration_config_to_json;
    pub const kreuzberg_acceleration_config_free = __root.kreuzberg_acceleration_config_free;
    pub const kreuzberg_acceleration_config_provider = __root.kreuzberg_acceleration_config_provider;
    pub const kreuzberg_acceleration_config_device_id = __root.kreuzberg_acceleration_config_device_id;
    pub const json = __root.kreuzberg_acceleration_config_to_json;
    pub const provider = __root.kreuzberg_acceleration_config_provider;
    pub const id = __root.kreuzberg_acceleration_config_device_id;
};
pub const KREUZBERGAccelerationConfig = struct_KREUZBERGAccelerationConfig;
pub const struct_KREUZBERGAnchorProperties = opaque {
    pub const kreuzberg_anchor_properties_to_json = __root.kreuzberg_anchor_properties_to_json;
    pub const kreuzberg_anchor_properties_free = __root.kreuzberg_anchor_properties_free;
    pub const kreuzberg_anchor_properties_behind_doc = __root.kreuzberg_anchor_properties_behind_doc;
    pub const kreuzberg_anchor_properties_layout_in_cell = __root.kreuzberg_anchor_properties_layout_in_cell;
    pub const kreuzberg_anchor_properties_relative_height = __root.kreuzberg_anchor_properties_relative_height;
    pub const json = __root.kreuzberg_anchor_properties_to_json;
    pub const doc = __root.kreuzberg_anchor_properties_behind_doc;
    pub const cell = __root.kreuzberg_anchor_properties_layout_in_cell;
    pub const height = __root.kreuzberg_anchor_properties_relative_height;
};
pub const KREUZBERGAnchorProperties = struct_KREUZBERGAnchorProperties;
pub const struct_KREUZBERGAnnotationKind = opaque {};
pub const KREUZBERGAnnotationKind = struct_KREUZBERGAnnotationKind;
pub const struct_KREUZBERGApiDoc = opaque {
    pub const kreuzberg_api_doc_free = __root.kreuzberg_api_doc_free;
};
pub const KREUZBERGApiDoc = struct_KREUZBERGApiDoc;
pub const struct_KREUZBERGArchiveEntry = opaque {
    pub const kreuzberg_archive_entry_to_json = __root.kreuzberg_archive_entry_to_json;
    pub const kreuzberg_archive_entry_free = __root.kreuzberg_archive_entry_free;
    pub const kreuzberg_archive_entry_path = __root.kreuzberg_archive_entry_path;
    pub const kreuzberg_archive_entry_mime_type = __root.kreuzberg_archive_entry_mime_type;
    pub const kreuzberg_archive_entry_result = __root.kreuzberg_archive_entry_result;
    pub const json = __root.kreuzberg_archive_entry_to_json;
    pub const path = __root.kreuzberg_archive_entry_path;
    pub const @"type" = __root.kreuzberg_archive_entry_mime_type;
    pub const result = __root.kreuzberg_archive_entry_result;
};
pub const KREUZBERGArchiveEntry = struct_KREUZBERGArchiveEntry;
pub const struct_KREUZBERGArchiveMetadata = opaque {
    pub const kreuzberg_archive_metadata_to_json = __root.kreuzberg_archive_metadata_to_json;
    pub const kreuzberg_archive_metadata_free = __root.kreuzberg_archive_metadata_free;
    pub const kreuzberg_archive_metadata_file_count = __root.kreuzberg_archive_metadata_file_count;
    pub const kreuzberg_archive_metadata_file_list = __root.kreuzberg_archive_metadata_file_list;
    pub const kreuzberg_archive_metadata_total_size = __root.kreuzberg_archive_metadata_total_size;
    pub const kreuzberg_archive_metadata_compressed_size = __root.kreuzberg_archive_metadata_compressed_size;
    pub const json = __root.kreuzberg_archive_metadata_to_json;
    pub const count = __root.kreuzberg_archive_metadata_file_count;
    pub const list = __root.kreuzberg_archive_metadata_file_list;
    pub const size = __root.kreuzberg_archive_metadata_total_size;
};
pub const KREUZBERGArchiveMetadata = struct_KREUZBERGArchiveMetadata;
pub const struct_KREUZBERGBBox = opaque {
    pub const kreuzberg_b_box_to_json = __root.kreuzberg_b_box_to_json;
    pub const kreuzberg_b_box_free = __root.kreuzberg_b_box_free;
    pub const kreuzberg_b_box_x1 = __root.kreuzberg_b_box_x1;
    pub const kreuzberg_b_box_y1 = __root.kreuzberg_b_box_y1;
    pub const kreuzberg_b_box_x2 = __root.kreuzberg_b_box_x2;
    pub const kreuzberg_b_box_y2 = __root.kreuzberg_b_box_y2;
    pub const json = __root.kreuzberg_b_box_to_json;
    pub const x1 = __root.kreuzberg_b_box_x1;
    pub const y1 = __root.kreuzberg_b_box_y1;
    pub const x2 = __root.kreuzberg_b_box_x2;
    pub const y2 = __root.kreuzberg_b_box_y2;
};
pub const KREUZBERGBBox = struct_KREUZBERGBBox;
pub const struct_KREUZBERGBatchBytesItem = opaque {
    pub const kreuzberg_batch_bytes_item_to_json = __root.kreuzberg_batch_bytes_item_to_json;
    pub const kreuzberg_batch_bytes_item_free = __root.kreuzberg_batch_bytes_item_free;
    pub const kreuzberg_batch_bytes_item_content = __root.kreuzberg_batch_bytes_item_content;
    pub const kreuzberg_batch_bytes_item_mime_type = __root.kreuzberg_batch_bytes_item_mime_type;
    pub const kreuzberg_batch_bytes_item_config = __root.kreuzberg_batch_bytes_item_config;
    pub const json = __root.kreuzberg_batch_bytes_item_to_json;
    pub const content = __root.kreuzberg_batch_bytes_item_content;
    pub const @"type" = __root.kreuzberg_batch_bytes_item_mime_type;
    pub const config = __root.kreuzberg_batch_bytes_item_config;
};
pub const KREUZBERGBatchBytesItem = struct_KREUZBERGBatchBytesItem;
pub const struct_KREUZBERGBatchFileItem = opaque {
    pub const kreuzberg_batch_file_item_to_json = __root.kreuzberg_batch_file_item_to_json;
    pub const kreuzberg_batch_file_item_free = __root.kreuzberg_batch_file_item_free;
    pub const kreuzberg_batch_file_item_path = __root.kreuzberg_batch_file_item_path;
    pub const kreuzberg_batch_file_item_config = __root.kreuzberg_batch_file_item_config;
    pub const json = __root.kreuzberg_batch_file_item_to_json;
    pub const path = __root.kreuzberg_batch_file_item_path;
    pub const config = __root.kreuzberg_batch_file_item_config;
};
pub const KREUZBERGBatchFileItem = struct_KREUZBERGBatchFileItem;
pub const struct_KREUZBERGBibtexMetadata = opaque {
    pub const kreuzberg_bibtex_metadata_to_json = __root.kreuzberg_bibtex_metadata_to_json;
    pub const kreuzberg_bibtex_metadata_free = __root.kreuzberg_bibtex_metadata_free;
    pub const kreuzberg_bibtex_metadata_entry_count = __root.kreuzberg_bibtex_metadata_entry_count;
    pub const kreuzberg_bibtex_metadata_citation_keys = __root.kreuzberg_bibtex_metadata_citation_keys;
    pub const kreuzberg_bibtex_metadata_authors = __root.kreuzberg_bibtex_metadata_authors;
    pub const kreuzberg_bibtex_metadata_year_range = __root.kreuzberg_bibtex_metadata_year_range;
    pub const kreuzberg_bibtex_metadata_entry_types = __root.kreuzberg_bibtex_metadata_entry_types;
    pub const json = __root.kreuzberg_bibtex_metadata_to_json;
    pub const count = __root.kreuzberg_bibtex_metadata_entry_count;
    pub const keys = __root.kreuzberg_bibtex_metadata_citation_keys;
    pub const authors = __root.kreuzberg_bibtex_metadata_authors;
    pub const range = __root.kreuzberg_bibtex_metadata_year_range;
    pub const types = __root.kreuzberg_bibtex_metadata_entry_types;
};
pub const KREUZBERGBibtexMetadata = struct_KREUZBERGBibtexMetadata;
pub const struct_KREUZBERGBlockType = opaque {};
pub const KREUZBERGBlockType = struct_KREUZBERGBlockType;
pub const struct_KREUZBERGByteBufferPool = opaque {
    pub const kreuzberg_byte_buffer_pool_free = __root.kreuzberg_byte_buffer_pool_free;
};
pub const KREUZBERGByteBufferPool = struct_KREUZBERGByteBufferPool;
pub const struct_KREUZBERGCacheWarmParams = opaque {
    pub const kreuzberg_cache_warm_params_to_json = __root.kreuzberg_cache_warm_params_to_json;
    pub const kreuzberg_cache_warm_params_free = __root.kreuzberg_cache_warm_params_free;
    pub const kreuzberg_cache_warm_params_all_embeddings = __root.kreuzberg_cache_warm_params_all_embeddings;
    pub const kreuzberg_cache_warm_params_embedding_model = __root.kreuzberg_cache_warm_params_embedding_model;
    pub const json = __root.kreuzberg_cache_warm_params_to_json;
    pub const embeddings = __root.kreuzberg_cache_warm_params_all_embeddings;
    pub const model = __root.kreuzberg_cache_warm_params_embedding_model;
};
pub const KREUZBERGCacheWarmParams = struct_KREUZBERGCacheWarmParams;
pub const struct_KREUZBERGCharShape = opaque {
    pub const kreuzberg_char_shape_free = __root.kreuzberg_char_shape_free;
    pub const kreuzberg_char_shape_bold = __root.kreuzberg_char_shape_bold;
    pub const kreuzberg_char_shape_italic = __root.kreuzberg_char_shape_italic;
    pub const kreuzberg_char_shape_underline = __root.kreuzberg_char_shape_underline;
    pub const bold = __root.kreuzberg_char_shape_bold;
    pub const italic = __root.kreuzberg_char_shape_italic;
    pub const underline = __root.kreuzberg_char_shape_underline;
};
pub const KREUZBERGCharShape = struct_KREUZBERGCharShape;
pub const struct_KREUZBERGChunk = opaque {
    pub const kreuzberg_chunk_to_json = __root.kreuzberg_chunk_to_json;
    pub const kreuzberg_chunk_free = __root.kreuzberg_chunk_free;
    pub const kreuzberg_chunk_content = __root.kreuzberg_chunk_content;
    pub const kreuzberg_chunk_chunk_type = __root.kreuzberg_chunk_chunk_type;
    pub const kreuzberg_chunk_embedding = __root.kreuzberg_chunk_embedding;
    pub const kreuzberg_chunk_metadata = __root.kreuzberg_chunk_metadata;
    pub const json = __root.kreuzberg_chunk_to_json;
    pub const content = __root.kreuzberg_chunk_content;
    pub const @"type" = __root.kreuzberg_chunk_chunk_type;
    pub const embedding = __root.kreuzberg_chunk_embedding;
    pub const metadata = __root.kreuzberg_chunk_metadata;
};
pub const KREUZBERGChunk = struct_KREUZBERGChunk;
pub const struct_KREUZBERGChunkMetadata = opaque {
    pub const kreuzberg_chunk_metadata_to_json = __root.kreuzberg_chunk_metadata_to_json;
    pub const kreuzberg_chunk_metadata_free = __root.kreuzberg_chunk_metadata_free;
    pub const kreuzberg_chunk_metadata_byte_start = __root.kreuzberg_chunk_metadata_byte_start;
    pub const kreuzberg_chunk_metadata_byte_end = __root.kreuzberg_chunk_metadata_byte_end;
    pub const kreuzberg_chunk_metadata_token_count = __root.kreuzberg_chunk_metadata_token_count;
    pub const kreuzberg_chunk_metadata_chunk_index = __root.kreuzberg_chunk_metadata_chunk_index;
    pub const kreuzberg_chunk_metadata_total_chunks = __root.kreuzberg_chunk_metadata_total_chunks;
    pub const kreuzberg_chunk_metadata_first_page = __root.kreuzberg_chunk_metadata_first_page;
    pub const kreuzberg_chunk_metadata_last_page = __root.kreuzberg_chunk_metadata_last_page;
    pub const kreuzberg_chunk_metadata_heading_context = __root.kreuzberg_chunk_metadata_heading_context;
    pub const json = __root.kreuzberg_chunk_metadata_to_json;
    pub const start = __root.kreuzberg_chunk_metadata_byte_start;
    pub const end = __root.kreuzberg_chunk_metadata_byte_end;
    pub const count = __root.kreuzberg_chunk_metadata_token_count;
    pub const index = __root.kreuzberg_chunk_metadata_chunk_index;
    pub const chunks = __root.kreuzberg_chunk_metadata_total_chunks;
    pub const page = __root.kreuzberg_chunk_metadata_first_page;
    pub const context = __root.kreuzberg_chunk_metadata_heading_context;
};
pub const KREUZBERGChunkMetadata = struct_KREUZBERGChunkMetadata;
pub const struct_KREUZBERGChunkRequest = opaque {
    pub const kreuzberg_chunk_request_to_json = __root.kreuzberg_chunk_request_to_json;
    pub const kreuzberg_chunk_request_free = __root.kreuzberg_chunk_request_free;
    pub const kreuzberg_chunk_request_text = __root.kreuzberg_chunk_request_text;
    pub const kreuzberg_chunk_request_chunker_type = __root.kreuzberg_chunk_request_chunker_type;
    pub const json = __root.kreuzberg_chunk_request_to_json;
    pub const text = __root.kreuzberg_chunk_request_text;
    pub const @"type" = __root.kreuzberg_chunk_request_chunker_type;
};
pub const KREUZBERGChunkRequest = struct_KREUZBERGChunkRequest;
pub const struct_KREUZBERGChunkResponse = opaque {
    pub const kreuzberg_chunk_response_to_json = __root.kreuzberg_chunk_response_to_json;
    pub const kreuzberg_chunk_response_free = __root.kreuzberg_chunk_response_free;
    pub const kreuzberg_chunk_response_chunk_count = __root.kreuzberg_chunk_response_chunk_count;
    pub const kreuzberg_chunk_response_input_size_bytes = __root.kreuzberg_chunk_response_input_size_bytes;
    pub const kreuzberg_chunk_response_chunker_type = __root.kreuzberg_chunk_response_chunker_type;
    pub const json = __root.kreuzberg_chunk_response_to_json;
    pub const count = __root.kreuzberg_chunk_response_chunk_count;
    pub const bytes = __root.kreuzberg_chunk_response_input_size_bytes;
    pub const @"type" = __root.kreuzberg_chunk_response_chunker_type;
};
pub const KREUZBERGChunkResponse = struct_KREUZBERGChunkResponse;
pub const struct_KREUZBERGChunkSizing = opaque {};
pub const KREUZBERGChunkSizing = struct_KREUZBERGChunkSizing;
pub const struct_KREUZBERGChunkTextParams = opaque {
    pub const kreuzberg_chunk_text_params_to_json = __root.kreuzberg_chunk_text_params_to_json;
    pub const kreuzberg_chunk_text_params_free = __root.kreuzberg_chunk_text_params_free;
    pub const kreuzberg_chunk_text_params_text = __root.kreuzberg_chunk_text_params_text;
    pub const kreuzberg_chunk_text_params_max_characters = __root.kreuzberg_chunk_text_params_max_characters;
    pub const kreuzberg_chunk_text_params_overlap = __root.kreuzberg_chunk_text_params_overlap;
    pub const kreuzberg_chunk_text_params_chunker_type = __root.kreuzberg_chunk_text_params_chunker_type;
    pub const kreuzberg_chunk_text_params_topic_threshold = __root.kreuzberg_chunk_text_params_topic_threshold;
    pub const json = __root.kreuzberg_chunk_text_params_to_json;
    pub const text = __root.kreuzberg_chunk_text_params_text;
    pub const characters = __root.kreuzberg_chunk_text_params_max_characters;
    pub const overlap = __root.kreuzberg_chunk_text_params_overlap;
    pub const @"type" = __root.kreuzberg_chunk_text_params_chunker_type;
    pub const threshold = __root.kreuzberg_chunk_text_params_topic_threshold;
};
pub const KREUZBERGChunkTextParams = struct_KREUZBERGChunkTextParams;
pub const struct_KREUZBERGChunkType = opaque {};
pub const KREUZBERGChunkType = struct_KREUZBERGChunkType;
pub const struct_KREUZBERGChunkerType = opaque {};
pub const KREUZBERGChunkerType = struct_KREUZBERGChunkerType;
pub const struct_KREUZBERGChunkingConfig = opaque {
    pub const kreuzberg_chunking_config_to_json = __root.kreuzberg_chunking_config_to_json;
    pub const kreuzberg_chunking_config_free = __root.kreuzberg_chunking_config_free;
    pub const kreuzberg_chunking_config_max_characters = __root.kreuzberg_chunking_config_max_characters;
    pub const kreuzberg_chunking_config_overlap = __root.kreuzberg_chunking_config_overlap;
    pub const kreuzberg_chunking_config_trim = __root.kreuzberg_chunking_config_trim;
    pub const kreuzberg_chunking_config_chunker_type = __root.kreuzberg_chunking_config_chunker_type;
    pub const kreuzberg_chunking_config_embedding = __root.kreuzberg_chunking_config_embedding;
    pub const kreuzberg_chunking_config_preset = __root.kreuzberg_chunking_config_preset;
    pub const kreuzberg_chunking_config_sizing = __root.kreuzberg_chunking_config_sizing;
    pub const kreuzberg_chunking_config_prepend_heading_context = __root.kreuzberg_chunking_config_prepend_heading_context;
    pub const kreuzberg_chunking_config_topic_threshold = __root.kreuzberg_chunking_config_topic_threshold;
    pub const json = __root.kreuzberg_chunking_config_to_json;
    pub const characters = __root.kreuzberg_chunking_config_max_characters;
    pub const overlap = __root.kreuzberg_chunking_config_overlap;
    pub const trim = __root.kreuzberg_chunking_config_trim;
    pub const @"type" = __root.kreuzberg_chunking_config_chunker_type;
    pub const embedding = __root.kreuzberg_chunking_config_embedding;
    pub const preset = __root.kreuzberg_chunking_config_preset;
    pub const sizing = __root.kreuzberg_chunking_config_sizing;
    pub const context = __root.kreuzberg_chunking_config_prepend_heading_context;
    pub const threshold = __root.kreuzberg_chunking_config_topic_threshold;
};
pub const KREUZBERGChunkingConfig = struct_KREUZBERGChunkingConfig;
pub const struct_KREUZBERGChunkingResult = opaque {
    pub const kreuzberg_chunking_result_to_json = __root.kreuzberg_chunking_result_to_json;
    pub const kreuzberg_chunking_result_free = __root.kreuzberg_chunking_result_free;
    pub const kreuzberg_chunking_result_chunks = __root.kreuzberg_chunking_result_chunks;
    pub const kreuzberg_chunking_result_chunk_count = __root.kreuzberg_chunking_result_chunk_count;
    pub const json = __root.kreuzberg_chunking_result_to_json;
    pub const chunks = __root.kreuzberg_chunking_result_chunks;
    pub const count = __root.kreuzberg_chunking_result_chunk_count;
};
pub const KREUZBERGChunkingResult = struct_KREUZBERGChunkingResult;
pub const struct_KREUZBERGCitationMetadata = opaque {
    pub const kreuzberg_citation_metadata_to_json = __root.kreuzberg_citation_metadata_to_json;
    pub const kreuzberg_citation_metadata_free = __root.kreuzberg_citation_metadata_free;
    pub const kreuzberg_citation_metadata_citation_count = __root.kreuzberg_citation_metadata_citation_count;
    pub const kreuzberg_citation_metadata_format = __root.kreuzberg_citation_metadata_format;
    pub const kreuzberg_citation_metadata_authors = __root.kreuzberg_citation_metadata_authors;
    pub const kreuzberg_citation_metadata_year_range = __root.kreuzberg_citation_metadata_year_range;
    pub const kreuzberg_citation_metadata_dois = __root.kreuzberg_citation_metadata_dois;
    pub const kreuzberg_citation_metadata_keywords = __root.kreuzberg_citation_metadata_keywords;
    pub const json = __root.kreuzberg_citation_metadata_to_json;
    pub const count = __root.kreuzberg_citation_metadata_citation_count;
    pub const format = __root.kreuzberg_citation_metadata_format;
    pub const authors = __root.kreuzberg_citation_metadata_authors;
    pub const range = __root.kreuzberg_citation_metadata_year_range;
    pub const dois = __root.kreuzberg_citation_metadata_dois;
    pub const keywords = __root.kreuzberg_citation_metadata_keywords;
};
pub const KREUZBERGCitationMetadata = struct_KREUZBERGCitationMetadata;
pub const struct_KREUZBERGCodeContentMode = opaque {};
pub const KREUZBERGCodeContentMode = struct_KREUZBERGCodeContentMode;
pub const struct_KREUZBERGContentFilterConfig = opaque {
    pub const kreuzberg_content_filter_config_to_json = __root.kreuzberg_content_filter_config_to_json;
    pub const kreuzberg_content_filter_config_free = __root.kreuzberg_content_filter_config_free;
    pub const kreuzberg_content_filter_config_include_headers = __root.kreuzberg_content_filter_config_include_headers;
    pub const kreuzberg_content_filter_config_include_footers = __root.kreuzberg_content_filter_config_include_footers;
    pub const kreuzberg_content_filter_config_strip_repeating_text = __root.kreuzberg_content_filter_config_strip_repeating_text;
    pub const kreuzberg_content_filter_config_include_watermarks = __root.kreuzberg_content_filter_config_include_watermarks;
    pub const json = __root.kreuzberg_content_filter_config_to_json;
    pub const headers = __root.kreuzberg_content_filter_config_include_headers;
    pub const footers = __root.kreuzberg_content_filter_config_include_footers;
    pub const text = __root.kreuzberg_content_filter_config_strip_repeating_text;
    pub const watermarks = __root.kreuzberg_content_filter_config_include_watermarks;
};
pub const KREUZBERGContentFilterConfig = struct_KREUZBERGContentFilterConfig;
pub const struct_KREUZBERGContentLayer = opaque {};
pub const KREUZBERGContentLayer = struct_KREUZBERGContentLayer;
pub const struct_KREUZBERGContributorRole = opaque {
    pub const kreuzberg_contributor_role_to_json = __root.kreuzberg_contributor_role_to_json;
    pub const kreuzberg_contributor_role_free = __root.kreuzberg_contributor_role_free;
    pub const kreuzberg_contributor_role_name = __root.kreuzberg_contributor_role_name;
    pub const kreuzberg_contributor_role_role = __root.kreuzberg_contributor_role_role;
    pub const json = __root.kreuzberg_contributor_role_to_json;
    pub const name = __root.kreuzberg_contributor_role_name;
    pub const role = __root.kreuzberg_contributor_role_role;
};
pub const KREUZBERGContributorRole = struct_KREUZBERGContributorRole;
pub const struct_KREUZBERGCsvMetadata = opaque {
    pub const kreuzberg_csv_metadata_to_json = __root.kreuzberg_csv_metadata_to_json;
    pub const kreuzberg_csv_metadata_free = __root.kreuzberg_csv_metadata_free;
    pub const kreuzberg_csv_metadata_row_count = __root.kreuzberg_csv_metadata_row_count;
    pub const kreuzberg_csv_metadata_column_count = __root.kreuzberg_csv_metadata_column_count;
    pub const kreuzberg_csv_metadata_delimiter = __root.kreuzberg_csv_metadata_delimiter;
    pub const kreuzberg_csv_metadata_has_header = __root.kreuzberg_csv_metadata_has_header;
    pub const kreuzberg_csv_metadata_column_types = __root.kreuzberg_csv_metadata_column_types;
    pub const json = __root.kreuzberg_csv_metadata_to_json;
    pub const count = __root.kreuzberg_csv_metadata_row_count;
    pub const delimiter = __root.kreuzberg_csv_metadata_delimiter;
    pub const header = __root.kreuzberg_csv_metadata_has_header;
    pub const types = __root.kreuzberg_csv_metadata_column_types;
};
pub const KREUZBERGCsvMetadata = struct_KREUZBERGCsvMetadata;
pub const struct_KREUZBERGCustomProperties = opaque {
    pub const kreuzberg_custom_properties_free = __root.kreuzberg_custom_properties_free;
};
pub const KREUZBERGCustomProperties = struct_KREUZBERGCustomProperties;
pub const struct_KREUZBERGDbfFieldInfo = opaque {
    pub const kreuzberg_dbf_field_info_to_json = __root.kreuzberg_dbf_field_info_to_json;
    pub const kreuzberg_dbf_field_info_free = __root.kreuzberg_dbf_field_info_free;
    pub const kreuzberg_dbf_field_info_name = __root.kreuzberg_dbf_field_info_name;
    pub const kreuzberg_dbf_field_info_field_type = __root.kreuzberg_dbf_field_info_field_type;
    pub const json = __root.kreuzberg_dbf_field_info_to_json;
    pub const name = __root.kreuzberg_dbf_field_info_name;
    pub const @"type" = __root.kreuzberg_dbf_field_info_field_type;
};
pub const KREUZBERGDbfFieldInfo = struct_KREUZBERGDbfFieldInfo;
pub const struct_KREUZBERGDbfMetadata = opaque {
    pub const kreuzberg_dbf_metadata_to_json = __root.kreuzberg_dbf_metadata_to_json;
    pub const kreuzberg_dbf_metadata_free = __root.kreuzberg_dbf_metadata_free;
    pub const kreuzberg_dbf_metadata_record_count = __root.kreuzberg_dbf_metadata_record_count;
    pub const kreuzberg_dbf_metadata_field_count = __root.kreuzberg_dbf_metadata_field_count;
    pub const kreuzberg_dbf_metadata_fields = __root.kreuzberg_dbf_metadata_fields;
    pub const json = __root.kreuzberg_dbf_metadata_to_json;
    pub const count = __root.kreuzberg_dbf_metadata_record_count;
    pub const fields = __root.kreuzberg_dbf_metadata_fields;
};
pub const KREUZBERGDbfMetadata = struct_KREUZBERGDbfMetadata;
pub const struct_KREUZBERGDetectMimeTypeParams = opaque {
    pub const kreuzberg_detect_mime_type_params_to_json = __root.kreuzberg_detect_mime_type_params_to_json;
    pub const kreuzberg_detect_mime_type_params_free = __root.kreuzberg_detect_mime_type_params_free;
    pub const kreuzberg_detect_mime_type_params_path = __root.kreuzberg_detect_mime_type_params_path;
    pub const kreuzberg_detect_mime_type_params_use_content = __root.kreuzberg_detect_mime_type_params_use_content;
    pub const json = __root.kreuzberg_detect_mime_type_params_to_json;
    pub const path = __root.kreuzberg_detect_mime_type_params_path;
    pub const content = __root.kreuzberg_detect_mime_type_params_use_content;
};
pub const KREUZBERGDetectMimeTypeParams = struct_KREUZBERGDetectMimeTypeParams;
pub const struct_KREUZBERGDetectResponse = opaque {
    pub const kreuzberg_detect_response_to_json = __root.kreuzberg_detect_response_to_json;
    pub const kreuzberg_detect_response_free = __root.kreuzberg_detect_response_free;
    pub const kreuzberg_detect_response_mime_type = __root.kreuzberg_detect_response_mime_type;
    pub const kreuzberg_detect_response_filename = __root.kreuzberg_detect_response_filename;
    pub const json = __root.kreuzberg_detect_response_to_json;
    pub const @"type" = __root.kreuzberg_detect_response_mime_type;
    pub const filename = __root.kreuzberg_detect_response_filename;
};
pub const KREUZBERGDetectResponse = struct_KREUZBERGDetectResponse;
pub const struct_KREUZBERGDetectedBoundary = opaque {
    pub const kreuzberg_detected_boundary_to_json = __root.kreuzberg_detected_boundary_to_json;
    pub const kreuzberg_detected_boundary_free = __root.kreuzberg_detected_boundary_free;
    pub const kreuzberg_detected_boundary_byte_offset = __root.kreuzberg_detected_boundary_byte_offset;
    pub const kreuzberg_detected_boundary_is_header = __root.kreuzberg_detected_boundary_is_header;
    pub const json = __root.kreuzberg_detected_boundary_to_json;
    pub const offset = __root.kreuzberg_detected_boundary_byte_offset;
    pub const header = __root.kreuzberg_detected_boundary_is_header;
};
pub const KREUZBERGDetectedBoundary = struct_KREUZBERGDetectedBoundary;
pub const struct_KREUZBERGDetectionResult = opaque {
    pub const kreuzberg_detection_result_to_json = __root.kreuzberg_detection_result_to_json;
    pub const kreuzberg_detection_result_free = __root.kreuzberg_detection_result_free;
    pub const kreuzberg_detection_result_page_width = __root.kreuzberg_detection_result_page_width;
    pub const kreuzberg_detection_result_page_height = __root.kreuzberg_detection_result_page_height;
    pub const kreuzberg_detection_result_detections = __root.kreuzberg_detection_result_detections;
    pub const json = __root.kreuzberg_detection_result_to_json;
    pub const width = __root.kreuzberg_detection_result_page_width;
    pub const height = __root.kreuzberg_detection_result_page_height;
    pub const detections = __root.kreuzberg_detection_result_detections;
};
pub const KREUZBERGDetectionResult = struct_KREUZBERGDetectionResult;
pub const struct_KREUZBERGDjotContent = opaque {
    pub const kreuzberg_djot_content_to_json = __root.kreuzberg_djot_content_to_json;
    pub const kreuzberg_djot_content_free = __root.kreuzberg_djot_content_free;
    pub const kreuzberg_djot_content_plain_text = __root.kreuzberg_djot_content_plain_text;
    pub const kreuzberg_djot_content_blocks = __root.kreuzberg_djot_content_blocks;
    pub const kreuzberg_djot_content_metadata = __root.kreuzberg_djot_content_metadata;
    pub const kreuzberg_djot_content_tables = __root.kreuzberg_djot_content_tables;
    pub const kreuzberg_djot_content_images = __root.kreuzberg_djot_content_images;
    pub const kreuzberg_djot_content_links = __root.kreuzberg_djot_content_links;
    pub const kreuzberg_djot_content_footnotes = __root.kreuzberg_djot_content_footnotes;
    pub const json = __root.kreuzberg_djot_content_to_json;
    pub const text = __root.kreuzberg_djot_content_plain_text;
    pub const blocks = __root.kreuzberg_djot_content_blocks;
    pub const metadata = __root.kreuzberg_djot_content_metadata;
    pub const tables = __root.kreuzberg_djot_content_tables;
    pub const images = __root.kreuzberg_djot_content_images;
    pub const links = __root.kreuzberg_djot_content_links;
    pub const footnotes = __root.kreuzberg_djot_content_footnotes;
};
pub const KREUZBERGDjotContent = struct_KREUZBERGDjotContent;
pub const struct_KREUZBERGDjotImage = opaque {
    pub const kreuzberg_djot_image_to_json = __root.kreuzberg_djot_image_to_json;
    pub const kreuzberg_djot_image_free = __root.kreuzberg_djot_image_free;
    pub const kreuzberg_djot_image_src = __root.kreuzberg_djot_image_src;
    pub const kreuzberg_djot_image_alt = __root.kreuzberg_djot_image_alt;
    pub const kreuzberg_djot_image_title = __root.kreuzberg_djot_image_title;
    pub const json = __root.kreuzberg_djot_image_to_json;
    pub const src = __root.kreuzberg_djot_image_src;
    pub const alt = __root.kreuzberg_djot_image_alt;
    pub const title = __root.kreuzberg_djot_image_title;
};
pub const KREUZBERGDjotImage = struct_KREUZBERGDjotImage;
pub const struct_KREUZBERGDjotLink = opaque {
    pub const kreuzberg_djot_link_to_json = __root.kreuzberg_djot_link_to_json;
    pub const kreuzberg_djot_link_free = __root.kreuzberg_djot_link_free;
    pub const kreuzberg_djot_link_url = __root.kreuzberg_djot_link_url;
    pub const kreuzberg_djot_link_text = __root.kreuzberg_djot_link_text;
    pub const kreuzberg_djot_link_title = __root.kreuzberg_djot_link_title;
    pub const json = __root.kreuzberg_djot_link_to_json;
    pub const url = __root.kreuzberg_djot_link_url;
    pub const text = __root.kreuzberg_djot_link_text;
    pub const title = __root.kreuzberg_djot_link_title;
};
pub const KREUZBERGDjotLink = struct_KREUZBERGDjotLink;
pub const struct_KREUZBERGDoclingCompatResponse = opaque {
    pub const kreuzberg_docling_compat_response_to_json = __root.kreuzberg_docling_compat_response_to_json;
    pub const kreuzberg_docling_compat_response_free = __root.kreuzberg_docling_compat_response_free;
    pub const kreuzberg_docling_compat_response_status = __root.kreuzberg_docling_compat_response_status;
    pub const json = __root.kreuzberg_docling_compat_response_to_json;
    pub const status = __root.kreuzberg_docling_compat_response_status;
};
pub const KREUZBERGDoclingCompatResponse = struct_KREUZBERGDoclingCompatResponse;
pub const struct_KREUZBERGDocumentExtractor = opaque {};
pub const KREUZBERGDocumentExtractor = struct_KREUZBERGDocumentExtractor;
pub const struct_KREUZBERGDocumentNode = opaque {
    pub const kreuzberg_document_node_to_json = __root.kreuzberg_document_node_to_json;
    pub const kreuzberg_document_node_free = __root.kreuzberg_document_node_free;
    pub const kreuzberg_document_node_content = __root.kreuzberg_document_node_content;
    pub const kreuzberg_document_node_parent = __root.kreuzberg_document_node_parent;
    pub const kreuzberg_document_node_children = __root.kreuzberg_document_node_children;
    pub const kreuzberg_document_node_content_layer = __root.kreuzberg_document_node_content_layer;
    pub const kreuzberg_document_node_page = __root.kreuzberg_document_node_page;
    pub const kreuzberg_document_node_page_end = __root.kreuzberg_document_node_page_end;
    pub const kreuzberg_document_node_annotations = __root.kreuzberg_document_node_annotations;
    pub const kreuzberg_document_node_attributes = __root.kreuzberg_document_node_attributes;
    pub const json = __root.kreuzberg_document_node_to_json;
    pub const content = __root.kreuzberg_document_node_content;
    pub const parent = __root.kreuzberg_document_node_parent;
    pub const children = __root.kreuzberg_document_node_children;
    pub const layer = __root.kreuzberg_document_node_content_layer;
    pub const page = __root.kreuzberg_document_node_page;
    pub const end = __root.kreuzberg_document_node_page_end;
    pub const annotations = __root.kreuzberg_document_node_annotations;
    pub const attributes = __root.kreuzberg_document_node_attributes;
};
pub const KREUZBERGDocumentNode = struct_KREUZBERGDocumentNode;
pub const struct_KREUZBERGDocumentRelationship = opaque {
    pub const kreuzberg_document_relationship_to_json = __root.kreuzberg_document_relationship_to_json;
    pub const kreuzberg_document_relationship_free = __root.kreuzberg_document_relationship_free;
    pub const kreuzberg_document_relationship_source = __root.kreuzberg_document_relationship_source;
    pub const kreuzberg_document_relationship_target = __root.kreuzberg_document_relationship_target;
    pub const kreuzberg_document_relationship_kind = __root.kreuzberg_document_relationship_kind;
    pub const json = __root.kreuzberg_document_relationship_to_json;
    pub const source = __root.kreuzberg_document_relationship_source;
    pub const target = __root.kreuzberg_document_relationship_target;
    pub const kind = __root.kreuzberg_document_relationship_kind;
};
pub const KREUZBERGDocumentRelationship = struct_KREUZBERGDocumentRelationship;
pub const struct_KREUZBERGDocumentStructure = opaque {
    pub const kreuzberg_document_structure_to_json = __root.kreuzberg_document_structure_to_json;
    pub const kreuzberg_document_structure_free = __root.kreuzberg_document_structure_free;
    pub const kreuzberg_document_structure_nodes = __root.kreuzberg_document_structure_nodes;
    pub const kreuzberg_document_structure_source_format = __root.kreuzberg_document_structure_source_format;
    pub const kreuzberg_document_structure_relationships = __root.kreuzberg_document_structure_relationships;
    pub const kreuzberg_document_structure_node_types = __root.kreuzberg_document_structure_node_types;
    pub const kreuzberg_document_structure_finalize_node_types = __root.kreuzberg_document_structure_finalize_node_types;
    pub const json = __root.kreuzberg_document_structure_to_json;
    pub const nodes = __root.kreuzberg_document_structure_nodes;
    pub const format = __root.kreuzberg_document_structure_source_format;
    pub const relationships = __root.kreuzberg_document_structure_relationships;
    pub const types = __root.kreuzberg_document_structure_node_types;
};
pub const KREUZBERGDocumentStructure = struct_KREUZBERGDocumentStructure;
pub const struct_KREUZBERGDocxMetadata = opaque {
    pub const kreuzberg_docx_metadata_to_json = __root.kreuzberg_docx_metadata_to_json;
    pub const kreuzberg_docx_metadata_free = __root.kreuzberg_docx_metadata_free;
    pub const kreuzberg_docx_metadata_custom_properties = __root.kreuzberg_docx_metadata_custom_properties;
    pub const json = __root.kreuzberg_docx_metadata_to_json;
    pub const properties = __root.kreuzberg_docx_metadata_custom_properties;
};
pub const KREUZBERGDocxMetadata = struct_KREUZBERGDocxMetadata;
pub const struct_KREUZBERGDrawing = opaque {
    pub const kreuzberg_drawing_to_json = __root.kreuzberg_drawing_to_json;
    pub const kreuzberg_drawing_free = __root.kreuzberg_drawing_free;
    pub const kreuzberg_drawing_image_ref = __root.kreuzberg_drawing_image_ref;
    pub const json = __root.kreuzberg_drawing_to_json;
    pub const ref = __root.kreuzberg_drawing_image_ref;
};
pub const KREUZBERGDrawing = struct_KREUZBERGDrawing;
pub const struct_KREUZBERGElement = opaque {
    pub const kreuzberg_element_to_json = __root.kreuzberg_element_to_json;
    pub const kreuzberg_element_free = __root.kreuzberg_element_free;
    pub const kreuzberg_element_element_type = __root.kreuzberg_element_element_type;
    pub const kreuzberg_element_text = __root.kreuzberg_element_text;
    pub const kreuzberg_element_metadata = __root.kreuzberg_element_metadata;
    pub const json = __root.kreuzberg_element_to_json;
    pub const @"type" = __root.kreuzberg_element_element_type;
    pub const text = __root.kreuzberg_element_text;
    pub const metadata = __root.kreuzberg_element_metadata;
};
pub const KREUZBERGElement = struct_KREUZBERGElement;
pub const struct_KREUZBERGElementMetadata = opaque {
    pub const kreuzberg_element_metadata_to_json = __root.kreuzberg_element_metadata_to_json;
    pub const kreuzberg_element_metadata_free = __root.kreuzberg_element_metadata_free;
    pub const kreuzberg_element_metadata_page_number = __root.kreuzberg_element_metadata_page_number;
    pub const kreuzberg_element_metadata_filename = __root.kreuzberg_element_metadata_filename;
    pub const kreuzberg_element_metadata_element_index = __root.kreuzberg_element_metadata_element_index;
    pub const kreuzberg_element_metadata_additional = __root.kreuzberg_element_metadata_additional;
    pub const json = __root.kreuzberg_element_metadata_to_json;
    pub const number = __root.kreuzberg_element_metadata_page_number;
    pub const filename = __root.kreuzberg_element_metadata_filename;
    pub const index = __root.kreuzberg_element_metadata_element_index;
    pub const additional = __root.kreuzberg_element_metadata_additional;
};
pub const KREUZBERGElementMetadata = struct_KREUZBERGElementMetadata;
pub const struct_KREUZBERGElementType = opaque {};
pub const KREUZBERGElementType = struct_KREUZBERGElementType;
pub const struct_KREUZBERGEmailAttachment = opaque {
    pub const kreuzberg_email_attachment_to_json = __root.kreuzberg_email_attachment_to_json;
    pub const kreuzberg_email_attachment_free = __root.kreuzberg_email_attachment_free;
    pub const kreuzberg_email_attachment_name = __root.kreuzberg_email_attachment_name;
    pub const kreuzberg_email_attachment_filename = __root.kreuzberg_email_attachment_filename;
    pub const kreuzberg_email_attachment_mime_type = __root.kreuzberg_email_attachment_mime_type;
    pub const kreuzberg_email_attachment_size = __root.kreuzberg_email_attachment_size;
    pub const kreuzberg_email_attachment_is_image = __root.kreuzberg_email_attachment_is_image;
    pub const kreuzberg_email_attachment_data = __root.kreuzberg_email_attachment_data;
    pub const json = __root.kreuzberg_email_attachment_to_json;
    pub const name = __root.kreuzberg_email_attachment_name;
    pub const filename = __root.kreuzberg_email_attachment_filename;
    pub const @"type" = __root.kreuzberg_email_attachment_mime_type;
    pub const size = __root.kreuzberg_email_attachment_size;
    pub const image = __root.kreuzberg_email_attachment_is_image;
    pub const data = __root.kreuzberg_email_attachment_data;
};
pub const KREUZBERGEmailAttachment = struct_KREUZBERGEmailAttachment;
pub const struct_KREUZBERGEmailConfig = opaque {
    pub const kreuzberg_email_config_to_json = __root.kreuzberg_email_config_to_json;
    pub const kreuzberg_email_config_free = __root.kreuzberg_email_config_free;
    pub const kreuzberg_email_config_msg_fallback_codepage = __root.kreuzberg_email_config_msg_fallback_codepage;
    pub const json = __root.kreuzberg_email_config_to_json;
    pub const codepage = __root.kreuzberg_email_config_msg_fallback_codepage;
};
pub const KREUZBERGEmailConfig = struct_KREUZBERGEmailConfig;
pub const struct_KREUZBERGEmailExtractionResult = opaque {
    pub const kreuzberg_email_extraction_result_to_json = __root.kreuzberg_email_extraction_result_to_json;
    pub const kreuzberg_email_extraction_result_free = __root.kreuzberg_email_extraction_result_free;
    pub const kreuzberg_email_extraction_result_subject = __root.kreuzberg_email_extraction_result_subject;
    pub const kreuzberg_email_extraction_result_from_email = __root.kreuzberg_email_extraction_result_from_email;
    pub const kreuzberg_email_extraction_result_to_emails = __root.kreuzberg_email_extraction_result_to_emails;
    pub const kreuzberg_email_extraction_result_cc_emails = __root.kreuzberg_email_extraction_result_cc_emails;
    pub const kreuzberg_email_extraction_result_bcc_emails = __root.kreuzberg_email_extraction_result_bcc_emails;
    pub const kreuzberg_email_extraction_result_date = __root.kreuzberg_email_extraction_result_date;
    pub const kreuzberg_email_extraction_result_message_id = __root.kreuzberg_email_extraction_result_message_id;
    pub const kreuzberg_email_extraction_result_plain_text = __root.kreuzberg_email_extraction_result_plain_text;
    pub const kreuzberg_email_extraction_result_html_content = __root.kreuzberg_email_extraction_result_html_content;
    pub const kreuzberg_email_extraction_result_content = __root.kreuzberg_email_extraction_result_content;
    pub const kreuzberg_email_extraction_result_attachments = __root.kreuzberg_email_extraction_result_attachments;
    pub const kreuzberg_email_extraction_result_metadata = __root.kreuzberg_email_extraction_result_metadata;
    pub const json = __root.kreuzberg_email_extraction_result_to_json;
    pub const subject = __root.kreuzberg_email_extraction_result_subject;
    pub const email = __root.kreuzberg_email_extraction_result_from_email;
    pub const emails = __root.kreuzberg_email_extraction_result_to_emails;
    pub const date = __root.kreuzberg_email_extraction_result_date;
    pub const id = __root.kreuzberg_email_extraction_result_message_id;
    pub const text = __root.kreuzberg_email_extraction_result_plain_text;
    pub const content = __root.kreuzberg_email_extraction_result_html_content;
    pub const attachments = __root.kreuzberg_email_extraction_result_attachments;
    pub const metadata = __root.kreuzberg_email_extraction_result_metadata;
};
pub const KREUZBERGEmailExtractionResult = struct_KREUZBERGEmailExtractionResult;
pub const struct_KREUZBERGEmailMetadata = opaque {
    pub const kreuzberg_email_metadata_to_json = __root.kreuzberg_email_metadata_to_json;
    pub const kreuzberg_email_metadata_free = __root.kreuzberg_email_metadata_free;
    pub const kreuzberg_email_metadata_from_email = __root.kreuzberg_email_metadata_from_email;
    pub const kreuzberg_email_metadata_from_name = __root.kreuzberg_email_metadata_from_name;
    pub const kreuzberg_email_metadata_to_emails = __root.kreuzberg_email_metadata_to_emails;
    pub const kreuzberg_email_metadata_cc_emails = __root.kreuzberg_email_metadata_cc_emails;
    pub const kreuzberg_email_metadata_bcc_emails = __root.kreuzberg_email_metadata_bcc_emails;
    pub const kreuzberg_email_metadata_message_id = __root.kreuzberg_email_metadata_message_id;
    pub const kreuzberg_email_metadata_attachments = __root.kreuzberg_email_metadata_attachments;
    pub const json = __root.kreuzberg_email_metadata_to_json;
    pub const email = __root.kreuzberg_email_metadata_from_email;
    pub const name = __root.kreuzberg_email_metadata_from_name;
    pub const emails = __root.kreuzberg_email_metadata_to_emails;
    pub const id = __root.kreuzberg_email_metadata_message_id;
    pub const attachments = __root.kreuzberg_email_metadata_attachments;
};
pub const KREUZBERGEmailMetadata = struct_KREUZBERGEmailMetadata;
pub const struct_KREUZBERGEmbedRequest = opaque {
    pub const kreuzberg_embed_request_to_json = __root.kreuzberg_embed_request_to_json;
    pub const kreuzberg_embed_request_free = __root.kreuzberg_embed_request_free;
    pub const kreuzberg_embed_request_texts = __root.kreuzberg_embed_request_texts;
    pub const kreuzberg_embed_request_config = __root.kreuzberg_embed_request_config;
    pub const json = __root.kreuzberg_embed_request_to_json;
    pub const texts = __root.kreuzberg_embed_request_texts;
    pub const config = __root.kreuzberg_embed_request_config;
};
pub const KREUZBERGEmbedRequest = struct_KREUZBERGEmbedRequest;
pub const struct_KREUZBERGEmbedResponse = opaque {
    pub const kreuzberg_embed_response_to_json = __root.kreuzberg_embed_response_to_json;
    pub const kreuzberg_embed_response_free = __root.kreuzberg_embed_response_free;
    pub const kreuzberg_embed_response_embeddings = __root.kreuzberg_embed_response_embeddings;
    pub const kreuzberg_embed_response_model = __root.kreuzberg_embed_response_model;
    pub const kreuzberg_embed_response_dimensions = __root.kreuzberg_embed_response_dimensions;
    pub const kreuzberg_embed_response_count = __root.kreuzberg_embed_response_count;
    pub const json = __root.kreuzberg_embed_response_to_json;
    pub const embeddings = __root.kreuzberg_embed_response_embeddings;
    pub const model = __root.kreuzberg_embed_response_model;
    pub const dimensions = __root.kreuzberg_embed_response_dimensions;
    pub const count = __root.kreuzberg_embed_response_count;
};
pub const KREUZBERGEmbedResponse = struct_KREUZBERGEmbedResponse;
pub const struct_KREUZBERGEmbedTextParams = opaque {
    pub const kreuzberg_embed_text_params_to_json = __root.kreuzberg_embed_text_params_to_json;
    pub const kreuzberg_embed_text_params_free = __root.kreuzberg_embed_text_params_free;
    pub const kreuzberg_embed_text_params_texts = __root.kreuzberg_embed_text_params_texts;
    pub const kreuzberg_embed_text_params_preset = __root.kreuzberg_embed_text_params_preset;
    pub const kreuzberg_embed_text_params_model = __root.kreuzberg_embed_text_params_model;
    pub const kreuzberg_embed_text_params_api_key = __root.kreuzberg_embed_text_params_api_key;
    pub const kreuzberg_embed_text_params_embedding_plugin = __root.kreuzberg_embed_text_params_embedding_plugin;
    pub const json = __root.kreuzberg_embed_text_params_to_json;
    pub const texts = __root.kreuzberg_embed_text_params_texts;
    pub const preset = __root.kreuzberg_embed_text_params_preset;
    pub const model = __root.kreuzberg_embed_text_params_model;
    pub const key = __root.kreuzberg_embed_text_params_api_key;
    pub const plugin = __root.kreuzberg_embed_text_params_embedding_plugin;
};
pub const KREUZBERGEmbedTextParams = struct_KREUZBERGEmbedTextParams;
pub const struct_KREUZBERGEmbeddedFile = opaque {
    pub const kreuzberg_embedded_file_free = __root.kreuzberg_embedded_file_free;
    pub const kreuzberg_embedded_file_name = __root.kreuzberg_embedded_file_name;
    pub const kreuzberg_embedded_file_data = __root.kreuzberg_embedded_file_data;
    pub const kreuzberg_embedded_file_mime_type = __root.kreuzberg_embedded_file_mime_type;
    pub const name = __root.kreuzberg_embedded_file_name;
    pub const data = __root.kreuzberg_embedded_file_data;
    pub const @"type" = __root.kreuzberg_embedded_file_mime_type;
};
pub const KREUZBERGEmbeddedFile = struct_KREUZBERGEmbeddedFile;
pub const struct_KREUZBERGEmbeddingBackend = opaque {};
pub const KREUZBERGEmbeddingBackend = struct_KREUZBERGEmbeddingBackend;
pub const struct_KREUZBERGEmbeddingConfig = opaque {
    pub const kreuzberg_embedding_config_to_json = __root.kreuzberg_embedding_config_to_json;
    pub const kreuzberg_embedding_config_free = __root.kreuzberg_embedding_config_free;
    pub const kreuzberg_embedding_config_model = __root.kreuzberg_embedding_config_model;
    pub const kreuzberg_embedding_config_normalize = __root.kreuzberg_embedding_config_normalize;
    pub const kreuzberg_embedding_config_batch_size = __root.kreuzberg_embedding_config_batch_size;
    pub const kreuzberg_embedding_config_show_download_progress = __root.kreuzberg_embedding_config_show_download_progress;
    pub const kreuzberg_embedding_config_cache_dir = __root.kreuzberg_embedding_config_cache_dir;
    pub const kreuzberg_embedding_config_acceleration = __root.kreuzberg_embedding_config_acceleration;
    pub const kreuzberg_embedding_config_max_embed_duration_secs = __root.kreuzberg_embedding_config_max_embed_duration_secs;
    pub const json = __root.kreuzberg_embedding_config_to_json;
    pub const model = __root.kreuzberg_embedding_config_model;
    pub const normalize = __root.kreuzberg_embedding_config_normalize;
    pub const size = __root.kreuzberg_embedding_config_batch_size;
    pub const progress = __root.kreuzberg_embedding_config_show_download_progress;
    pub const dir = __root.kreuzberg_embedding_config_cache_dir;
    pub const acceleration = __root.kreuzberg_embedding_config_acceleration;
    pub const secs = __root.kreuzberg_embedding_config_max_embed_duration_secs;
};
pub const KREUZBERGEmbeddingConfig = struct_KREUZBERGEmbeddingConfig;
pub const struct_KREUZBERGEmbeddingModelType = opaque {};
pub const KREUZBERGEmbeddingModelType = struct_KREUZBERGEmbeddingModelType;
pub const struct_KREUZBERGEmbeddingPreset = opaque {
    pub const kreuzberg_embedding_preset_to_json = __root.kreuzberg_embedding_preset_to_json;
    pub const kreuzberg_embedding_preset_free = __root.kreuzberg_embedding_preset_free;
    pub const kreuzberg_embedding_preset_name = __root.kreuzberg_embedding_preset_name;
    pub const kreuzberg_embedding_preset_chunk_size = __root.kreuzberg_embedding_preset_chunk_size;
    pub const kreuzberg_embedding_preset_overlap = __root.kreuzberg_embedding_preset_overlap;
    pub const kreuzberg_embedding_preset_model_repo = __root.kreuzberg_embedding_preset_model_repo;
    pub const kreuzberg_embedding_preset_pooling = __root.kreuzberg_embedding_preset_pooling;
    pub const kreuzberg_embedding_preset_model_file = __root.kreuzberg_embedding_preset_model_file;
    pub const kreuzberg_embedding_preset_dimensions = __root.kreuzberg_embedding_preset_dimensions;
    pub const kreuzberg_embedding_preset_description = __root.kreuzberg_embedding_preset_description;
    pub const json = __root.kreuzberg_embedding_preset_to_json;
    pub const name = __root.kreuzberg_embedding_preset_name;
    pub const size = __root.kreuzberg_embedding_preset_chunk_size;
    pub const overlap = __root.kreuzberg_embedding_preset_overlap;
    pub const repo = __root.kreuzberg_embedding_preset_model_repo;
    pub const pooling = __root.kreuzberg_embedding_preset_pooling;
    pub const file = __root.kreuzberg_embedding_preset_model_file;
    pub const dimensions = __root.kreuzberg_embedding_preset_dimensions;
    pub const description = __root.kreuzberg_embedding_preset_description;
};
pub const KREUZBERGEmbeddingPreset = struct_KREUZBERGEmbeddingPreset;
pub const struct_KREUZBERGEpubMetadata = opaque {
    pub const kreuzberg_epub_metadata_to_json = __root.kreuzberg_epub_metadata_to_json;
    pub const kreuzberg_epub_metadata_free = __root.kreuzberg_epub_metadata_free;
    pub const kreuzberg_epub_metadata_coverage = __root.kreuzberg_epub_metadata_coverage;
    pub const kreuzberg_epub_metadata_dc_format = __root.kreuzberg_epub_metadata_dc_format;
    pub const kreuzberg_epub_metadata_relation = __root.kreuzberg_epub_metadata_relation;
    pub const kreuzberg_epub_metadata_source = __root.kreuzberg_epub_metadata_source;
    pub const kreuzberg_epub_metadata_dc_type = __root.kreuzberg_epub_metadata_dc_type;
    pub const kreuzberg_epub_metadata_cover_image = __root.kreuzberg_epub_metadata_cover_image;
    pub const json = __root.kreuzberg_epub_metadata_to_json;
    pub const coverage = __root.kreuzberg_epub_metadata_coverage;
    pub const format = __root.kreuzberg_epub_metadata_dc_format;
    pub const relation = __root.kreuzberg_epub_metadata_relation;
    pub const source = __root.kreuzberg_epub_metadata_source;
    pub const @"type" = __root.kreuzberg_epub_metadata_dc_type;
    pub const image = __root.kreuzberg_epub_metadata_cover_image;
};
pub const KREUZBERGEpubMetadata = struct_KREUZBERGEpubMetadata;
pub const struct_KREUZBERGErrorMetadata = opaque {
    pub const kreuzberg_error_metadata_to_json = __root.kreuzberg_error_metadata_to_json;
    pub const kreuzberg_error_metadata_free = __root.kreuzberg_error_metadata_free;
    pub const kreuzberg_error_metadata_error_type = __root.kreuzberg_error_metadata_error_type;
    pub const kreuzberg_error_metadata_message = __root.kreuzberg_error_metadata_message;
    pub const json = __root.kreuzberg_error_metadata_to_json;
    pub const @"type" = __root.kreuzberg_error_metadata_error_type;
    pub const message = __root.kreuzberg_error_metadata_message;
};
pub const KREUZBERGErrorMetadata = struct_KREUZBERGErrorMetadata;
pub const struct_KREUZBERGExcelMetadata = opaque {
    pub const kreuzberg_excel_metadata_to_json = __root.kreuzberg_excel_metadata_to_json;
    pub const kreuzberg_excel_metadata_free = __root.kreuzberg_excel_metadata_free;
    pub const kreuzberg_excel_metadata_sheet_count = __root.kreuzberg_excel_metadata_sheet_count;
    pub const kreuzberg_excel_metadata_sheet_names = __root.kreuzberg_excel_metadata_sheet_names;
    pub const json = __root.kreuzberg_excel_metadata_to_json;
    pub const count = __root.kreuzberg_excel_metadata_sheet_count;
    pub const names = __root.kreuzberg_excel_metadata_sheet_names;
};
pub const KREUZBERGExcelMetadata = struct_KREUZBERGExcelMetadata;
pub const struct_KREUZBERGExcelSheet = opaque {
    pub const kreuzberg_excel_sheet_to_json = __root.kreuzberg_excel_sheet_to_json;
    pub const kreuzberg_excel_sheet_free = __root.kreuzberg_excel_sheet_free;
    pub const kreuzberg_excel_sheet_name = __root.kreuzberg_excel_sheet_name;
    pub const kreuzberg_excel_sheet_markdown = __root.kreuzberg_excel_sheet_markdown;
    pub const kreuzberg_excel_sheet_row_count = __root.kreuzberg_excel_sheet_row_count;
    pub const kreuzberg_excel_sheet_col_count = __root.kreuzberg_excel_sheet_col_count;
    pub const kreuzberg_excel_sheet_cell_count = __root.kreuzberg_excel_sheet_cell_count;
    pub const kreuzberg_excel_sheet_table_cells = __root.kreuzberg_excel_sheet_table_cells;
    pub const json = __root.kreuzberg_excel_sheet_to_json;
    pub const name = __root.kreuzberg_excel_sheet_name;
    pub const markdown = __root.kreuzberg_excel_sheet_markdown;
    pub const count = __root.kreuzberg_excel_sheet_row_count;
    pub const cells = __root.kreuzberg_excel_sheet_table_cells;
};
pub const KREUZBERGExcelSheet = struct_KREUZBERGExcelSheet;
pub const struct_KREUZBERGExcelWorkbook = opaque {
    pub const kreuzberg_excel_workbook_to_json = __root.kreuzberg_excel_workbook_to_json;
    pub const kreuzberg_excel_workbook_free = __root.kreuzberg_excel_workbook_free;
    pub const kreuzberg_excel_workbook_sheets = __root.kreuzberg_excel_workbook_sheets;
    pub const kreuzberg_excel_workbook_metadata = __root.kreuzberg_excel_workbook_metadata;
    pub const json = __root.kreuzberg_excel_workbook_to_json;
    pub const sheets = __root.kreuzberg_excel_workbook_sheets;
    pub const metadata = __root.kreuzberg_excel_workbook_metadata;
};
pub const KREUZBERGExcelWorkbook = struct_KREUZBERGExcelWorkbook;
pub const struct_KREUZBERGExecutionProviderType = opaque {};
pub const KREUZBERGExecutionProviderType = struct_KREUZBERGExecutionProviderType;
pub const struct_KREUZBERGExtractResponse = opaque {
    pub const kreuzberg_extract_response_free = __root.kreuzberg_extract_response_free;
};
pub const KREUZBERGExtractResponse = struct_KREUZBERGExtractResponse;
pub const struct_KREUZBERGExtractStructuredParams = opaque {
    pub const kreuzberg_extract_structured_params_to_json = __root.kreuzberg_extract_structured_params_to_json;
    pub const kreuzberg_extract_structured_params_free = __root.kreuzberg_extract_structured_params_free;
    pub const kreuzberg_extract_structured_params_path = __root.kreuzberg_extract_structured_params_path;
    pub const kreuzberg_extract_structured_params_schema = __root.kreuzberg_extract_structured_params_schema;
    pub const kreuzberg_extract_structured_params_model = __root.kreuzberg_extract_structured_params_model;
    pub const kreuzberg_extract_structured_params_schema_name = __root.kreuzberg_extract_structured_params_schema_name;
    pub const kreuzberg_extract_structured_params_schema_description = __root.kreuzberg_extract_structured_params_schema_description;
    pub const kreuzberg_extract_structured_params_prompt = __root.kreuzberg_extract_structured_params_prompt;
    pub const kreuzberg_extract_structured_params_api_key = __root.kreuzberg_extract_structured_params_api_key;
    pub const kreuzberg_extract_structured_params_strict = __root.kreuzberg_extract_structured_params_strict;
    pub const json = __root.kreuzberg_extract_structured_params_to_json;
    pub const path = __root.kreuzberg_extract_structured_params_path;
    pub const schema = __root.kreuzberg_extract_structured_params_schema;
    pub const model = __root.kreuzberg_extract_structured_params_model;
    pub const name = __root.kreuzberg_extract_structured_params_schema_name;
    pub const description = __root.kreuzberg_extract_structured_params_schema_description;
    pub const prompt = __root.kreuzberg_extract_structured_params_prompt;
    pub const key = __root.kreuzberg_extract_structured_params_api_key;
    pub const strict = __root.kreuzberg_extract_structured_params_strict;
};
pub const KREUZBERGExtractStructuredParams = struct_KREUZBERGExtractStructuredParams;
pub const struct_KREUZBERGExtractedImage = opaque {
    pub const kreuzberg_extracted_image_to_json = __root.kreuzberg_extracted_image_to_json;
    pub const kreuzberg_extracted_image_free = __root.kreuzberg_extracted_image_free;
    pub const kreuzberg_extracted_image_data = __root.kreuzberg_extracted_image_data;
    pub const kreuzberg_extracted_image_image_index = __root.kreuzberg_extracted_image_image_index;
    pub const kreuzberg_extracted_image_page_number = __root.kreuzberg_extracted_image_page_number;
    pub const kreuzberg_extracted_image_width = __root.kreuzberg_extracted_image_width;
    pub const kreuzberg_extracted_image_height = __root.kreuzberg_extracted_image_height;
    pub const kreuzberg_extracted_image_colorspace = __root.kreuzberg_extracted_image_colorspace;
    pub const kreuzberg_extracted_image_bits_per_component = __root.kreuzberg_extracted_image_bits_per_component;
    pub const kreuzberg_extracted_image_is_mask = __root.kreuzberg_extracted_image_is_mask;
    pub const kreuzberg_extracted_image_description = __root.kreuzberg_extracted_image_description;
    pub const kreuzberg_extracted_image_ocr_result = __root.kreuzberg_extracted_image_ocr_result;
    pub const kreuzberg_extracted_image_source_path = __root.kreuzberg_extracted_image_source_path;
    pub const kreuzberg_extracted_image_image_kind = __root.kreuzberg_extracted_image_image_kind;
    pub const kreuzberg_extracted_image_kind_confidence = __root.kreuzberg_extracted_image_kind_confidence;
    pub const kreuzberg_extracted_image_cluster_id = __root.kreuzberg_extracted_image_cluster_id;
    pub const json = __root.kreuzberg_extracted_image_to_json;
    pub const data = __root.kreuzberg_extracted_image_data;
    pub const index = __root.kreuzberg_extracted_image_image_index;
    pub const number = __root.kreuzberg_extracted_image_page_number;
    pub const width = __root.kreuzberg_extracted_image_width;
    pub const height = __root.kreuzberg_extracted_image_height;
    pub const colorspace = __root.kreuzberg_extracted_image_colorspace;
    pub const component = __root.kreuzberg_extracted_image_bits_per_component;
    pub const mask = __root.kreuzberg_extracted_image_is_mask;
    pub const description = __root.kreuzberg_extracted_image_description;
    pub const result = __root.kreuzberg_extracted_image_ocr_result;
    pub const path = __root.kreuzberg_extracted_image_source_path;
    pub const kind = __root.kreuzberg_extracted_image_image_kind;
    pub const confidence = __root.kreuzberg_extracted_image_kind_confidence;
    pub const id = __root.kreuzberg_extracted_image_cluster_id;
};
pub const KREUZBERGExtractedImage = struct_KREUZBERGExtractedImage;
pub const struct_KREUZBERGExtractedInlineImage = opaque {
    pub const kreuzberg_extracted_inline_image_to_json = __root.kreuzberg_extracted_inline_image_to_json;
    pub const kreuzberg_extracted_inline_image_free = __root.kreuzberg_extracted_inline_image_free;
    pub const kreuzberg_extracted_inline_image_data = __root.kreuzberg_extracted_inline_image_data;
    pub const kreuzberg_extracted_inline_image_format = __root.kreuzberg_extracted_inline_image_format;
    pub const kreuzberg_extracted_inline_image_filename = __root.kreuzberg_extracted_inline_image_filename;
    pub const kreuzberg_extracted_inline_image_description = __root.kreuzberg_extracted_inline_image_description;
    pub const json = __root.kreuzberg_extracted_inline_image_to_json;
    pub const data = __root.kreuzberg_extracted_inline_image_data;
    pub const format = __root.kreuzberg_extracted_inline_image_format;
    pub const filename = __root.kreuzberg_extracted_inline_image_filename;
    pub const description = __root.kreuzberg_extracted_inline_image_description;
};
pub const KREUZBERGExtractedInlineImage = struct_KREUZBERGExtractedInlineImage;
pub const struct_KREUZBERGExtractionConfig = opaque {
    pub const kreuzberg_extraction_config_to_json = __root.kreuzberg_extraction_config_to_json;
    pub const kreuzberg_extraction_config_free = __root.kreuzberg_extraction_config_free;
    pub const kreuzberg_extraction_config_use_cache = __root.kreuzberg_extraction_config_use_cache;
    pub const kreuzberg_extraction_config_enable_quality_processing = __root.kreuzberg_extraction_config_enable_quality_processing;
    pub const kreuzberg_extraction_config_ocr = __root.kreuzberg_extraction_config_ocr;
    pub const kreuzberg_extraction_config_force_ocr = __root.kreuzberg_extraction_config_force_ocr;
    pub const kreuzberg_extraction_config_force_ocr_pages = __root.kreuzberg_extraction_config_force_ocr_pages;
    pub const kreuzberg_extraction_config_disable_ocr = __root.kreuzberg_extraction_config_disable_ocr;
    pub const kreuzberg_extraction_config_chunking = __root.kreuzberg_extraction_config_chunking;
    pub const kreuzberg_extraction_config_content_filter = __root.kreuzberg_extraction_config_content_filter;
    pub const kreuzberg_extraction_config_images = __root.kreuzberg_extraction_config_images;
    pub const kreuzberg_extraction_config_pdf_options = __root.kreuzberg_extraction_config_pdf_options;
    pub const kreuzberg_extraction_config_token_reduction = __root.kreuzberg_extraction_config_token_reduction;
    pub const kreuzberg_extraction_config_language_detection = __root.kreuzberg_extraction_config_language_detection;
    pub const kreuzberg_extraction_config_pages = __root.kreuzberg_extraction_config_pages;
    pub const kreuzberg_extraction_config_keywords = __root.kreuzberg_extraction_config_keywords;
    pub const kreuzberg_extraction_config_postprocessor = __root.kreuzberg_extraction_config_postprocessor;
    pub const kreuzberg_extraction_config_html_output = __root.kreuzberg_extraction_config_html_output;
    pub const kreuzberg_extraction_config_extraction_timeout_secs = __root.kreuzberg_extraction_config_extraction_timeout_secs;
    pub const kreuzberg_extraction_config_max_concurrent_extractions = __root.kreuzberg_extraction_config_max_concurrent_extractions;
    pub const kreuzberg_extraction_config_result_format = __root.kreuzberg_extraction_config_result_format;
    pub const kreuzberg_extraction_config_security_limits = __root.kreuzberg_extraction_config_security_limits;
    pub const kreuzberg_extraction_config_output_format = __root.kreuzberg_extraction_config_output_format;
    pub const kreuzberg_extraction_config_layout = __root.kreuzberg_extraction_config_layout;
    pub const kreuzberg_extraction_config_include_document_structure = __root.kreuzberg_extraction_config_include_document_structure;
    pub const kreuzberg_extraction_config_acceleration = __root.kreuzberg_extraction_config_acceleration;
    pub const kreuzberg_extraction_config_cache_namespace = __root.kreuzberg_extraction_config_cache_namespace;
    pub const kreuzberg_extraction_config_cache_ttl_secs = __root.kreuzberg_extraction_config_cache_ttl_secs;
    pub const kreuzberg_extraction_config_email = __root.kreuzberg_extraction_config_email;
    pub const kreuzberg_extraction_config_max_archive_depth = __root.kreuzberg_extraction_config_max_archive_depth;
    pub const kreuzberg_extraction_config_tree_sitter = __root.kreuzberg_extraction_config_tree_sitter;
    pub const kreuzberg_extraction_config_structured_extraction = __root.kreuzberg_extraction_config_structured_extraction;
    pub const kreuzberg_extraction_config_needs_image_processing = __root.kreuzberg_extraction_config_needs_image_processing;
    pub const json = __root.kreuzberg_extraction_config_to_json;
    pub const cache = __root.kreuzberg_extraction_config_use_cache;
    pub const processing = __root.kreuzberg_extraction_config_enable_quality_processing;
    pub const ocr = __root.kreuzberg_extraction_config_ocr;
    pub const pages = __root.kreuzberg_extraction_config_force_ocr_pages;
    pub const chunking = __root.kreuzberg_extraction_config_chunking;
    pub const filter = __root.kreuzberg_extraction_config_content_filter;
    pub const images = __root.kreuzberg_extraction_config_images;
    pub const options = __root.kreuzberg_extraction_config_pdf_options;
    pub const reduction = __root.kreuzberg_extraction_config_token_reduction;
    pub const detection = __root.kreuzberg_extraction_config_language_detection;
    pub const keywords = __root.kreuzberg_extraction_config_keywords;
    pub const postprocessor = __root.kreuzberg_extraction_config_postprocessor;
    pub const output = __root.kreuzberg_extraction_config_html_output;
    pub const secs = __root.kreuzberg_extraction_config_extraction_timeout_secs;
    pub const extractions = __root.kreuzberg_extraction_config_max_concurrent_extractions;
    pub const format = __root.kreuzberg_extraction_config_result_format;
    pub const limits = __root.kreuzberg_extraction_config_security_limits;
    pub const layout = __root.kreuzberg_extraction_config_layout;
    pub const structure = __root.kreuzberg_extraction_config_include_document_structure;
    pub const acceleration = __root.kreuzberg_extraction_config_acceleration;
    pub const namespace = __root.kreuzberg_extraction_config_cache_namespace;
    pub const email = __root.kreuzberg_extraction_config_email;
    pub const depth = __root.kreuzberg_extraction_config_max_archive_depth;
    pub const sitter = __root.kreuzberg_extraction_config_tree_sitter;
    pub const extraction = __root.kreuzberg_extraction_config_structured_extraction;
};
pub const KREUZBERGExtractionConfig = struct_KREUZBERGExtractionConfig;
pub const struct_KREUZBERGExtractionMethod = opaque {};
pub const KREUZBERGExtractionMethod = struct_KREUZBERGExtractionMethod;
pub const struct_KREUZBERGExtractionResult = opaque {
    pub const kreuzberg_extraction_result_to_json = __root.kreuzberg_extraction_result_to_json;
    pub const kreuzberg_extraction_result_free = __root.kreuzberg_extraction_result_free;
    pub const kreuzberg_extraction_result_content = __root.kreuzberg_extraction_result_content;
    pub const kreuzberg_extraction_result_metadata = __root.kreuzberg_extraction_result_metadata;
    pub const kreuzberg_extraction_result_extraction_method = __root.kreuzberg_extraction_result_extraction_method;
    pub const kreuzberg_extraction_result_tables = __root.kreuzberg_extraction_result_tables;
    pub const kreuzberg_extraction_result_detected_languages = __root.kreuzberg_extraction_result_detected_languages;
    pub const kreuzberg_extraction_result_chunks = __root.kreuzberg_extraction_result_chunks;
    pub const kreuzberg_extraction_result_images = __root.kreuzberg_extraction_result_images;
    pub const kreuzberg_extraction_result_pages = __root.kreuzberg_extraction_result_pages;
    pub const kreuzberg_extraction_result_elements = __root.kreuzberg_extraction_result_elements;
    pub const kreuzberg_extraction_result_djot_content = __root.kreuzberg_extraction_result_djot_content;
    pub const kreuzberg_extraction_result_ocr_elements = __root.kreuzberg_extraction_result_ocr_elements;
    pub const kreuzberg_extraction_result_document = __root.kreuzberg_extraction_result_document;
    pub const kreuzberg_extraction_result_extracted_keywords = __root.kreuzberg_extraction_result_extracted_keywords;
    pub const kreuzberg_extraction_result_quality_score = __root.kreuzberg_extraction_result_quality_score;
    pub const kreuzberg_extraction_result_processing_warnings = __root.kreuzberg_extraction_result_processing_warnings;
    pub const kreuzberg_extraction_result_annotations = __root.kreuzberg_extraction_result_annotations;
    pub const kreuzberg_extraction_result_children = __root.kreuzberg_extraction_result_children;
    pub const kreuzberg_extraction_result_uris = __root.kreuzberg_extraction_result_uris;
    pub const kreuzberg_extraction_result_structured_output = __root.kreuzberg_extraction_result_structured_output;
    pub const kreuzberg_extraction_result_llm_usage = __root.kreuzberg_extraction_result_llm_usage;
    pub const kreuzberg_extraction_result_formatted_content = __root.kreuzberg_extraction_result_formatted_content;
    pub const json = __root.kreuzberg_extraction_result_to_json;
    pub const content = __root.kreuzberg_extraction_result_content;
    pub const metadata = __root.kreuzberg_extraction_result_metadata;
    pub const method = __root.kreuzberg_extraction_result_extraction_method;
    pub const tables = __root.kreuzberg_extraction_result_tables;
    pub const languages = __root.kreuzberg_extraction_result_detected_languages;
    pub const chunks = __root.kreuzberg_extraction_result_chunks;
    pub const images = __root.kreuzberg_extraction_result_images;
    pub const pages = __root.kreuzberg_extraction_result_pages;
    pub const elements = __root.kreuzberg_extraction_result_elements;
    pub const document = __root.kreuzberg_extraction_result_document;
    pub const keywords = __root.kreuzberg_extraction_result_extracted_keywords;
    pub const score = __root.kreuzberg_extraction_result_quality_score;
    pub const warnings = __root.kreuzberg_extraction_result_processing_warnings;
    pub const annotations = __root.kreuzberg_extraction_result_annotations;
    pub const children = __root.kreuzberg_extraction_result_children;
    pub const uris = __root.kreuzberg_extraction_result_uris;
    pub const output = __root.kreuzberg_extraction_result_structured_output;
    pub const usage = __root.kreuzberg_extraction_result_llm_usage;
};
pub const KREUZBERGExtractionResult = struct_KREUZBERGExtractionResult;
pub const struct_KREUZBERGFictionBookMetadata = opaque {
    pub const kreuzberg_fiction_book_metadata_to_json = __root.kreuzberg_fiction_book_metadata_to_json;
    pub const kreuzberg_fiction_book_metadata_free = __root.kreuzberg_fiction_book_metadata_free;
    pub const kreuzberg_fiction_book_metadata_genres = __root.kreuzberg_fiction_book_metadata_genres;
    pub const kreuzberg_fiction_book_metadata_sequences = __root.kreuzberg_fiction_book_metadata_sequences;
    pub const kreuzberg_fiction_book_metadata_annotation = __root.kreuzberg_fiction_book_metadata_annotation;
    pub const json = __root.kreuzberg_fiction_book_metadata_to_json;
    pub const genres = __root.kreuzberg_fiction_book_metadata_genres;
    pub const sequences = __root.kreuzberg_fiction_book_metadata_sequences;
    pub const annotation = __root.kreuzberg_fiction_book_metadata_annotation;
};
pub const KREUZBERGFictionBookMetadata = struct_KREUZBERGFictionBookMetadata;
pub const struct_KREUZBERGFileExtractionConfig = opaque {
    pub const kreuzberg_file_extraction_config_to_json = __root.kreuzberg_file_extraction_config_to_json;
    pub const kreuzberg_file_extraction_config_free = __root.kreuzberg_file_extraction_config_free;
    pub const kreuzberg_file_extraction_config_enable_quality_processing = __root.kreuzberg_file_extraction_config_enable_quality_processing;
    pub const kreuzberg_file_extraction_config_ocr = __root.kreuzberg_file_extraction_config_ocr;
    pub const kreuzberg_file_extraction_config_force_ocr = __root.kreuzberg_file_extraction_config_force_ocr;
    pub const kreuzberg_file_extraction_config_force_ocr_pages = __root.kreuzberg_file_extraction_config_force_ocr_pages;
    pub const kreuzberg_file_extraction_config_disable_ocr = __root.kreuzberg_file_extraction_config_disable_ocr;
    pub const kreuzberg_file_extraction_config_chunking = __root.kreuzberg_file_extraction_config_chunking;
    pub const kreuzberg_file_extraction_config_content_filter = __root.kreuzberg_file_extraction_config_content_filter;
    pub const kreuzberg_file_extraction_config_images = __root.kreuzberg_file_extraction_config_images;
    pub const kreuzberg_file_extraction_config_pdf_options = __root.kreuzberg_file_extraction_config_pdf_options;
    pub const kreuzberg_file_extraction_config_token_reduction = __root.kreuzberg_file_extraction_config_token_reduction;
    pub const kreuzberg_file_extraction_config_language_detection = __root.kreuzberg_file_extraction_config_language_detection;
    pub const kreuzberg_file_extraction_config_pages = __root.kreuzberg_file_extraction_config_pages;
    pub const kreuzberg_file_extraction_config_keywords = __root.kreuzberg_file_extraction_config_keywords;
    pub const kreuzberg_file_extraction_config_postprocessor = __root.kreuzberg_file_extraction_config_postprocessor;
    pub const kreuzberg_file_extraction_config_result_format = __root.kreuzberg_file_extraction_config_result_format;
    pub const kreuzberg_file_extraction_config_output_format = __root.kreuzberg_file_extraction_config_output_format;
    pub const kreuzberg_file_extraction_config_include_document_structure = __root.kreuzberg_file_extraction_config_include_document_structure;
    pub const kreuzberg_file_extraction_config_layout = __root.kreuzberg_file_extraction_config_layout;
    pub const kreuzberg_file_extraction_config_timeout_secs = __root.kreuzberg_file_extraction_config_timeout_secs;
    pub const kreuzberg_file_extraction_config_tree_sitter = __root.kreuzberg_file_extraction_config_tree_sitter;
    pub const kreuzberg_file_extraction_config_structured_extraction = __root.kreuzberg_file_extraction_config_structured_extraction;
    pub const json = __root.kreuzberg_file_extraction_config_to_json;
    pub const processing = __root.kreuzberg_file_extraction_config_enable_quality_processing;
    pub const ocr = __root.kreuzberg_file_extraction_config_ocr;
    pub const pages = __root.kreuzberg_file_extraction_config_force_ocr_pages;
    pub const chunking = __root.kreuzberg_file_extraction_config_chunking;
    pub const filter = __root.kreuzberg_file_extraction_config_content_filter;
    pub const images = __root.kreuzberg_file_extraction_config_images;
    pub const options = __root.kreuzberg_file_extraction_config_pdf_options;
    pub const reduction = __root.kreuzberg_file_extraction_config_token_reduction;
    pub const detection = __root.kreuzberg_file_extraction_config_language_detection;
    pub const keywords = __root.kreuzberg_file_extraction_config_keywords;
    pub const postprocessor = __root.kreuzberg_file_extraction_config_postprocessor;
    pub const format = __root.kreuzberg_file_extraction_config_result_format;
    pub const structure = __root.kreuzberg_file_extraction_config_include_document_structure;
    pub const layout = __root.kreuzberg_file_extraction_config_layout;
    pub const secs = __root.kreuzberg_file_extraction_config_timeout_secs;
    pub const sitter = __root.kreuzberg_file_extraction_config_tree_sitter;
    pub const extraction = __root.kreuzberg_file_extraction_config_structured_extraction;
};
pub const KREUZBERGFileExtractionConfig = struct_KREUZBERGFileExtractionConfig;
pub const struct_KREUZBERGFootnote = opaque {
    pub const kreuzberg_footnote_to_json = __root.kreuzberg_footnote_to_json;
    pub const kreuzberg_footnote_free = __root.kreuzberg_footnote_free;
    pub const kreuzberg_footnote_label = __root.kreuzberg_footnote_label;
    pub const kreuzberg_footnote_content = __root.kreuzberg_footnote_content;
    pub const json = __root.kreuzberg_footnote_to_json;
    pub const label = __root.kreuzberg_footnote_label;
    pub const content = __root.kreuzberg_footnote_content;
};
pub const KREUZBERGFootnote = struct_KREUZBERGFootnote;
pub const struct_KREUZBERGFormatMetadata = opaque {};
pub const KREUZBERGFormatMetadata = struct_KREUZBERGFormatMetadata;
pub const struct_KREUZBERGFormattedBlock = opaque {
    pub const kreuzberg_formatted_block_to_json = __root.kreuzberg_formatted_block_to_json;
    pub const kreuzberg_formatted_block_free = __root.kreuzberg_formatted_block_free;
    pub const kreuzberg_formatted_block_block_type = __root.kreuzberg_formatted_block_block_type;
    pub const kreuzberg_formatted_block_level = __root.kreuzberg_formatted_block_level;
    pub const kreuzberg_formatted_block_inline_content = __root.kreuzberg_formatted_block_inline_content;
    pub const kreuzberg_formatted_block_language = __root.kreuzberg_formatted_block_language;
    pub const kreuzberg_formatted_block_code = __root.kreuzberg_formatted_block_code;
    pub const kreuzberg_formatted_block_children = __root.kreuzberg_formatted_block_children;
    pub const json = __root.kreuzberg_formatted_block_to_json;
    pub const @"type" = __root.kreuzberg_formatted_block_block_type;
    pub const level = __root.kreuzberg_formatted_block_level;
    pub const content = __root.kreuzberg_formatted_block_inline_content;
    pub const language = __root.kreuzberg_formatted_block_language;
    pub const code = __root.kreuzberg_formatted_block_code;
    pub const children = __root.kreuzberg_formatted_block_children;
};
pub const KREUZBERGFormattedBlock = struct_KREUZBERGFormattedBlock;
pub const struct_KREUZBERGFracType = opaque {};
pub const KREUZBERGFracType = struct_KREUZBERGFracType;
pub const struct_KREUZBERGGridCell = opaque {
    pub const kreuzberg_grid_cell_to_json = __root.kreuzberg_grid_cell_to_json;
    pub const kreuzberg_grid_cell_free = __root.kreuzberg_grid_cell_free;
    pub const kreuzberg_grid_cell_content = __root.kreuzberg_grid_cell_content;
    pub const kreuzberg_grid_cell_row = __root.kreuzberg_grid_cell_row;
    pub const kreuzberg_grid_cell_col = __root.kreuzberg_grid_cell_col;
    pub const kreuzberg_grid_cell_row_span = __root.kreuzberg_grid_cell_row_span;
    pub const kreuzberg_grid_cell_col_span = __root.kreuzberg_grid_cell_col_span;
    pub const kreuzberg_grid_cell_is_header = __root.kreuzberg_grid_cell_is_header;
    pub const json = __root.kreuzberg_grid_cell_to_json;
    pub const content = __root.kreuzberg_grid_cell_content;
    pub const row = __root.kreuzberg_grid_cell_row;
    pub const col = __root.kreuzberg_grid_cell_col;
    pub const span = __root.kreuzberg_grid_cell_row_span;
    pub const header = __root.kreuzberg_grid_cell_is_header;
};
pub const KREUZBERGGridCell = struct_KREUZBERGGridCell;
pub const struct_KREUZBERGHeaderMetadata = opaque {
    pub const kreuzberg_header_metadata_to_json = __root.kreuzberg_header_metadata_to_json;
    pub const kreuzberg_header_metadata_free = __root.kreuzberg_header_metadata_free;
    pub const kreuzberg_header_metadata_level = __root.kreuzberg_header_metadata_level;
    pub const kreuzberg_header_metadata_text = __root.kreuzberg_header_metadata_text;
    pub const kreuzberg_header_metadata_id = __root.kreuzberg_header_metadata_id;
    pub const kreuzberg_header_metadata_depth = __root.kreuzberg_header_metadata_depth;
    pub const kreuzberg_header_metadata_html_offset = __root.kreuzberg_header_metadata_html_offset;
    pub const json = __root.kreuzberg_header_metadata_to_json;
    pub const level = __root.kreuzberg_header_metadata_level;
    pub const text = __root.kreuzberg_header_metadata_text;
    pub const id = __root.kreuzberg_header_metadata_id;
    pub const depth = __root.kreuzberg_header_metadata_depth;
    pub const offset = __root.kreuzberg_header_metadata_html_offset;
};
pub const KREUZBERGHeaderMetadata = struct_KREUZBERGHeaderMetadata;
pub const struct_KREUZBERGHeadingContext = opaque {
    pub const kreuzberg_heading_context_to_json = __root.kreuzberg_heading_context_to_json;
    pub const kreuzberg_heading_context_free = __root.kreuzberg_heading_context_free;
    pub const kreuzberg_heading_context_headings = __root.kreuzberg_heading_context_headings;
    pub const json = __root.kreuzberg_heading_context_to_json;
    pub const headings = __root.kreuzberg_heading_context_headings;
};
pub const KREUZBERGHeadingContext = struct_KREUZBERGHeadingContext;
pub const struct_KREUZBERGHeadingLevel = opaque {
    pub const kreuzberg_heading_level_to_json = __root.kreuzberg_heading_level_to_json;
    pub const kreuzberg_heading_level_free = __root.kreuzberg_heading_level_free;
    pub const kreuzberg_heading_level_level = __root.kreuzberg_heading_level_level;
    pub const kreuzberg_heading_level_text = __root.kreuzberg_heading_level_text;
    pub const json = __root.kreuzberg_heading_level_to_json;
    pub const level = __root.kreuzberg_heading_level_level;
    pub const text = __root.kreuzberg_heading_level_text;
};
pub const KREUZBERGHeadingLevel = struct_KREUZBERGHeadingLevel;
pub const struct_KREUZBERGHierarchicalBlock = opaque {
    pub const kreuzberg_hierarchical_block_to_json = __root.kreuzberg_hierarchical_block_to_json;
    pub const kreuzberg_hierarchical_block_free = __root.kreuzberg_hierarchical_block_free;
    pub const kreuzberg_hierarchical_block_text = __root.kreuzberg_hierarchical_block_text;
    pub const kreuzberg_hierarchical_block_font_size = __root.kreuzberg_hierarchical_block_font_size;
    pub const kreuzberg_hierarchical_block_level = __root.kreuzberg_hierarchical_block_level;
    pub const json = __root.kreuzberg_hierarchical_block_to_json;
    pub const text = __root.kreuzberg_hierarchical_block_text;
    pub const size = __root.kreuzberg_hierarchical_block_font_size;
    pub const level = __root.kreuzberg_hierarchical_block_level;
};
pub const KREUZBERGHierarchicalBlock = struct_KREUZBERGHierarchicalBlock;
pub const struct_KREUZBERGHierarchyConfig = opaque {
    pub const kreuzberg_hierarchy_config_to_json = __root.kreuzberg_hierarchy_config_to_json;
    pub const kreuzberg_hierarchy_config_free = __root.kreuzberg_hierarchy_config_free;
    pub const kreuzberg_hierarchy_config_enabled = __root.kreuzberg_hierarchy_config_enabled;
    pub const kreuzberg_hierarchy_config_k_clusters = __root.kreuzberg_hierarchy_config_k_clusters;
    pub const kreuzberg_hierarchy_config_include_bbox = __root.kreuzberg_hierarchy_config_include_bbox;
    pub const kreuzberg_hierarchy_config_ocr_coverage_threshold = __root.kreuzberg_hierarchy_config_ocr_coverage_threshold;
    pub const json = __root.kreuzberg_hierarchy_config_to_json;
    pub const enabled = __root.kreuzberg_hierarchy_config_enabled;
    pub const clusters = __root.kreuzberg_hierarchy_config_k_clusters;
    pub const bbox = __root.kreuzberg_hierarchy_config_include_bbox;
    pub const threshold = __root.kreuzberg_hierarchy_config_ocr_coverage_threshold;
};
pub const KREUZBERGHierarchyConfig = struct_KREUZBERGHierarchyConfig;
pub const struct_KREUZBERGHtmlExtractionResult = opaque {
    pub const kreuzberg_html_extraction_result_to_json = __root.kreuzberg_html_extraction_result_to_json;
    pub const kreuzberg_html_extraction_result_free = __root.kreuzberg_html_extraction_result_free;
    pub const kreuzberg_html_extraction_result_markdown = __root.kreuzberg_html_extraction_result_markdown;
    pub const kreuzberg_html_extraction_result_images = __root.kreuzberg_html_extraction_result_images;
    pub const kreuzberg_html_extraction_result_warnings = __root.kreuzberg_html_extraction_result_warnings;
    pub const json = __root.kreuzberg_html_extraction_result_to_json;
    pub const markdown = __root.kreuzberg_html_extraction_result_markdown;
    pub const images = __root.kreuzberg_html_extraction_result_images;
    pub const warnings = __root.kreuzberg_html_extraction_result_warnings;
};
pub const KREUZBERGHtmlExtractionResult = struct_KREUZBERGHtmlExtractionResult;
pub const struct_KREUZBERGHtmlMetadata = opaque {
    pub const kreuzberg_html_metadata_to_json = __root.kreuzberg_html_metadata_to_json;
    pub const kreuzberg_html_metadata_free = __root.kreuzberg_html_metadata_free;
    pub const kreuzberg_html_metadata_title = __root.kreuzberg_html_metadata_title;
    pub const kreuzberg_html_metadata_description = __root.kreuzberg_html_metadata_description;
    pub const kreuzberg_html_metadata_keywords = __root.kreuzberg_html_metadata_keywords;
    pub const kreuzberg_html_metadata_author = __root.kreuzberg_html_metadata_author;
    pub const kreuzberg_html_metadata_canonical_url = __root.kreuzberg_html_metadata_canonical_url;
    pub const kreuzberg_html_metadata_base_href = __root.kreuzberg_html_metadata_base_href;
    pub const kreuzberg_html_metadata_language = __root.kreuzberg_html_metadata_language;
    pub const kreuzberg_html_metadata_text_direction = __root.kreuzberg_html_metadata_text_direction;
    pub const kreuzberg_html_metadata_open_graph = __root.kreuzberg_html_metadata_open_graph;
    pub const kreuzberg_html_metadata_twitter_card = __root.kreuzberg_html_metadata_twitter_card;
    pub const kreuzberg_html_metadata_meta_tags = __root.kreuzberg_html_metadata_meta_tags;
    pub const kreuzberg_html_metadata_headers = __root.kreuzberg_html_metadata_headers;
    pub const kreuzberg_html_metadata_links = __root.kreuzberg_html_metadata_links;
    pub const kreuzberg_html_metadata_images = __root.kreuzberg_html_metadata_images;
    pub const kreuzberg_html_metadata_structured_data = __root.kreuzberg_html_metadata_structured_data;
    pub const kreuzberg_html_metadata_from = __root.kreuzberg_html_metadata_from;
    pub const json = __root.kreuzberg_html_metadata_to_json;
    pub const title = __root.kreuzberg_html_metadata_title;
    pub const description = __root.kreuzberg_html_metadata_description;
    pub const keywords = __root.kreuzberg_html_metadata_keywords;
    pub const author = __root.kreuzberg_html_metadata_author;
    pub const url = __root.kreuzberg_html_metadata_canonical_url;
    pub const href = __root.kreuzberg_html_metadata_base_href;
    pub const language = __root.kreuzberg_html_metadata_language;
    pub const direction = __root.kreuzberg_html_metadata_text_direction;
    pub const graph = __root.kreuzberg_html_metadata_open_graph;
    pub const card = __root.kreuzberg_html_metadata_twitter_card;
    pub const tags = __root.kreuzberg_html_metadata_meta_tags;
    pub const headers = __root.kreuzberg_html_metadata_headers;
    pub const links = __root.kreuzberg_html_metadata_links;
    pub const images = __root.kreuzberg_html_metadata_images;
    pub const data = __root.kreuzberg_html_metadata_structured_data;
    pub const from = __root.kreuzberg_html_metadata_from;
};
pub const KREUZBERGHtmlMetadata = struct_KREUZBERGHtmlMetadata;
pub const struct_KREUZBERGHtmlOutputConfig = opaque {
    pub const kreuzberg_html_output_config_to_json = __root.kreuzberg_html_output_config_to_json;
    pub const kreuzberg_html_output_config_free = __root.kreuzberg_html_output_config_free;
    pub const kreuzberg_html_output_config_css = __root.kreuzberg_html_output_config_css;
    pub const kreuzberg_html_output_config_css_file = __root.kreuzberg_html_output_config_css_file;
    pub const kreuzberg_html_output_config_theme = __root.kreuzberg_html_output_config_theme;
    pub const kreuzberg_html_output_config_class_prefix = __root.kreuzberg_html_output_config_class_prefix;
    pub const kreuzberg_html_output_config_embed_css = __root.kreuzberg_html_output_config_embed_css;
    pub const json = __root.kreuzberg_html_output_config_to_json;
    pub const css = __root.kreuzberg_html_output_config_css;
    pub const file = __root.kreuzberg_html_output_config_css_file;
    pub const theme = __root.kreuzberg_html_output_config_theme;
    pub const prefix = __root.kreuzberg_html_output_config_class_prefix;
};
pub const KREUZBERGHtmlOutputConfig = struct_KREUZBERGHtmlOutputConfig;
pub const struct_KREUZBERGHtmlTheme = opaque {};
pub const KREUZBERGHtmlTheme = struct_KREUZBERGHtmlTheme;
pub const struct_KREUZBERGHwpImage = opaque {
    pub const kreuzberg_hwp_image_free = __root.kreuzberg_hwp_image_free;
    pub const kreuzberg_hwp_image_name = __root.kreuzberg_hwp_image_name;
    pub const kreuzberg_hwp_image_data = __root.kreuzberg_hwp_image_data;
    pub const name = __root.kreuzberg_hwp_image_name;
    pub const data = __root.kreuzberg_hwp_image_data;
};
pub const KREUZBERGHwpImage = struct_KREUZBERGHwpImage;
pub const struct_KREUZBERGImageExtractionConfig = opaque {
    pub const kreuzberg_image_extraction_config_to_json = __root.kreuzberg_image_extraction_config_to_json;
    pub const kreuzberg_image_extraction_config_free = __root.kreuzberg_image_extraction_config_free;
    pub const kreuzberg_image_extraction_config_extract_images = __root.kreuzberg_image_extraction_config_extract_images;
    pub const kreuzberg_image_extraction_config_target_dpi = __root.kreuzberg_image_extraction_config_target_dpi;
    pub const kreuzberg_image_extraction_config_max_image_dimension = __root.kreuzberg_image_extraction_config_max_image_dimension;
    pub const kreuzberg_image_extraction_config_inject_placeholders = __root.kreuzberg_image_extraction_config_inject_placeholders;
    pub const kreuzberg_image_extraction_config_auto_adjust_dpi = __root.kreuzberg_image_extraction_config_auto_adjust_dpi;
    pub const kreuzberg_image_extraction_config_min_dpi = __root.kreuzberg_image_extraction_config_min_dpi;
    pub const kreuzberg_image_extraction_config_max_dpi = __root.kreuzberg_image_extraction_config_max_dpi;
    pub const kreuzberg_image_extraction_config_max_images_per_page = __root.kreuzberg_image_extraction_config_max_images_per_page;
    pub const kreuzberg_image_extraction_config_classify = __root.kreuzberg_image_extraction_config_classify;
    pub const json = __root.kreuzberg_image_extraction_config_to_json;
    pub const images = __root.kreuzberg_image_extraction_config_extract_images;
    pub const dpi = __root.kreuzberg_image_extraction_config_target_dpi;
    pub const dimension = __root.kreuzberg_image_extraction_config_max_image_dimension;
    pub const placeholders = __root.kreuzberg_image_extraction_config_inject_placeholders;
    pub const page = __root.kreuzberg_image_extraction_config_max_images_per_page;
    pub const classify = __root.kreuzberg_image_extraction_config_classify;
};
pub const KREUZBERGImageExtractionConfig = struct_KREUZBERGImageExtractionConfig;
pub const struct_KREUZBERGImageKind = opaque {};
pub const KREUZBERGImageKind = struct_KREUZBERGImageKind;
pub const struct_KREUZBERGImageMetadataType = opaque {
    pub const kreuzberg_image_metadata_type_to_json = __root.kreuzberg_image_metadata_type_to_json;
    pub const kreuzberg_image_metadata_type_free = __root.kreuzberg_image_metadata_type_free;
    pub const kreuzberg_image_metadata_type_src = __root.kreuzberg_image_metadata_type_src;
    pub const kreuzberg_image_metadata_type_alt = __root.kreuzberg_image_metadata_type_alt;
    pub const kreuzberg_image_metadata_type_title = __root.kreuzberg_image_metadata_type_title;
    pub const kreuzberg_image_metadata_type_image_type = __root.kreuzberg_image_metadata_type_image_type;
    pub const json = __root.kreuzberg_image_metadata_type_to_json;
    pub const src = __root.kreuzberg_image_metadata_type_src;
    pub const alt = __root.kreuzberg_image_metadata_type_alt;
    pub const title = __root.kreuzberg_image_metadata_type_title;
    pub const @"type" = __root.kreuzberg_image_metadata_type_image_type;
};
pub const KREUZBERGImageMetadataType = struct_KREUZBERGImageMetadataType;
pub const struct_KREUZBERGImageOcrResult = opaque {
    pub const kreuzberg_image_ocr_result_free = __root.kreuzberg_image_ocr_result_free;
    pub const kreuzberg_image_ocr_result_content = __root.kreuzberg_image_ocr_result_content;
    pub const kreuzberg_image_ocr_result_boundaries = __root.kreuzberg_image_ocr_result_boundaries;
    pub const kreuzberg_image_ocr_result_page_contents = __root.kreuzberg_image_ocr_result_page_contents;
    pub const content = __root.kreuzberg_image_ocr_result_content;
    pub const boundaries = __root.kreuzberg_image_ocr_result_boundaries;
    pub const contents = __root.kreuzberg_image_ocr_result_page_contents;
};
pub const KREUZBERGImageOcrResult = struct_KREUZBERGImageOcrResult;
pub const struct_KREUZBERGImagePreprocessingConfig = opaque {
    pub const kreuzberg_image_preprocessing_config_to_json = __root.kreuzberg_image_preprocessing_config_to_json;
    pub const kreuzberg_image_preprocessing_config_free = __root.kreuzberg_image_preprocessing_config_free;
    pub const kreuzberg_image_preprocessing_config_target_dpi = __root.kreuzberg_image_preprocessing_config_target_dpi;
    pub const kreuzberg_image_preprocessing_config_auto_rotate = __root.kreuzberg_image_preprocessing_config_auto_rotate;
    pub const kreuzberg_image_preprocessing_config_deskew = __root.kreuzberg_image_preprocessing_config_deskew;
    pub const kreuzberg_image_preprocessing_config_denoise = __root.kreuzberg_image_preprocessing_config_denoise;
    pub const kreuzberg_image_preprocessing_config_contrast_enhance = __root.kreuzberg_image_preprocessing_config_contrast_enhance;
    pub const kreuzberg_image_preprocessing_config_binarization_method = __root.kreuzberg_image_preprocessing_config_binarization_method;
    pub const kreuzberg_image_preprocessing_config_invert_colors = __root.kreuzberg_image_preprocessing_config_invert_colors;
    pub const json = __root.kreuzberg_image_preprocessing_config_to_json;
    pub const dpi = __root.kreuzberg_image_preprocessing_config_target_dpi;
    pub const rotate = __root.kreuzberg_image_preprocessing_config_auto_rotate;
    pub const deskew = __root.kreuzberg_image_preprocessing_config_deskew;
    pub const denoise = __root.kreuzberg_image_preprocessing_config_denoise;
    pub const enhance = __root.kreuzberg_image_preprocessing_config_contrast_enhance;
    pub const method = __root.kreuzberg_image_preprocessing_config_binarization_method;
    pub const colors = __root.kreuzberg_image_preprocessing_config_invert_colors;
};
pub const KREUZBERGImagePreprocessingConfig = struct_KREUZBERGImagePreprocessingConfig;
pub const struct_KREUZBERGImagePreprocessingMetadata = opaque {
    pub const kreuzberg_image_preprocessing_metadata_to_json = __root.kreuzberg_image_preprocessing_metadata_to_json;
    pub const kreuzberg_image_preprocessing_metadata_free = __root.kreuzberg_image_preprocessing_metadata_free;
    pub const kreuzberg_image_preprocessing_metadata_target_dpi = __root.kreuzberg_image_preprocessing_metadata_target_dpi;
    pub const kreuzberg_image_preprocessing_metadata_scale_factor = __root.kreuzberg_image_preprocessing_metadata_scale_factor;
    pub const kreuzberg_image_preprocessing_metadata_auto_adjusted = __root.kreuzberg_image_preprocessing_metadata_auto_adjusted;
    pub const kreuzberg_image_preprocessing_metadata_final_dpi = __root.kreuzberg_image_preprocessing_metadata_final_dpi;
    pub const kreuzberg_image_preprocessing_metadata_resample_method = __root.kreuzberg_image_preprocessing_metadata_resample_method;
    pub const kreuzberg_image_preprocessing_metadata_dimension_clamped = __root.kreuzberg_image_preprocessing_metadata_dimension_clamped;
    pub const kreuzberg_image_preprocessing_metadata_calculated_dpi = __root.kreuzberg_image_preprocessing_metadata_calculated_dpi;
    pub const kreuzberg_image_preprocessing_metadata_skipped_resize = __root.kreuzberg_image_preprocessing_metadata_skipped_resize;
    pub const kreuzberg_image_preprocessing_metadata_resize_error = __root.kreuzberg_image_preprocessing_metadata_resize_error;
    pub const json = __root.kreuzberg_image_preprocessing_metadata_to_json;
    pub const dpi = __root.kreuzberg_image_preprocessing_metadata_target_dpi;
    pub const factor = __root.kreuzberg_image_preprocessing_metadata_scale_factor;
    pub const adjusted = __root.kreuzberg_image_preprocessing_metadata_auto_adjusted;
    pub const method = __root.kreuzberg_image_preprocessing_metadata_resample_method;
    pub const clamped = __root.kreuzberg_image_preprocessing_metadata_dimension_clamped;
    pub const resize = __root.kreuzberg_image_preprocessing_metadata_skipped_resize;
    pub const @"error" = __root.kreuzberg_image_preprocessing_metadata_resize_error;
};
pub const KREUZBERGImagePreprocessingMetadata = struct_KREUZBERGImagePreprocessingMetadata;
pub const struct_KREUZBERGImageType = opaque {};
pub const KREUZBERGImageType = struct_KREUZBERGImageType;
pub const struct_KREUZBERGInfoResponse = opaque {
    pub const kreuzberg_info_response_to_json = __root.kreuzberg_info_response_to_json;
    pub const kreuzberg_info_response_free = __root.kreuzberg_info_response_free;
    pub const kreuzberg_info_response_version = __root.kreuzberg_info_response_version;
    pub const kreuzberg_info_response_rust_backend = __root.kreuzberg_info_response_rust_backend;
    pub const json = __root.kreuzberg_info_response_to_json;
    pub const version = __root.kreuzberg_info_response_version;
    pub const backend = __root.kreuzberg_info_response_rust_backend;
};
pub const KREUZBERGInfoResponse = struct_KREUZBERGInfoResponse;
pub const struct_KREUZBERGInlineElement = opaque {
    pub const kreuzberg_inline_element_to_json = __root.kreuzberg_inline_element_to_json;
    pub const kreuzberg_inline_element_free = __root.kreuzberg_inline_element_free;
    pub const kreuzberg_inline_element_element_type = __root.kreuzberg_inline_element_element_type;
    pub const kreuzberg_inline_element_content = __root.kreuzberg_inline_element_content;
    pub const kreuzberg_inline_element_metadata = __root.kreuzberg_inline_element_metadata;
    pub const json = __root.kreuzberg_inline_element_to_json;
    pub const @"type" = __root.kreuzberg_inline_element_element_type;
    pub const content = __root.kreuzberg_inline_element_content;
    pub const metadata = __root.kreuzberg_inline_element_metadata;
};
pub const KREUZBERGInlineElement = struct_KREUZBERGInlineElement;
pub const struct_KREUZBERGInlineType = opaque {};
pub const KREUZBERGInlineType = struct_KREUZBERGInlineType;
pub const struct_KREUZBERGJatsMetadata = opaque {
    pub const kreuzberg_jats_metadata_to_json = __root.kreuzberg_jats_metadata_to_json;
    pub const kreuzberg_jats_metadata_free = __root.kreuzberg_jats_metadata_free;
    pub const kreuzberg_jats_metadata_copyright = __root.kreuzberg_jats_metadata_copyright;
    pub const kreuzberg_jats_metadata_license = __root.kreuzberg_jats_metadata_license;
    pub const kreuzberg_jats_metadata_history_dates = __root.kreuzberg_jats_metadata_history_dates;
    pub const kreuzberg_jats_metadata_contributor_roles = __root.kreuzberg_jats_metadata_contributor_roles;
    pub const json = __root.kreuzberg_jats_metadata_to_json;
    pub const copyright = __root.kreuzberg_jats_metadata_copyright;
    pub const license = __root.kreuzberg_jats_metadata_license;
    pub const dates = __root.kreuzberg_jats_metadata_history_dates;
    pub const roles = __root.kreuzberg_jats_metadata_contributor_roles;
};
pub const KREUZBERGJatsMetadata = struct_KREUZBERGJatsMetadata;
pub const struct_KREUZBERGKeyword = opaque {
    pub const kreuzberg_keyword_to_json = __root.kreuzberg_keyword_to_json;
    pub const kreuzberg_keyword_free = __root.kreuzberg_keyword_free;
    pub const kreuzberg_keyword_text = __root.kreuzberg_keyword_text;
    pub const kreuzberg_keyword_score = __root.kreuzberg_keyword_score;
    pub const kreuzberg_keyword_algorithm = __root.kreuzberg_keyword_algorithm;
    pub const kreuzberg_keyword_positions = __root.kreuzberg_keyword_positions;
    pub const json = __root.kreuzberg_keyword_to_json;
    pub const text = __root.kreuzberg_keyword_text;
    pub const score = __root.kreuzberg_keyword_score;
    pub const algorithm = __root.kreuzberg_keyword_algorithm;
    pub const positions = __root.kreuzberg_keyword_positions;
};
pub const KREUZBERGKeyword = struct_KREUZBERGKeyword;
pub const struct_KREUZBERGKeywordAlgorithm = opaque {};
pub const KREUZBERGKeywordAlgorithm = struct_KREUZBERGKeywordAlgorithm;
pub const struct_KREUZBERGKeywordConfig = opaque {
    pub const kreuzberg_keyword_config_to_json = __root.kreuzberg_keyword_config_to_json;
    pub const kreuzberg_keyword_config_free = __root.kreuzberg_keyword_config_free;
    pub const kreuzberg_keyword_config_algorithm = __root.kreuzberg_keyword_config_algorithm;
    pub const kreuzberg_keyword_config_max_keywords = __root.kreuzberg_keyword_config_max_keywords;
    pub const kreuzberg_keyword_config_min_score = __root.kreuzberg_keyword_config_min_score;
    pub const kreuzberg_keyword_config_language = __root.kreuzberg_keyword_config_language;
    pub const kreuzberg_keyword_config_yake_params = __root.kreuzberg_keyword_config_yake_params;
    pub const kreuzberg_keyword_config_rake_params = __root.kreuzberg_keyword_config_rake_params;
    pub const json = __root.kreuzberg_keyword_config_to_json;
    pub const algorithm = __root.kreuzberg_keyword_config_algorithm;
    pub const keywords = __root.kreuzberg_keyword_config_max_keywords;
    pub const score = __root.kreuzberg_keyword_config_min_score;
    pub const language = __root.kreuzberg_keyword_config_language;
    pub const params = __root.kreuzberg_keyword_config_yake_params;
};
pub const KREUZBERGKeywordConfig = struct_KREUZBERGKeywordConfig;
pub const struct_KREUZBERGLanguageDetectionConfig = opaque {
    pub const kreuzberg_language_detection_config_to_json = __root.kreuzberg_language_detection_config_to_json;
    pub const kreuzberg_language_detection_config_free = __root.kreuzberg_language_detection_config_free;
    pub const kreuzberg_language_detection_config_enabled = __root.kreuzberg_language_detection_config_enabled;
    pub const kreuzberg_language_detection_config_min_confidence = __root.kreuzberg_language_detection_config_min_confidence;
    pub const kreuzberg_language_detection_config_detect_multiple = __root.kreuzberg_language_detection_config_detect_multiple;
    pub const json = __root.kreuzberg_language_detection_config_to_json;
    pub const enabled = __root.kreuzberg_language_detection_config_enabled;
    pub const confidence = __root.kreuzberg_language_detection_config_min_confidence;
    pub const multiple = __root.kreuzberg_language_detection_config_detect_multiple;
};
pub const KREUZBERGLanguageDetectionConfig = struct_KREUZBERGLanguageDetectionConfig;
pub const struct_KREUZBERGLayoutClass = opaque {};
pub const KREUZBERGLayoutClass = struct_KREUZBERGLayoutClass;
pub const struct_KREUZBERGLayoutDetection = opaque {
    pub const kreuzberg_layout_detection_to_json = __root.kreuzberg_layout_detection_to_json;
    pub const kreuzberg_layout_detection_free = __root.kreuzberg_layout_detection_free;
    pub const kreuzberg_layout_detection_class_name = __root.kreuzberg_layout_detection_class_name;
    pub const kreuzberg_layout_detection_confidence = __root.kreuzberg_layout_detection_confidence;
    pub const kreuzberg_layout_detection_bbox = __root.kreuzberg_layout_detection_bbox;
    pub const json = __root.kreuzberg_layout_detection_to_json;
    pub const name = __root.kreuzberg_layout_detection_class_name;
    pub const confidence = __root.kreuzberg_layout_detection_confidence;
    pub const bbox = __root.kreuzberg_layout_detection_bbox;
};
pub const KREUZBERGLayoutDetection = struct_KREUZBERGLayoutDetection;
pub const struct_KREUZBERGLayoutDetectionConfig = opaque {
    pub const kreuzberg_layout_detection_config_to_json = __root.kreuzberg_layout_detection_config_to_json;
    pub const kreuzberg_layout_detection_config_free = __root.kreuzberg_layout_detection_config_free;
    pub const kreuzberg_layout_detection_config_confidence_threshold = __root.kreuzberg_layout_detection_config_confidence_threshold;
    pub const kreuzberg_layout_detection_config_apply_heuristics = __root.kreuzberg_layout_detection_config_apply_heuristics;
    pub const kreuzberg_layout_detection_config_table_model = __root.kreuzberg_layout_detection_config_table_model;
    pub const kreuzberg_layout_detection_config_acceleration = __root.kreuzberg_layout_detection_config_acceleration;
    pub const json = __root.kreuzberg_layout_detection_config_to_json;
    pub const threshold = __root.kreuzberg_layout_detection_config_confidence_threshold;
    pub const heuristics = __root.kreuzberg_layout_detection_config_apply_heuristics;
    pub const model = __root.kreuzberg_layout_detection_config_table_model;
    pub const acceleration = __root.kreuzberg_layout_detection_config_acceleration;
};
pub const KREUZBERGLayoutDetectionConfig = struct_KREUZBERGLayoutDetectionConfig;
pub const struct_KREUZBERGLayoutRegion = opaque {
    pub const kreuzberg_layout_region_to_json = __root.kreuzberg_layout_region_to_json;
    pub const kreuzberg_layout_region_free = __root.kreuzberg_layout_region_free;
    pub const kreuzberg_layout_region_class_name = __root.kreuzberg_layout_region_class_name;
    pub const kreuzberg_layout_region_confidence = __root.kreuzberg_layout_region_confidence;
    pub const kreuzberg_layout_region_area_fraction = __root.kreuzberg_layout_region_area_fraction;
    pub const json = __root.kreuzberg_layout_region_to_json;
    pub const name = __root.kreuzberg_layout_region_class_name;
    pub const confidence = __root.kreuzberg_layout_region_confidence;
    pub const fraction = __root.kreuzberg_layout_region_area_fraction;
};
pub const KREUZBERGLayoutRegion = struct_KREUZBERGLayoutRegion;
pub const struct_KREUZBERGLinkMetadata = opaque {
    pub const kreuzberg_link_metadata_to_json = __root.kreuzberg_link_metadata_to_json;
    pub const kreuzberg_link_metadata_free = __root.kreuzberg_link_metadata_free;
    pub const kreuzberg_link_metadata_href = __root.kreuzberg_link_metadata_href;
    pub const kreuzberg_link_metadata_text = __root.kreuzberg_link_metadata_text;
    pub const kreuzberg_link_metadata_title = __root.kreuzberg_link_metadata_title;
    pub const kreuzberg_link_metadata_link_type = __root.kreuzberg_link_metadata_link_type;
    pub const kreuzberg_link_metadata_rel = __root.kreuzberg_link_metadata_rel;
    pub const json = __root.kreuzberg_link_metadata_to_json;
    pub const href = __root.kreuzberg_link_metadata_href;
    pub const text = __root.kreuzberg_link_metadata_text;
    pub const title = __root.kreuzberg_link_metadata_title;
    pub const @"type" = __root.kreuzberg_link_metadata_link_type;
    pub const rel = __root.kreuzberg_link_metadata_rel;
};
pub const KREUZBERGLinkMetadata = struct_KREUZBERGLinkMetadata;
pub const struct_KREUZBERGLinkType = opaque {};
pub const KREUZBERGLinkType = struct_KREUZBERGLinkType;
pub const struct_KREUZBERGLlmConfig = opaque {
    pub const kreuzberg_llm_config_to_json = __root.kreuzberg_llm_config_to_json;
    pub const kreuzberg_llm_config_free = __root.kreuzberg_llm_config_free;
    pub const kreuzberg_llm_config_model = __root.kreuzberg_llm_config_model;
    pub const kreuzberg_llm_config_api_key = __root.kreuzberg_llm_config_api_key;
    pub const kreuzberg_llm_config_base_url = __root.kreuzberg_llm_config_base_url;
    pub const kreuzberg_llm_config_timeout_secs = __root.kreuzberg_llm_config_timeout_secs;
    pub const kreuzberg_llm_config_max_retries = __root.kreuzberg_llm_config_max_retries;
    pub const kreuzberg_llm_config_temperature = __root.kreuzberg_llm_config_temperature;
    pub const kreuzberg_llm_config_max_tokens = __root.kreuzberg_llm_config_max_tokens;
    pub const json = __root.kreuzberg_llm_config_to_json;
    pub const model = __root.kreuzberg_llm_config_model;
    pub const key = __root.kreuzberg_llm_config_api_key;
    pub const url = __root.kreuzberg_llm_config_base_url;
    pub const secs = __root.kreuzberg_llm_config_timeout_secs;
    pub const retries = __root.kreuzberg_llm_config_max_retries;
    pub const temperature = __root.kreuzberg_llm_config_temperature;
    pub const tokens = __root.kreuzberg_llm_config_max_tokens;
};
pub const KREUZBERGLlmConfig = struct_KREUZBERGLlmConfig;
pub const struct_KREUZBERGLlmUsage = opaque {
    pub const kreuzberg_llm_usage_to_json = __root.kreuzberg_llm_usage_to_json;
    pub const kreuzberg_llm_usage_free = __root.kreuzberg_llm_usage_free;
    pub const kreuzberg_llm_usage_model = __root.kreuzberg_llm_usage_model;
    pub const kreuzberg_llm_usage_source = __root.kreuzberg_llm_usage_source;
    pub const kreuzberg_llm_usage_input_tokens = __root.kreuzberg_llm_usage_input_tokens;
    pub const kreuzberg_llm_usage_output_tokens = __root.kreuzberg_llm_usage_output_tokens;
    pub const kreuzberg_llm_usage_total_tokens = __root.kreuzberg_llm_usage_total_tokens;
    pub const kreuzberg_llm_usage_estimated_cost = __root.kreuzberg_llm_usage_estimated_cost;
    pub const kreuzberg_llm_usage_finish_reason = __root.kreuzberg_llm_usage_finish_reason;
    pub const json = __root.kreuzberg_llm_usage_to_json;
    pub const model = __root.kreuzberg_llm_usage_model;
    pub const source = __root.kreuzberg_llm_usage_source;
    pub const tokens = __root.kreuzberg_llm_usage_input_tokens;
    pub const cost = __root.kreuzberg_llm_usage_estimated_cost;
    pub const reason = __root.kreuzberg_llm_usage_finish_reason;
};
pub const KREUZBERGLlmUsage = struct_KREUZBERGLlmUsage;
pub const struct_KREUZBERGManifestEntryResponse = opaque {
    pub const kreuzberg_manifest_entry_response_to_json = __root.kreuzberg_manifest_entry_response_to_json;
    pub const kreuzberg_manifest_entry_response_free = __root.kreuzberg_manifest_entry_response_free;
    pub const kreuzberg_manifest_entry_response_relative_path = __root.kreuzberg_manifest_entry_response_relative_path;
    pub const kreuzberg_manifest_entry_response_sha256 = __root.kreuzberg_manifest_entry_response_sha256;
    pub const kreuzberg_manifest_entry_response_size_bytes = __root.kreuzberg_manifest_entry_response_size_bytes;
    pub const kreuzberg_manifest_entry_response_source_url = __root.kreuzberg_manifest_entry_response_source_url;
    pub const json = __root.kreuzberg_manifest_entry_response_to_json;
    pub const path = __root.kreuzberg_manifest_entry_response_relative_path;
    pub const sha256 = __root.kreuzberg_manifest_entry_response_sha256;
    pub const bytes = __root.kreuzberg_manifest_entry_response_size_bytes;
    pub const url = __root.kreuzberg_manifest_entry_response_source_url;
};
pub const KREUZBERGManifestEntryResponse = struct_KREUZBERGManifestEntryResponse;
pub const struct_KREUZBERGManifestResponse = opaque {
    pub const kreuzberg_manifest_response_to_json = __root.kreuzberg_manifest_response_to_json;
    pub const kreuzberg_manifest_response_free = __root.kreuzberg_manifest_response_free;
    pub const kreuzberg_manifest_response_kreuzberg_version = __root.kreuzberg_manifest_response_kreuzberg_version;
    pub const kreuzberg_manifest_response_total_size_bytes = __root.kreuzberg_manifest_response_total_size_bytes;
    pub const kreuzberg_manifest_response_model_count = __root.kreuzberg_manifest_response_model_count;
    pub const kreuzberg_manifest_response_models = __root.kreuzberg_manifest_response_models;
    pub const json = __root.kreuzberg_manifest_response_to_json;
    pub const version = __root.kreuzberg_manifest_response_kreuzberg_version;
    pub const bytes = __root.kreuzberg_manifest_response_total_size_bytes;
    pub const count = __root.kreuzberg_manifest_response_model_count;
    pub const models = __root.kreuzberg_manifest_response_models;
};
pub const KREUZBERGManifestResponse = struct_KREUZBERGManifestResponse;
pub const struct_KREUZBERGMergedChunk = opaque {
    pub const kreuzberg_merged_chunk_free = __root.kreuzberg_merged_chunk_free;
    pub const kreuzberg_merged_chunk_text = __root.kreuzberg_merged_chunk_text;
    pub const kreuzberg_merged_chunk_byte_start = __root.kreuzberg_merged_chunk_byte_start;
    pub const kreuzberg_merged_chunk_byte_end = __root.kreuzberg_merged_chunk_byte_end;
    pub const text = __root.kreuzberg_merged_chunk_text;
    pub const start = __root.kreuzberg_merged_chunk_byte_start;
    pub const end = __root.kreuzberg_merged_chunk_byte_end;
};
pub const KREUZBERGMergedChunk = struct_KREUZBERGMergedChunk;
pub const struct_KREUZBERGMetadata = opaque {
    pub const kreuzberg_metadata_to_json = __root.kreuzberg_metadata_to_json;
    pub const kreuzberg_metadata_free = __root.kreuzberg_metadata_free;
    pub const kreuzberg_metadata_title = __root.kreuzberg_metadata_title;
    pub const kreuzberg_metadata_subject = __root.kreuzberg_metadata_subject;
    pub const kreuzberg_metadata_authors = __root.kreuzberg_metadata_authors;
    pub const kreuzberg_metadata_keywords = __root.kreuzberg_metadata_keywords;
    pub const kreuzberg_metadata_language = __root.kreuzberg_metadata_language;
    pub const kreuzberg_metadata_created_at = __root.kreuzberg_metadata_created_at;
    pub const kreuzberg_metadata_modified_at = __root.kreuzberg_metadata_modified_at;
    pub const kreuzberg_metadata_created_by = __root.kreuzberg_metadata_created_by;
    pub const kreuzberg_metadata_modified_by = __root.kreuzberg_metadata_modified_by;
    pub const kreuzberg_metadata_pages = __root.kreuzberg_metadata_pages;
    pub const kreuzberg_metadata_format = __root.kreuzberg_metadata_format;
    pub const kreuzberg_metadata_image_preprocessing = __root.kreuzberg_metadata_image_preprocessing;
    pub const kreuzberg_metadata_json_schema = __root.kreuzberg_metadata_json_schema;
    pub const kreuzberg_metadata_error = __root.kreuzberg_metadata_error;
    pub const kreuzberg_metadata_extraction_duration_ms = __root.kreuzberg_metadata_extraction_duration_ms;
    pub const kreuzberg_metadata_category = __root.kreuzberg_metadata_category;
    pub const kreuzberg_metadata_tags = __root.kreuzberg_metadata_tags;
    pub const kreuzberg_metadata_document_version = __root.kreuzberg_metadata_document_version;
    pub const kreuzberg_metadata_abstract_text = __root.kreuzberg_metadata_abstract_text;
    pub const kreuzberg_metadata_output_format = __root.kreuzberg_metadata_output_format;
    pub const kreuzberg_metadata_additional = __root.kreuzberg_metadata_additional;
    pub const kreuzberg_metadata_is_empty = __root.kreuzberg_metadata_is_empty;
    pub const json = __root.kreuzberg_metadata_to_json;
    pub const title = __root.kreuzberg_metadata_title;
    pub const subject = __root.kreuzberg_metadata_subject;
    pub const authors = __root.kreuzberg_metadata_authors;
    pub const keywords = __root.kreuzberg_metadata_keywords;
    pub const language = __root.kreuzberg_metadata_language;
    pub const at = __root.kreuzberg_metadata_created_at;
    pub const by = __root.kreuzberg_metadata_created_by;
    pub const pages = __root.kreuzberg_metadata_pages;
    pub const format = __root.kreuzberg_metadata_format;
    pub const preprocessing = __root.kreuzberg_metadata_image_preprocessing;
    pub const schema = __root.kreuzberg_metadata_json_schema;
    pub const @"error" = __root.kreuzberg_metadata_error;
    pub const ms = __root.kreuzberg_metadata_extraction_duration_ms;
    pub const category = __root.kreuzberg_metadata_category;
    pub const tags = __root.kreuzberg_metadata_tags;
    pub const version = __root.kreuzberg_metadata_document_version;
    pub const text = __root.kreuzberg_metadata_abstract_text;
    pub const additional = __root.kreuzberg_metadata_additional;
    pub const empty = __root.kreuzberg_metadata_is_empty;
};
pub const KREUZBERGMetadata = struct_KREUZBERGMetadata;
pub const struct_KREUZBERGModelPaths = opaque {
    pub const kreuzberg_model_paths_to_json = __root.kreuzberg_model_paths_to_json;
    pub const kreuzberg_model_paths_free = __root.kreuzberg_model_paths_free;
    pub const kreuzberg_model_paths_det_model = __root.kreuzberg_model_paths_det_model;
    pub const kreuzberg_model_paths_cls_model = __root.kreuzberg_model_paths_cls_model;
    pub const kreuzberg_model_paths_rec_model = __root.kreuzberg_model_paths_rec_model;
    pub const kreuzberg_model_paths_dict_file = __root.kreuzberg_model_paths_dict_file;
    pub const json = __root.kreuzberg_model_paths_to_json;
    pub const model = __root.kreuzberg_model_paths_det_model;
    pub const file = __root.kreuzberg_model_paths_dict_file;
};
pub const KREUZBERGModelPaths = struct_KREUZBERGModelPaths;
pub const struct_KREUZBERGNodeContent = opaque {};
pub const KREUZBERGNodeContent = struct_KREUZBERGNodeContent;
pub const struct_KREUZBERGOcrBackend = opaque {};
pub const KREUZBERGOcrBackend = struct_KREUZBERGOcrBackend;
pub const struct_KREUZBERGOcrBackendType = opaque {};
pub const KREUZBERGOcrBackendType = struct_KREUZBERGOcrBackendType;
pub const struct_KREUZBERGOcrBoundingGeometry = opaque {};
pub const KREUZBERGOcrBoundingGeometry = struct_KREUZBERGOcrBoundingGeometry;
pub const struct_KREUZBERGOcrCacheStats = opaque {
    pub const kreuzberg_ocr_cache_stats_free = __root.kreuzberg_ocr_cache_stats_free;
    pub const kreuzberg_ocr_cache_stats_total_files = __root.kreuzberg_ocr_cache_stats_total_files;
    pub const kreuzberg_ocr_cache_stats_total_size_mb = __root.kreuzberg_ocr_cache_stats_total_size_mb;
    pub const files = __root.kreuzberg_ocr_cache_stats_total_files;
    pub const mb = __root.kreuzberg_ocr_cache_stats_total_size_mb;
};
pub const KREUZBERGOcrCacheStats = struct_KREUZBERGOcrCacheStats;
pub const struct_KREUZBERGOcrConfidence = opaque {
    pub const kreuzberg_ocr_confidence_to_json = __root.kreuzberg_ocr_confidence_to_json;
    pub const kreuzberg_ocr_confidence_free = __root.kreuzberg_ocr_confidence_free;
    pub const kreuzberg_ocr_confidence_detection = __root.kreuzberg_ocr_confidence_detection;
    pub const kreuzberg_ocr_confidence_recognition = __root.kreuzberg_ocr_confidence_recognition;
    pub const json = __root.kreuzberg_ocr_confidence_to_json;
    pub const detection = __root.kreuzberg_ocr_confidence_detection;
    pub const recognition = __root.kreuzberg_ocr_confidence_recognition;
};
pub const KREUZBERGOcrConfidence = struct_KREUZBERGOcrConfidence;
pub const struct_KREUZBERGOcrConfig = opaque {
    pub const kreuzberg_ocr_config_to_json = __root.kreuzberg_ocr_config_to_json;
    pub const kreuzberg_ocr_config_free = __root.kreuzberg_ocr_config_free;
    pub const kreuzberg_ocr_config_enabled = __root.kreuzberg_ocr_config_enabled;
    pub const kreuzberg_ocr_config_backend = __root.kreuzberg_ocr_config_backend;
    pub const kreuzberg_ocr_config_language = __root.kreuzberg_ocr_config_language;
    pub const kreuzberg_ocr_config_tesseract_config = __root.kreuzberg_ocr_config_tesseract_config;
    pub const kreuzberg_ocr_config_output_format = __root.kreuzberg_ocr_config_output_format;
    pub const kreuzberg_ocr_config_paddle_ocr_config = __root.kreuzberg_ocr_config_paddle_ocr_config;
    pub const kreuzberg_ocr_config_element_config = __root.kreuzberg_ocr_config_element_config;
    pub const kreuzberg_ocr_config_quality_thresholds = __root.kreuzberg_ocr_config_quality_thresholds;
    pub const kreuzberg_ocr_config_pipeline = __root.kreuzberg_ocr_config_pipeline;
    pub const kreuzberg_ocr_config_auto_rotate = __root.kreuzberg_ocr_config_auto_rotate;
    pub const kreuzberg_ocr_config_vlm_config = __root.kreuzberg_ocr_config_vlm_config;
    pub const kreuzberg_ocr_config_vlm_prompt = __root.kreuzberg_ocr_config_vlm_prompt;
    pub const kreuzberg_ocr_config_acceleration = __root.kreuzberg_ocr_config_acceleration;
    pub const json = __root.kreuzberg_ocr_config_to_json;
    pub const enabled = __root.kreuzberg_ocr_config_enabled;
    pub const backend = __root.kreuzberg_ocr_config_backend;
    pub const language = __root.kreuzberg_ocr_config_language;
    pub const config = __root.kreuzberg_ocr_config_tesseract_config;
    pub const format = __root.kreuzberg_ocr_config_output_format;
    pub const thresholds = __root.kreuzberg_ocr_config_quality_thresholds;
    pub const pipeline = __root.kreuzberg_ocr_config_pipeline;
    pub const rotate = __root.kreuzberg_ocr_config_auto_rotate;
    pub const prompt = __root.kreuzberg_ocr_config_vlm_prompt;
    pub const acceleration = __root.kreuzberg_ocr_config_acceleration;
};
pub const KREUZBERGOcrConfig = struct_KREUZBERGOcrConfig;
pub const struct_KREUZBERGOcrElement = opaque {
    pub const kreuzberg_ocr_element_to_json = __root.kreuzberg_ocr_element_to_json;
    pub const kreuzberg_ocr_element_free = __root.kreuzberg_ocr_element_free;
    pub const kreuzberg_ocr_element_text = __root.kreuzberg_ocr_element_text;
    pub const kreuzberg_ocr_element_geometry = __root.kreuzberg_ocr_element_geometry;
    pub const kreuzberg_ocr_element_confidence = __root.kreuzberg_ocr_element_confidence;
    pub const kreuzberg_ocr_element_level = __root.kreuzberg_ocr_element_level;
    pub const kreuzberg_ocr_element_rotation = __root.kreuzberg_ocr_element_rotation;
    pub const kreuzberg_ocr_element_page_number = __root.kreuzberg_ocr_element_page_number;
    pub const kreuzberg_ocr_element_parent_id = __root.kreuzberg_ocr_element_parent_id;
    pub const kreuzberg_ocr_element_backend_metadata = __root.kreuzberg_ocr_element_backend_metadata;
    pub const json = __root.kreuzberg_ocr_element_to_json;
    pub const text = __root.kreuzberg_ocr_element_text;
    pub const geometry = __root.kreuzberg_ocr_element_geometry;
    pub const confidence = __root.kreuzberg_ocr_element_confidence;
    pub const level = __root.kreuzberg_ocr_element_level;
    pub const rotation = __root.kreuzberg_ocr_element_rotation;
    pub const number = __root.kreuzberg_ocr_element_page_number;
    pub const id = __root.kreuzberg_ocr_element_parent_id;
    pub const metadata = __root.kreuzberg_ocr_element_backend_metadata;
};
pub const KREUZBERGOcrElement = struct_KREUZBERGOcrElement;
pub const struct_KREUZBERGOcrElementConfig = opaque {
    pub const kreuzberg_ocr_element_config_to_json = __root.kreuzberg_ocr_element_config_to_json;
    pub const kreuzberg_ocr_element_config_free = __root.kreuzberg_ocr_element_config_free;
    pub const kreuzberg_ocr_element_config_include_elements = __root.kreuzberg_ocr_element_config_include_elements;
    pub const kreuzberg_ocr_element_config_min_level = __root.kreuzberg_ocr_element_config_min_level;
    pub const kreuzberg_ocr_element_config_min_confidence = __root.kreuzberg_ocr_element_config_min_confidence;
    pub const kreuzberg_ocr_element_config_build_hierarchy = __root.kreuzberg_ocr_element_config_build_hierarchy;
    pub const json = __root.kreuzberg_ocr_element_config_to_json;
    pub const elements = __root.kreuzberg_ocr_element_config_include_elements;
    pub const level = __root.kreuzberg_ocr_element_config_min_level;
    pub const confidence = __root.kreuzberg_ocr_element_config_min_confidence;
    pub const hierarchy = __root.kreuzberg_ocr_element_config_build_hierarchy;
};
pub const KREUZBERGOcrElementConfig = struct_KREUZBERGOcrElementConfig;
pub const struct_KREUZBERGOcrElementLevel = opaque {};
pub const KREUZBERGOcrElementLevel = struct_KREUZBERGOcrElementLevel;
pub const struct_KREUZBERGOcrExtractionResult = opaque {
    pub const kreuzberg_ocr_extraction_result_to_json = __root.kreuzberg_ocr_extraction_result_to_json;
    pub const kreuzberg_ocr_extraction_result_free = __root.kreuzberg_ocr_extraction_result_free;
    pub const kreuzberg_ocr_extraction_result_content = __root.kreuzberg_ocr_extraction_result_content;
    pub const kreuzberg_ocr_extraction_result_mime_type = __root.kreuzberg_ocr_extraction_result_mime_type;
    pub const kreuzberg_ocr_extraction_result_metadata = __root.kreuzberg_ocr_extraction_result_metadata;
    pub const kreuzberg_ocr_extraction_result_tables = __root.kreuzberg_ocr_extraction_result_tables;
    pub const kreuzberg_ocr_extraction_result_ocr_elements = __root.kreuzberg_ocr_extraction_result_ocr_elements;
    pub const json = __root.kreuzberg_ocr_extraction_result_to_json;
    pub const content = __root.kreuzberg_ocr_extraction_result_content;
    pub const @"type" = __root.kreuzberg_ocr_extraction_result_mime_type;
    pub const metadata = __root.kreuzberg_ocr_extraction_result_metadata;
    pub const tables = __root.kreuzberg_ocr_extraction_result_tables;
    pub const elements = __root.kreuzberg_ocr_extraction_result_ocr_elements;
};
pub const KREUZBERGOcrExtractionResult = struct_KREUZBERGOcrExtractionResult;
pub const struct_KREUZBERGOcrMetadata = opaque {
    pub const kreuzberg_ocr_metadata_to_json = __root.kreuzberg_ocr_metadata_to_json;
    pub const kreuzberg_ocr_metadata_free = __root.kreuzberg_ocr_metadata_free;
    pub const kreuzberg_ocr_metadata_language = __root.kreuzberg_ocr_metadata_language;
    pub const kreuzberg_ocr_metadata_psm = __root.kreuzberg_ocr_metadata_psm;
    pub const kreuzberg_ocr_metadata_output_format = __root.kreuzberg_ocr_metadata_output_format;
    pub const kreuzberg_ocr_metadata_table_count = __root.kreuzberg_ocr_metadata_table_count;
    pub const kreuzberg_ocr_metadata_table_rows = __root.kreuzberg_ocr_metadata_table_rows;
    pub const kreuzberg_ocr_metadata_table_cols = __root.kreuzberg_ocr_metadata_table_cols;
    pub const json = __root.kreuzberg_ocr_metadata_to_json;
    pub const language = __root.kreuzberg_ocr_metadata_language;
    pub const psm = __root.kreuzberg_ocr_metadata_psm;
    pub const format = __root.kreuzberg_ocr_metadata_output_format;
    pub const count = __root.kreuzberg_ocr_metadata_table_count;
    pub const rows = __root.kreuzberg_ocr_metadata_table_rows;
    pub const cols = __root.kreuzberg_ocr_metadata_table_cols;
};
pub const KREUZBERGOcrMetadata = struct_KREUZBERGOcrMetadata;
pub const struct_KREUZBERGOcrPipelineConfig = opaque {
    pub const kreuzberg_ocr_pipeline_config_to_json = __root.kreuzberg_ocr_pipeline_config_to_json;
    pub const kreuzberg_ocr_pipeline_config_free = __root.kreuzberg_ocr_pipeline_config_free;
    pub const kreuzberg_ocr_pipeline_config_stages = __root.kreuzberg_ocr_pipeline_config_stages;
    pub const kreuzberg_ocr_pipeline_config_quality_thresholds = __root.kreuzberg_ocr_pipeline_config_quality_thresholds;
    pub const json = __root.kreuzberg_ocr_pipeline_config_to_json;
    pub const stages = __root.kreuzberg_ocr_pipeline_config_stages;
    pub const thresholds = __root.kreuzberg_ocr_pipeline_config_quality_thresholds;
};
pub const KREUZBERGOcrPipelineConfig = struct_KREUZBERGOcrPipelineConfig;
pub const struct_KREUZBERGOcrPipelineStage = opaque {
    pub const kreuzberg_ocr_pipeline_stage_to_json = __root.kreuzberg_ocr_pipeline_stage_to_json;
    pub const kreuzberg_ocr_pipeline_stage_free = __root.kreuzberg_ocr_pipeline_stage_free;
    pub const kreuzberg_ocr_pipeline_stage_backend = __root.kreuzberg_ocr_pipeline_stage_backend;
    pub const kreuzberg_ocr_pipeline_stage_priority = __root.kreuzberg_ocr_pipeline_stage_priority;
    pub const kreuzberg_ocr_pipeline_stage_language = __root.kreuzberg_ocr_pipeline_stage_language;
    pub const kreuzberg_ocr_pipeline_stage_tesseract_config = __root.kreuzberg_ocr_pipeline_stage_tesseract_config;
    pub const kreuzberg_ocr_pipeline_stage_paddle_ocr_config = __root.kreuzberg_ocr_pipeline_stage_paddle_ocr_config;
    pub const kreuzberg_ocr_pipeline_stage_vlm_config = __root.kreuzberg_ocr_pipeline_stage_vlm_config;
    pub const json = __root.kreuzberg_ocr_pipeline_stage_to_json;
    pub const backend = __root.kreuzberg_ocr_pipeline_stage_backend;
    pub const priority = __root.kreuzberg_ocr_pipeline_stage_priority;
    pub const language = __root.kreuzberg_ocr_pipeline_stage_language;
    pub const config = __root.kreuzberg_ocr_pipeline_stage_tesseract_config;
};
pub const KREUZBERGOcrPipelineStage = struct_KREUZBERGOcrPipelineStage;
pub const struct_KREUZBERGOcrQualityThresholds = opaque {
    pub const kreuzberg_ocr_quality_thresholds_to_json = __root.kreuzberg_ocr_quality_thresholds_to_json;
    pub const kreuzberg_ocr_quality_thresholds_free = __root.kreuzberg_ocr_quality_thresholds_free;
    pub const kreuzberg_ocr_quality_thresholds_min_total_non_whitespace = __root.kreuzberg_ocr_quality_thresholds_min_total_non_whitespace;
    pub const kreuzberg_ocr_quality_thresholds_min_non_whitespace_per_page = __root.kreuzberg_ocr_quality_thresholds_min_non_whitespace_per_page;
    pub const kreuzberg_ocr_quality_thresholds_min_meaningful_word_len = __root.kreuzberg_ocr_quality_thresholds_min_meaningful_word_len;
    pub const kreuzberg_ocr_quality_thresholds_min_meaningful_words = __root.kreuzberg_ocr_quality_thresholds_min_meaningful_words;
    pub const kreuzberg_ocr_quality_thresholds_min_alnum_ratio = __root.kreuzberg_ocr_quality_thresholds_min_alnum_ratio;
    pub const kreuzberg_ocr_quality_thresholds_min_garbage_chars = __root.kreuzberg_ocr_quality_thresholds_min_garbage_chars;
    pub const kreuzberg_ocr_quality_thresholds_max_fragmented_word_ratio = __root.kreuzberg_ocr_quality_thresholds_max_fragmented_word_ratio;
    pub const kreuzberg_ocr_quality_thresholds_critical_fragmented_word_ratio = __root.kreuzberg_ocr_quality_thresholds_critical_fragmented_word_ratio;
    pub const kreuzberg_ocr_quality_thresholds_min_avg_word_length = __root.kreuzberg_ocr_quality_thresholds_min_avg_word_length;
    pub const kreuzberg_ocr_quality_thresholds_min_words_for_avg_length_check = __root.kreuzberg_ocr_quality_thresholds_min_words_for_avg_length_check;
    pub const kreuzberg_ocr_quality_thresholds_min_consecutive_repeat_ratio = __root.kreuzberg_ocr_quality_thresholds_min_consecutive_repeat_ratio;
    pub const kreuzberg_ocr_quality_thresholds_min_words_for_repeat_check = __root.kreuzberg_ocr_quality_thresholds_min_words_for_repeat_check;
    pub const kreuzberg_ocr_quality_thresholds_substantive_min_chars = __root.kreuzberg_ocr_quality_thresholds_substantive_min_chars;
    pub const kreuzberg_ocr_quality_thresholds_non_text_min_chars = __root.kreuzberg_ocr_quality_thresholds_non_text_min_chars;
    pub const kreuzberg_ocr_quality_thresholds_alnum_ws_ratio_threshold = __root.kreuzberg_ocr_quality_thresholds_alnum_ws_ratio_threshold;
    pub const kreuzberg_ocr_quality_thresholds_pipeline_min_quality = __root.kreuzberg_ocr_quality_thresholds_pipeline_min_quality;
    pub const json = __root.kreuzberg_ocr_quality_thresholds_to_json;
    pub const whitespace = __root.kreuzberg_ocr_quality_thresholds_min_total_non_whitespace;
    pub const page = __root.kreuzberg_ocr_quality_thresholds_min_non_whitespace_per_page;
    pub const len = __root.kreuzberg_ocr_quality_thresholds_min_meaningful_word_len;
    pub const words = __root.kreuzberg_ocr_quality_thresholds_min_meaningful_words;
    pub const ratio = __root.kreuzberg_ocr_quality_thresholds_min_alnum_ratio;
    pub const chars = __root.kreuzberg_ocr_quality_thresholds_min_garbage_chars;
    pub const length = __root.kreuzberg_ocr_quality_thresholds_min_avg_word_length;
    pub const check = __root.kreuzberg_ocr_quality_thresholds_min_words_for_avg_length_check;
    pub const threshold = __root.kreuzberg_ocr_quality_thresholds_alnum_ws_ratio_threshold;
    pub const quality = __root.kreuzberg_ocr_quality_thresholds_pipeline_min_quality;
};
pub const KREUZBERGOcrQualityThresholds = struct_KREUZBERGOcrQualityThresholds;
pub const struct_KREUZBERGOcrRotation = opaque {
    pub const kreuzberg_ocr_rotation_to_json = __root.kreuzberg_ocr_rotation_to_json;
    pub const kreuzberg_ocr_rotation_free = __root.kreuzberg_ocr_rotation_free;
    pub const kreuzberg_ocr_rotation_angle_degrees = __root.kreuzberg_ocr_rotation_angle_degrees;
    pub const kreuzberg_ocr_rotation_confidence = __root.kreuzberg_ocr_rotation_confidence;
    pub const json = __root.kreuzberg_ocr_rotation_to_json;
    pub const degrees = __root.kreuzberg_ocr_rotation_angle_degrees;
    pub const confidence = __root.kreuzberg_ocr_rotation_confidence;
};
pub const KREUZBERGOcrRotation = struct_KREUZBERGOcrRotation;
pub const struct_KREUZBERGOcrTable = opaque {
    pub const kreuzberg_ocr_table_to_json = __root.kreuzberg_ocr_table_to_json;
    pub const kreuzberg_ocr_table_free = __root.kreuzberg_ocr_table_free;
    pub const kreuzberg_ocr_table_cells = __root.kreuzberg_ocr_table_cells;
    pub const kreuzberg_ocr_table_markdown = __root.kreuzberg_ocr_table_markdown;
    pub const kreuzberg_ocr_table_page_number = __root.kreuzberg_ocr_table_page_number;
    pub const kreuzberg_ocr_table_bounding_box = __root.kreuzberg_ocr_table_bounding_box;
    pub const json = __root.kreuzberg_ocr_table_to_json;
    pub const cells = __root.kreuzberg_ocr_table_cells;
    pub const markdown = __root.kreuzberg_ocr_table_markdown;
    pub const number = __root.kreuzberg_ocr_table_page_number;
    pub const box = __root.kreuzberg_ocr_table_bounding_box;
};
pub const KREUZBERGOcrTable = struct_KREUZBERGOcrTable;
pub const struct_KREUZBERGOcrTableBoundingBox = opaque {
    pub const kreuzberg_ocr_table_bounding_box_to_json = __root.kreuzberg_ocr_table_bounding_box_to_json;
    pub const kreuzberg_ocr_table_bounding_box_free = __root.kreuzberg_ocr_table_bounding_box_free;
    pub const kreuzberg_ocr_table_bounding_box_left = __root.kreuzberg_ocr_table_bounding_box_left;
    pub const kreuzberg_ocr_table_bounding_box_top = __root.kreuzberg_ocr_table_bounding_box_top;
    pub const kreuzberg_ocr_table_bounding_box_right = __root.kreuzberg_ocr_table_bounding_box_right;
    pub const kreuzberg_ocr_table_bounding_box_bottom = __root.kreuzberg_ocr_table_bounding_box_bottom;
    pub const json = __root.kreuzberg_ocr_table_bounding_box_to_json;
    pub const left = __root.kreuzberg_ocr_table_bounding_box_left;
    pub const top = __root.kreuzberg_ocr_table_bounding_box_top;
    pub const right = __root.kreuzberg_ocr_table_bounding_box_right;
    pub const bottom = __root.kreuzberg_ocr_table_bounding_box_bottom;
};
pub const KREUZBERGOcrTableBoundingBox = struct_KREUZBERGOcrTableBoundingBox;
pub const struct_KREUZBERGOdtProperties = opaque {
    pub const kreuzberg_odt_properties_free = __root.kreuzberg_odt_properties_free;
    pub const kreuzberg_odt_properties_title = __root.kreuzberg_odt_properties_title;
    pub const kreuzberg_odt_properties_subject = __root.kreuzberg_odt_properties_subject;
    pub const kreuzberg_odt_properties_creator = __root.kreuzberg_odt_properties_creator;
    pub const kreuzberg_odt_properties_initial_creator = __root.kreuzberg_odt_properties_initial_creator;
    pub const kreuzberg_odt_properties_keywords = __root.kreuzberg_odt_properties_keywords;
    pub const kreuzberg_odt_properties_description = __root.kreuzberg_odt_properties_description;
    pub const kreuzberg_odt_properties_date = __root.kreuzberg_odt_properties_date;
    pub const kreuzberg_odt_properties_creation_date = __root.kreuzberg_odt_properties_creation_date;
    pub const kreuzberg_odt_properties_language = __root.kreuzberg_odt_properties_language;
    pub const kreuzberg_odt_properties_generator = __root.kreuzberg_odt_properties_generator;
    pub const kreuzberg_odt_properties_editing_duration = __root.kreuzberg_odt_properties_editing_duration;
    pub const kreuzberg_odt_properties_editing_cycles = __root.kreuzberg_odt_properties_editing_cycles;
    pub const kreuzberg_odt_properties_page_count = __root.kreuzberg_odt_properties_page_count;
    pub const kreuzberg_odt_properties_word_count = __root.kreuzberg_odt_properties_word_count;
    pub const kreuzberg_odt_properties_character_count = __root.kreuzberg_odt_properties_character_count;
    pub const kreuzberg_odt_properties_paragraph_count = __root.kreuzberg_odt_properties_paragraph_count;
    pub const kreuzberg_odt_properties_table_count = __root.kreuzberg_odt_properties_table_count;
    pub const kreuzberg_odt_properties_image_count = __root.kreuzberg_odt_properties_image_count;
    pub const title = __root.kreuzberg_odt_properties_title;
    pub const subject = __root.kreuzberg_odt_properties_subject;
    pub const creator = __root.kreuzberg_odt_properties_creator;
    pub const keywords = __root.kreuzberg_odt_properties_keywords;
    pub const description = __root.kreuzberg_odt_properties_description;
    pub const date = __root.kreuzberg_odt_properties_date;
    pub const language = __root.kreuzberg_odt_properties_language;
    pub const generator = __root.kreuzberg_odt_properties_generator;
    pub const duration = __root.kreuzberg_odt_properties_editing_duration;
    pub const cycles = __root.kreuzberg_odt_properties_editing_cycles;
    pub const count = __root.kreuzberg_odt_properties_page_count;
};
pub const KREUZBERGOdtProperties = struct_KREUZBERGOdtProperties;
pub const struct_KREUZBERGOpenWebDocumentResponse = opaque {
    pub const kreuzberg_open_web_document_response_to_json = __root.kreuzberg_open_web_document_response_to_json;
    pub const kreuzberg_open_web_document_response_free = __root.kreuzberg_open_web_document_response_free;
    pub const kreuzberg_open_web_document_response_page_content = __root.kreuzberg_open_web_document_response_page_content;
    pub const json = __root.kreuzberg_open_web_document_response_to_json;
    pub const content = __root.kreuzberg_open_web_document_response_page_content;
};
pub const KREUZBERGOpenWebDocumentResponse = struct_KREUZBERGOpenWebDocumentResponse;
pub const struct_KREUZBERGOrientationResult = opaque {
    pub const kreuzberg_orientation_result_to_json = __root.kreuzberg_orientation_result_to_json;
    pub const kreuzberg_orientation_result_free = __root.kreuzberg_orientation_result_free;
    pub const kreuzberg_orientation_result_degrees = __root.kreuzberg_orientation_result_degrees;
    pub const kreuzberg_orientation_result_confidence = __root.kreuzberg_orientation_result_confidence;
    pub const json = __root.kreuzberg_orientation_result_to_json;
    pub const degrees = __root.kreuzberg_orientation_result_degrees;
    pub const confidence = __root.kreuzberg_orientation_result_confidence;
};
pub const KREUZBERGOrientationResult = struct_KREUZBERGOrientationResult;
pub const struct_KREUZBERGOutputFormat = opaque {};
pub const KREUZBERGOutputFormat = struct_KREUZBERGOutputFormat;
pub const struct_KREUZBERGPSMMode = opaque {};
pub const KREUZBERGPSMMode = struct_KREUZBERGPSMMode;
pub const struct_KREUZBERGPaddleLanguage = opaque {};
pub const KREUZBERGPaddleLanguage = struct_KREUZBERGPaddleLanguage;
pub const struct_KREUZBERGPaddleOcrConfig = opaque {
    pub const kreuzberg_paddle_ocr_config_to_json = __root.kreuzberg_paddle_ocr_config_to_json;
    pub const kreuzberg_paddle_ocr_config_free = __root.kreuzberg_paddle_ocr_config_free;
    pub const kreuzberg_paddle_ocr_config_language = __root.kreuzberg_paddle_ocr_config_language;
    pub const kreuzberg_paddle_ocr_config_cache_dir = __root.kreuzberg_paddle_ocr_config_cache_dir;
    pub const kreuzberg_paddle_ocr_config_use_angle_cls = __root.kreuzberg_paddle_ocr_config_use_angle_cls;
    pub const kreuzberg_paddle_ocr_config_enable_table_detection = __root.kreuzberg_paddle_ocr_config_enable_table_detection;
    pub const kreuzberg_paddle_ocr_config_det_db_thresh = __root.kreuzberg_paddle_ocr_config_det_db_thresh;
    pub const kreuzberg_paddle_ocr_config_det_db_box_thresh = __root.kreuzberg_paddle_ocr_config_det_db_box_thresh;
    pub const kreuzberg_paddle_ocr_config_det_db_unclip_ratio = __root.kreuzberg_paddle_ocr_config_det_db_unclip_ratio;
    pub const kreuzberg_paddle_ocr_config_det_limit_side_len = __root.kreuzberg_paddle_ocr_config_det_limit_side_len;
    pub const kreuzberg_paddle_ocr_config_rec_batch_num = __root.kreuzberg_paddle_ocr_config_rec_batch_num;
    pub const kreuzberg_paddle_ocr_config_padding = __root.kreuzberg_paddle_ocr_config_padding;
    pub const kreuzberg_paddle_ocr_config_drop_score = __root.kreuzberg_paddle_ocr_config_drop_score;
    pub const kreuzberg_paddle_ocr_config_model_tier = __root.kreuzberg_paddle_ocr_config_model_tier;
    pub const kreuzberg_paddle_ocr_config_with_cache_dir = __root.kreuzberg_paddle_ocr_config_with_cache_dir;
    pub const kreuzberg_paddle_ocr_config_with_table_detection = __root.kreuzberg_paddle_ocr_config_with_table_detection;
    pub const kreuzberg_paddle_ocr_config_with_angle_cls = __root.kreuzberg_paddle_ocr_config_with_angle_cls;
    pub const kreuzberg_paddle_ocr_config_with_det_db_thresh = __root.kreuzberg_paddle_ocr_config_with_det_db_thresh;
    pub const kreuzberg_paddle_ocr_config_with_det_db_box_thresh = __root.kreuzberg_paddle_ocr_config_with_det_db_box_thresh;
    pub const kreuzberg_paddle_ocr_config_with_det_db_unclip_ratio = __root.kreuzberg_paddle_ocr_config_with_det_db_unclip_ratio;
    pub const kreuzberg_paddle_ocr_config_with_det_limit_side_len = __root.kreuzberg_paddle_ocr_config_with_det_limit_side_len;
    pub const kreuzberg_paddle_ocr_config_with_rec_batch_num = __root.kreuzberg_paddle_ocr_config_with_rec_batch_num;
    pub const kreuzberg_paddle_ocr_config_with_drop_score = __root.kreuzberg_paddle_ocr_config_with_drop_score;
    pub const kreuzberg_paddle_ocr_config_with_padding = __root.kreuzberg_paddle_ocr_config_with_padding;
    pub const kreuzberg_paddle_ocr_config_with_model_tier = __root.kreuzberg_paddle_ocr_config_with_model_tier;
    pub const json = __root.kreuzberg_paddle_ocr_config_to_json;
    pub const language = __root.kreuzberg_paddle_ocr_config_language;
    pub const dir = __root.kreuzberg_paddle_ocr_config_cache_dir;
    pub const cls = __root.kreuzberg_paddle_ocr_config_use_angle_cls;
    pub const detection = __root.kreuzberg_paddle_ocr_config_enable_table_detection;
    pub const thresh = __root.kreuzberg_paddle_ocr_config_det_db_thresh;
    pub const ratio = __root.kreuzberg_paddle_ocr_config_det_db_unclip_ratio;
    pub const len = __root.kreuzberg_paddle_ocr_config_det_limit_side_len;
    pub const num = __root.kreuzberg_paddle_ocr_config_rec_batch_num;
    pub const padding = __root.kreuzberg_paddle_ocr_config_padding;
    pub const score = __root.kreuzberg_paddle_ocr_config_drop_score;
    pub const tier = __root.kreuzberg_paddle_ocr_config_model_tier;
};
pub const KREUZBERGPaddleOcrConfig = struct_KREUZBERGPaddleOcrConfig;
pub const struct_KREUZBERGPageBoundary = opaque {
    pub const kreuzberg_page_boundary_to_json = __root.kreuzberg_page_boundary_to_json;
    pub const kreuzberg_page_boundary_free = __root.kreuzberg_page_boundary_free;
    pub const kreuzberg_page_boundary_byte_start = __root.kreuzberg_page_boundary_byte_start;
    pub const kreuzberg_page_boundary_byte_end = __root.kreuzberg_page_boundary_byte_end;
    pub const kreuzberg_page_boundary_page_number = __root.kreuzberg_page_boundary_page_number;
    pub const json = __root.kreuzberg_page_boundary_to_json;
    pub const start = __root.kreuzberg_page_boundary_byte_start;
    pub const end = __root.kreuzberg_page_boundary_byte_end;
    pub const number = __root.kreuzberg_page_boundary_page_number;
};
pub const KREUZBERGPageBoundary = struct_KREUZBERGPageBoundary;
pub const struct_KREUZBERGPageConfig = opaque {
    pub const kreuzberg_page_config_to_json = __root.kreuzberg_page_config_to_json;
    pub const kreuzberg_page_config_free = __root.kreuzberg_page_config_free;
    pub const kreuzberg_page_config_extract_pages = __root.kreuzberg_page_config_extract_pages;
    pub const kreuzberg_page_config_insert_page_markers = __root.kreuzberg_page_config_insert_page_markers;
    pub const kreuzberg_page_config_marker_format = __root.kreuzberg_page_config_marker_format;
    pub const json = __root.kreuzberg_page_config_to_json;
    pub const pages = __root.kreuzberg_page_config_extract_pages;
    pub const markers = __root.kreuzberg_page_config_insert_page_markers;
    pub const format = __root.kreuzberg_page_config_marker_format;
};
pub const KREUZBERGPageConfig = struct_KREUZBERGPageConfig;
pub const struct_KREUZBERGPageContent = opaque {
    pub const kreuzberg_page_content_to_json = __root.kreuzberg_page_content_to_json;
    pub const kreuzberg_page_content_free = __root.kreuzberg_page_content_free;
    pub const kreuzberg_page_content_page_number = __root.kreuzberg_page_content_page_number;
    pub const kreuzberg_page_content_content = __root.kreuzberg_page_content_content;
    pub const kreuzberg_page_content_tables = __root.kreuzberg_page_content_tables;
    pub const kreuzberg_page_content_images = __root.kreuzberg_page_content_images;
    pub const kreuzberg_page_content_hierarchy = __root.kreuzberg_page_content_hierarchy;
    pub const kreuzberg_page_content_is_blank = __root.kreuzberg_page_content_is_blank;
    pub const kreuzberg_page_content_layout_regions = __root.kreuzberg_page_content_layout_regions;
    pub const json = __root.kreuzberg_page_content_to_json;
    pub const number = __root.kreuzberg_page_content_page_number;
    pub const content = __root.kreuzberg_page_content_content;
    pub const tables = __root.kreuzberg_page_content_tables;
    pub const images = __root.kreuzberg_page_content_images;
    pub const hierarchy = __root.kreuzberg_page_content_hierarchy;
    pub const blank = __root.kreuzberg_page_content_is_blank;
    pub const regions = __root.kreuzberg_page_content_layout_regions;
};
pub const KREUZBERGPageContent = struct_KREUZBERGPageContent;
pub const struct_KREUZBERGPageHierarchy = opaque {
    pub const kreuzberg_page_hierarchy_to_json = __root.kreuzberg_page_hierarchy_to_json;
    pub const kreuzberg_page_hierarchy_free = __root.kreuzberg_page_hierarchy_free;
    pub const kreuzberg_page_hierarchy_block_count = __root.kreuzberg_page_hierarchy_block_count;
    pub const kreuzberg_page_hierarchy_blocks = __root.kreuzberg_page_hierarchy_blocks;
    pub const json = __root.kreuzberg_page_hierarchy_to_json;
    pub const count = __root.kreuzberg_page_hierarchy_block_count;
    pub const blocks = __root.kreuzberg_page_hierarchy_blocks;
};
pub const KREUZBERGPageHierarchy = struct_KREUZBERGPageHierarchy;
pub const struct_KREUZBERGPageInfo = opaque {
    pub const kreuzberg_page_info_to_json = __root.kreuzberg_page_info_to_json;
    pub const kreuzberg_page_info_free = __root.kreuzberg_page_info_free;
    pub const kreuzberg_page_info_number = __root.kreuzberg_page_info_number;
    pub const kreuzberg_page_info_title = __root.kreuzberg_page_info_title;
    pub const kreuzberg_page_info_image_count = __root.kreuzberg_page_info_image_count;
    pub const kreuzberg_page_info_table_count = __root.kreuzberg_page_info_table_count;
    pub const kreuzberg_page_info_hidden = __root.kreuzberg_page_info_hidden;
    pub const kreuzberg_page_info_is_blank = __root.kreuzberg_page_info_is_blank;
    pub const kreuzberg_page_info_has_vector_graphics = __root.kreuzberg_page_info_has_vector_graphics;
    pub const json = __root.kreuzberg_page_info_to_json;
    pub const number = __root.kreuzberg_page_info_number;
    pub const title = __root.kreuzberg_page_info_title;
    pub const count = __root.kreuzberg_page_info_image_count;
    pub const hidden = __root.kreuzberg_page_info_hidden;
    pub const blank = __root.kreuzberg_page_info_is_blank;
    pub const graphics = __root.kreuzberg_page_info_has_vector_graphics;
};
pub const KREUZBERGPageInfo = struct_KREUZBERGPageInfo;
pub const struct_KREUZBERGPageMarginsPoints = opaque {
    pub const kreuzberg_page_margins_points_free = __root.kreuzberg_page_margins_points_free;
    pub const kreuzberg_page_margins_points_top = __root.kreuzberg_page_margins_points_top;
    pub const kreuzberg_page_margins_points_right = __root.kreuzberg_page_margins_points_right;
    pub const kreuzberg_page_margins_points_bottom = __root.kreuzberg_page_margins_points_bottom;
    pub const kreuzberg_page_margins_points_left = __root.kreuzberg_page_margins_points_left;
    pub const kreuzberg_page_margins_points_header = __root.kreuzberg_page_margins_points_header;
    pub const kreuzberg_page_margins_points_footer = __root.kreuzberg_page_margins_points_footer;
    pub const kreuzberg_page_margins_points_gutter = __root.kreuzberg_page_margins_points_gutter;
    pub const top = __root.kreuzberg_page_margins_points_top;
    pub const right = __root.kreuzberg_page_margins_points_right;
    pub const bottom = __root.kreuzberg_page_margins_points_bottom;
    pub const left = __root.kreuzberg_page_margins_points_left;
    pub const header = __root.kreuzberg_page_margins_points_header;
    pub const footer = __root.kreuzberg_page_margins_points_footer;
    pub const gutter = __root.kreuzberg_page_margins_points_gutter;
};
pub const KREUZBERGPageMarginsPoints = struct_KREUZBERGPageMarginsPoints;
pub const struct_KREUZBERGPageStructure = opaque {
    pub const kreuzberg_page_structure_to_json = __root.kreuzberg_page_structure_to_json;
    pub const kreuzberg_page_structure_free = __root.kreuzberg_page_structure_free;
    pub const kreuzberg_page_structure_total_count = __root.kreuzberg_page_structure_total_count;
    pub const kreuzberg_page_structure_unit_type = __root.kreuzberg_page_structure_unit_type;
    pub const kreuzberg_page_structure_boundaries = __root.kreuzberg_page_structure_boundaries;
    pub const kreuzberg_page_structure_pages = __root.kreuzberg_page_structure_pages;
    pub const json = __root.kreuzberg_page_structure_to_json;
    pub const count = __root.kreuzberg_page_structure_total_count;
    pub const @"type" = __root.kreuzberg_page_structure_unit_type;
    pub const boundaries = __root.kreuzberg_page_structure_boundaries;
    pub const pages = __root.kreuzberg_page_structure_pages;
};
pub const KREUZBERGPageStructure = struct_KREUZBERGPageStructure;
pub const struct_KREUZBERGPageUnitType = opaque {};
pub const KREUZBERGPageUnitType = struct_KREUZBERGPageUnitType;
pub const struct_KREUZBERGPdfAnnotation = opaque {
    pub const kreuzberg_pdf_annotation_to_json = __root.kreuzberg_pdf_annotation_to_json;
    pub const kreuzberg_pdf_annotation_free = __root.kreuzberg_pdf_annotation_free;
    pub const kreuzberg_pdf_annotation_annotation_type = __root.kreuzberg_pdf_annotation_annotation_type;
    pub const kreuzberg_pdf_annotation_content = __root.kreuzberg_pdf_annotation_content;
    pub const kreuzberg_pdf_annotation_page_number = __root.kreuzberg_pdf_annotation_page_number;
    pub const json = __root.kreuzberg_pdf_annotation_to_json;
    pub const @"type" = __root.kreuzberg_pdf_annotation_annotation_type;
    pub const content = __root.kreuzberg_pdf_annotation_content;
    pub const number = __root.kreuzberg_pdf_annotation_page_number;
};
pub const KREUZBERGPdfAnnotation = struct_KREUZBERGPdfAnnotation;
pub const struct_KREUZBERGPdfAnnotationType = opaque {};
pub const KREUZBERGPdfAnnotationType = struct_KREUZBERGPdfAnnotationType;
pub const struct_KREUZBERGPdfConfig = opaque {
    pub const kreuzberg_pdf_config_to_json = __root.kreuzberg_pdf_config_to_json;
    pub const kreuzberg_pdf_config_free = __root.kreuzberg_pdf_config_free;
    pub const kreuzberg_pdf_config_extract_images = __root.kreuzberg_pdf_config_extract_images;
    pub const kreuzberg_pdf_config_passwords = __root.kreuzberg_pdf_config_passwords;
    pub const kreuzberg_pdf_config_extract_metadata = __root.kreuzberg_pdf_config_extract_metadata;
    pub const kreuzberg_pdf_config_hierarchy = __root.kreuzberg_pdf_config_hierarchy;
    pub const kreuzberg_pdf_config_extract_annotations = __root.kreuzberg_pdf_config_extract_annotations;
    pub const kreuzberg_pdf_config_top_margin_fraction = __root.kreuzberg_pdf_config_top_margin_fraction;
    pub const kreuzberg_pdf_config_bottom_margin_fraction = __root.kreuzberg_pdf_config_bottom_margin_fraction;
    pub const kreuzberg_pdf_config_allow_single_column_tables = __root.kreuzberg_pdf_config_allow_single_column_tables;
    pub const json = __root.kreuzberg_pdf_config_to_json;
    pub const images = __root.kreuzberg_pdf_config_extract_images;
    pub const passwords = __root.kreuzberg_pdf_config_passwords;
    pub const metadata = __root.kreuzberg_pdf_config_extract_metadata;
    pub const hierarchy = __root.kreuzberg_pdf_config_hierarchy;
    pub const annotations = __root.kreuzberg_pdf_config_extract_annotations;
    pub const fraction = __root.kreuzberg_pdf_config_top_margin_fraction;
    pub const tables = __root.kreuzberg_pdf_config_allow_single_column_tables;
};
pub const KREUZBERGPdfConfig = struct_KREUZBERGPdfConfig;
pub const struct_KREUZBERGPlugin = opaque {};
pub const KREUZBERGPlugin = struct_KREUZBERGPlugin;
pub const struct_KREUZBERGPoolError = opaque {};
pub const KREUZBERGPoolError = struct_KREUZBERGPoolError;
pub const struct_KREUZBERGPostProcessor = opaque {};
pub const KREUZBERGPostProcessor = struct_KREUZBERGPostProcessor;
pub const struct_KREUZBERGPostProcessorConfig = opaque {
    pub const kreuzberg_post_processor_config_to_json = __root.kreuzberg_post_processor_config_to_json;
    pub const kreuzberg_post_processor_config_free = __root.kreuzberg_post_processor_config_free;
    pub const kreuzberg_post_processor_config_enabled = __root.kreuzberg_post_processor_config_enabled;
    pub const kreuzberg_post_processor_config_enabled_processors = __root.kreuzberg_post_processor_config_enabled_processors;
    pub const kreuzberg_post_processor_config_disabled_processors = __root.kreuzberg_post_processor_config_disabled_processors;
    pub const json = __root.kreuzberg_post_processor_config_to_json;
    pub const enabled = __root.kreuzberg_post_processor_config_enabled;
    pub const processors = __root.kreuzberg_post_processor_config_enabled_processors;
};
pub const KREUZBERGPostProcessorConfig = struct_KREUZBERGPostProcessorConfig;
pub const struct_KREUZBERGPptxAppProperties = opaque {
    pub const kreuzberg_pptx_app_properties_to_json = __root.kreuzberg_pptx_app_properties_to_json;
    pub const kreuzberg_pptx_app_properties_free = __root.kreuzberg_pptx_app_properties_free;
    pub const kreuzberg_pptx_app_properties_application = __root.kreuzberg_pptx_app_properties_application;
    pub const kreuzberg_pptx_app_properties_app_version = __root.kreuzberg_pptx_app_properties_app_version;
    pub const kreuzberg_pptx_app_properties_total_time = __root.kreuzberg_pptx_app_properties_total_time;
    pub const kreuzberg_pptx_app_properties_company = __root.kreuzberg_pptx_app_properties_company;
    pub const kreuzberg_pptx_app_properties_doc_security = __root.kreuzberg_pptx_app_properties_doc_security;
    pub const kreuzberg_pptx_app_properties_scale_crop = __root.kreuzberg_pptx_app_properties_scale_crop;
    pub const kreuzberg_pptx_app_properties_links_up_to_date = __root.kreuzberg_pptx_app_properties_links_up_to_date;
    pub const kreuzberg_pptx_app_properties_shared_doc = __root.kreuzberg_pptx_app_properties_shared_doc;
    pub const kreuzberg_pptx_app_properties_hyperlinks_changed = __root.kreuzberg_pptx_app_properties_hyperlinks_changed;
    pub const kreuzberg_pptx_app_properties_slides = __root.kreuzberg_pptx_app_properties_slides;
    pub const kreuzberg_pptx_app_properties_notes = __root.kreuzberg_pptx_app_properties_notes;
    pub const kreuzberg_pptx_app_properties_hidden_slides = __root.kreuzberg_pptx_app_properties_hidden_slides;
    pub const kreuzberg_pptx_app_properties_multimedia_clips = __root.kreuzberg_pptx_app_properties_multimedia_clips;
    pub const kreuzberg_pptx_app_properties_presentation_format = __root.kreuzberg_pptx_app_properties_presentation_format;
    pub const kreuzberg_pptx_app_properties_slide_titles = __root.kreuzberg_pptx_app_properties_slide_titles;
    pub const json = __root.kreuzberg_pptx_app_properties_to_json;
    pub const application = __root.kreuzberg_pptx_app_properties_application;
    pub const version = __root.kreuzberg_pptx_app_properties_app_version;
    pub const time = __root.kreuzberg_pptx_app_properties_total_time;
    pub const company = __root.kreuzberg_pptx_app_properties_company;
    pub const security = __root.kreuzberg_pptx_app_properties_doc_security;
    pub const crop = __root.kreuzberg_pptx_app_properties_scale_crop;
    pub const date = __root.kreuzberg_pptx_app_properties_links_up_to_date;
    pub const doc = __root.kreuzberg_pptx_app_properties_shared_doc;
    pub const changed = __root.kreuzberg_pptx_app_properties_hyperlinks_changed;
    pub const slides = __root.kreuzberg_pptx_app_properties_slides;
    pub const notes = __root.kreuzberg_pptx_app_properties_notes;
    pub const clips = __root.kreuzberg_pptx_app_properties_multimedia_clips;
    pub const format = __root.kreuzberg_pptx_app_properties_presentation_format;
    pub const titles = __root.kreuzberg_pptx_app_properties_slide_titles;
};
pub const KREUZBERGPptxAppProperties = struct_KREUZBERGPptxAppProperties;
pub const struct_KREUZBERGPptxExtractionResult = opaque {
    pub const kreuzberg_pptx_extraction_result_to_json = __root.kreuzberg_pptx_extraction_result_to_json;
    pub const kreuzberg_pptx_extraction_result_free = __root.kreuzberg_pptx_extraction_result_free;
    pub const kreuzberg_pptx_extraction_result_content = __root.kreuzberg_pptx_extraction_result_content;
    pub const kreuzberg_pptx_extraction_result_metadata = __root.kreuzberg_pptx_extraction_result_metadata;
    pub const kreuzberg_pptx_extraction_result_slide_count = __root.kreuzberg_pptx_extraction_result_slide_count;
    pub const kreuzberg_pptx_extraction_result_image_count = __root.kreuzberg_pptx_extraction_result_image_count;
    pub const kreuzberg_pptx_extraction_result_table_count = __root.kreuzberg_pptx_extraction_result_table_count;
    pub const kreuzberg_pptx_extraction_result_images = __root.kreuzberg_pptx_extraction_result_images;
    pub const kreuzberg_pptx_extraction_result_page_structure = __root.kreuzberg_pptx_extraction_result_page_structure;
    pub const kreuzberg_pptx_extraction_result_page_contents = __root.kreuzberg_pptx_extraction_result_page_contents;
    pub const kreuzberg_pptx_extraction_result_document = __root.kreuzberg_pptx_extraction_result_document;
    pub const kreuzberg_pptx_extraction_result_office_metadata = __root.kreuzberg_pptx_extraction_result_office_metadata;
    pub const json = __root.kreuzberg_pptx_extraction_result_to_json;
    pub const content = __root.kreuzberg_pptx_extraction_result_content;
    pub const metadata = __root.kreuzberg_pptx_extraction_result_metadata;
    pub const count = __root.kreuzberg_pptx_extraction_result_slide_count;
    pub const images = __root.kreuzberg_pptx_extraction_result_images;
    pub const structure = __root.kreuzberg_pptx_extraction_result_page_structure;
    pub const contents = __root.kreuzberg_pptx_extraction_result_page_contents;
    pub const document = __root.kreuzberg_pptx_extraction_result_document;
};
pub const KREUZBERGPptxExtractionResult = struct_KREUZBERGPptxExtractionResult;
pub const struct_KREUZBERGPptxMetadata = opaque {
    pub const kreuzberg_pptx_metadata_to_json = __root.kreuzberg_pptx_metadata_to_json;
    pub const kreuzberg_pptx_metadata_free = __root.kreuzberg_pptx_metadata_free;
    pub const kreuzberg_pptx_metadata_slide_count = __root.kreuzberg_pptx_metadata_slide_count;
    pub const kreuzberg_pptx_metadata_slide_names = __root.kreuzberg_pptx_metadata_slide_names;
    pub const kreuzberg_pptx_metadata_image_count = __root.kreuzberg_pptx_metadata_image_count;
    pub const kreuzberg_pptx_metadata_table_count = __root.kreuzberg_pptx_metadata_table_count;
    pub const json = __root.kreuzberg_pptx_metadata_to_json;
    pub const count = __root.kreuzberg_pptx_metadata_slide_count;
    pub const names = __root.kreuzberg_pptx_metadata_slide_names;
};
pub const KREUZBERGPptxMetadata = struct_KREUZBERGPptxMetadata;
pub const struct_KREUZBERGProcessingStage = opaque {};
pub const KREUZBERGProcessingStage = struct_KREUZBERGProcessingStage;
pub const struct_KREUZBERGProcessingWarning = opaque {
    pub const kreuzberg_processing_warning_to_json = __root.kreuzberg_processing_warning_to_json;
    pub const kreuzberg_processing_warning_free = __root.kreuzberg_processing_warning_free;
    pub const json = __root.kreuzberg_processing_warning_to_json;
};
pub const KREUZBERGProcessingWarning = struct_KREUZBERGProcessingWarning;
pub const struct_KREUZBERGPstMetadata = opaque {
    pub const kreuzberg_pst_metadata_to_json = __root.kreuzberg_pst_metadata_to_json;
    pub const kreuzberg_pst_metadata_free = __root.kreuzberg_pst_metadata_free;
    pub const kreuzberg_pst_metadata_message_count = __root.kreuzberg_pst_metadata_message_count;
    pub const json = __root.kreuzberg_pst_metadata_to_json;
    pub const count = __root.kreuzberg_pst_metadata_message_count;
};
pub const KREUZBERGPstMetadata = struct_KREUZBERGPstMetadata;
pub const struct_KREUZBERGRakeParams = opaque {
    pub const kreuzberg_rake_params_to_json = __root.kreuzberg_rake_params_to_json;
    pub const kreuzberg_rake_params_free = __root.kreuzberg_rake_params_free;
    pub const kreuzberg_rake_params_min_word_length = __root.kreuzberg_rake_params_min_word_length;
    pub const kreuzberg_rake_params_max_words_per_phrase = __root.kreuzberg_rake_params_max_words_per_phrase;
    pub const json = __root.kreuzberg_rake_params_to_json;
    pub const length = __root.kreuzberg_rake_params_min_word_length;
    pub const phrase = __root.kreuzberg_rake_params_max_words_per_phrase;
};
pub const KREUZBERGRakeParams = struct_KREUZBERGRakeParams;
pub const struct_KREUZBERGRecognizedTable = opaque {
    pub const kreuzberg_recognized_table_to_json = __root.kreuzberg_recognized_table_to_json;
    pub const kreuzberg_recognized_table_free = __root.kreuzberg_recognized_table_free;
    pub const kreuzberg_recognized_table_detection_bbox = __root.kreuzberg_recognized_table_detection_bbox;
    pub const kreuzberg_recognized_table_cells = __root.kreuzberg_recognized_table_cells;
    pub const kreuzberg_recognized_table_markdown = __root.kreuzberg_recognized_table_markdown;
    pub const json = __root.kreuzberg_recognized_table_to_json;
    pub const bbox = __root.kreuzberg_recognized_table_detection_bbox;
    pub const cells = __root.kreuzberg_recognized_table_cells;
    pub const markdown = __root.kreuzberg_recognized_table_markdown;
};
pub const KREUZBERGRecognizedTable = struct_KREUZBERGRecognizedTable;
pub const struct_KREUZBERGRecyclable = opaque {};
pub const KREUZBERGRecyclable = struct_KREUZBERGRecyclable;
pub const struct_KREUZBERGReductionLevel = opaque {};
pub const KREUZBERGReductionLevel = struct_KREUZBERGReductionLevel;
pub const struct_KREUZBERGRelationshipKind = opaque {};
pub const KREUZBERGRelationshipKind = struct_KREUZBERGRelationshipKind;
pub const struct_KREUZBERGResolvedStyle = opaque {
    pub const kreuzberg_resolved_style_free = __root.kreuzberg_resolved_style_free;
};
pub const KREUZBERGResolvedStyle = struct_KREUZBERGResolvedStyle;
pub const struct_KREUZBERGResultFormat = opaque {};
pub const KREUZBERGResultFormat = struct_KREUZBERGResultFormat;
pub const struct_KREUZBERGSecurityLimits = opaque {
    pub const kreuzberg_security_limits_to_json = __root.kreuzberg_security_limits_to_json;
    pub const kreuzberg_security_limits_free = __root.kreuzberg_security_limits_free;
    pub const kreuzberg_security_limits_max_archive_size = __root.kreuzberg_security_limits_max_archive_size;
    pub const kreuzberg_security_limits_max_compression_ratio = __root.kreuzberg_security_limits_max_compression_ratio;
    pub const kreuzberg_security_limits_max_files_in_archive = __root.kreuzberg_security_limits_max_files_in_archive;
    pub const kreuzberg_security_limits_max_nesting_depth = __root.kreuzberg_security_limits_max_nesting_depth;
    pub const kreuzberg_security_limits_max_entity_length = __root.kreuzberg_security_limits_max_entity_length;
    pub const kreuzberg_security_limits_max_content_size = __root.kreuzberg_security_limits_max_content_size;
    pub const kreuzberg_security_limits_max_iterations = __root.kreuzberg_security_limits_max_iterations;
    pub const kreuzberg_security_limits_max_xml_depth = __root.kreuzberg_security_limits_max_xml_depth;
    pub const kreuzberg_security_limits_max_table_cells = __root.kreuzberg_security_limits_max_table_cells;
    pub const json = __root.kreuzberg_security_limits_to_json;
    pub const size = __root.kreuzberg_security_limits_max_archive_size;
    pub const ratio = __root.kreuzberg_security_limits_max_compression_ratio;
    pub const archive = __root.kreuzberg_security_limits_max_files_in_archive;
    pub const depth = __root.kreuzberg_security_limits_max_nesting_depth;
    pub const length = __root.kreuzberg_security_limits_max_entity_length;
    pub const iterations = __root.kreuzberg_security_limits_max_iterations;
    pub const cells = __root.kreuzberg_security_limits_max_table_cells;
};
pub const KREUZBERGSecurityLimits = struct_KREUZBERGSecurityLimits;
pub const struct_KREUZBERGServerConfig = opaque {
    pub const kreuzberg_server_config_to_json = __root.kreuzberg_server_config_to_json;
    pub const kreuzberg_server_config_free = __root.kreuzberg_server_config_free;
    pub const kreuzberg_server_config_host = __root.kreuzberg_server_config_host;
    pub const kreuzberg_server_config_port = __root.kreuzberg_server_config_port;
    pub const kreuzberg_server_config_cors_origins = __root.kreuzberg_server_config_cors_origins;
    pub const kreuzberg_server_config_max_request_body_bytes = __root.kreuzberg_server_config_max_request_body_bytes;
    pub const kreuzberg_server_config_max_multipart_field_bytes = __root.kreuzberg_server_config_max_multipart_field_bytes;
    pub const kreuzberg_server_config_listen_addr = __root.kreuzberg_server_config_listen_addr;
    pub const kreuzberg_server_config_cors_allows_all = __root.kreuzberg_server_config_cors_allows_all;
    pub const kreuzberg_server_config_is_origin_allowed = __root.kreuzberg_server_config_is_origin_allowed;
    pub const kreuzberg_server_config_max_request_body_mb = __root.kreuzberg_server_config_max_request_body_mb;
    pub const kreuzberg_server_config_max_multipart_field_mb = __root.kreuzberg_server_config_max_multipart_field_mb;
    pub const json = __root.kreuzberg_server_config_to_json;
    pub const host = __root.kreuzberg_server_config_host;
    pub const port = __root.kreuzberg_server_config_port;
    pub const origins = __root.kreuzberg_server_config_cors_origins;
    pub const bytes = __root.kreuzberg_server_config_max_request_body_bytes;
    pub const addr = __root.kreuzberg_server_config_listen_addr;
    pub const all = __root.kreuzberg_server_config_cors_allows_all;
    pub const allowed = __root.kreuzberg_server_config_is_origin_allowed;
    pub const mb = __root.kreuzberg_server_config_max_request_body_mb;
};
pub const KREUZBERGServerConfig = struct_KREUZBERGServerConfig;
pub const struct_KREUZBERGStreamReader = opaque {
    pub const kreuzberg_stream_reader_free = __root.kreuzberg_stream_reader_free;
};
pub const KREUZBERGStreamReader = struct_KREUZBERGStreamReader;
pub const struct_KREUZBERGStringBufferPool = opaque {
    pub const kreuzberg_string_buffer_pool_free = __root.kreuzberg_string_buffer_pool_free;
};
pub const KREUZBERGStringBufferPool = struct_KREUZBERGStringBufferPool;
pub const struct_KREUZBERGStructuredData = opaque {
    pub const kreuzberg_structured_data_to_json = __root.kreuzberg_structured_data_to_json;
    pub const kreuzberg_structured_data_free = __root.kreuzberg_structured_data_free;
    pub const kreuzberg_structured_data_data_type = __root.kreuzberg_structured_data_data_type;
    pub const kreuzberg_structured_data_raw_json = __root.kreuzberg_structured_data_raw_json;
    pub const kreuzberg_structured_data_schema_type = __root.kreuzberg_structured_data_schema_type;
    pub const json = __root.kreuzberg_structured_data_to_json;
    pub const @"type" = __root.kreuzberg_structured_data_data_type;
};
pub const KREUZBERGStructuredData = struct_KREUZBERGStructuredData;
pub const struct_KREUZBERGStructuredDataResult = opaque {
    pub const kreuzberg_structured_data_result_to_json = __root.kreuzberg_structured_data_result_to_json;
    pub const kreuzberg_structured_data_result_free = __root.kreuzberg_structured_data_result_free;
    pub const kreuzberg_structured_data_result_content = __root.kreuzberg_structured_data_result_content;
    pub const kreuzberg_structured_data_result_metadata = __root.kreuzberg_structured_data_result_metadata;
    pub const kreuzberg_structured_data_result_text_fields = __root.kreuzberg_structured_data_result_text_fields;
    pub const json = __root.kreuzberg_structured_data_result_to_json;
    pub const content = __root.kreuzberg_structured_data_result_content;
    pub const metadata = __root.kreuzberg_structured_data_result_metadata;
    pub const fields = __root.kreuzberg_structured_data_result_text_fields;
};
pub const KREUZBERGStructuredDataResult = struct_KREUZBERGStructuredDataResult;
pub const struct_KREUZBERGStructuredDataType = opaque {};
pub const KREUZBERGStructuredDataType = struct_KREUZBERGStructuredDataType;
pub const struct_KREUZBERGStructuredExtractionConfig = opaque {
    pub const kreuzberg_structured_extraction_config_to_json = __root.kreuzberg_structured_extraction_config_to_json;
    pub const kreuzberg_structured_extraction_config_free = __root.kreuzberg_structured_extraction_config_free;
    pub const kreuzberg_structured_extraction_config_schema = __root.kreuzberg_structured_extraction_config_schema;
    pub const kreuzberg_structured_extraction_config_schema_name = __root.kreuzberg_structured_extraction_config_schema_name;
    pub const kreuzberg_structured_extraction_config_schema_description = __root.kreuzberg_structured_extraction_config_schema_description;
    pub const kreuzberg_structured_extraction_config_strict = __root.kreuzberg_structured_extraction_config_strict;
    pub const kreuzberg_structured_extraction_config_prompt = __root.kreuzberg_structured_extraction_config_prompt;
    pub const kreuzberg_structured_extraction_config_llm = __root.kreuzberg_structured_extraction_config_llm;
    pub const json = __root.kreuzberg_structured_extraction_config_to_json;
    pub const schema = __root.kreuzberg_structured_extraction_config_schema;
    pub const name = __root.kreuzberg_structured_extraction_config_schema_name;
    pub const description = __root.kreuzberg_structured_extraction_config_schema_description;
    pub const strict = __root.kreuzberg_structured_extraction_config_strict;
    pub const prompt = __root.kreuzberg_structured_extraction_config_prompt;
    pub const llm = __root.kreuzberg_structured_extraction_config_llm;
};
pub const KREUZBERGStructuredExtractionConfig = struct_KREUZBERGStructuredExtractionConfig;
pub const struct_KREUZBERGStructuredExtractionResponse = opaque {
    pub const kreuzberg_structured_extraction_response_to_json = __root.kreuzberg_structured_extraction_response_to_json;
    pub const kreuzberg_structured_extraction_response_free = __root.kreuzberg_structured_extraction_response_free;
    pub const kreuzberg_structured_extraction_response_structured_output = __root.kreuzberg_structured_extraction_response_structured_output;
    pub const kreuzberg_structured_extraction_response_content = __root.kreuzberg_structured_extraction_response_content;
    pub const kreuzberg_structured_extraction_response_mime_type = __root.kreuzberg_structured_extraction_response_mime_type;
    pub const json = __root.kreuzberg_structured_extraction_response_to_json;
    pub const output = __root.kreuzberg_structured_extraction_response_structured_output;
    pub const content = __root.kreuzberg_structured_extraction_response_content;
    pub const @"type" = __root.kreuzberg_structured_extraction_response_mime_type;
};
pub const KREUZBERGStructuredExtractionResponse = struct_KREUZBERGStructuredExtractionResponse;
pub const struct_KREUZBERGStyleDefinition = opaque {
    pub const kreuzberg_style_definition_free = __root.kreuzberg_style_definition_free;
    pub const kreuzberg_style_definition_id = __root.kreuzberg_style_definition_id;
    pub const kreuzberg_style_definition_name = __root.kreuzberg_style_definition_name;
    pub const kreuzberg_style_definition_based_on = __root.kreuzberg_style_definition_based_on;
    pub const kreuzberg_style_definition_next_style = __root.kreuzberg_style_definition_next_style;
    pub const kreuzberg_style_definition_is_default = __root.kreuzberg_style_definition_is_default;
    pub const id = __root.kreuzberg_style_definition_id;
    pub const name = __root.kreuzberg_style_definition_name;
    pub const on = __root.kreuzberg_style_definition_based_on;
    pub const style = __root.kreuzberg_style_definition_next_style;
    pub const default = __root.kreuzberg_style_definition_is_default;
};
pub const KREUZBERGStyleDefinition = struct_KREUZBERGStyleDefinition;
pub const struct_KREUZBERGSupportedFormat = opaque {
    pub const kreuzberg_supported_format_to_json = __root.kreuzberg_supported_format_to_json;
    pub const kreuzberg_supported_format_free = __root.kreuzberg_supported_format_free;
    pub const kreuzberg_supported_format_extension = __root.kreuzberg_supported_format_extension;
    pub const kreuzberg_supported_format_mime_type = __root.kreuzberg_supported_format_mime_type;
    pub const json = __root.kreuzberg_supported_format_to_json;
    pub const extension = __root.kreuzberg_supported_format_extension;
    pub const @"type" = __root.kreuzberg_supported_format_mime_type;
};
pub const KREUZBERGSupportedFormat = struct_KREUZBERGSupportedFormat;
pub const struct_KREUZBERGSyncExtractor = opaque {};
pub const KREUZBERGSyncExtractor = struct_KREUZBERGSyncExtractor;
pub const struct_KREUZBERGTable = opaque {
    pub const kreuzberg_table_to_json = __root.kreuzberg_table_to_json;
    pub const kreuzberg_table_free = __root.kreuzberg_table_free;
    pub const kreuzberg_table_cells = __root.kreuzberg_table_cells;
    pub const kreuzberg_table_markdown = __root.kreuzberg_table_markdown;
    pub const kreuzberg_table_page_number = __root.kreuzberg_table_page_number;
    pub const json = __root.kreuzberg_table_to_json;
    pub const cells = __root.kreuzberg_table_cells;
    pub const markdown = __root.kreuzberg_table_markdown;
    pub const number = __root.kreuzberg_table_page_number;
};
pub const KREUZBERGTable = struct_KREUZBERGTable;
pub const struct_KREUZBERGTableCell = opaque {
    pub const kreuzberg_table_cell_to_json = __root.kreuzberg_table_cell_to_json;
    pub const kreuzberg_table_cell_free = __root.kreuzberg_table_cell_free;
    pub const kreuzberg_table_cell_content = __root.kreuzberg_table_cell_content;
    pub const kreuzberg_table_cell_row_span = __root.kreuzberg_table_cell_row_span;
    pub const kreuzberg_table_cell_col_span = __root.kreuzberg_table_cell_col_span;
    pub const kreuzberg_table_cell_is_header = __root.kreuzberg_table_cell_is_header;
    pub const json = __root.kreuzberg_table_cell_to_json;
    pub const content = __root.kreuzberg_table_cell_content;
    pub const span = __root.kreuzberg_table_cell_row_span;
    pub const header = __root.kreuzberg_table_cell_is_header;
};
pub const KREUZBERGTableCell = struct_KREUZBERGTableCell;
pub const struct_KREUZBERGTableGrid = opaque {
    pub const kreuzberg_table_grid_to_json = __root.kreuzberg_table_grid_to_json;
    pub const kreuzberg_table_grid_free = __root.kreuzberg_table_grid_free;
    pub const kreuzberg_table_grid_rows = __root.kreuzberg_table_grid_rows;
    pub const kreuzberg_table_grid_cols = __root.kreuzberg_table_grid_cols;
    pub const kreuzberg_table_grid_cells = __root.kreuzberg_table_grid_cells;
    pub const json = __root.kreuzberg_table_grid_to_json;
    pub const rows = __root.kreuzberg_table_grid_rows;
    pub const cols = __root.kreuzberg_table_grid_cols;
    pub const cells = __root.kreuzberg_table_grid_cells;
};
pub const KREUZBERGTableGrid = struct_KREUZBERGTableGrid;
pub const struct_KREUZBERGTableModel = opaque {};
pub const KREUZBERGTableModel = struct_KREUZBERGTableModel;
pub const struct_KREUZBERGTableProperties = opaque {
    pub const kreuzberg_table_properties_to_json = __root.kreuzberg_table_properties_to_json;
    pub const kreuzberg_table_properties_free = __root.kreuzberg_table_properties_free;
    pub const kreuzberg_table_properties_style_id = __root.kreuzberg_table_properties_style_id;
    pub const kreuzberg_table_properties_alignment = __root.kreuzberg_table_properties_alignment;
    pub const kreuzberg_table_properties_layout = __root.kreuzberg_table_properties_layout;
    pub const kreuzberg_table_properties_caption = __root.kreuzberg_table_properties_caption;
    pub const json = __root.kreuzberg_table_properties_to_json;
    pub const id = __root.kreuzberg_table_properties_style_id;
    pub const alignment = __root.kreuzberg_table_properties_alignment;
    pub const layout = __root.kreuzberg_table_properties_layout;
    pub const caption = __root.kreuzberg_table_properties_caption;
};
pub const KREUZBERGTableProperties = struct_KREUZBERGTableProperties;
pub const struct_KREUZBERGTessdataManager = opaque {
    pub const kreuzberg_tessdata_manager_free = __root.kreuzberg_tessdata_manager_free;
    pub const kreuzberg_tessdata_manager_cache_dir = __root.kreuzberg_tessdata_manager_cache_dir;
    pub const kreuzberg_tessdata_manager_is_language_cached = __root.kreuzberg_tessdata_manager_is_language_cached;
    pub const kreuzberg_tessdata_manager_ensure_all_languages = __root.kreuzberg_tessdata_manager_ensure_all_languages;
    pub const dir = __root.kreuzberg_tessdata_manager_cache_dir;
    pub const cached = __root.kreuzberg_tessdata_manager_is_language_cached;
    pub const languages = __root.kreuzberg_tessdata_manager_ensure_all_languages;
};
pub const KREUZBERGTessdataManager = struct_KREUZBERGTessdataManager;
pub const struct_KREUZBERGTesseractConfig = opaque {
    pub const kreuzberg_tesseract_config_to_json = __root.kreuzberg_tesseract_config_to_json;
    pub const kreuzberg_tesseract_config_free = __root.kreuzberg_tesseract_config_free;
    pub const kreuzberg_tesseract_config_language = __root.kreuzberg_tesseract_config_language;
    pub const kreuzberg_tesseract_config_psm = __root.kreuzberg_tesseract_config_psm;
    pub const kreuzberg_tesseract_config_output_format = __root.kreuzberg_tesseract_config_output_format;
    pub const kreuzberg_tesseract_config_oem = __root.kreuzberg_tesseract_config_oem;
    pub const kreuzberg_tesseract_config_min_confidence = __root.kreuzberg_tesseract_config_min_confidence;
    pub const kreuzberg_tesseract_config_preprocessing = __root.kreuzberg_tesseract_config_preprocessing;
    pub const kreuzberg_tesseract_config_enable_table_detection = __root.kreuzberg_tesseract_config_enable_table_detection;
    pub const kreuzberg_tesseract_config_table_min_confidence = __root.kreuzberg_tesseract_config_table_min_confidence;
    pub const kreuzberg_tesseract_config_table_column_threshold = __root.kreuzberg_tesseract_config_table_column_threshold;
    pub const kreuzberg_tesseract_config_table_row_threshold_ratio = __root.kreuzberg_tesseract_config_table_row_threshold_ratio;
    pub const kreuzberg_tesseract_config_use_cache = __root.kreuzberg_tesseract_config_use_cache;
    pub const kreuzberg_tesseract_config_classify_use_pre_adapted_templates = __root.kreuzberg_tesseract_config_classify_use_pre_adapted_templates;
    pub const kreuzberg_tesseract_config_language_model_ngram_on = __root.kreuzberg_tesseract_config_language_model_ngram_on;
    pub const kreuzberg_tesseract_config_tessedit_dont_blkrej_good_wds = __root.kreuzberg_tesseract_config_tessedit_dont_blkrej_good_wds;
    pub const kreuzberg_tesseract_config_tessedit_dont_rowrej_good_wds = __root.kreuzberg_tesseract_config_tessedit_dont_rowrej_good_wds;
    pub const kreuzberg_tesseract_config_tessedit_enable_dict_correction = __root.kreuzberg_tesseract_config_tessedit_enable_dict_correction;
    pub const kreuzberg_tesseract_config_tessedit_char_whitelist = __root.kreuzberg_tesseract_config_tessedit_char_whitelist;
    pub const kreuzberg_tesseract_config_tessedit_char_blacklist = __root.kreuzberg_tesseract_config_tessedit_char_blacklist;
    pub const kreuzberg_tesseract_config_tessedit_use_primary_params_model = __root.kreuzberg_tesseract_config_tessedit_use_primary_params_model;
    pub const kreuzberg_tesseract_config_textord_space_size_is_variable = __root.kreuzberg_tesseract_config_textord_space_size_is_variable;
    pub const kreuzberg_tesseract_config_thresholding_method = __root.kreuzberg_tesseract_config_thresholding_method;
    pub const json = __root.kreuzberg_tesseract_config_to_json;
    pub const language = __root.kreuzberg_tesseract_config_language;
    pub const psm = __root.kreuzberg_tesseract_config_psm;
    pub const format = __root.kreuzberg_tesseract_config_output_format;
    pub const oem = __root.kreuzberg_tesseract_config_oem;
    pub const confidence = __root.kreuzberg_tesseract_config_min_confidence;
    pub const preprocessing = __root.kreuzberg_tesseract_config_preprocessing;
    pub const detection = __root.kreuzberg_tesseract_config_enable_table_detection;
    pub const threshold = __root.kreuzberg_tesseract_config_table_column_threshold;
    pub const ratio = __root.kreuzberg_tesseract_config_table_row_threshold_ratio;
    pub const cache = __root.kreuzberg_tesseract_config_use_cache;
    pub const templates = __root.kreuzberg_tesseract_config_classify_use_pre_adapted_templates;
    pub const on = __root.kreuzberg_tesseract_config_language_model_ngram_on;
    pub const wds = __root.kreuzberg_tesseract_config_tessedit_dont_blkrej_good_wds;
    pub const correction = __root.kreuzberg_tesseract_config_tessedit_enable_dict_correction;
    pub const whitelist = __root.kreuzberg_tesseract_config_tessedit_char_whitelist;
    pub const blacklist = __root.kreuzberg_tesseract_config_tessedit_char_blacklist;
    pub const model = __root.kreuzberg_tesseract_config_tessedit_use_primary_params_model;
    pub const variable = __root.kreuzberg_tesseract_config_textord_space_size_is_variable;
    pub const method = __root.kreuzberg_tesseract_config_thresholding_method;
};
pub const KREUZBERGTesseractConfig = struct_KREUZBERGTesseractConfig;
pub const struct_KREUZBERGTextAnnotation = opaque {
    pub const kreuzberg_text_annotation_to_json = __root.kreuzberg_text_annotation_to_json;
    pub const kreuzberg_text_annotation_free = __root.kreuzberg_text_annotation_free;
    pub const kreuzberg_text_annotation_start = __root.kreuzberg_text_annotation_start;
    pub const kreuzberg_text_annotation_end = __root.kreuzberg_text_annotation_end;
    pub const kreuzberg_text_annotation_kind = __root.kreuzberg_text_annotation_kind;
    pub const json = __root.kreuzberg_text_annotation_to_json;
    pub const start = __root.kreuzberg_text_annotation_start;
    pub const end = __root.kreuzberg_text_annotation_end;
    pub const kind = __root.kreuzberg_text_annotation_kind;
};
pub const KREUZBERGTextAnnotation = struct_KREUZBERGTextAnnotation;
pub const struct_KREUZBERGTextDirection = opaque {};
pub const KREUZBERGTextDirection = struct_KREUZBERGTextDirection;
pub const struct_KREUZBERGTextExtractionResult = opaque {
    pub const kreuzberg_text_extraction_result_to_json = __root.kreuzberg_text_extraction_result_to_json;
    pub const kreuzberg_text_extraction_result_free = __root.kreuzberg_text_extraction_result_free;
    pub const kreuzberg_text_extraction_result_content = __root.kreuzberg_text_extraction_result_content;
    pub const kreuzberg_text_extraction_result_line_count = __root.kreuzberg_text_extraction_result_line_count;
    pub const kreuzberg_text_extraction_result_word_count = __root.kreuzberg_text_extraction_result_word_count;
    pub const kreuzberg_text_extraction_result_character_count = __root.kreuzberg_text_extraction_result_character_count;
    pub const kreuzberg_text_extraction_result_headers = __root.kreuzberg_text_extraction_result_headers;
    pub const json = __root.kreuzberg_text_extraction_result_to_json;
    pub const content = __root.kreuzberg_text_extraction_result_content;
    pub const count = __root.kreuzberg_text_extraction_result_line_count;
    pub const headers = __root.kreuzberg_text_extraction_result_headers;
};
pub const KREUZBERGTextExtractionResult = struct_KREUZBERGTextExtractionResult;
pub const struct_KREUZBERGTextMetadata = opaque {
    pub const kreuzberg_text_metadata_to_json = __root.kreuzberg_text_metadata_to_json;
    pub const kreuzberg_text_metadata_free = __root.kreuzberg_text_metadata_free;
    pub const kreuzberg_text_metadata_line_count = __root.kreuzberg_text_metadata_line_count;
    pub const kreuzberg_text_metadata_word_count = __root.kreuzberg_text_metadata_word_count;
    pub const kreuzberg_text_metadata_character_count = __root.kreuzberg_text_metadata_character_count;
    pub const kreuzberg_text_metadata_headers = __root.kreuzberg_text_metadata_headers;
    pub const json = __root.kreuzberg_text_metadata_to_json;
    pub const count = __root.kreuzberg_text_metadata_line_count;
    pub const headers = __root.kreuzberg_text_metadata_headers;
};
pub const KREUZBERGTextMetadata = struct_KREUZBERGTextMetadata;
pub const struct_KREUZBERGTokenReductionConfig = opaque {
    pub const kreuzberg_token_reduction_config_to_json = __root.kreuzberg_token_reduction_config_to_json;
    pub const kreuzberg_token_reduction_config_free = __root.kreuzberg_token_reduction_config_free;
    pub const kreuzberg_token_reduction_config_level = __root.kreuzberg_token_reduction_config_level;
    pub const kreuzberg_token_reduction_config_language_hint = __root.kreuzberg_token_reduction_config_language_hint;
    pub const kreuzberg_token_reduction_config_preserve_markdown = __root.kreuzberg_token_reduction_config_preserve_markdown;
    pub const kreuzberg_token_reduction_config_preserve_code = __root.kreuzberg_token_reduction_config_preserve_code;
    pub const kreuzberg_token_reduction_config_semantic_threshold = __root.kreuzberg_token_reduction_config_semantic_threshold;
    pub const kreuzberg_token_reduction_config_enable_parallel = __root.kreuzberg_token_reduction_config_enable_parallel;
    pub const kreuzberg_token_reduction_config_use_simd = __root.kreuzberg_token_reduction_config_use_simd;
    pub const kreuzberg_token_reduction_config_custom_stopwords = __root.kreuzberg_token_reduction_config_custom_stopwords;
    pub const kreuzberg_token_reduction_config_preserve_patterns = __root.kreuzberg_token_reduction_config_preserve_patterns;
    pub const kreuzberg_token_reduction_config_target_reduction = __root.kreuzberg_token_reduction_config_target_reduction;
    pub const kreuzberg_token_reduction_config_enable_semantic_clustering = __root.kreuzberg_token_reduction_config_enable_semantic_clustering;
    pub const json = __root.kreuzberg_token_reduction_config_to_json;
    pub const level = __root.kreuzberg_token_reduction_config_level;
    pub const hint = __root.kreuzberg_token_reduction_config_language_hint;
    pub const markdown = __root.kreuzberg_token_reduction_config_preserve_markdown;
    pub const code = __root.kreuzberg_token_reduction_config_preserve_code;
    pub const threshold = __root.kreuzberg_token_reduction_config_semantic_threshold;
    pub const parallel = __root.kreuzberg_token_reduction_config_enable_parallel;
    pub const simd = __root.kreuzberg_token_reduction_config_use_simd;
    pub const stopwords = __root.kreuzberg_token_reduction_config_custom_stopwords;
    pub const patterns = __root.kreuzberg_token_reduction_config_preserve_patterns;
    pub const reduction = __root.kreuzberg_token_reduction_config_target_reduction;
    pub const clustering = __root.kreuzberg_token_reduction_config_enable_semantic_clustering;
};
pub const KREUZBERGTokenReductionConfig = struct_KREUZBERGTokenReductionConfig;
pub const struct_KREUZBERGTokenReductionOptions = opaque {
    pub const kreuzberg_token_reduction_options_to_json = __root.kreuzberg_token_reduction_options_to_json;
    pub const kreuzberg_token_reduction_options_free = __root.kreuzberg_token_reduction_options_free;
    pub const kreuzberg_token_reduction_options_mode = __root.kreuzberg_token_reduction_options_mode;
    pub const kreuzberg_token_reduction_options_preserve_important_words = __root.kreuzberg_token_reduction_options_preserve_important_words;
    pub const json = __root.kreuzberg_token_reduction_options_to_json;
    pub const mode = __root.kreuzberg_token_reduction_options_mode;
    pub const words = __root.kreuzberg_token_reduction_options_preserve_important_words;
};
pub const KREUZBERGTokenReductionOptions = struct_KREUZBERGTokenReductionOptions;
pub const struct_KREUZBERGTracingLayer = opaque {
    pub const kreuzberg_tracing_layer_free = __root.kreuzberg_tracing_layer_free;
};
pub const KREUZBERGTracingLayer = struct_KREUZBERGTracingLayer;
pub const struct_KREUZBERGTreeSitterConfig = opaque {
    pub const kreuzberg_tree_sitter_config_to_json = __root.kreuzberg_tree_sitter_config_to_json;
    pub const kreuzberg_tree_sitter_config_free = __root.kreuzberg_tree_sitter_config_free;
    pub const kreuzberg_tree_sitter_config_enabled = __root.kreuzberg_tree_sitter_config_enabled;
    pub const kreuzberg_tree_sitter_config_cache_dir = __root.kreuzberg_tree_sitter_config_cache_dir;
    pub const kreuzberg_tree_sitter_config_languages = __root.kreuzberg_tree_sitter_config_languages;
    pub const kreuzberg_tree_sitter_config_groups = __root.kreuzberg_tree_sitter_config_groups;
    pub const kreuzberg_tree_sitter_config_process = __root.kreuzberg_tree_sitter_config_process;
    pub const json = __root.kreuzberg_tree_sitter_config_to_json;
    pub const enabled = __root.kreuzberg_tree_sitter_config_enabled;
    pub const dir = __root.kreuzberg_tree_sitter_config_cache_dir;
    pub const languages = __root.kreuzberg_tree_sitter_config_languages;
    pub const groups = __root.kreuzberg_tree_sitter_config_groups;
    pub const process = __root.kreuzberg_tree_sitter_config_process;
};
pub const KREUZBERGTreeSitterConfig = struct_KREUZBERGTreeSitterConfig;
pub const struct_KREUZBERGTreeSitterProcessConfig = opaque {
    pub const kreuzberg_tree_sitter_process_config_to_json = __root.kreuzberg_tree_sitter_process_config_to_json;
    pub const kreuzberg_tree_sitter_process_config_free = __root.kreuzberg_tree_sitter_process_config_free;
    pub const kreuzberg_tree_sitter_process_config_structure = __root.kreuzberg_tree_sitter_process_config_structure;
    pub const kreuzberg_tree_sitter_process_config_imports = __root.kreuzberg_tree_sitter_process_config_imports;
    pub const kreuzberg_tree_sitter_process_config_exports = __root.kreuzberg_tree_sitter_process_config_exports;
    pub const kreuzberg_tree_sitter_process_config_comments = __root.kreuzberg_tree_sitter_process_config_comments;
    pub const kreuzberg_tree_sitter_process_config_docstrings = __root.kreuzberg_tree_sitter_process_config_docstrings;
    pub const kreuzberg_tree_sitter_process_config_symbols = __root.kreuzberg_tree_sitter_process_config_symbols;
    pub const kreuzberg_tree_sitter_process_config_diagnostics = __root.kreuzberg_tree_sitter_process_config_diagnostics;
    pub const kreuzberg_tree_sitter_process_config_chunk_max_size = __root.kreuzberg_tree_sitter_process_config_chunk_max_size;
    pub const kreuzberg_tree_sitter_process_config_content_mode = __root.kreuzberg_tree_sitter_process_config_content_mode;
    pub const json = __root.kreuzberg_tree_sitter_process_config_to_json;
    pub const structure = __root.kreuzberg_tree_sitter_process_config_structure;
    pub const imports = __root.kreuzberg_tree_sitter_process_config_imports;
    pub const exports = __root.kreuzberg_tree_sitter_process_config_exports;
    pub const comments = __root.kreuzberg_tree_sitter_process_config_comments;
    pub const docstrings = __root.kreuzberg_tree_sitter_process_config_docstrings;
    pub const symbols = __root.kreuzberg_tree_sitter_process_config_symbols;
    pub const diagnostics = __root.kreuzberg_tree_sitter_process_config_diagnostics;
    pub const size = __root.kreuzberg_tree_sitter_process_config_chunk_max_size;
    pub const mode = __root.kreuzberg_tree_sitter_process_config_content_mode;
};
pub const KREUZBERGTreeSitterProcessConfig = struct_KREUZBERGTreeSitterProcessConfig;
pub const struct_KREUZBERGUri = opaque {
    pub const kreuzberg_uri_to_json = __root.kreuzberg_uri_to_json;
    pub const kreuzberg_uri_free = __root.kreuzberg_uri_free;
    pub const kreuzberg_uri_url = __root.kreuzberg_uri_url;
    pub const kreuzberg_uri_label = __root.kreuzberg_uri_label;
    pub const kreuzberg_uri_page = __root.kreuzberg_uri_page;
    pub const kreuzberg_uri_kind = __root.kreuzberg_uri_kind;
    pub const json = __root.kreuzberg_uri_to_json;
    pub const url = __root.kreuzberg_uri_url;
    pub const label = __root.kreuzberg_uri_label;
    pub const page = __root.kreuzberg_uri_page;
    pub const kind = __root.kreuzberg_uri_kind;
};
pub const KREUZBERGUri = struct_KREUZBERGUri;
pub const struct_KREUZBERGUriKind = opaque {};
pub const KREUZBERGUriKind = struct_KREUZBERGUriKind;
pub const struct_KREUZBERGValidator = opaque {};
pub const KREUZBERGValidator = struct_KREUZBERGValidator;
pub const struct_KREUZBERGWarmResponse = opaque {
    pub const kreuzberg_warm_response_to_json = __root.kreuzberg_warm_response_to_json;
    pub const kreuzberg_warm_response_free = __root.kreuzberg_warm_response_free;
    pub const kreuzberg_warm_response_cache_dir = __root.kreuzberg_warm_response_cache_dir;
    pub const kreuzberg_warm_response_downloaded = __root.kreuzberg_warm_response_downloaded;
    pub const kreuzberg_warm_response_already_cached = __root.kreuzberg_warm_response_already_cached;
    pub const json = __root.kreuzberg_warm_response_to_json;
    pub const dir = __root.kreuzberg_warm_response_cache_dir;
    pub const downloaded = __root.kreuzberg_warm_response_downloaded;
    pub const cached = __root.kreuzberg_warm_response_already_cached;
};
pub const KREUZBERGWarmResponse = struct_KREUZBERGWarmResponse;
pub const struct_KREUZBERGXlsxAppProperties = opaque {
    pub const kreuzberg_xlsx_app_properties_to_json = __root.kreuzberg_xlsx_app_properties_to_json;
    pub const kreuzberg_xlsx_app_properties_free = __root.kreuzberg_xlsx_app_properties_free;
    pub const kreuzberg_xlsx_app_properties_application = __root.kreuzberg_xlsx_app_properties_application;
    pub const kreuzberg_xlsx_app_properties_app_version = __root.kreuzberg_xlsx_app_properties_app_version;
    pub const kreuzberg_xlsx_app_properties_doc_security = __root.kreuzberg_xlsx_app_properties_doc_security;
    pub const kreuzberg_xlsx_app_properties_scale_crop = __root.kreuzberg_xlsx_app_properties_scale_crop;
    pub const kreuzberg_xlsx_app_properties_links_up_to_date = __root.kreuzberg_xlsx_app_properties_links_up_to_date;
    pub const kreuzberg_xlsx_app_properties_shared_doc = __root.kreuzberg_xlsx_app_properties_shared_doc;
    pub const kreuzberg_xlsx_app_properties_hyperlinks_changed = __root.kreuzberg_xlsx_app_properties_hyperlinks_changed;
    pub const kreuzberg_xlsx_app_properties_company = __root.kreuzberg_xlsx_app_properties_company;
    pub const kreuzberg_xlsx_app_properties_worksheet_names = __root.kreuzberg_xlsx_app_properties_worksheet_names;
    pub const json = __root.kreuzberg_xlsx_app_properties_to_json;
    pub const application = __root.kreuzberg_xlsx_app_properties_application;
    pub const version = __root.kreuzberg_xlsx_app_properties_app_version;
    pub const security = __root.kreuzberg_xlsx_app_properties_doc_security;
    pub const crop = __root.kreuzberg_xlsx_app_properties_scale_crop;
    pub const date = __root.kreuzberg_xlsx_app_properties_links_up_to_date;
    pub const doc = __root.kreuzberg_xlsx_app_properties_shared_doc;
    pub const changed = __root.kreuzberg_xlsx_app_properties_hyperlinks_changed;
    pub const company = __root.kreuzberg_xlsx_app_properties_company;
    pub const names = __root.kreuzberg_xlsx_app_properties_worksheet_names;
};
pub const KREUZBERGXlsxAppProperties = struct_KREUZBERGXlsxAppProperties;
pub const struct_KREUZBERGXmlExtractionResult = opaque {
    pub const kreuzberg_xml_extraction_result_to_json = __root.kreuzberg_xml_extraction_result_to_json;
    pub const kreuzberg_xml_extraction_result_free = __root.kreuzberg_xml_extraction_result_free;
    pub const kreuzberg_xml_extraction_result_content = __root.kreuzberg_xml_extraction_result_content;
    pub const kreuzberg_xml_extraction_result_element_count = __root.kreuzberg_xml_extraction_result_element_count;
    pub const kreuzberg_xml_extraction_result_unique_elements = __root.kreuzberg_xml_extraction_result_unique_elements;
    pub const json = __root.kreuzberg_xml_extraction_result_to_json;
    pub const content = __root.kreuzberg_xml_extraction_result_content;
    pub const count = __root.kreuzberg_xml_extraction_result_element_count;
    pub const elements = __root.kreuzberg_xml_extraction_result_unique_elements;
};
pub const KREUZBERGXmlExtractionResult = struct_KREUZBERGXmlExtractionResult;
pub const struct_KREUZBERGXmlMetadata = opaque {
    pub const kreuzberg_xml_metadata_to_json = __root.kreuzberg_xml_metadata_to_json;
    pub const kreuzberg_xml_metadata_free = __root.kreuzberg_xml_metadata_free;
    pub const kreuzberg_xml_metadata_element_count = __root.kreuzberg_xml_metadata_element_count;
    pub const kreuzberg_xml_metadata_unique_elements = __root.kreuzberg_xml_metadata_unique_elements;
    pub const json = __root.kreuzberg_xml_metadata_to_json;
    pub const count = __root.kreuzberg_xml_metadata_element_count;
    pub const elements = __root.kreuzberg_xml_metadata_unique_elements;
};
pub const KREUZBERGXmlMetadata = struct_KREUZBERGXmlMetadata;
pub const struct_KREUZBERGYakeParams = opaque {
    pub const kreuzberg_yake_params_to_json = __root.kreuzberg_yake_params_to_json;
    pub const kreuzberg_yake_params_free = __root.kreuzberg_yake_params_free;
    pub const kreuzberg_yake_params_window_size = __root.kreuzberg_yake_params_window_size;
    pub const json = __root.kreuzberg_yake_params_to_json;
    pub const size = __root.kreuzberg_yake_params_window_size;
};
pub const KREUZBERGYakeParams = struct_KREUZBERGYakeParams;
pub const struct_KREUZBERGYearRange = opaque {
    pub const kreuzberg_year_range_to_json = __root.kreuzberg_year_range_to_json;
    pub const kreuzberg_year_range_free = __root.kreuzberg_year_range_free;
    pub const kreuzberg_year_range_min = __root.kreuzberg_year_range_min;
    pub const kreuzberg_year_range_max = __root.kreuzberg_year_range_max;
    pub const kreuzberg_year_range_years = __root.kreuzberg_year_range_years;
    pub const json = __root.kreuzberg_year_range_to_json;
    pub const min = __root.kreuzberg_year_range_min;
    pub const max = __root.kreuzberg_year_range_max;
    pub const years = __root.kreuzberg_year_range_years;
};
pub const KREUZBERGYearRange = struct_KREUZBERGYearRange;
pub const struct_KREUZBERGZipBombValidator = opaque {
    pub const kreuzberg_zip_bomb_validator_free = __root.kreuzberg_zip_bomb_validator_free;
};
pub const KREUZBERGZipBombValidator = struct_KREUZBERGZipBombValidator;
pub const struct_KREUZBERGKreuzbergOcrBackendVTable = extern struct {
    name_fn: ?*const fn (user_data: ?*const anyopaque, out_name: [*c][*c]u8) callconv(.c) void = null,
    version_fn: ?*const fn (user_data: ?*const anyopaque, out_version: [*c][*c]u8) callconv(.c) void = null,
    initialize_fn: ?*const fn (user_data: ?*const anyopaque, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    shutdown_fn: ?*const fn (user_data: ?*const anyopaque, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    process_image: ?*const fn (user_data: ?*const anyopaque, image_bytes: [*c]const u8, config: [*c]const u8, out_result: [*c][*c]u8, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    process_image_file: ?*const fn (user_data: ?*const anyopaque, path: [*c]const u8, config: [*c]const u8, out_result: [*c][*c]u8, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    supports_language: ?*const fn (user_data: ?*const anyopaque, lang: [*c]const u8) callconv(.c) i32 = null,
    backend_type: ?*const fn (user_data: ?*const anyopaque, out_result: [*c][*c]u8) callconv(.c) i32 = null,
    supported_languages: ?*const fn (user_data: ?*const anyopaque, out_result: [*c][*c]u8) callconv(.c) i32 = null,
    supports_table_detection: ?*const fn (user_data: ?*const anyopaque) callconv(.c) i32 = null,
    supports_document_processing: ?*const fn (user_data: ?*const anyopaque) callconv(.c) i32 = null,
    process_document: ?*const fn (user_data: ?*const anyopaque, _path: [*c]const u8, _config: [*c]const u8, out_result: [*c][*c]u8, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    free_user_data: ?*const fn (?*anyopaque) callconv(.c) void = null,
};
pub const KREUZBERGKreuzbergOcrBackendVTable = struct_KREUZBERGKreuzbergOcrBackendVTable;
pub const struct_KREUZBERGKreuzbergPostProcessorVTable = extern struct {
    name_fn: ?*const fn (user_data: ?*const anyopaque, out_name: [*c][*c]u8) callconv(.c) void = null,
    version_fn: ?*const fn (user_data: ?*const anyopaque, out_version: [*c][*c]u8) callconv(.c) void = null,
    initialize_fn: ?*const fn (user_data: ?*const anyopaque, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    shutdown_fn: ?*const fn (user_data: ?*const anyopaque, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    process: ?*const fn (user_data: ?*const anyopaque, result: [*c]const u8, config: [*c]const u8, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    processing_stage: ?*const fn (user_data: ?*const anyopaque, out_result: [*c][*c]u8) callconv(.c) i32 = null,
    should_process: ?*const fn (user_data: ?*const anyopaque, _result: [*c]const u8, _config: [*c]const u8) callconv(.c) i32 = null,
    estimated_duration_ms: ?*const fn (user_data: ?*const anyopaque, _result: [*c]const u8) callconv(.c) u64 = null,
    priority: ?*const fn (user_data: ?*const anyopaque) callconv(.c) i32 = null,
    free_user_data: ?*const fn (?*anyopaque) callconv(.c) void = null,
};
pub const KREUZBERGKreuzbergPostProcessorVTable = struct_KREUZBERGKreuzbergPostProcessorVTable;
pub const struct_KREUZBERGKreuzbergValidatorVTable = extern struct {
    name_fn: ?*const fn (user_data: ?*const anyopaque, out_name: [*c][*c]u8) callconv(.c) void = null,
    version_fn: ?*const fn (user_data: ?*const anyopaque, out_version: [*c][*c]u8) callconv(.c) void = null,
    initialize_fn: ?*const fn (user_data: ?*const anyopaque, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    shutdown_fn: ?*const fn (user_data: ?*const anyopaque, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    validate: ?*const fn (user_data: ?*const anyopaque, result: [*c]const u8, config: [*c]const u8, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    should_validate: ?*const fn (user_data: ?*const anyopaque, _result: [*c]const u8, _config: [*c]const u8) callconv(.c) i32 = null,
    priority: ?*const fn (user_data: ?*const anyopaque) callconv(.c) i32 = null,
    free_user_data: ?*const fn (?*anyopaque) callconv(.c) void = null,
};
pub const KREUZBERGKreuzbergValidatorVTable = struct_KREUZBERGKreuzbergValidatorVTable;
pub const struct_KREUZBERGKreuzbergEmbeddingBackendVTable = extern struct {
    name_fn: ?*const fn (user_data: ?*const anyopaque, out_name: [*c][*c]u8) callconv(.c) void = null,
    version_fn: ?*const fn (user_data: ?*const anyopaque, out_version: [*c][*c]u8) callconv(.c) void = null,
    initialize_fn: ?*const fn (user_data: ?*const anyopaque, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    shutdown_fn: ?*const fn (user_data: ?*const anyopaque, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    dimensions: ?*const fn (user_data: ?*const anyopaque) callconv(.c) usize = null,
    embed: ?*const fn (user_data: ?*const anyopaque, texts: [*c]const u8, out_result: [*c][*c]u8, out_error: [*c][*c]u8) callconv(.c) i32 = null,
    free_user_data: ?*const fn (?*anyopaque) callconv(.c) void = null,
};
pub const KREUZBERGKreuzbergEmbeddingBackendVTable = struct_KREUZBERGKreuzbergEmbeddingBackendVTable;
pub extern fn kreuzberg_last_error_code() i32;
pub extern fn kreuzberg_last_error_context() [*c]const u8;
pub extern fn kreuzberg_free_string(ptr: [*c]u8) void;
pub extern fn kreuzberg_version() [*c]const u8;
pub extern fn kreuzberg_acceleration_config_from_json(json: [*c]const u8) ?*KREUZBERGAccelerationConfig;
pub extern fn kreuzberg_acceleration_config_to_json(ptr: ?*const KREUZBERGAccelerationConfig) [*c]u8;
pub extern fn kreuzberg_acceleration_config_free(ptr: ?*KREUZBERGAccelerationConfig) void;
pub extern fn kreuzberg_acceleration_config_provider(ptr: ?*const KREUZBERGAccelerationConfig) ?*KREUZBERGExecutionProviderType;
pub extern fn kreuzberg_acceleration_config_device_id(ptr: ?*const KREUZBERGAccelerationConfig) u32;
pub extern fn kreuzberg_content_filter_config_from_json(json: [*c]const u8) ?*KREUZBERGContentFilterConfig;
pub extern fn kreuzberg_content_filter_config_to_json(ptr: ?*const KREUZBERGContentFilterConfig) [*c]u8;
pub extern fn kreuzberg_content_filter_config_free(ptr: ?*KREUZBERGContentFilterConfig) void;
pub extern fn kreuzberg_content_filter_config_include_headers(ptr: ?*const KREUZBERGContentFilterConfig) i32;
pub extern fn kreuzberg_content_filter_config_include_footers(ptr: ?*const KREUZBERGContentFilterConfig) i32;
pub extern fn kreuzberg_content_filter_config_strip_repeating_text(ptr: ?*const KREUZBERGContentFilterConfig) i32;
pub extern fn kreuzberg_content_filter_config_include_watermarks(ptr: ?*const KREUZBERGContentFilterConfig) i32;
pub extern fn kreuzberg_content_filter_config_default() ?*KREUZBERGContentFilterConfig;
pub extern fn kreuzberg_email_config_from_json(json: [*c]const u8) ?*KREUZBERGEmailConfig;
pub extern fn kreuzberg_email_config_to_json(ptr: ?*const KREUZBERGEmailConfig) [*c]u8;
pub extern fn kreuzberg_email_config_free(ptr: ?*KREUZBERGEmailConfig) void;
pub extern fn kreuzberg_email_config_msg_fallback_codepage(ptr: ?*const KREUZBERGEmailConfig) u32;
pub extern fn kreuzberg_extraction_config_from_json(json: [*c]const u8) ?*KREUZBERGExtractionConfig;
pub extern fn kreuzberg_extraction_config_to_json(ptr: ?*const KREUZBERGExtractionConfig) [*c]u8;
pub extern fn kreuzberg_extraction_config_free(ptr: ?*KREUZBERGExtractionConfig) void;
pub extern fn kreuzberg_extraction_config_use_cache(ptr: ?*const KREUZBERGExtractionConfig) i32;
pub extern fn kreuzberg_extraction_config_enable_quality_processing(ptr: ?*const KREUZBERGExtractionConfig) i32;
pub extern fn kreuzberg_extraction_config_ocr(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGOcrConfig;
pub extern fn kreuzberg_extraction_config_force_ocr(ptr: ?*const KREUZBERGExtractionConfig) i32;
pub extern fn kreuzberg_extraction_config_force_ocr_pages(ptr: ?*const KREUZBERGExtractionConfig) [*c]u8;
pub extern fn kreuzberg_extraction_config_disable_ocr(ptr: ?*const KREUZBERGExtractionConfig) i32;
pub extern fn kreuzberg_extraction_config_chunking(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGChunkingConfig;
pub extern fn kreuzberg_extraction_config_content_filter(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGContentFilterConfig;
pub extern fn kreuzberg_extraction_config_images(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGImageExtractionConfig;
pub extern fn kreuzberg_extraction_config_pdf_options(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGPdfConfig;
pub extern fn kreuzberg_extraction_config_token_reduction(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGTokenReductionOptions;
pub extern fn kreuzberg_extraction_config_language_detection(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGLanguageDetectionConfig;
pub extern fn kreuzberg_extraction_config_pages(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGPageConfig;
pub extern fn kreuzberg_extraction_config_keywords(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGKeywordConfig;
pub extern fn kreuzberg_extraction_config_postprocessor(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGPostProcessorConfig;
pub extern fn kreuzberg_extraction_config_html_output(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGHtmlOutputConfig;
pub extern fn kreuzberg_extraction_config_extraction_timeout_secs(ptr: ?*const KREUZBERGExtractionConfig) u64;
pub extern fn kreuzberg_extraction_config_max_concurrent_extractions(ptr: ?*const KREUZBERGExtractionConfig) usize;
pub extern fn kreuzberg_extraction_config_result_format(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGResultFormat;
pub extern fn kreuzberg_extraction_config_security_limits(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGSecurityLimits;
pub extern fn kreuzberg_extraction_config_output_format(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGOutputFormat;
pub extern fn kreuzberg_extraction_config_layout(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGLayoutDetectionConfig;
pub extern fn kreuzberg_extraction_config_include_document_structure(ptr: ?*const KREUZBERGExtractionConfig) i32;
pub extern fn kreuzberg_extraction_config_acceleration(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGAccelerationConfig;
pub extern fn kreuzberg_extraction_config_cache_namespace(ptr: ?*const KREUZBERGExtractionConfig) [*c]u8;
pub extern fn kreuzberg_extraction_config_cache_ttl_secs(ptr: ?*const KREUZBERGExtractionConfig) u64;
pub extern fn kreuzberg_extraction_config_email(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGEmailConfig;
pub extern fn kreuzberg_extraction_config_max_archive_depth(ptr: ?*const KREUZBERGExtractionConfig) usize;
pub extern fn kreuzberg_extraction_config_tree_sitter(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGTreeSitterConfig;
pub extern fn kreuzberg_extraction_config_structured_extraction(ptr: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGStructuredExtractionConfig;
pub extern fn kreuzberg_extraction_config_default() ?*KREUZBERGExtractionConfig;
pub extern fn kreuzberg_extraction_config_needs_image_processing(this_: ?*const KREUZBERGExtractionConfig) i32;
pub extern fn kreuzberg_file_extraction_config_from_json(json: [*c]const u8) ?*KREUZBERGFileExtractionConfig;
pub extern fn kreuzberg_file_extraction_config_to_json(ptr: ?*const KREUZBERGFileExtractionConfig) [*c]u8;
pub extern fn kreuzberg_file_extraction_config_free(ptr: ?*KREUZBERGFileExtractionConfig) void;
pub extern fn kreuzberg_file_extraction_config_enable_quality_processing(ptr: ?*const KREUZBERGFileExtractionConfig) i32;
pub extern fn kreuzberg_file_extraction_config_ocr(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGOcrConfig;
pub extern fn kreuzberg_file_extraction_config_force_ocr(ptr: ?*const KREUZBERGFileExtractionConfig) i32;
pub extern fn kreuzberg_file_extraction_config_force_ocr_pages(ptr: ?*const KREUZBERGFileExtractionConfig) [*c]u8;
pub extern fn kreuzberg_file_extraction_config_disable_ocr(ptr: ?*const KREUZBERGFileExtractionConfig) i32;
pub extern fn kreuzberg_file_extraction_config_chunking(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGChunkingConfig;
pub extern fn kreuzberg_file_extraction_config_content_filter(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGContentFilterConfig;
pub extern fn kreuzberg_file_extraction_config_images(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGImageExtractionConfig;
pub extern fn kreuzberg_file_extraction_config_pdf_options(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGPdfConfig;
pub extern fn kreuzberg_file_extraction_config_token_reduction(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGTokenReductionOptions;
pub extern fn kreuzberg_file_extraction_config_language_detection(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGLanguageDetectionConfig;
pub extern fn kreuzberg_file_extraction_config_pages(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGPageConfig;
pub extern fn kreuzberg_file_extraction_config_keywords(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGKeywordConfig;
pub extern fn kreuzberg_file_extraction_config_postprocessor(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGPostProcessorConfig;
pub extern fn kreuzberg_file_extraction_config_result_format(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGResultFormat;
pub extern fn kreuzberg_file_extraction_config_output_format(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGOutputFormat;
pub extern fn kreuzberg_file_extraction_config_include_document_structure(ptr: ?*const KREUZBERGFileExtractionConfig) i32;
pub extern fn kreuzberg_file_extraction_config_layout(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGLayoutDetectionConfig;
pub extern fn kreuzberg_file_extraction_config_timeout_secs(ptr: ?*const KREUZBERGFileExtractionConfig) u64;
pub extern fn kreuzberg_file_extraction_config_tree_sitter(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGTreeSitterConfig;
pub extern fn kreuzberg_file_extraction_config_structured_extraction(ptr: ?*const KREUZBERGFileExtractionConfig) ?*KREUZBERGStructuredExtractionConfig;
pub extern fn kreuzberg_batch_bytes_item_from_json(json: [*c]const u8) ?*KREUZBERGBatchBytesItem;
pub extern fn kreuzberg_batch_bytes_item_to_json(ptr: ?*const KREUZBERGBatchBytesItem) [*c]u8;
pub extern fn kreuzberg_batch_bytes_item_free(ptr: ?*KREUZBERGBatchBytesItem) void;
pub extern fn kreuzberg_batch_bytes_item_content(ptr: ?*const KREUZBERGBatchBytesItem, out_len: [*c]usize) [*c]u8;
pub extern fn kreuzberg_batch_bytes_item_mime_type(ptr: ?*const KREUZBERGBatchBytesItem) [*c]u8;
pub extern fn kreuzberg_batch_bytes_item_config(ptr: ?*const KREUZBERGBatchBytesItem) ?*KREUZBERGFileExtractionConfig;
pub extern fn kreuzberg_batch_file_item_from_json(json: [*c]const u8) ?*KREUZBERGBatchFileItem;
pub extern fn kreuzberg_batch_file_item_to_json(ptr: ?*const KREUZBERGBatchFileItem) [*c]u8;
pub extern fn kreuzberg_batch_file_item_free(ptr: ?*KREUZBERGBatchFileItem) void;
pub extern fn kreuzberg_batch_file_item_path(ptr: ?*const KREUZBERGBatchFileItem) [*c]u8;
pub extern fn kreuzberg_batch_file_item_config(ptr: ?*const KREUZBERGBatchFileItem) ?*KREUZBERGFileExtractionConfig;
pub extern fn kreuzberg_image_extraction_config_from_json(json: [*c]const u8) ?*KREUZBERGImageExtractionConfig;
pub extern fn kreuzberg_image_extraction_config_to_json(ptr: ?*const KREUZBERGImageExtractionConfig) [*c]u8;
pub extern fn kreuzberg_image_extraction_config_free(ptr: ?*KREUZBERGImageExtractionConfig) void;
pub extern fn kreuzberg_image_extraction_config_extract_images(ptr: ?*const KREUZBERGImageExtractionConfig) i32;
pub extern fn kreuzberg_image_extraction_config_target_dpi(ptr: ?*const KREUZBERGImageExtractionConfig) i32;
pub extern fn kreuzberg_image_extraction_config_max_image_dimension(ptr: ?*const KREUZBERGImageExtractionConfig) i32;
pub extern fn kreuzberg_image_extraction_config_inject_placeholders(ptr: ?*const KREUZBERGImageExtractionConfig) i32;
pub extern fn kreuzberg_image_extraction_config_auto_adjust_dpi(ptr: ?*const KREUZBERGImageExtractionConfig) i32;
pub extern fn kreuzberg_image_extraction_config_min_dpi(ptr: ?*const KREUZBERGImageExtractionConfig) i32;
pub extern fn kreuzberg_image_extraction_config_max_dpi(ptr: ?*const KREUZBERGImageExtractionConfig) i32;
pub extern fn kreuzberg_image_extraction_config_max_images_per_page(ptr: ?*const KREUZBERGImageExtractionConfig) u32;
pub extern fn kreuzberg_image_extraction_config_classify(ptr: ?*const KREUZBERGImageExtractionConfig) i32;
pub extern fn kreuzberg_image_extraction_config_default() ?*KREUZBERGImageExtractionConfig;
pub extern fn kreuzberg_token_reduction_options_from_json(json: [*c]const u8) ?*KREUZBERGTokenReductionOptions;
pub extern fn kreuzberg_token_reduction_options_to_json(ptr: ?*const KREUZBERGTokenReductionOptions) [*c]u8;
pub extern fn kreuzberg_token_reduction_options_free(ptr: ?*KREUZBERGTokenReductionOptions) void;
pub extern fn kreuzberg_token_reduction_options_mode(ptr: ?*const KREUZBERGTokenReductionOptions) [*c]u8;
pub extern fn kreuzberg_token_reduction_options_preserve_important_words(ptr: ?*const KREUZBERGTokenReductionOptions) i32;
pub extern fn kreuzberg_token_reduction_options_default() ?*KREUZBERGTokenReductionOptions;
pub extern fn kreuzberg_language_detection_config_from_json(json: [*c]const u8) ?*KREUZBERGLanguageDetectionConfig;
pub extern fn kreuzberg_language_detection_config_to_json(ptr: ?*const KREUZBERGLanguageDetectionConfig) [*c]u8;
pub extern fn kreuzberg_language_detection_config_free(ptr: ?*KREUZBERGLanguageDetectionConfig) void;
pub extern fn kreuzberg_language_detection_config_enabled(ptr: ?*const KREUZBERGLanguageDetectionConfig) i32;
pub extern fn kreuzberg_language_detection_config_min_confidence(ptr: ?*const KREUZBERGLanguageDetectionConfig) f64;
pub extern fn kreuzberg_language_detection_config_detect_multiple(ptr: ?*const KREUZBERGLanguageDetectionConfig) i32;
pub extern fn kreuzberg_language_detection_config_default() ?*KREUZBERGLanguageDetectionConfig;
pub extern fn kreuzberg_html_output_config_from_json(json: [*c]const u8) ?*KREUZBERGHtmlOutputConfig;
pub extern fn kreuzberg_html_output_config_to_json(ptr: ?*const KREUZBERGHtmlOutputConfig) [*c]u8;
pub extern fn kreuzberg_html_output_config_free(ptr: ?*KREUZBERGHtmlOutputConfig) void;
pub extern fn kreuzberg_html_output_config_css(ptr: ?*const KREUZBERGHtmlOutputConfig) [*c]u8;
pub extern fn kreuzberg_html_output_config_css_file(ptr: ?*const KREUZBERGHtmlOutputConfig) [*c]u8;
pub extern fn kreuzberg_html_output_config_theme(ptr: ?*const KREUZBERGHtmlOutputConfig) ?*KREUZBERGHtmlTheme;
pub extern fn kreuzberg_html_output_config_class_prefix(ptr: ?*const KREUZBERGHtmlOutputConfig) [*c]u8;
pub extern fn kreuzberg_html_output_config_embed_css(ptr: ?*const KREUZBERGHtmlOutputConfig) i32;
pub extern fn kreuzberg_html_output_config_default() ?*KREUZBERGHtmlOutputConfig;
pub extern fn kreuzberg_layout_detection_config_from_json(json: [*c]const u8) ?*KREUZBERGLayoutDetectionConfig;
pub extern fn kreuzberg_layout_detection_config_to_json(ptr: ?*const KREUZBERGLayoutDetectionConfig) [*c]u8;
pub extern fn kreuzberg_layout_detection_config_free(ptr: ?*KREUZBERGLayoutDetectionConfig) void;
pub extern fn kreuzberg_layout_detection_config_confidence_threshold(ptr: ?*const KREUZBERGLayoutDetectionConfig) f32;
pub extern fn kreuzberg_layout_detection_config_apply_heuristics(ptr: ?*const KREUZBERGLayoutDetectionConfig) i32;
pub extern fn kreuzberg_layout_detection_config_table_model(ptr: ?*const KREUZBERGLayoutDetectionConfig) ?*KREUZBERGTableModel;
pub extern fn kreuzberg_layout_detection_config_acceleration(ptr: ?*const KREUZBERGLayoutDetectionConfig) ?*KREUZBERGAccelerationConfig;
pub extern fn kreuzberg_layout_detection_config_default() ?*KREUZBERGLayoutDetectionConfig;
pub extern fn kreuzberg_llm_config_from_json(json: [*c]const u8) ?*KREUZBERGLlmConfig;
pub extern fn kreuzberg_llm_config_to_json(ptr: ?*const KREUZBERGLlmConfig) [*c]u8;
pub extern fn kreuzberg_llm_config_free(ptr: ?*KREUZBERGLlmConfig) void;
pub extern fn kreuzberg_llm_config_model(ptr: ?*const KREUZBERGLlmConfig) [*c]u8;
pub extern fn kreuzberg_llm_config_api_key(ptr: ?*const KREUZBERGLlmConfig) [*c]u8;
pub extern fn kreuzberg_llm_config_base_url(ptr: ?*const KREUZBERGLlmConfig) [*c]u8;
pub extern fn kreuzberg_llm_config_timeout_secs(ptr: ?*const KREUZBERGLlmConfig) u64;
pub extern fn kreuzberg_llm_config_max_retries(ptr: ?*const KREUZBERGLlmConfig) u32;
pub extern fn kreuzberg_llm_config_temperature(ptr: ?*const KREUZBERGLlmConfig) f64;
pub extern fn kreuzberg_llm_config_max_tokens(ptr: ?*const KREUZBERGLlmConfig) u64;
pub extern fn kreuzberg_structured_extraction_config_from_json(json: [*c]const u8) ?*KREUZBERGStructuredExtractionConfig;
pub extern fn kreuzberg_structured_extraction_config_to_json(ptr: ?*const KREUZBERGStructuredExtractionConfig) [*c]u8;
pub extern fn kreuzberg_structured_extraction_config_free(ptr: ?*KREUZBERGStructuredExtractionConfig) void;
pub extern fn kreuzberg_structured_extraction_config_schema(ptr: ?*const KREUZBERGStructuredExtractionConfig) [*c]u8;
pub extern fn kreuzberg_structured_extraction_config_schema_name(ptr: ?*const KREUZBERGStructuredExtractionConfig) [*c]u8;
pub extern fn kreuzberg_structured_extraction_config_schema_description(ptr: ?*const KREUZBERGStructuredExtractionConfig) [*c]u8;
pub extern fn kreuzberg_structured_extraction_config_strict(ptr: ?*const KREUZBERGStructuredExtractionConfig) i32;
pub extern fn kreuzberg_structured_extraction_config_prompt(ptr: ?*const KREUZBERGStructuredExtractionConfig) [*c]u8;
pub extern fn kreuzberg_structured_extraction_config_llm(ptr: ?*const KREUZBERGStructuredExtractionConfig) ?*KREUZBERGLlmConfig;
pub extern fn kreuzberg_ocr_quality_thresholds_from_json(json: [*c]const u8) ?*KREUZBERGOcrQualityThresholds;
pub extern fn kreuzberg_ocr_quality_thresholds_to_json(ptr: ?*const KREUZBERGOcrQualityThresholds) [*c]u8;
pub extern fn kreuzberg_ocr_quality_thresholds_free(ptr: ?*KREUZBERGOcrQualityThresholds) void;
pub extern fn kreuzberg_ocr_quality_thresholds_min_total_non_whitespace(ptr: ?*const KREUZBERGOcrQualityThresholds) usize;
pub extern fn kreuzberg_ocr_quality_thresholds_min_non_whitespace_per_page(ptr: ?*const KREUZBERGOcrQualityThresholds) f64;
pub extern fn kreuzberg_ocr_quality_thresholds_min_meaningful_word_len(ptr: ?*const KREUZBERGOcrQualityThresholds) usize;
pub extern fn kreuzberg_ocr_quality_thresholds_min_meaningful_words(ptr: ?*const KREUZBERGOcrQualityThresholds) usize;
pub extern fn kreuzberg_ocr_quality_thresholds_min_alnum_ratio(ptr: ?*const KREUZBERGOcrQualityThresholds) f64;
pub extern fn kreuzberg_ocr_quality_thresholds_min_garbage_chars(ptr: ?*const KREUZBERGOcrQualityThresholds) usize;
pub extern fn kreuzberg_ocr_quality_thresholds_max_fragmented_word_ratio(ptr: ?*const KREUZBERGOcrQualityThresholds) f64;
pub extern fn kreuzberg_ocr_quality_thresholds_critical_fragmented_word_ratio(ptr: ?*const KREUZBERGOcrQualityThresholds) f64;
pub extern fn kreuzberg_ocr_quality_thresholds_min_avg_word_length(ptr: ?*const KREUZBERGOcrQualityThresholds) f64;
pub extern fn kreuzberg_ocr_quality_thresholds_min_words_for_avg_length_check(ptr: ?*const KREUZBERGOcrQualityThresholds) usize;
pub extern fn kreuzberg_ocr_quality_thresholds_min_consecutive_repeat_ratio(ptr: ?*const KREUZBERGOcrQualityThresholds) f64;
pub extern fn kreuzberg_ocr_quality_thresholds_min_words_for_repeat_check(ptr: ?*const KREUZBERGOcrQualityThresholds) usize;
pub extern fn kreuzberg_ocr_quality_thresholds_substantive_min_chars(ptr: ?*const KREUZBERGOcrQualityThresholds) usize;
pub extern fn kreuzberg_ocr_quality_thresholds_non_text_min_chars(ptr: ?*const KREUZBERGOcrQualityThresholds) usize;
pub extern fn kreuzberg_ocr_quality_thresholds_alnum_ws_ratio_threshold(ptr: ?*const KREUZBERGOcrQualityThresholds) f64;
pub extern fn kreuzberg_ocr_quality_thresholds_pipeline_min_quality(ptr: ?*const KREUZBERGOcrQualityThresholds) f64;
pub extern fn kreuzberg_ocr_quality_thresholds_default() ?*KREUZBERGOcrQualityThresholds;
pub extern fn kreuzberg_ocr_pipeline_stage_from_json(json: [*c]const u8) ?*KREUZBERGOcrPipelineStage;
pub extern fn kreuzberg_ocr_pipeline_stage_to_json(ptr: ?*const KREUZBERGOcrPipelineStage) [*c]u8;
pub extern fn kreuzberg_ocr_pipeline_stage_free(ptr: ?*KREUZBERGOcrPipelineStage) void;
pub extern fn kreuzberg_ocr_pipeline_stage_backend(ptr: ?*const KREUZBERGOcrPipelineStage) [*c]u8;
pub extern fn kreuzberg_ocr_pipeline_stage_priority(ptr: ?*const KREUZBERGOcrPipelineStage) u32;
pub extern fn kreuzberg_ocr_pipeline_stage_language(ptr: ?*const KREUZBERGOcrPipelineStage) [*c]u8;
pub extern fn kreuzberg_ocr_pipeline_stage_tesseract_config(ptr: ?*const KREUZBERGOcrPipelineStage) ?*KREUZBERGTesseractConfig;
pub extern fn kreuzberg_ocr_pipeline_stage_paddle_ocr_config(ptr: ?*const KREUZBERGOcrPipelineStage) [*c]u8;
pub extern fn kreuzberg_ocr_pipeline_stage_vlm_config(ptr: ?*const KREUZBERGOcrPipelineStage) ?*KREUZBERGLlmConfig;
pub extern fn kreuzberg_ocr_pipeline_config_from_json(json: [*c]const u8) ?*KREUZBERGOcrPipelineConfig;
pub extern fn kreuzberg_ocr_pipeline_config_to_json(ptr: ?*const KREUZBERGOcrPipelineConfig) [*c]u8;
pub extern fn kreuzberg_ocr_pipeline_config_free(ptr: ?*KREUZBERGOcrPipelineConfig) void;
pub extern fn kreuzberg_ocr_pipeline_config_stages(ptr: ?*const KREUZBERGOcrPipelineConfig) [*c]u8;
pub extern fn kreuzberg_ocr_pipeline_config_quality_thresholds(ptr: ?*const KREUZBERGOcrPipelineConfig) ?*KREUZBERGOcrQualityThresholds;
pub extern fn kreuzberg_ocr_config_from_json(json: [*c]const u8) ?*KREUZBERGOcrConfig;
pub extern fn kreuzberg_ocr_config_to_json(ptr: ?*const KREUZBERGOcrConfig) [*c]u8;
pub extern fn kreuzberg_ocr_config_free(ptr: ?*KREUZBERGOcrConfig) void;
pub extern fn kreuzberg_ocr_config_enabled(ptr: ?*const KREUZBERGOcrConfig) i32;
pub extern fn kreuzberg_ocr_config_backend(ptr: ?*const KREUZBERGOcrConfig) [*c]u8;
pub extern fn kreuzberg_ocr_config_language(ptr: ?*const KREUZBERGOcrConfig) [*c]u8;
pub extern fn kreuzberg_ocr_config_tesseract_config(ptr: ?*const KREUZBERGOcrConfig) ?*KREUZBERGTesseractConfig;
pub extern fn kreuzberg_ocr_config_output_format(ptr: ?*const KREUZBERGOcrConfig) ?*KREUZBERGOutputFormat;
pub extern fn kreuzberg_ocr_config_paddle_ocr_config(ptr: ?*const KREUZBERGOcrConfig) [*c]u8;
pub extern fn kreuzberg_ocr_config_element_config(ptr: ?*const KREUZBERGOcrConfig) ?*KREUZBERGOcrElementConfig;
pub extern fn kreuzberg_ocr_config_quality_thresholds(ptr: ?*const KREUZBERGOcrConfig) ?*KREUZBERGOcrQualityThresholds;
pub extern fn kreuzberg_ocr_config_pipeline(ptr: ?*const KREUZBERGOcrConfig) ?*KREUZBERGOcrPipelineConfig;
pub extern fn kreuzberg_ocr_config_auto_rotate(ptr: ?*const KREUZBERGOcrConfig) i32;
pub extern fn kreuzberg_ocr_config_vlm_config(ptr: ?*const KREUZBERGOcrConfig) ?*KREUZBERGLlmConfig;
pub extern fn kreuzberg_ocr_config_vlm_prompt(ptr: ?*const KREUZBERGOcrConfig) [*c]u8;
pub extern fn kreuzberg_ocr_config_acceleration(ptr: ?*const KREUZBERGOcrConfig) ?*KREUZBERGAccelerationConfig;
pub extern fn kreuzberg_ocr_config_default() ?*KREUZBERGOcrConfig;
pub extern fn kreuzberg_page_config_from_json(json: [*c]const u8) ?*KREUZBERGPageConfig;
pub extern fn kreuzberg_page_config_to_json(ptr: ?*const KREUZBERGPageConfig) [*c]u8;
pub extern fn kreuzberg_page_config_free(ptr: ?*KREUZBERGPageConfig) void;
pub extern fn kreuzberg_page_config_extract_pages(ptr: ?*const KREUZBERGPageConfig) i32;
pub extern fn kreuzberg_page_config_insert_page_markers(ptr: ?*const KREUZBERGPageConfig) i32;
pub extern fn kreuzberg_page_config_marker_format(ptr: ?*const KREUZBERGPageConfig) [*c]u8;
pub extern fn kreuzberg_page_config_default() ?*KREUZBERGPageConfig;
pub extern fn kreuzberg_pdf_config_from_json(json: [*c]const u8) ?*KREUZBERGPdfConfig;
pub extern fn kreuzberg_pdf_config_to_json(ptr: ?*const KREUZBERGPdfConfig) [*c]u8;
pub extern fn kreuzberg_pdf_config_free(ptr: ?*KREUZBERGPdfConfig) void;
pub extern fn kreuzberg_pdf_config_extract_images(ptr: ?*const KREUZBERGPdfConfig) i32;
pub extern fn kreuzberg_pdf_config_passwords(ptr: ?*const KREUZBERGPdfConfig) [*c]u8;
pub extern fn kreuzberg_pdf_config_extract_metadata(ptr: ?*const KREUZBERGPdfConfig) i32;
pub extern fn kreuzberg_pdf_config_hierarchy(ptr: ?*const KREUZBERGPdfConfig) ?*KREUZBERGHierarchyConfig;
pub extern fn kreuzberg_pdf_config_extract_annotations(ptr: ?*const KREUZBERGPdfConfig) i32;
pub extern fn kreuzberg_pdf_config_top_margin_fraction(ptr: ?*const KREUZBERGPdfConfig) f32;
pub extern fn kreuzberg_pdf_config_bottom_margin_fraction(ptr: ?*const KREUZBERGPdfConfig) f32;
pub extern fn kreuzberg_pdf_config_allow_single_column_tables(ptr: ?*const KREUZBERGPdfConfig) i32;
pub extern fn kreuzberg_pdf_config_default() ?*KREUZBERGPdfConfig;
pub extern fn kreuzberg_hierarchy_config_from_json(json: [*c]const u8) ?*KREUZBERGHierarchyConfig;
pub extern fn kreuzberg_hierarchy_config_to_json(ptr: ?*const KREUZBERGHierarchyConfig) [*c]u8;
pub extern fn kreuzberg_hierarchy_config_free(ptr: ?*KREUZBERGHierarchyConfig) void;
pub extern fn kreuzberg_hierarchy_config_enabled(ptr: ?*const KREUZBERGHierarchyConfig) i32;
pub extern fn kreuzberg_hierarchy_config_k_clusters(ptr: ?*const KREUZBERGHierarchyConfig) usize;
pub extern fn kreuzberg_hierarchy_config_include_bbox(ptr: ?*const KREUZBERGHierarchyConfig) i32;
pub extern fn kreuzberg_hierarchy_config_ocr_coverage_threshold(ptr: ?*const KREUZBERGHierarchyConfig) f32;
pub extern fn kreuzberg_hierarchy_config_default() ?*KREUZBERGHierarchyConfig;
pub extern fn kreuzberg_post_processor_config_from_json(json: [*c]const u8) ?*KREUZBERGPostProcessorConfig;
pub extern fn kreuzberg_post_processor_config_to_json(ptr: ?*const KREUZBERGPostProcessorConfig) [*c]u8;
pub extern fn kreuzberg_post_processor_config_free(ptr: ?*KREUZBERGPostProcessorConfig) void;
pub extern fn kreuzberg_post_processor_config_enabled(ptr: ?*const KREUZBERGPostProcessorConfig) i32;
pub extern fn kreuzberg_post_processor_config_enabled_processors(ptr: ?*const KREUZBERGPostProcessorConfig) [*c]u8;
pub extern fn kreuzberg_post_processor_config_disabled_processors(ptr: ?*const KREUZBERGPostProcessorConfig) [*c]u8;
pub extern fn kreuzberg_post_processor_config_default() ?*KREUZBERGPostProcessorConfig;
pub extern fn kreuzberg_chunking_config_from_json(json: [*c]const u8) ?*KREUZBERGChunkingConfig;
pub extern fn kreuzberg_chunking_config_to_json(ptr: ?*const KREUZBERGChunkingConfig) [*c]u8;
pub extern fn kreuzberg_chunking_config_free(ptr: ?*KREUZBERGChunkingConfig) void;
pub extern fn kreuzberg_chunking_config_max_characters(ptr: ?*const KREUZBERGChunkingConfig) usize;
pub extern fn kreuzberg_chunking_config_overlap(ptr: ?*const KREUZBERGChunkingConfig) usize;
pub extern fn kreuzberg_chunking_config_trim(ptr: ?*const KREUZBERGChunkingConfig) i32;
pub extern fn kreuzberg_chunking_config_chunker_type(ptr: ?*const KREUZBERGChunkingConfig) ?*KREUZBERGChunkerType;
pub extern fn kreuzberg_chunking_config_embedding(ptr: ?*const KREUZBERGChunkingConfig) ?*KREUZBERGEmbeddingConfig;
pub extern fn kreuzberg_chunking_config_preset(ptr: ?*const KREUZBERGChunkingConfig) [*c]u8;
pub extern fn kreuzberg_chunking_config_sizing(ptr: ?*const KREUZBERGChunkingConfig) ?*KREUZBERGChunkSizing;
pub extern fn kreuzberg_chunking_config_prepend_heading_context(ptr: ?*const KREUZBERGChunkingConfig) i32;
pub extern fn kreuzberg_chunking_config_topic_threshold(ptr: ?*const KREUZBERGChunkingConfig) f32;
pub extern fn kreuzberg_chunking_config_default() ?*KREUZBERGChunkingConfig;
pub extern fn kreuzberg_embedding_config_from_json(json: [*c]const u8) ?*KREUZBERGEmbeddingConfig;
pub extern fn kreuzberg_embedding_config_to_json(ptr: ?*const KREUZBERGEmbeddingConfig) [*c]u8;
pub extern fn kreuzberg_embedding_config_free(ptr: ?*KREUZBERGEmbeddingConfig) void;
pub extern fn kreuzberg_embedding_config_model(ptr: ?*const KREUZBERGEmbeddingConfig) ?*KREUZBERGEmbeddingModelType;
pub extern fn kreuzberg_embedding_config_normalize(ptr: ?*const KREUZBERGEmbeddingConfig) i32;
pub extern fn kreuzberg_embedding_config_batch_size(ptr: ?*const KREUZBERGEmbeddingConfig) usize;
pub extern fn kreuzberg_embedding_config_show_download_progress(ptr: ?*const KREUZBERGEmbeddingConfig) i32;
pub extern fn kreuzberg_embedding_config_cache_dir(ptr: ?*const KREUZBERGEmbeddingConfig) [*c]u8;
pub extern fn kreuzberg_embedding_config_acceleration(ptr: ?*const KREUZBERGEmbeddingConfig) ?*KREUZBERGAccelerationConfig;
pub extern fn kreuzberg_embedding_config_max_embed_duration_secs(ptr: ?*const KREUZBERGEmbeddingConfig) u64;
pub extern fn kreuzberg_embedding_config_default() ?*KREUZBERGEmbeddingConfig;
pub extern fn kreuzberg_tree_sitter_config_from_json(json: [*c]const u8) ?*KREUZBERGTreeSitterConfig;
pub extern fn kreuzberg_tree_sitter_config_to_json(ptr: ?*const KREUZBERGTreeSitterConfig) [*c]u8;
pub extern fn kreuzberg_tree_sitter_config_free(ptr: ?*KREUZBERGTreeSitterConfig) void;
pub extern fn kreuzberg_tree_sitter_config_enabled(ptr: ?*const KREUZBERGTreeSitterConfig) i32;
pub extern fn kreuzberg_tree_sitter_config_cache_dir(ptr: ?*const KREUZBERGTreeSitterConfig) [*c]u8;
pub extern fn kreuzberg_tree_sitter_config_languages(ptr: ?*const KREUZBERGTreeSitterConfig) [*c]u8;
pub extern fn kreuzberg_tree_sitter_config_groups(ptr: ?*const KREUZBERGTreeSitterConfig) [*c]u8;
pub extern fn kreuzberg_tree_sitter_config_process(ptr: ?*const KREUZBERGTreeSitterConfig) ?*KREUZBERGTreeSitterProcessConfig;
pub extern fn kreuzberg_tree_sitter_config_default() ?*KREUZBERGTreeSitterConfig;
pub extern fn kreuzberg_tree_sitter_process_config_from_json(json: [*c]const u8) ?*KREUZBERGTreeSitterProcessConfig;
pub extern fn kreuzberg_tree_sitter_process_config_to_json(ptr: ?*const KREUZBERGTreeSitterProcessConfig) [*c]u8;
pub extern fn kreuzberg_tree_sitter_process_config_free(ptr: ?*KREUZBERGTreeSitterProcessConfig) void;
pub extern fn kreuzberg_tree_sitter_process_config_structure(ptr: ?*const KREUZBERGTreeSitterProcessConfig) i32;
pub extern fn kreuzberg_tree_sitter_process_config_imports(ptr: ?*const KREUZBERGTreeSitterProcessConfig) i32;
pub extern fn kreuzberg_tree_sitter_process_config_exports(ptr: ?*const KREUZBERGTreeSitterProcessConfig) i32;
pub extern fn kreuzberg_tree_sitter_process_config_comments(ptr: ?*const KREUZBERGTreeSitterProcessConfig) i32;
pub extern fn kreuzberg_tree_sitter_process_config_docstrings(ptr: ?*const KREUZBERGTreeSitterProcessConfig) i32;
pub extern fn kreuzberg_tree_sitter_process_config_symbols(ptr: ?*const KREUZBERGTreeSitterProcessConfig) i32;
pub extern fn kreuzberg_tree_sitter_process_config_diagnostics(ptr: ?*const KREUZBERGTreeSitterProcessConfig) i32;
pub extern fn kreuzberg_tree_sitter_process_config_chunk_max_size(ptr: ?*const KREUZBERGTreeSitterProcessConfig) usize;
pub extern fn kreuzberg_tree_sitter_process_config_content_mode(ptr: ?*const KREUZBERGTreeSitterProcessConfig) ?*KREUZBERGCodeContentMode;
pub extern fn kreuzberg_tree_sitter_process_config_default() ?*KREUZBERGTreeSitterProcessConfig;
pub extern fn kreuzberg_supported_format_from_json(json: [*c]const u8) ?*KREUZBERGSupportedFormat;
pub extern fn kreuzberg_supported_format_to_json(ptr: ?*const KREUZBERGSupportedFormat) [*c]u8;
pub extern fn kreuzberg_supported_format_free(ptr: ?*KREUZBERGSupportedFormat) void;
pub extern fn kreuzberg_supported_format_extension(ptr: ?*const KREUZBERGSupportedFormat) [*c]u8;
pub extern fn kreuzberg_supported_format_mime_type(ptr: ?*const KREUZBERGSupportedFormat) [*c]u8;
pub extern fn kreuzberg_server_config_from_json(json: [*c]const u8) ?*KREUZBERGServerConfig;
pub extern fn kreuzberg_server_config_to_json(ptr: ?*const KREUZBERGServerConfig) [*c]u8;
pub extern fn kreuzberg_server_config_free(ptr: ?*KREUZBERGServerConfig) void;
pub extern fn kreuzberg_server_config_host(ptr: ?*const KREUZBERGServerConfig) [*c]u8;
pub extern fn kreuzberg_server_config_port(ptr: ?*const KREUZBERGServerConfig) u16;
pub extern fn kreuzberg_server_config_cors_origins(ptr: ?*const KREUZBERGServerConfig) [*c]u8;
pub extern fn kreuzberg_server_config_max_request_body_bytes(ptr: ?*const KREUZBERGServerConfig) usize;
pub extern fn kreuzberg_server_config_max_multipart_field_bytes(ptr: ?*const KREUZBERGServerConfig) usize;
pub extern fn kreuzberg_server_config_default() ?*KREUZBERGServerConfig;
pub extern fn kreuzberg_server_config_listen_addr(this_: ?*const KREUZBERGServerConfig) [*c]u8;
pub extern fn kreuzberg_server_config_cors_allows_all(this_: ?*const KREUZBERGServerConfig) i32;
pub extern fn kreuzberg_server_config_is_origin_allowed(this_: ?*const KREUZBERGServerConfig, origin: [*c]const u8) i32;
pub extern fn kreuzberg_server_config_max_request_body_mb(this_: ?*const KREUZBERGServerConfig) usize;
pub extern fn kreuzberg_server_config_max_multipart_field_mb(this_: ?*const KREUZBERGServerConfig) usize;
pub extern fn kreuzberg_structured_data_result_from_json(json: [*c]const u8) ?*KREUZBERGStructuredDataResult;
pub extern fn kreuzberg_structured_data_result_to_json(ptr: ?*const KREUZBERGStructuredDataResult) [*c]u8;
pub extern fn kreuzberg_structured_data_result_free(ptr: ?*KREUZBERGStructuredDataResult) void;
pub extern fn kreuzberg_structured_data_result_content(ptr: ?*const KREUZBERGStructuredDataResult) [*c]u8;
pub extern fn kreuzberg_structured_data_result_metadata(ptr: ?*const KREUZBERGStructuredDataResult) [*c]u8;
pub extern fn kreuzberg_structured_data_result_text_fields(ptr: ?*const KREUZBERGStructuredDataResult) [*c]u8;
pub extern fn kreuzberg_char_shape_free(ptr: ?*KREUZBERGCharShape) void;
pub extern fn kreuzberg_char_shape_bold(ptr: ?*const KREUZBERGCharShape) i32;
pub extern fn kreuzberg_char_shape_italic(ptr: ?*const KREUZBERGCharShape) i32;
pub extern fn kreuzberg_char_shape_underline(ptr: ?*const KREUZBERGCharShape) i32;
pub extern fn kreuzberg_hwp_image_free(ptr: ?*KREUZBERGHwpImage) void;
pub extern fn kreuzberg_hwp_image_name(ptr: ?*const KREUZBERGHwpImage) [*c]u8;
pub extern fn kreuzberg_hwp_image_data(ptr: ?*const KREUZBERGHwpImage, out_len: [*c]usize) [*c]u8;
pub extern fn kreuzberg_stream_reader_free(ptr: ?*KREUZBERGStreamReader) void;
pub extern fn kreuzberg_image_ocr_result_free(ptr: ?*KREUZBERGImageOcrResult) void;
pub extern fn kreuzberg_image_ocr_result_content(ptr: ?*const KREUZBERGImageOcrResult) [*c]u8;
pub extern fn kreuzberg_image_ocr_result_boundaries(ptr: ?*const KREUZBERGImageOcrResult) [*c]u8;
pub extern fn kreuzberg_image_ocr_result_page_contents(ptr: ?*const KREUZBERGImageOcrResult) [*c]u8;
pub extern fn kreuzberg_html_extraction_result_from_json(json: [*c]const u8) ?*KREUZBERGHtmlExtractionResult;
pub extern fn kreuzberg_html_extraction_result_to_json(ptr: ?*const KREUZBERGHtmlExtractionResult) [*c]u8;
pub extern fn kreuzberg_html_extraction_result_free(ptr: ?*KREUZBERGHtmlExtractionResult) void;
pub extern fn kreuzberg_html_extraction_result_markdown(ptr: ?*const KREUZBERGHtmlExtractionResult) [*c]u8;
pub extern fn kreuzberg_html_extraction_result_images(ptr: ?*const KREUZBERGHtmlExtractionResult) [*c]u8;
pub extern fn kreuzberg_html_extraction_result_warnings(ptr: ?*const KREUZBERGHtmlExtractionResult) [*c]u8;
pub extern fn kreuzberg_extracted_inline_image_from_json(json: [*c]const u8) ?*KREUZBERGExtractedInlineImage;
pub extern fn kreuzberg_extracted_inline_image_to_json(ptr: ?*const KREUZBERGExtractedInlineImage) [*c]u8;
pub extern fn kreuzberg_extracted_inline_image_free(ptr: ?*KREUZBERGExtractedInlineImage) void;
pub extern fn kreuzberg_extracted_inline_image_data(ptr: ?*const KREUZBERGExtractedInlineImage, out_len: [*c]usize) [*c]u8;
pub extern fn kreuzberg_extracted_inline_image_format(ptr: ?*const KREUZBERGExtractedInlineImage) [*c]u8;
pub extern fn kreuzberg_extracted_inline_image_filename(ptr: ?*const KREUZBERGExtractedInlineImage) [*c]u8;
pub extern fn kreuzberg_extracted_inline_image_description(ptr: ?*const KREUZBERGExtractedInlineImage) [*c]u8;
pub extern fn kreuzberg_drawing_from_json(json: [*c]const u8) ?*KREUZBERGDrawing;
pub extern fn kreuzberg_drawing_to_json(ptr: ?*const KREUZBERGDrawing) [*c]u8;
pub extern fn kreuzberg_drawing_free(ptr: ?*KREUZBERGDrawing) void;
pub extern fn kreuzberg_drawing_image_ref(ptr: ?*const KREUZBERGDrawing) [*c]u8;
pub extern fn kreuzberg_anchor_properties_from_json(json: [*c]const u8) ?*KREUZBERGAnchorProperties;
pub extern fn kreuzberg_anchor_properties_to_json(ptr: ?*const KREUZBERGAnchorProperties) [*c]u8;
pub extern fn kreuzberg_anchor_properties_free(ptr: ?*KREUZBERGAnchorProperties) void;
pub extern fn kreuzberg_anchor_properties_behind_doc(ptr: ?*const KREUZBERGAnchorProperties) i32;
pub extern fn kreuzberg_anchor_properties_layout_in_cell(ptr: ?*const KREUZBERGAnchorProperties) i32;
pub extern fn kreuzberg_anchor_properties_relative_height(ptr: ?*const KREUZBERGAnchorProperties) i64;
pub extern fn kreuzberg_page_margins_points_free(ptr: ?*KREUZBERGPageMarginsPoints) void;
pub extern fn kreuzberg_page_margins_points_top(ptr: ?*const KREUZBERGPageMarginsPoints) f64;
pub extern fn kreuzberg_page_margins_points_right(ptr: ?*const KREUZBERGPageMarginsPoints) f64;
pub extern fn kreuzberg_page_margins_points_bottom(ptr: ?*const KREUZBERGPageMarginsPoints) f64;
pub extern fn kreuzberg_page_margins_points_left(ptr: ?*const KREUZBERGPageMarginsPoints) f64;
pub extern fn kreuzberg_page_margins_points_header(ptr: ?*const KREUZBERGPageMarginsPoints) f64;
pub extern fn kreuzberg_page_margins_points_footer(ptr: ?*const KREUZBERGPageMarginsPoints) f64;
pub extern fn kreuzberg_page_margins_points_gutter(ptr: ?*const KREUZBERGPageMarginsPoints) f64;
pub extern fn kreuzberg_style_definition_free(ptr: ?*KREUZBERGStyleDefinition) void;
pub extern fn kreuzberg_style_definition_id(ptr: ?*const KREUZBERGStyleDefinition) [*c]u8;
pub extern fn kreuzberg_style_definition_name(ptr: ?*const KREUZBERGStyleDefinition) [*c]u8;
pub extern fn kreuzberg_style_definition_based_on(ptr: ?*const KREUZBERGStyleDefinition) [*c]u8;
pub extern fn kreuzberg_style_definition_next_style(ptr: ?*const KREUZBERGStyleDefinition) [*c]u8;
pub extern fn kreuzberg_style_definition_is_default(ptr: ?*const KREUZBERGStyleDefinition) i32;
pub extern fn kreuzberg_resolved_style_free(ptr: ?*KREUZBERGResolvedStyle) void;
pub extern fn kreuzberg_table_properties_from_json(json: [*c]const u8) ?*KREUZBERGTableProperties;
pub extern fn kreuzberg_table_properties_to_json(ptr: ?*const KREUZBERGTableProperties) [*c]u8;
pub extern fn kreuzberg_table_properties_free(ptr: ?*KREUZBERGTableProperties) void;
pub extern fn kreuzberg_table_properties_style_id(ptr: ?*const KREUZBERGTableProperties) [*c]u8;
pub extern fn kreuzberg_table_properties_alignment(ptr: ?*const KREUZBERGTableProperties) [*c]u8;
pub extern fn kreuzberg_table_properties_layout(ptr: ?*const KREUZBERGTableProperties) [*c]u8;
pub extern fn kreuzberg_table_properties_caption(ptr: ?*const KREUZBERGTableProperties) [*c]u8;
pub extern fn kreuzberg_xlsx_app_properties_from_json(json: [*c]const u8) ?*KREUZBERGXlsxAppProperties;
pub extern fn kreuzberg_xlsx_app_properties_to_json(ptr: ?*const KREUZBERGXlsxAppProperties) [*c]u8;
pub extern fn kreuzberg_xlsx_app_properties_free(ptr: ?*KREUZBERGXlsxAppProperties) void;
pub extern fn kreuzberg_xlsx_app_properties_application(ptr: ?*const KREUZBERGXlsxAppProperties) [*c]u8;
pub extern fn kreuzberg_xlsx_app_properties_app_version(ptr: ?*const KREUZBERGXlsxAppProperties) [*c]u8;
pub extern fn kreuzberg_xlsx_app_properties_doc_security(ptr: ?*const KREUZBERGXlsxAppProperties) i32;
pub extern fn kreuzberg_xlsx_app_properties_scale_crop(ptr: ?*const KREUZBERGXlsxAppProperties) i32;
pub extern fn kreuzberg_xlsx_app_properties_links_up_to_date(ptr: ?*const KREUZBERGXlsxAppProperties) i32;
pub extern fn kreuzberg_xlsx_app_properties_shared_doc(ptr: ?*const KREUZBERGXlsxAppProperties) i32;
pub extern fn kreuzberg_xlsx_app_properties_hyperlinks_changed(ptr: ?*const KREUZBERGXlsxAppProperties) i32;
pub extern fn kreuzberg_xlsx_app_properties_company(ptr: ?*const KREUZBERGXlsxAppProperties) [*c]u8;
pub extern fn kreuzberg_xlsx_app_properties_worksheet_names(ptr: ?*const KREUZBERGXlsxAppProperties) [*c]u8;
pub extern fn kreuzberg_pptx_app_properties_from_json(json: [*c]const u8) ?*KREUZBERGPptxAppProperties;
pub extern fn kreuzberg_pptx_app_properties_to_json(ptr: ?*const KREUZBERGPptxAppProperties) [*c]u8;
pub extern fn kreuzberg_pptx_app_properties_free(ptr: ?*KREUZBERGPptxAppProperties) void;
pub extern fn kreuzberg_pptx_app_properties_application(ptr: ?*const KREUZBERGPptxAppProperties) [*c]u8;
pub extern fn kreuzberg_pptx_app_properties_app_version(ptr: ?*const KREUZBERGPptxAppProperties) [*c]u8;
pub extern fn kreuzberg_pptx_app_properties_total_time(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_company(ptr: ?*const KREUZBERGPptxAppProperties) [*c]u8;
pub extern fn kreuzberg_pptx_app_properties_doc_security(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_scale_crop(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_links_up_to_date(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_shared_doc(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_hyperlinks_changed(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_slides(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_notes(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_hidden_slides(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_multimedia_clips(ptr: ?*const KREUZBERGPptxAppProperties) i32;
pub extern fn kreuzberg_pptx_app_properties_presentation_format(ptr: ?*const KREUZBERGPptxAppProperties) [*c]u8;
pub extern fn kreuzberg_pptx_app_properties_slide_titles(ptr: ?*const KREUZBERGPptxAppProperties) [*c]u8;
pub extern fn kreuzberg_custom_properties_free(ptr: ?*KREUZBERGCustomProperties) void;
pub extern fn kreuzberg_odt_properties_free(ptr: ?*KREUZBERGOdtProperties) void;
pub extern fn kreuzberg_odt_properties_title(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_subject(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_creator(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_initial_creator(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_keywords(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_description(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_date(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_creation_date(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_language(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_generator(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_editing_duration(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_editing_cycles(ptr: ?*const KREUZBERGOdtProperties) [*c]u8;
pub extern fn kreuzberg_odt_properties_page_count(ptr: ?*const KREUZBERGOdtProperties) i32;
pub extern fn kreuzberg_odt_properties_word_count(ptr: ?*const KREUZBERGOdtProperties) i32;
pub extern fn kreuzberg_odt_properties_character_count(ptr: ?*const KREUZBERGOdtProperties) i32;
pub extern fn kreuzberg_odt_properties_paragraph_count(ptr: ?*const KREUZBERGOdtProperties) i32;
pub extern fn kreuzberg_odt_properties_table_count(ptr: ?*const KREUZBERGOdtProperties) i32;
pub extern fn kreuzberg_odt_properties_image_count(ptr: ?*const KREUZBERGOdtProperties) i32;
pub extern fn kreuzberg_security_limits_from_json(json: [*c]const u8) ?*KREUZBERGSecurityLimits;
pub extern fn kreuzberg_security_limits_to_json(ptr: ?*const KREUZBERGSecurityLimits) [*c]u8;
pub extern fn kreuzberg_security_limits_free(ptr: ?*KREUZBERGSecurityLimits) void;
pub extern fn kreuzberg_security_limits_max_archive_size(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_max_compression_ratio(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_max_files_in_archive(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_max_nesting_depth(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_max_entity_length(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_max_content_size(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_max_iterations(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_max_xml_depth(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_max_table_cells(ptr: ?*const KREUZBERGSecurityLimits) usize;
pub extern fn kreuzberg_security_limits_default() ?*KREUZBERGSecurityLimits;
pub extern fn kreuzberg_zip_bomb_validator_free(ptr: ?*KREUZBERGZipBombValidator) void;
pub extern fn kreuzberg_token_reduction_config_from_json(json: [*c]const u8) ?*KREUZBERGTokenReductionConfig;
pub extern fn kreuzberg_token_reduction_config_to_json(ptr: ?*const KREUZBERGTokenReductionConfig) [*c]u8;
pub extern fn kreuzberg_token_reduction_config_free(ptr: ?*KREUZBERGTokenReductionConfig) void;
pub extern fn kreuzberg_token_reduction_config_level(ptr: ?*const KREUZBERGTokenReductionConfig) ?*KREUZBERGReductionLevel;
pub extern fn kreuzberg_token_reduction_config_language_hint(ptr: ?*const KREUZBERGTokenReductionConfig) [*c]u8;
pub extern fn kreuzberg_token_reduction_config_preserve_markdown(ptr: ?*const KREUZBERGTokenReductionConfig) i32;
pub extern fn kreuzberg_token_reduction_config_preserve_code(ptr: ?*const KREUZBERGTokenReductionConfig) i32;
pub extern fn kreuzberg_token_reduction_config_semantic_threshold(ptr: ?*const KREUZBERGTokenReductionConfig) f32;
pub extern fn kreuzberg_token_reduction_config_enable_parallel(ptr: ?*const KREUZBERGTokenReductionConfig) i32;
pub extern fn kreuzberg_token_reduction_config_use_simd(ptr: ?*const KREUZBERGTokenReductionConfig) i32;
pub extern fn kreuzberg_token_reduction_config_custom_stopwords(ptr: ?*const KREUZBERGTokenReductionConfig) [*c]u8;
pub extern fn kreuzberg_token_reduction_config_preserve_patterns(ptr: ?*const KREUZBERGTokenReductionConfig) [*c]u8;
pub extern fn kreuzberg_token_reduction_config_target_reduction(ptr: ?*const KREUZBERGTokenReductionConfig) f32;
pub extern fn kreuzberg_token_reduction_config_enable_semantic_clustering(ptr: ?*const KREUZBERGTokenReductionConfig) i32;
pub extern fn kreuzberg_token_reduction_config_default() ?*KREUZBERGTokenReductionConfig;
pub extern fn kreuzberg_pdf_annotation_from_json(json: [*c]const u8) ?*KREUZBERGPdfAnnotation;
pub extern fn kreuzberg_pdf_annotation_to_json(ptr: ?*const KREUZBERGPdfAnnotation) [*c]u8;
pub extern fn kreuzberg_pdf_annotation_free(ptr: ?*KREUZBERGPdfAnnotation) void;
pub extern fn kreuzberg_pdf_annotation_annotation_type(ptr: ?*const KREUZBERGPdfAnnotation) ?*KREUZBERGPdfAnnotationType;
pub extern fn kreuzberg_pdf_annotation_content(ptr: ?*const KREUZBERGPdfAnnotation) [*c]u8;
pub extern fn kreuzberg_pdf_annotation_page_number(ptr: ?*const KREUZBERGPdfAnnotation) usize;
pub extern fn kreuzberg_djot_content_from_json(json: [*c]const u8) ?*KREUZBERGDjotContent;
pub extern fn kreuzberg_djot_content_to_json(ptr: ?*const KREUZBERGDjotContent) [*c]u8;
pub extern fn kreuzberg_djot_content_free(ptr: ?*KREUZBERGDjotContent) void;
pub extern fn kreuzberg_djot_content_plain_text(ptr: ?*const KREUZBERGDjotContent) [*c]u8;
pub extern fn kreuzberg_djot_content_blocks(ptr: ?*const KREUZBERGDjotContent) [*c]u8;
pub extern fn kreuzberg_djot_content_metadata(ptr: ?*const KREUZBERGDjotContent) ?*KREUZBERGMetadata;
pub extern fn kreuzberg_djot_content_tables(ptr: ?*const KREUZBERGDjotContent) [*c]u8;
pub extern fn kreuzberg_djot_content_images(ptr: ?*const KREUZBERGDjotContent) [*c]u8;
pub extern fn kreuzberg_djot_content_links(ptr: ?*const KREUZBERGDjotContent) [*c]u8;
pub extern fn kreuzberg_djot_content_footnotes(ptr: ?*const KREUZBERGDjotContent) [*c]u8;
pub extern fn kreuzberg_formatted_block_from_json(json: [*c]const u8) ?*KREUZBERGFormattedBlock;
pub extern fn kreuzberg_formatted_block_to_json(ptr: ?*const KREUZBERGFormattedBlock) [*c]u8;
pub extern fn kreuzberg_formatted_block_free(ptr: ?*KREUZBERGFormattedBlock) void;
pub extern fn kreuzberg_formatted_block_block_type(ptr: ?*const KREUZBERGFormattedBlock) ?*KREUZBERGBlockType;
pub extern fn kreuzberg_formatted_block_level(ptr: ?*const KREUZBERGFormattedBlock) usize;
pub extern fn kreuzberg_formatted_block_inline_content(ptr: ?*const KREUZBERGFormattedBlock) [*c]u8;
pub extern fn kreuzberg_formatted_block_language(ptr: ?*const KREUZBERGFormattedBlock) [*c]u8;
pub extern fn kreuzberg_formatted_block_code(ptr: ?*const KREUZBERGFormattedBlock) [*c]u8;
pub extern fn kreuzberg_formatted_block_children(ptr: ?*const KREUZBERGFormattedBlock) [*c]u8;
pub extern fn kreuzberg_inline_element_from_json(json: [*c]const u8) ?*KREUZBERGInlineElement;
pub extern fn kreuzberg_inline_element_to_json(ptr: ?*const KREUZBERGInlineElement) [*c]u8;
pub extern fn kreuzberg_inline_element_free(ptr: ?*KREUZBERGInlineElement) void;
pub extern fn kreuzberg_inline_element_element_type(ptr: ?*const KREUZBERGInlineElement) ?*KREUZBERGInlineType;
pub extern fn kreuzberg_inline_element_content(ptr: ?*const KREUZBERGInlineElement) [*c]u8;
pub extern fn kreuzberg_inline_element_metadata(ptr: ?*const KREUZBERGInlineElement) [*c]u8;
pub extern fn kreuzberg_djot_image_from_json(json: [*c]const u8) ?*KREUZBERGDjotImage;
pub extern fn kreuzberg_djot_image_to_json(ptr: ?*const KREUZBERGDjotImage) [*c]u8;
pub extern fn kreuzberg_djot_image_free(ptr: ?*KREUZBERGDjotImage) void;
pub extern fn kreuzberg_djot_image_src(ptr: ?*const KREUZBERGDjotImage) [*c]u8;
pub extern fn kreuzberg_djot_image_alt(ptr: ?*const KREUZBERGDjotImage) [*c]u8;
pub extern fn kreuzberg_djot_image_title(ptr: ?*const KREUZBERGDjotImage) [*c]u8;
pub extern fn kreuzberg_djot_link_from_json(json: [*c]const u8) ?*KREUZBERGDjotLink;
pub extern fn kreuzberg_djot_link_to_json(ptr: ?*const KREUZBERGDjotLink) [*c]u8;
pub extern fn kreuzberg_djot_link_free(ptr: ?*KREUZBERGDjotLink) void;
pub extern fn kreuzberg_djot_link_url(ptr: ?*const KREUZBERGDjotLink) [*c]u8;
pub extern fn kreuzberg_djot_link_text(ptr: ?*const KREUZBERGDjotLink) [*c]u8;
pub extern fn kreuzberg_djot_link_title(ptr: ?*const KREUZBERGDjotLink) [*c]u8;
pub extern fn kreuzberg_footnote_from_json(json: [*c]const u8) ?*KREUZBERGFootnote;
pub extern fn kreuzberg_footnote_to_json(ptr: ?*const KREUZBERGFootnote) [*c]u8;
pub extern fn kreuzberg_footnote_free(ptr: ?*KREUZBERGFootnote) void;
pub extern fn kreuzberg_footnote_label(ptr: ?*const KREUZBERGFootnote) [*c]u8;
pub extern fn kreuzberg_footnote_content(ptr: ?*const KREUZBERGFootnote) [*c]u8;
pub extern fn kreuzberg_document_structure_from_json(json: [*c]const u8) ?*KREUZBERGDocumentStructure;
pub extern fn kreuzberg_document_structure_to_json(ptr: ?*const KREUZBERGDocumentStructure) [*c]u8;
pub extern fn kreuzberg_document_structure_free(ptr: ?*KREUZBERGDocumentStructure) void;
pub extern fn kreuzberg_document_structure_nodes(ptr: ?*const KREUZBERGDocumentStructure) [*c]u8;
pub extern fn kreuzberg_document_structure_source_format(ptr: ?*const KREUZBERGDocumentStructure) [*c]u8;
pub extern fn kreuzberg_document_structure_relationships(ptr: ?*const KREUZBERGDocumentStructure) [*c]u8;
pub extern fn kreuzberg_document_structure_node_types(ptr: ?*const KREUZBERGDocumentStructure) [*c]u8;
pub extern fn kreuzberg_document_structure_finalize_node_types(this_: ?*KREUZBERGDocumentStructure) void;
pub extern fn kreuzberg_document_structure_default() ?*KREUZBERGDocumentStructure;
pub extern fn kreuzberg_document_relationship_from_json(json: [*c]const u8) ?*KREUZBERGDocumentRelationship;
pub extern fn kreuzberg_document_relationship_to_json(ptr: ?*const KREUZBERGDocumentRelationship) [*c]u8;
pub extern fn kreuzberg_document_relationship_free(ptr: ?*KREUZBERGDocumentRelationship) void;
pub extern fn kreuzberg_document_relationship_source(ptr: ?*const KREUZBERGDocumentRelationship) u32;
pub extern fn kreuzberg_document_relationship_target(ptr: ?*const KREUZBERGDocumentRelationship) u32;
pub extern fn kreuzberg_document_relationship_kind(ptr: ?*const KREUZBERGDocumentRelationship) ?*KREUZBERGRelationshipKind;
pub extern fn kreuzberg_document_node_from_json(json: [*c]const u8) ?*KREUZBERGDocumentNode;
pub extern fn kreuzberg_document_node_to_json(ptr: ?*const KREUZBERGDocumentNode) [*c]u8;
pub extern fn kreuzberg_document_node_free(ptr: ?*KREUZBERGDocumentNode) void;
pub extern fn kreuzberg_document_node_content(ptr: ?*const KREUZBERGDocumentNode) ?*KREUZBERGNodeContent;
pub extern fn kreuzberg_document_node_parent(ptr: ?*const KREUZBERGDocumentNode) u32;
pub extern fn kreuzberg_document_node_children(ptr: ?*const KREUZBERGDocumentNode) [*c]u8;
pub extern fn kreuzberg_document_node_content_layer(ptr: ?*const KREUZBERGDocumentNode) ?*KREUZBERGContentLayer;
pub extern fn kreuzberg_document_node_page(ptr: ?*const KREUZBERGDocumentNode) u32;
pub extern fn kreuzberg_document_node_page_end(ptr: ?*const KREUZBERGDocumentNode) u32;
pub extern fn kreuzberg_document_node_annotations(ptr: ?*const KREUZBERGDocumentNode) [*c]u8;
pub extern fn kreuzberg_document_node_attributes(ptr: ?*const KREUZBERGDocumentNode) [*c]u8;
pub extern fn kreuzberg_table_grid_from_json(json: [*c]const u8) ?*KREUZBERGTableGrid;
pub extern fn kreuzberg_table_grid_to_json(ptr: ?*const KREUZBERGTableGrid) [*c]u8;
pub extern fn kreuzberg_table_grid_free(ptr: ?*KREUZBERGTableGrid) void;
pub extern fn kreuzberg_table_grid_rows(ptr: ?*const KREUZBERGTableGrid) u32;
pub extern fn kreuzberg_table_grid_cols(ptr: ?*const KREUZBERGTableGrid) u32;
pub extern fn kreuzberg_table_grid_cells(ptr: ?*const KREUZBERGTableGrid) [*c]u8;
pub extern fn kreuzberg_grid_cell_from_json(json: [*c]const u8) ?*KREUZBERGGridCell;
pub extern fn kreuzberg_grid_cell_to_json(ptr: ?*const KREUZBERGGridCell) [*c]u8;
pub extern fn kreuzberg_grid_cell_free(ptr: ?*KREUZBERGGridCell) void;
pub extern fn kreuzberg_grid_cell_content(ptr: ?*const KREUZBERGGridCell) [*c]u8;
pub extern fn kreuzberg_grid_cell_row(ptr: ?*const KREUZBERGGridCell) u32;
pub extern fn kreuzberg_grid_cell_col(ptr: ?*const KREUZBERGGridCell) u32;
pub extern fn kreuzberg_grid_cell_row_span(ptr: ?*const KREUZBERGGridCell) u32;
pub extern fn kreuzberg_grid_cell_col_span(ptr: ?*const KREUZBERGGridCell) u32;
pub extern fn kreuzberg_grid_cell_is_header(ptr: ?*const KREUZBERGGridCell) i32;
pub extern fn kreuzberg_text_annotation_from_json(json: [*c]const u8) ?*KREUZBERGTextAnnotation;
pub extern fn kreuzberg_text_annotation_to_json(ptr: ?*const KREUZBERGTextAnnotation) [*c]u8;
pub extern fn kreuzberg_text_annotation_free(ptr: ?*KREUZBERGTextAnnotation) void;
pub extern fn kreuzberg_text_annotation_start(ptr: ?*const KREUZBERGTextAnnotation) u32;
pub extern fn kreuzberg_text_annotation_end(ptr: ?*const KREUZBERGTextAnnotation) u32;
pub extern fn kreuzberg_text_annotation_kind(ptr: ?*const KREUZBERGTextAnnotation) ?*KREUZBERGAnnotationKind;
pub extern fn kreuzberg_extraction_result_from_json(json: [*c]const u8) ?*KREUZBERGExtractionResult;
pub extern fn kreuzberg_extraction_result_to_json(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_free(ptr: ?*KREUZBERGExtractionResult) void;
pub extern fn kreuzberg_extraction_result_content(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_metadata(ptr: ?*const KREUZBERGExtractionResult) ?*KREUZBERGMetadata;
pub extern fn kreuzberg_extraction_result_extraction_method(ptr: ?*const KREUZBERGExtractionResult) ?*KREUZBERGExtractionMethod;
pub extern fn kreuzberg_extraction_result_tables(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_detected_languages(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_chunks(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_images(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_pages(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_elements(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_djot_content(ptr: ?*const KREUZBERGExtractionResult) ?*KREUZBERGDjotContent;
pub extern fn kreuzberg_extraction_result_ocr_elements(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_document(ptr: ?*const KREUZBERGExtractionResult) ?*KREUZBERGDocumentStructure;
pub extern fn kreuzberg_extraction_result_extracted_keywords(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_quality_score(ptr: ?*const KREUZBERGExtractionResult) f64;
pub extern fn kreuzberg_extraction_result_processing_warnings(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_annotations(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_children(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_uris(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_structured_output(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_llm_usage(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_extraction_result_formatted_content(ptr: ?*const KREUZBERGExtractionResult) [*c]u8;
pub extern fn kreuzberg_archive_entry_from_json(json: [*c]const u8) ?*KREUZBERGArchiveEntry;
pub extern fn kreuzberg_archive_entry_to_json(ptr: ?*const KREUZBERGArchiveEntry) [*c]u8;
pub extern fn kreuzberg_archive_entry_free(ptr: ?*KREUZBERGArchiveEntry) void;
pub extern fn kreuzberg_archive_entry_path(ptr: ?*const KREUZBERGArchiveEntry) [*c]u8;
pub extern fn kreuzberg_archive_entry_mime_type(ptr: ?*const KREUZBERGArchiveEntry) [*c]u8;
pub extern fn kreuzberg_archive_entry_result(ptr: ?*const KREUZBERGArchiveEntry) ?*KREUZBERGExtractionResult;
pub extern fn kreuzberg_processing_warning_from_json(json: [*c]const u8) ?*KREUZBERGProcessingWarning;
pub extern fn kreuzberg_processing_warning_to_json(ptr: ?*const KREUZBERGProcessingWarning) [*c]u8;
pub extern fn kreuzberg_processing_warning_free(ptr: ?*KREUZBERGProcessingWarning) void;
pub extern fn kreuzberg_llm_usage_from_json(json: [*c]const u8) ?*KREUZBERGLlmUsage;
pub extern fn kreuzberg_llm_usage_to_json(ptr: ?*const KREUZBERGLlmUsage) [*c]u8;
pub extern fn kreuzberg_llm_usage_free(ptr: ?*KREUZBERGLlmUsage) void;
pub extern fn kreuzberg_llm_usage_model(ptr: ?*const KREUZBERGLlmUsage) [*c]u8;
pub extern fn kreuzberg_llm_usage_source(ptr: ?*const KREUZBERGLlmUsage) [*c]u8;
pub extern fn kreuzberg_llm_usage_input_tokens(ptr: ?*const KREUZBERGLlmUsage) u64;
pub extern fn kreuzberg_llm_usage_output_tokens(ptr: ?*const KREUZBERGLlmUsage) u64;
pub extern fn kreuzberg_llm_usage_total_tokens(ptr: ?*const KREUZBERGLlmUsage) u64;
pub extern fn kreuzberg_llm_usage_estimated_cost(ptr: ?*const KREUZBERGLlmUsage) f64;
pub extern fn kreuzberg_llm_usage_finish_reason(ptr: ?*const KREUZBERGLlmUsage) [*c]u8;
pub extern fn kreuzberg_chunk_from_json(json: [*c]const u8) ?*KREUZBERGChunk;
pub extern fn kreuzberg_chunk_to_json(ptr: ?*const KREUZBERGChunk) [*c]u8;
pub extern fn kreuzberg_chunk_free(ptr: ?*KREUZBERGChunk) void;
pub extern fn kreuzberg_chunk_content(ptr: ?*const KREUZBERGChunk) [*c]u8;
pub extern fn kreuzberg_chunk_chunk_type(ptr: ?*const KREUZBERGChunk) ?*KREUZBERGChunkType;
pub extern fn kreuzberg_chunk_embedding(ptr: ?*const KREUZBERGChunk) [*c]u8;
pub extern fn kreuzberg_chunk_metadata(ptr: ?*const KREUZBERGChunk) ?*KREUZBERGChunkMetadata;
pub extern fn kreuzberg_heading_context_from_json(json: [*c]const u8) ?*KREUZBERGHeadingContext;
pub extern fn kreuzberg_heading_context_to_json(ptr: ?*const KREUZBERGHeadingContext) [*c]u8;
pub extern fn kreuzberg_heading_context_free(ptr: ?*KREUZBERGHeadingContext) void;
pub extern fn kreuzberg_heading_context_headings(ptr: ?*const KREUZBERGHeadingContext) [*c]u8;
pub extern fn kreuzberg_heading_level_from_json(json: [*c]const u8) ?*KREUZBERGHeadingLevel;
pub extern fn kreuzberg_heading_level_to_json(ptr: ?*const KREUZBERGHeadingLevel) [*c]u8;
pub extern fn kreuzberg_heading_level_free(ptr: ?*KREUZBERGHeadingLevel) void;
pub extern fn kreuzberg_heading_level_level(ptr: ?*const KREUZBERGHeadingLevel) u8;
pub extern fn kreuzberg_heading_level_text(ptr: ?*const KREUZBERGHeadingLevel) [*c]u8;
pub extern fn kreuzberg_chunk_metadata_from_json(json: [*c]const u8) ?*KREUZBERGChunkMetadata;
pub extern fn kreuzberg_chunk_metadata_to_json(ptr: ?*const KREUZBERGChunkMetadata) [*c]u8;
pub extern fn kreuzberg_chunk_metadata_free(ptr: ?*KREUZBERGChunkMetadata) void;
pub extern fn kreuzberg_chunk_metadata_byte_start(ptr: ?*const KREUZBERGChunkMetadata) usize;
pub extern fn kreuzberg_chunk_metadata_byte_end(ptr: ?*const KREUZBERGChunkMetadata) usize;
pub extern fn kreuzberg_chunk_metadata_token_count(ptr: ?*const KREUZBERGChunkMetadata) usize;
pub extern fn kreuzberg_chunk_metadata_chunk_index(ptr: ?*const KREUZBERGChunkMetadata) usize;
pub extern fn kreuzberg_chunk_metadata_total_chunks(ptr: ?*const KREUZBERGChunkMetadata) usize;
pub extern fn kreuzberg_chunk_metadata_first_page(ptr: ?*const KREUZBERGChunkMetadata) usize;
pub extern fn kreuzberg_chunk_metadata_last_page(ptr: ?*const KREUZBERGChunkMetadata) usize;
pub extern fn kreuzberg_chunk_metadata_heading_context(ptr: ?*const KREUZBERGChunkMetadata) ?*KREUZBERGHeadingContext;
pub extern fn kreuzberg_extracted_image_from_json(json: [*c]const u8) ?*KREUZBERGExtractedImage;
pub extern fn kreuzberg_extracted_image_to_json(ptr: ?*const KREUZBERGExtractedImage) [*c]u8;
pub extern fn kreuzberg_extracted_image_free(ptr: ?*KREUZBERGExtractedImage) void;
pub extern fn kreuzberg_extracted_image_data(ptr: ?*const KREUZBERGExtractedImage, out_len: [*c]usize) [*c]u8;
pub extern fn kreuzberg_extracted_image_image_index(ptr: ?*const KREUZBERGExtractedImage) usize;
pub extern fn kreuzberg_extracted_image_page_number(ptr: ?*const KREUZBERGExtractedImage) usize;
pub extern fn kreuzberg_extracted_image_width(ptr: ?*const KREUZBERGExtractedImage) u32;
pub extern fn kreuzberg_extracted_image_height(ptr: ?*const KREUZBERGExtractedImage) u32;
pub extern fn kreuzberg_extracted_image_colorspace(ptr: ?*const KREUZBERGExtractedImage) [*c]u8;
pub extern fn kreuzberg_extracted_image_bits_per_component(ptr: ?*const KREUZBERGExtractedImage) u32;
pub extern fn kreuzberg_extracted_image_is_mask(ptr: ?*const KREUZBERGExtractedImage) i32;
pub extern fn kreuzberg_extracted_image_description(ptr: ?*const KREUZBERGExtractedImage) [*c]u8;
pub extern fn kreuzberg_extracted_image_ocr_result(ptr: ?*const KREUZBERGExtractedImage) ?*KREUZBERGExtractionResult;
pub extern fn kreuzberg_extracted_image_source_path(ptr: ?*const KREUZBERGExtractedImage) [*c]u8;
pub extern fn kreuzberg_extracted_image_image_kind(ptr: ?*const KREUZBERGExtractedImage) ?*KREUZBERGImageKind;
pub extern fn kreuzberg_extracted_image_kind_confidence(ptr: ?*const KREUZBERGExtractedImage) f32;
pub extern fn kreuzberg_extracted_image_cluster_id(ptr: ?*const KREUZBERGExtractedImage) u32;
pub extern fn kreuzberg_element_metadata_from_json(json: [*c]const u8) ?*KREUZBERGElementMetadata;
pub extern fn kreuzberg_element_metadata_to_json(ptr: ?*const KREUZBERGElementMetadata) [*c]u8;
pub extern fn kreuzberg_element_metadata_free(ptr: ?*KREUZBERGElementMetadata) void;
pub extern fn kreuzberg_element_metadata_page_number(ptr: ?*const KREUZBERGElementMetadata) usize;
pub extern fn kreuzberg_element_metadata_filename(ptr: ?*const KREUZBERGElementMetadata) [*c]u8;
pub extern fn kreuzberg_element_metadata_element_index(ptr: ?*const KREUZBERGElementMetadata) usize;
pub extern fn kreuzberg_element_metadata_additional(ptr: ?*const KREUZBERGElementMetadata) [*c]u8;
pub extern fn kreuzberg_element_from_json(json: [*c]const u8) ?*KREUZBERGElement;
pub extern fn kreuzberg_element_to_json(ptr: ?*const KREUZBERGElement) [*c]u8;
pub extern fn kreuzberg_element_free(ptr: ?*KREUZBERGElement) void;
pub extern fn kreuzberg_element_element_type(ptr: ?*const KREUZBERGElement) ?*KREUZBERGElementType;
pub extern fn kreuzberg_element_text(ptr: ?*const KREUZBERGElement) [*c]u8;
pub extern fn kreuzberg_element_metadata(ptr: ?*const KREUZBERGElement) ?*KREUZBERGElementMetadata;
pub extern fn kreuzberg_excel_workbook_from_json(json: [*c]const u8) ?*KREUZBERGExcelWorkbook;
pub extern fn kreuzberg_excel_workbook_to_json(ptr: ?*const KREUZBERGExcelWorkbook) [*c]u8;
pub extern fn kreuzberg_excel_workbook_free(ptr: ?*KREUZBERGExcelWorkbook) void;
pub extern fn kreuzberg_excel_workbook_sheets(ptr: ?*const KREUZBERGExcelWorkbook) [*c]u8;
pub extern fn kreuzberg_excel_workbook_metadata(ptr: ?*const KREUZBERGExcelWorkbook) [*c]u8;
pub extern fn kreuzberg_excel_sheet_from_json(json: [*c]const u8) ?*KREUZBERGExcelSheet;
pub extern fn kreuzberg_excel_sheet_to_json(ptr: ?*const KREUZBERGExcelSheet) [*c]u8;
pub extern fn kreuzberg_excel_sheet_free(ptr: ?*KREUZBERGExcelSheet) void;
pub extern fn kreuzberg_excel_sheet_name(ptr: ?*const KREUZBERGExcelSheet) [*c]u8;
pub extern fn kreuzberg_excel_sheet_markdown(ptr: ?*const KREUZBERGExcelSheet) [*c]u8;
pub extern fn kreuzberg_excel_sheet_row_count(ptr: ?*const KREUZBERGExcelSheet) usize;
pub extern fn kreuzberg_excel_sheet_col_count(ptr: ?*const KREUZBERGExcelSheet) usize;
pub extern fn kreuzberg_excel_sheet_cell_count(ptr: ?*const KREUZBERGExcelSheet) usize;
pub extern fn kreuzberg_excel_sheet_table_cells(ptr: ?*const KREUZBERGExcelSheet) [*c]u8;
pub extern fn kreuzberg_xml_extraction_result_from_json(json: [*c]const u8) ?*KREUZBERGXmlExtractionResult;
pub extern fn kreuzberg_xml_extraction_result_to_json(ptr: ?*const KREUZBERGXmlExtractionResult) [*c]u8;
pub extern fn kreuzberg_xml_extraction_result_free(ptr: ?*KREUZBERGXmlExtractionResult) void;
pub extern fn kreuzberg_xml_extraction_result_content(ptr: ?*const KREUZBERGXmlExtractionResult) [*c]u8;
pub extern fn kreuzberg_xml_extraction_result_element_count(ptr: ?*const KREUZBERGXmlExtractionResult) usize;
pub extern fn kreuzberg_xml_extraction_result_unique_elements(ptr: ?*const KREUZBERGXmlExtractionResult) [*c]u8;
pub extern fn kreuzberg_text_extraction_result_from_json(json: [*c]const u8) ?*KREUZBERGTextExtractionResult;
pub extern fn kreuzberg_text_extraction_result_to_json(ptr: ?*const KREUZBERGTextExtractionResult) [*c]u8;
pub extern fn kreuzberg_text_extraction_result_free(ptr: ?*KREUZBERGTextExtractionResult) void;
pub extern fn kreuzberg_text_extraction_result_content(ptr: ?*const KREUZBERGTextExtractionResult) [*c]u8;
pub extern fn kreuzberg_text_extraction_result_line_count(ptr: ?*const KREUZBERGTextExtractionResult) usize;
pub extern fn kreuzberg_text_extraction_result_word_count(ptr: ?*const KREUZBERGTextExtractionResult) usize;
pub extern fn kreuzberg_text_extraction_result_character_count(ptr: ?*const KREUZBERGTextExtractionResult) usize;
pub extern fn kreuzberg_text_extraction_result_headers(ptr: ?*const KREUZBERGTextExtractionResult) [*c]u8;
pub extern fn kreuzberg_pptx_extraction_result_from_json(json: [*c]const u8) ?*KREUZBERGPptxExtractionResult;
pub extern fn kreuzberg_pptx_extraction_result_to_json(ptr: ?*const KREUZBERGPptxExtractionResult) [*c]u8;
pub extern fn kreuzberg_pptx_extraction_result_free(ptr: ?*KREUZBERGPptxExtractionResult) void;
pub extern fn kreuzberg_pptx_extraction_result_content(ptr: ?*const KREUZBERGPptxExtractionResult) [*c]u8;
pub extern fn kreuzberg_pptx_extraction_result_metadata(ptr: ?*const KREUZBERGPptxExtractionResult) ?*KREUZBERGPptxMetadata;
pub extern fn kreuzberg_pptx_extraction_result_slide_count(ptr: ?*const KREUZBERGPptxExtractionResult) usize;
pub extern fn kreuzberg_pptx_extraction_result_image_count(ptr: ?*const KREUZBERGPptxExtractionResult) usize;
pub extern fn kreuzberg_pptx_extraction_result_table_count(ptr: ?*const KREUZBERGPptxExtractionResult) usize;
pub extern fn kreuzberg_pptx_extraction_result_images(ptr: ?*const KREUZBERGPptxExtractionResult) [*c]u8;
pub extern fn kreuzberg_pptx_extraction_result_page_structure(ptr: ?*const KREUZBERGPptxExtractionResult) ?*KREUZBERGPageStructure;
pub extern fn kreuzberg_pptx_extraction_result_page_contents(ptr: ?*const KREUZBERGPptxExtractionResult) [*c]u8;
pub extern fn kreuzberg_pptx_extraction_result_document(ptr: ?*const KREUZBERGPptxExtractionResult) ?*KREUZBERGDocumentStructure;
pub extern fn kreuzberg_pptx_extraction_result_office_metadata(ptr: ?*const KREUZBERGPptxExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_from_json(json: [*c]const u8) ?*KREUZBERGEmailExtractionResult;
pub extern fn kreuzberg_email_extraction_result_to_json(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_free(ptr: ?*KREUZBERGEmailExtractionResult) void;
pub extern fn kreuzberg_email_extraction_result_subject(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_from_email(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_to_emails(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_cc_emails(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_bcc_emails(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_date(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_message_id(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_plain_text(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_html_content(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_content(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_attachments(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_extraction_result_metadata(ptr: ?*const KREUZBERGEmailExtractionResult) [*c]u8;
pub extern fn kreuzberg_email_attachment_from_json(json: [*c]const u8) ?*KREUZBERGEmailAttachment;
pub extern fn kreuzberg_email_attachment_to_json(ptr: ?*const KREUZBERGEmailAttachment) [*c]u8;
pub extern fn kreuzberg_email_attachment_free(ptr: ?*KREUZBERGEmailAttachment) void;
pub extern fn kreuzberg_email_attachment_name(ptr: ?*const KREUZBERGEmailAttachment) [*c]u8;
pub extern fn kreuzberg_email_attachment_filename(ptr: ?*const KREUZBERGEmailAttachment) [*c]u8;
pub extern fn kreuzberg_email_attachment_mime_type(ptr: ?*const KREUZBERGEmailAttachment) [*c]u8;
pub extern fn kreuzberg_email_attachment_size(ptr: ?*const KREUZBERGEmailAttachment) usize;
pub extern fn kreuzberg_email_attachment_is_image(ptr: ?*const KREUZBERGEmailAttachment) i32;
pub extern fn kreuzberg_email_attachment_data(ptr: ?*const KREUZBERGEmailAttachment) [*c]u8;
pub extern fn kreuzberg_ocr_extraction_result_from_json(json: [*c]const u8) ?*KREUZBERGOcrExtractionResult;
pub extern fn kreuzberg_ocr_extraction_result_to_json(ptr: ?*const KREUZBERGOcrExtractionResult) [*c]u8;
pub extern fn kreuzberg_ocr_extraction_result_free(ptr: ?*KREUZBERGOcrExtractionResult) void;
pub extern fn kreuzberg_ocr_extraction_result_content(ptr: ?*const KREUZBERGOcrExtractionResult) [*c]u8;
pub extern fn kreuzberg_ocr_extraction_result_mime_type(ptr: ?*const KREUZBERGOcrExtractionResult) [*c]u8;
pub extern fn kreuzberg_ocr_extraction_result_metadata(ptr: ?*const KREUZBERGOcrExtractionResult) [*c]u8;
pub extern fn kreuzberg_ocr_extraction_result_tables(ptr: ?*const KREUZBERGOcrExtractionResult) [*c]u8;
pub extern fn kreuzberg_ocr_extraction_result_ocr_elements(ptr: ?*const KREUZBERGOcrExtractionResult) [*c]u8;
pub extern fn kreuzberg_ocr_table_from_json(json: [*c]const u8) ?*KREUZBERGOcrTable;
pub extern fn kreuzberg_ocr_table_to_json(ptr: ?*const KREUZBERGOcrTable) [*c]u8;
pub extern fn kreuzberg_ocr_table_free(ptr: ?*KREUZBERGOcrTable) void;
pub extern fn kreuzberg_ocr_table_cells(ptr: ?*const KREUZBERGOcrTable) [*c]u8;
pub extern fn kreuzberg_ocr_table_markdown(ptr: ?*const KREUZBERGOcrTable) [*c]u8;
pub extern fn kreuzberg_ocr_table_page_number(ptr: ?*const KREUZBERGOcrTable) usize;
pub extern fn kreuzberg_ocr_table_bounding_box(ptr: ?*const KREUZBERGOcrTable) ?*KREUZBERGOcrTableBoundingBox;
pub extern fn kreuzberg_ocr_table_bounding_box_from_json(json: [*c]const u8) ?*KREUZBERGOcrTableBoundingBox;
pub extern fn kreuzberg_ocr_table_bounding_box_to_json(ptr: ?*const KREUZBERGOcrTableBoundingBox) [*c]u8;
pub extern fn kreuzberg_ocr_table_bounding_box_free(ptr: ?*KREUZBERGOcrTableBoundingBox) void;
pub extern fn kreuzberg_ocr_table_bounding_box_left(ptr: ?*const KREUZBERGOcrTableBoundingBox) u32;
pub extern fn kreuzberg_ocr_table_bounding_box_top(ptr: ?*const KREUZBERGOcrTableBoundingBox) u32;
pub extern fn kreuzberg_ocr_table_bounding_box_right(ptr: ?*const KREUZBERGOcrTableBoundingBox) u32;
pub extern fn kreuzberg_ocr_table_bounding_box_bottom(ptr: ?*const KREUZBERGOcrTableBoundingBox) u32;
pub extern fn kreuzberg_image_preprocessing_config_from_json(json: [*c]const u8) ?*KREUZBERGImagePreprocessingConfig;
pub extern fn kreuzberg_image_preprocessing_config_to_json(ptr: ?*const KREUZBERGImagePreprocessingConfig) [*c]u8;
pub extern fn kreuzberg_image_preprocessing_config_free(ptr: ?*KREUZBERGImagePreprocessingConfig) void;
pub extern fn kreuzberg_image_preprocessing_config_target_dpi(ptr: ?*const KREUZBERGImagePreprocessingConfig) i32;
pub extern fn kreuzberg_image_preprocessing_config_auto_rotate(ptr: ?*const KREUZBERGImagePreprocessingConfig) i32;
pub extern fn kreuzberg_image_preprocessing_config_deskew(ptr: ?*const KREUZBERGImagePreprocessingConfig) i32;
pub extern fn kreuzberg_image_preprocessing_config_denoise(ptr: ?*const KREUZBERGImagePreprocessingConfig) i32;
pub extern fn kreuzberg_image_preprocessing_config_contrast_enhance(ptr: ?*const KREUZBERGImagePreprocessingConfig) i32;
pub extern fn kreuzberg_image_preprocessing_config_binarization_method(ptr: ?*const KREUZBERGImagePreprocessingConfig) [*c]u8;
pub extern fn kreuzberg_image_preprocessing_config_invert_colors(ptr: ?*const KREUZBERGImagePreprocessingConfig) i32;
pub extern fn kreuzberg_image_preprocessing_config_default() ?*KREUZBERGImagePreprocessingConfig;
pub extern fn kreuzberg_tesseract_config_from_json(json: [*c]const u8) ?*KREUZBERGTesseractConfig;
pub extern fn kreuzberg_tesseract_config_to_json(ptr: ?*const KREUZBERGTesseractConfig) [*c]u8;
pub extern fn kreuzberg_tesseract_config_free(ptr: ?*KREUZBERGTesseractConfig) void;
pub extern fn kreuzberg_tesseract_config_language(ptr: ?*const KREUZBERGTesseractConfig) [*c]u8;
pub extern fn kreuzberg_tesseract_config_psm(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_output_format(ptr: ?*const KREUZBERGTesseractConfig) [*c]u8;
pub extern fn kreuzberg_tesseract_config_oem(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_min_confidence(ptr: ?*const KREUZBERGTesseractConfig) f64;
pub extern fn kreuzberg_tesseract_config_preprocessing(ptr: ?*const KREUZBERGTesseractConfig) ?*KREUZBERGImagePreprocessingConfig;
pub extern fn kreuzberg_tesseract_config_enable_table_detection(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_table_min_confidence(ptr: ?*const KREUZBERGTesseractConfig) f64;
pub extern fn kreuzberg_tesseract_config_table_column_threshold(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_table_row_threshold_ratio(ptr: ?*const KREUZBERGTesseractConfig) f64;
pub extern fn kreuzberg_tesseract_config_use_cache(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_classify_use_pre_adapted_templates(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_language_model_ngram_on(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_tessedit_dont_blkrej_good_wds(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_tessedit_dont_rowrej_good_wds(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_tessedit_enable_dict_correction(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_tessedit_char_whitelist(ptr: ?*const KREUZBERGTesseractConfig) [*c]u8;
pub extern fn kreuzberg_tesseract_config_tessedit_char_blacklist(ptr: ?*const KREUZBERGTesseractConfig) [*c]u8;
pub extern fn kreuzberg_tesseract_config_tessedit_use_primary_params_model(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_textord_space_size_is_variable(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_thresholding_method(ptr: ?*const KREUZBERGTesseractConfig) i32;
pub extern fn kreuzberg_tesseract_config_default() ?*KREUZBERGTesseractConfig;
pub extern fn kreuzberg_image_preprocessing_metadata_from_json(json: [*c]const u8) ?*KREUZBERGImagePreprocessingMetadata;
pub extern fn kreuzberg_image_preprocessing_metadata_to_json(ptr: ?*const KREUZBERGImagePreprocessingMetadata) [*c]u8;
pub extern fn kreuzberg_image_preprocessing_metadata_free(ptr: ?*KREUZBERGImagePreprocessingMetadata) void;
pub extern fn kreuzberg_image_preprocessing_metadata_target_dpi(ptr: ?*const KREUZBERGImagePreprocessingMetadata) i32;
pub extern fn kreuzberg_image_preprocessing_metadata_scale_factor(ptr: ?*const KREUZBERGImagePreprocessingMetadata) f64;
pub extern fn kreuzberg_image_preprocessing_metadata_auto_adjusted(ptr: ?*const KREUZBERGImagePreprocessingMetadata) i32;
pub extern fn kreuzberg_image_preprocessing_metadata_final_dpi(ptr: ?*const KREUZBERGImagePreprocessingMetadata) i32;
pub extern fn kreuzberg_image_preprocessing_metadata_resample_method(ptr: ?*const KREUZBERGImagePreprocessingMetadata) [*c]u8;
pub extern fn kreuzberg_image_preprocessing_metadata_dimension_clamped(ptr: ?*const KREUZBERGImagePreprocessingMetadata) i32;
pub extern fn kreuzberg_image_preprocessing_metadata_calculated_dpi(ptr: ?*const KREUZBERGImagePreprocessingMetadata) i32;
pub extern fn kreuzberg_image_preprocessing_metadata_skipped_resize(ptr: ?*const KREUZBERGImagePreprocessingMetadata) i32;
pub extern fn kreuzberg_image_preprocessing_metadata_resize_error(ptr: ?*const KREUZBERGImagePreprocessingMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_from_json(json: [*c]const u8) ?*KREUZBERGMetadata;
pub extern fn kreuzberg_metadata_to_json(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_free(ptr: ?*KREUZBERGMetadata) void;
pub extern fn kreuzberg_metadata_title(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_subject(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_authors(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_keywords(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_language(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_created_at(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_modified_at(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_created_by(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_modified_by(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_pages(ptr: ?*const KREUZBERGMetadata) ?*KREUZBERGPageStructure;
pub extern fn kreuzberg_metadata_format(ptr: ?*const KREUZBERGMetadata) ?*KREUZBERGFormatMetadata;
pub extern fn kreuzberg_metadata_image_preprocessing(ptr: ?*const KREUZBERGMetadata) ?*KREUZBERGImagePreprocessingMetadata;
pub extern fn kreuzberg_metadata_json_schema(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_error(ptr: ?*const KREUZBERGMetadata) ?*KREUZBERGErrorMetadata;
pub extern fn kreuzberg_metadata_extraction_duration_ms(ptr: ?*const KREUZBERGMetadata) u64;
pub extern fn kreuzberg_metadata_category(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_tags(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_document_version(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_abstract_text(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_output_format(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_additional(ptr: ?*const KREUZBERGMetadata) [*c]u8;
pub extern fn kreuzberg_metadata_is_empty(this_: ?*const KREUZBERGMetadata) i32;
pub extern fn kreuzberg_excel_metadata_from_json(json: [*c]const u8) ?*KREUZBERGExcelMetadata;
pub extern fn kreuzberg_excel_metadata_to_json(ptr: ?*const KREUZBERGExcelMetadata) [*c]u8;
pub extern fn kreuzberg_excel_metadata_free(ptr: ?*KREUZBERGExcelMetadata) void;
pub extern fn kreuzberg_excel_metadata_sheet_count(ptr: ?*const KREUZBERGExcelMetadata) usize;
pub extern fn kreuzberg_excel_metadata_sheet_names(ptr: ?*const KREUZBERGExcelMetadata) [*c]u8;
pub extern fn kreuzberg_email_metadata_from_json(json: [*c]const u8) ?*KREUZBERGEmailMetadata;
pub extern fn kreuzberg_email_metadata_to_json(ptr: ?*const KREUZBERGEmailMetadata) [*c]u8;
pub extern fn kreuzberg_email_metadata_free(ptr: ?*KREUZBERGEmailMetadata) void;
pub extern fn kreuzberg_email_metadata_from_email(ptr: ?*const KREUZBERGEmailMetadata) [*c]u8;
pub extern fn kreuzberg_email_metadata_from_name(ptr: ?*const KREUZBERGEmailMetadata) [*c]u8;
pub extern fn kreuzberg_email_metadata_to_emails(ptr: ?*const KREUZBERGEmailMetadata) [*c]u8;
pub extern fn kreuzberg_email_metadata_cc_emails(ptr: ?*const KREUZBERGEmailMetadata) [*c]u8;
pub extern fn kreuzberg_email_metadata_bcc_emails(ptr: ?*const KREUZBERGEmailMetadata) [*c]u8;
pub extern fn kreuzberg_email_metadata_message_id(ptr: ?*const KREUZBERGEmailMetadata) [*c]u8;
pub extern fn kreuzberg_email_metadata_attachments(ptr: ?*const KREUZBERGEmailMetadata) [*c]u8;
pub extern fn kreuzberg_archive_metadata_from_json(json: [*c]const u8) ?*KREUZBERGArchiveMetadata;
pub extern fn kreuzberg_archive_metadata_to_json(ptr: ?*const KREUZBERGArchiveMetadata) [*c]u8;
pub extern fn kreuzberg_archive_metadata_free(ptr: ?*KREUZBERGArchiveMetadata) void;
pub extern fn kreuzberg_archive_metadata_file_count(ptr: ?*const KREUZBERGArchiveMetadata) usize;
pub extern fn kreuzberg_archive_metadata_file_list(ptr: ?*const KREUZBERGArchiveMetadata) [*c]u8;
pub extern fn kreuzberg_archive_metadata_total_size(ptr: ?*const KREUZBERGArchiveMetadata) usize;
pub extern fn kreuzberg_archive_metadata_compressed_size(ptr: ?*const KREUZBERGArchiveMetadata) usize;
pub extern fn kreuzberg_xml_metadata_from_json(json: [*c]const u8) ?*KREUZBERGXmlMetadata;
pub extern fn kreuzberg_xml_metadata_to_json(ptr: ?*const KREUZBERGXmlMetadata) [*c]u8;
pub extern fn kreuzberg_xml_metadata_free(ptr: ?*KREUZBERGXmlMetadata) void;
pub extern fn kreuzberg_xml_metadata_element_count(ptr: ?*const KREUZBERGXmlMetadata) usize;
pub extern fn kreuzberg_xml_metadata_unique_elements(ptr: ?*const KREUZBERGXmlMetadata) [*c]u8;
pub extern fn kreuzberg_text_metadata_from_json(json: [*c]const u8) ?*KREUZBERGTextMetadata;
pub extern fn kreuzberg_text_metadata_to_json(ptr: ?*const KREUZBERGTextMetadata) [*c]u8;
pub extern fn kreuzberg_text_metadata_free(ptr: ?*KREUZBERGTextMetadata) void;
pub extern fn kreuzberg_text_metadata_line_count(ptr: ?*const KREUZBERGTextMetadata) usize;
pub extern fn kreuzberg_text_metadata_word_count(ptr: ?*const KREUZBERGTextMetadata) usize;
pub extern fn kreuzberg_text_metadata_character_count(ptr: ?*const KREUZBERGTextMetadata) usize;
pub extern fn kreuzberg_text_metadata_headers(ptr: ?*const KREUZBERGTextMetadata) [*c]u8;
pub extern fn kreuzberg_header_metadata_from_json(json: [*c]const u8) ?*KREUZBERGHeaderMetadata;
pub extern fn kreuzberg_header_metadata_to_json(ptr: ?*const KREUZBERGHeaderMetadata) [*c]u8;
pub extern fn kreuzberg_header_metadata_free(ptr: ?*KREUZBERGHeaderMetadata) void;
pub extern fn kreuzberg_header_metadata_level(ptr: ?*const KREUZBERGHeaderMetadata) u8;
pub extern fn kreuzberg_header_metadata_text(ptr: ?*const KREUZBERGHeaderMetadata) [*c]u8;
pub extern fn kreuzberg_header_metadata_id(ptr: ?*const KREUZBERGHeaderMetadata) [*c]u8;
pub extern fn kreuzberg_header_metadata_depth(ptr: ?*const KREUZBERGHeaderMetadata) usize;
pub extern fn kreuzberg_header_metadata_html_offset(ptr: ?*const KREUZBERGHeaderMetadata) usize;
pub extern fn kreuzberg_link_metadata_from_json(json: [*c]const u8) ?*KREUZBERGLinkMetadata;
pub extern fn kreuzberg_link_metadata_to_json(ptr: ?*const KREUZBERGLinkMetadata) [*c]u8;
pub extern fn kreuzberg_link_metadata_free(ptr: ?*KREUZBERGLinkMetadata) void;
pub extern fn kreuzberg_link_metadata_href(ptr: ?*const KREUZBERGLinkMetadata) [*c]u8;
pub extern fn kreuzberg_link_metadata_text(ptr: ?*const KREUZBERGLinkMetadata) [*c]u8;
pub extern fn kreuzberg_link_metadata_title(ptr: ?*const KREUZBERGLinkMetadata) [*c]u8;
pub extern fn kreuzberg_link_metadata_link_type(ptr: ?*const KREUZBERGLinkMetadata) ?*KREUZBERGLinkType;
pub extern fn kreuzberg_link_metadata_rel(ptr: ?*const KREUZBERGLinkMetadata) [*c]u8;
pub extern fn kreuzberg_image_metadata_type_from_json(json: [*c]const u8) ?*KREUZBERGImageMetadataType;
pub extern fn kreuzberg_image_metadata_type_to_json(ptr: ?*const KREUZBERGImageMetadataType) [*c]u8;
pub extern fn kreuzberg_image_metadata_type_free(ptr: ?*KREUZBERGImageMetadataType) void;
pub extern fn kreuzberg_image_metadata_type_src(ptr: ?*const KREUZBERGImageMetadataType) [*c]u8;
pub extern fn kreuzberg_image_metadata_type_alt(ptr: ?*const KREUZBERGImageMetadataType) [*c]u8;
pub extern fn kreuzberg_image_metadata_type_title(ptr: ?*const KREUZBERGImageMetadataType) [*c]u8;
pub extern fn kreuzberg_image_metadata_type_image_type(ptr: ?*const KREUZBERGImageMetadataType) ?*KREUZBERGImageType;
pub extern fn kreuzberg_structured_data_from_json(json: [*c]const u8) ?*KREUZBERGStructuredData;
pub extern fn kreuzberg_structured_data_to_json(ptr: ?*const KREUZBERGStructuredData) [*c]u8;
pub extern fn kreuzberg_structured_data_free(ptr: ?*KREUZBERGStructuredData) void;
pub extern fn kreuzberg_structured_data_data_type(ptr: ?*const KREUZBERGStructuredData) ?*KREUZBERGStructuredDataType;
pub extern fn kreuzberg_structured_data_raw_json(ptr: ?*const KREUZBERGStructuredData) [*c]u8;
pub extern fn kreuzberg_structured_data_schema_type(ptr: ?*const KREUZBERGStructuredData) [*c]u8;
pub extern fn kreuzberg_html_metadata_from_json(json: [*c]const u8) ?*KREUZBERGHtmlMetadata;
pub extern fn kreuzberg_html_metadata_to_json(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_free(ptr: ?*KREUZBERGHtmlMetadata) void;
pub extern fn kreuzberg_html_metadata_title(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_description(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_keywords(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_author(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_canonical_url(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_base_href(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_language(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_text_direction(ptr: ?*const KREUZBERGHtmlMetadata) ?*KREUZBERGTextDirection;
pub extern fn kreuzberg_html_metadata_open_graph(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_twitter_card(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_meta_tags(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_headers(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_links(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_images(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_structured_data(ptr: ?*const KREUZBERGHtmlMetadata) [*c]u8;
pub extern fn kreuzberg_html_metadata_from(metadata: ?*const KREUZBERGHtmlMetadata) ?*KREUZBERGHtmlMetadata;
pub extern fn kreuzberg_ocr_metadata_from_json(json: [*c]const u8) ?*KREUZBERGOcrMetadata;
pub extern fn kreuzberg_ocr_metadata_to_json(ptr: ?*const KREUZBERGOcrMetadata) [*c]u8;
pub extern fn kreuzberg_ocr_metadata_free(ptr: ?*KREUZBERGOcrMetadata) void;
pub extern fn kreuzberg_ocr_metadata_language(ptr: ?*const KREUZBERGOcrMetadata) [*c]u8;
pub extern fn kreuzberg_ocr_metadata_psm(ptr: ?*const KREUZBERGOcrMetadata) i32;
pub extern fn kreuzberg_ocr_metadata_output_format(ptr: ?*const KREUZBERGOcrMetadata) [*c]u8;
pub extern fn kreuzberg_ocr_metadata_table_count(ptr: ?*const KREUZBERGOcrMetadata) usize;
pub extern fn kreuzberg_ocr_metadata_table_rows(ptr: ?*const KREUZBERGOcrMetadata) usize;
pub extern fn kreuzberg_ocr_metadata_table_cols(ptr: ?*const KREUZBERGOcrMetadata) usize;
pub extern fn kreuzberg_error_metadata_from_json(json: [*c]const u8) ?*KREUZBERGErrorMetadata;
pub extern fn kreuzberg_error_metadata_to_json(ptr: ?*const KREUZBERGErrorMetadata) [*c]u8;
pub extern fn kreuzberg_error_metadata_free(ptr: ?*KREUZBERGErrorMetadata) void;
pub extern fn kreuzberg_error_metadata_error_type(ptr: ?*const KREUZBERGErrorMetadata) [*c]u8;
pub extern fn kreuzberg_error_metadata_message(ptr: ?*const KREUZBERGErrorMetadata) [*c]u8;
pub extern fn kreuzberg_pptx_metadata_from_json(json: [*c]const u8) ?*KREUZBERGPptxMetadata;
pub extern fn kreuzberg_pptx_metadata_to_json(ptr: ?*const KREUZBERGPptxMetadata) [*c]u8;
pub extern fn kreuzberg_pptx_metadata_free(ptr: ?*KREUZBERGPptxMetadata) void;
pub extern fn kreuzberg_pptx_metadata_slide_count(ptr: ?*const KREUZBERGPptxMetadata) usize;
pub extern fn kreuzberg_pptx_metadata_slide_names(ptr: ?*const KREUZBERGPptxMetadata) [*c]u8;
pub extern fn kreuzberg_pptx_metadata_image_count(ptr: ?*const KREUZBERGPptxMetadata) usize;
pub extern fn kreuzberg_pptx_metadata_table_count(ptr: ?*const KREUZBERGPptxMetadata) usize;
pub extern fn kreuzberg_docx_metadata_from_json(json: [*c]const u8) ?*KREUZBERGDocxMetadata;
pub extern fn kreuzberg_docx_metadata_to_json(ptr: ?*const KREUZBERGDocxMetadata) [*c]u8;
pub extern fn kreuzberg_docx_metadata_free(ptr: ?*KREUZBERGDocxMetadata) void;
pub extern fn kreuzberg_docx_metadata_custom_properties(ptr: ?*const KREUZBERGDocxMetadata) [*c]u8;
pub extern fn kreuzberg_csv_metadata_from_json(json: [*c]const u8) ?*KREUZBERGCsvMetadata;
pub extern fn kreuzberg_csv_metadata_to_json(ptr: ?*const KREUZBERGCsvMetadata) [*c]u8;
pub extern fn kreuzberg_csv_metadata_free(ptr: ?*KREUZBERGCsvMetadata) void;
pub extern fn kreuzberg_csv_metadata_row_count(ptr: ?*const KREUZBERGCsvMetadata) usize;
pub extern fn kreuzberg_csv_metadata_column_count(ptr: ?*const KREUZBERGCsvMetadata) usize;
pub extern fn kreuzberg_csv_metadata_delimiter(ptr: ?*const KREUZBERGCsvMetadata) [*c]u8;
pub extern fn kreuzberg_csv_metadata_has_header(ptr: ?*const KREUZBERGCsvMetadata) i32;
pub extern fn kreuzberg_csv_metadata_column_types(ptr: ?*const KREUZBERGCsvMetadata) [*c]u8;
pub extern fn kreuzberg_bibtex_metadata_from_json(json: [*c]const u8) ?*KREUZBERGBibtexMetadata;
pub extern fn kreuzberg_bibtex_metadata_to_json(ptr: ?*const KREUZBERGBibtexMetadata) [*c]u8;
pub extern fn kreuzberg_bibtex_metadata_free(ptr: ?*KREUZBERGBibtexMetadata) void;
pub extern fn kreuzberg_bibtex_metadata_entry_count(ptr: ?*const KREUZBERGBibtexMetadata) usize;
pub extern fn kreuzberg_bibtex_metadata_citation_keys(ptr: ?*const KREUZBERGBibtexMetadata) [*c]u8;
pub extern fn kreuzberg_bibtex_metadata_authors(ptr: ?*const KREUZBERGBibtexMetadata) [*c]u8;
pub extern fn kreuzberg_bibtex_metadata_year_range(ptr: ?*const KREUZBERGBibtexMetadata) ?*KREUZBERGYearRange;
pub extern fn kreuzberg_bibtex_metadata_entry_types(ptr: ?*const KREUZBERGBibtexMetadata) [*c]u8;
pub extern fn kreuzberg_citation_metadata_from_json(json: [*c]const u8) ?*KREUZBERGCitationMetadata;
pub extern fn kreuzberg_citation_metadata_to_json(ptr: ?*const KREUZBERGCitationMetadata) [*c]u8;
pub extern fn kreuzberg_citation_metadata_free(ptr: ?*KREUZBERGCitationMetadata) void;
pub extern fn kreuzberg_citation_metadata_citation_count(ptr: ?*const KREUZBERGCitationMetadata) usize;
pub extern fn kreuzberg_citation_metadata_format(ptr: ?*const KREUZBERGCitationMetadata) [*c]u8;
pub extern fn kreuzberg_citation_metadata_authors(ptr: ?*const KREUZBERGCitationMetadata) [*c]u8;
pub extern fn kreuzberg_citation_metadata_year_range(ptr: ?*const KREUZBERGCitationMetadata) ?*KREUZBERGYearRange;
pub extern fn kreuzberg_citation_metadata_dois(ptr: ?*const KREUZBERGCitationMetadata) [*c]u8;
pub extern fn kreuzberg_citation_metadata_keywords(ptr: ?*const KREUZBERGCitationMetadata) [*c]u8;
pub extern fn kreuzberg_year_range_from_json(json: [*c]const u8) ?*KREUZBERGYearRange;
pub extern fn kreuzberg_year_range_to_json(ptr: ?*const KREUZBERGYearRange) [*c]u8;
pub extern fn kreuzberg_year_range_free(ptr: ?*KREUZBERGYearRange) void;
pub extern fn kreuzberg_year_range_min(ptr: ?*const KREUZBERGYearRange) u32;
pub extern fn kreuzberg_year_range_max(ptr: ?*const KREUZBERGYearRange) u32;
pub extern fn kreuzberg_year_range_years(ptr: ?*const KREUZBERGYearRange) [*c]u8;
pub extern fn kreuzberg_fiction_book_metadata_from_json(json: [*c]const u8) ?*KREUZBERGFictionBookMetadata;
pub extern fn kreuzberg_fiction_book_metadata_to_json(ptr: ?*const KREUZBERGFictionBookMetadata) [*c]u8;
pub extern fn kreuzberg_fiction_book_metadata_free(ptr: ?*KREUZBERGFictionBookMetadata) void;
pub extern fn kreuzberg_fiction_book_metadata_genres(ptr: ?*const KREUZBERGFictionBookMetadata) [*c]u8;
pub extern fn kreuzberg_fiction_book_metadata_sequences(ptr: ?*const KREUZBERGFictionBookMetadata) [*c]u8;
pub extern fn kreuzberg_fiction_book_metadata_annotation(ptr: ?*const KREUZBERGFictionBookMetadata) [*c]u8;
pub extern fn kreuzberg_dbf_metadata_from_json(json: [*c]const u8) ?*KREUZBERGDbfMetadata;
pub extern fn kreuzberg_dbf_metadata_to_json(ptr: ?*const KREUZBERGDbfMetadata) [*c]u8;
pub extern fn kreuzberg_dbf_metadata_free(ptr: ?*KREUZBERGDbfMetadata) void;
pub extern fn kreuzberg_dbf_metadata_record_count(ptr: ?*const KREUZBERGDbfMetadata) usize;
pub extern fn kreuzberg_dbf_metadata_field_count(ptr: ?*const KREUZBERGDbfMetadata) usize;
pub extern fn kreuzberg_dbf_metadata_fields(ptr: ?*const KREUZBERGDbfMetadata) [*c]u8;
pub extern fn kreuzberg_dbf_field_info_from_json(json: [*c]const u8) ?*KREUZBERGDbfFieldInfo;
pub extern fn kreuzberg_dbf_field_info_to_json(ptr: ?*const KREUZBERGDbfFieldInfo) [*c]u8;
pub extern fn kreuzberg_dbf_field_info_free(ptr: ?*KREUZBERGDbfFieldInfo) void;
pub extern fn kreuzberg_dbf_field_info_name(ptr: ?*const KREUZBERGDbfFieldInfo) [*c]u8;
pub extern fn kreuzberg_dbf_field_info_field_type(ptr: ?*const KREUZBERGDbfFieldInfo) [*c]u8;
pub extern fn kreuzberg_jats_metadata_from_json(json: [*c]const u8) ?*KREUZBERGJatsMetadata;
pub extern fn kreuzberg_jats_metadata_to_json(ptr: ?*const KREUZBERGJatsMetadata) [*c]u8;
pub extern fn kreuzberg_jats_metadata_free(ptr: ?*KREUZBERGJatsMetadata) void;
pub extern fn kreuzberg_jats_metadata_copyright(ptr: ?*const KREUZBERGJatsMetadata) [*c]u8;
pub extern fn kreuzberg_jats_metadata_license(ptr: ?*const KREUZBERGJatsMetadata) [*c]u8;
pub extern fn kreuzberg_jats_metadata_history_dates(ptr: ?*const KREUZBERGJatsMetadata) [*c]u8;
pub extern fn kreuzberg_jats_metadata_contributor_roles(ptr: ?*const KREUZBERGJatsMetadata) [*c]u8;
pub extern fn kreuzberg_contributor_role_from_json(json: [*c]const u8) ?*KREUZBERGContributorRole;
pub extern fn kreuzberg_contributor_role_to_json(ptr: ?*const KREUZBERGContributorRole) [*c]u8;
pub extern fn kreuzberg_contributor_role_free(ptr: ?*KREUZBERGContributorRole) void;
pub extern fn kreuzberg_contributor_role_name(ptr: ?*const KREUZBERGContributorRole) [*c]u8;
pub extern fn kreuzberg_contributor_role_role(ptr: ?*const KREUZBERGContributorRole) [*c]u8;
pub extern fn kreuzberg_epub_metadata_from_json(json: [*c]const u8) ?*KREUZBERGEpubMetadata;
pub extern fn kreuzberg_epub_metadata_to_json(ptr: ?*const KREUZBERGEpubMetadata) [*c]u8;
pub extern fn kreuzberg_epub_metadata_free(ptr: ?*KREUZBERGEpubMetadata) void;
pub extern fn kreuzberg_epub_metadata_coverage(ptr: ?*const KREUZBERGEpubMetadata) [*c]u8;
pub extern fn kreuzberg_epub_metadata_dc_format(ptr: ?*const KREUZBERGEpubMetadata) [*c]u8;
pub extern fn kreuzberg_epub_metadata_relation(ptr: ?*const KREUZBERGEpubMetadata) [*c]u8;
pub extern fn kreuzberg_epub_metadata_source(ptr: ?*const KREUZBERGEpubMetadata) [*c]u8;
pub extern fn kreuzberg_epub_metadata_dc_type(ptr: ?*const KREUZBERGEpubMetadata) [*c]u8;
pub extern fn kreuzberg_epub_metadata_cover_image(ptr: ?*const KREUZBERGEpubMetadata) [*c]u8;
pub extern fn kreuzberg_pst_metadata_from_json(json: [*c]const u8) ?*KREUZBERGPstMetadata;
pub extern fn kreuzberg_pst_metadata_to_json(ptr: ?*const KREUZBERGPstMetadata) [*c]u8;
pub extern fn kreuzberg_pst_metadata_free(ptr: ?*KREUZBERGPstMetadata) void;
pub extern fn kreuzberg_pst_metadata_message_count(ptr: ?*const KREUZBERGPstMetadata) usize;
pub extern fn kreuzberg_ocr_confidence_from_json(json: [*c]const u8) ?*KREUZBERGOcrConfidence;
pub extern fn kreuzberg_ocr_confidence_to_json(ptr: ?*const KREUZBERGOcrConfidence) [*c]u8;
pub extern fn kreuzberg_ocr_confidence_free(ptr: ?*KREUZBERGOcrConfidence) void;
pub extern fn kreuzberg_ocr_confidence_detection(ptr: ?*const KREUZBERGOcrConfidence) f64;
pub extern fn kreuzberg_ocr_confidence_recognition(ptr: ?*const KREUZBERGOcrConfidence) f64;
pub extern fn kreuzberg_ocr_rotation_from_json(json: [*c]const u8) ?*KREUZBERGOcrRotation;
pub extern fn kreuzberg_ocr_rotation_to_json(ptr: ?*const KREUZBERGOcrRotation) [*c]u8;
pub extern fn kreuzberg_ocr_rotation_free(ptr: ?*KREUZBERGOcrRotation) void;
pub extern fn kreuzberg_ocr_rotation_angle_degrees(ptr: ?*const KREUZBERGOcrRotation) f64;
pub extern fn kreuzberg_ocr_rotation_confidence(ptr: ?*const KREUZBERGOcrRotation) f64;
pub extern fn kreuzberg_ocr_element_from_json(json: [*c]const u8) ?*KREUZBERGOcrElement;
pub extern fn kreuzberg_ocr_element_to_json(ptr: ?*const KREUZBERGOcrElement) [*c]u8;
pub extern fn kreuzberg_ocr_element_free(ptr: ?*KREUZBERGOcrElement) void;
pub extern fn kreuzberg_ocr_element_text(ptr: ?*const KREUZBERGOcrElement) [*c]u8;
pub extern fn kreuzberg_ocr_element_geometry(ptr: ?*const KREUZBERGOcrElement) ?*KREUZBERGOcrBoundingGeometry;
pub extern fn kreuzberg_ocr_element_confidence(ptr: ?*const KREUZBERGOcrElement) ?*KREUZBERGOcrConfidence;
pub extern fn kreuzberg_ocr_element_level(ptr: ?*const KREUZBERGOcrElement) ?*KREUZBERGOcrElementLevel;
pub extern fn kreuzberg_ocr_element_rotation(ptr: ?*const KREUZBERGOcrElement) ?*KREUZBERGOcrRotation;
pub extern fn kreuzberg_ocr_element_page_number(ptr: ?*const KREUZBERGOcrElement) usize;
pub extern fn kreuzberg_ocr_element_parent_id(ptr: ?*const KREUZBERGOcrElement) [*c]u8;
pub extern fn kreuzberg_ocr_element_backend_metadata(ptr: ?*const KREUZBERGOcrElement) [*c]u8;
pub extern fn kreuzberg_ocr_element_config_from_json(json: [*c]const u8) ?*KREUZBERGOcrElementConfig;
pub extern fn kreuzberg_ocr_element_config_to_json(ptr: ?*const KREUZBERGOcrElementConfig) [*c]u8;
pub extern fn kreuzberg_ocr_element_config_free(ptr: ?*KREUZBERGOcrElementConfig) void;
pub extern fn kreuzberg_ocr_element_config_include_elements(ptr: ?*const KREUZBERGOcrElementConfig) i32;
pub extern fn kreuzberg_ocr_element_config_min_level(ptr: ?*const KREUZBERGOcrElementConfig) ?*KREUZBERGOcrElementLevel;
pub extern fn kreuzberg_ocr_element_config_min_confidence(ptr: ?*const KREUZBERGOcrElementConfig) f64;
pub extern fn kreuzberg_ocr_element_config_build_hierarchy(ptr: ?*const KREUZBERGOcrElementConfig) i32;
pub extern fn kreuzberg_page_structure_from_json(json: [*c]const u8) ?*KREUZBERGPageStructure;
pub extern fn kreuzberg_page_structure_to_json(ptr: ?*const KREUZBERGPageStructure) [*c]u8;
pub extern fn kreuzberg_page_structure_free(ptr: ?*KREUZBERGPageStructure) void;
pub extern fn kreuzberg_page_structure_total_count(ptr: ?*const KREUZBERGPageStructure) usize;
pub extern fn kreuzberg_page_structure_unit_type(ptr: ?*const KREUZBERGPageStructure) ?*KREUZBERGPageUnitType;
pub extern fn kreuzberg_page_structure_boundaries(ptr: ?*const KREUZBERGPageStructure) [*c]u8;
pub extern fn kreuzberg_page_structure_pages(ptr: ?*const KREUZBERGPageStructure) [*c]u8;
pub extern fn kreuzberg_page_boundary_from_json(json: [*c]const u8) ?*KREUZBERGPageBoundary;
pub extern fn kreuzberg_page_boundary_to_json(ptr: ?*const KREUZBERGPageBoundary) [*c]u8;
pub extern fn kreuzberg_page_boundary_free(ptr: ?*KREUZBERGPageBoundary) void;
pub extern fn kreuzberg_page_boundary_byte_start(ptr: ?*const KREUZBERGPageBoundary) usize;
pub extern fn kreuzberg_page_boundary_byte_end(ptr: ?*const KREUZBERGPageBoundary) usize;
pub extern fn kreuzberg_page_boundary_page_number(ptr: ?*const KREUZBERGPageBoundary) usize;
pub extern fn kreuzberg_page_info_from_json(json: [*c]const u8) ?*KREUZBERGPageInfo;
pub extern fn kreuzberg_page_info_to_json(ptr: ?*const KREUZBERGPageInfo) [*c]u8;
pub extern fn kreuzberg_page_info_free(ptr: ?*KREUZBERGPageInfo) void;
pub extern fn kreuzberg_page_info_number(ptr: ?*const KREUZBERGPageInfo) usize;
pub extern fn kreuzberg_page_info_title(ptr: ?*const KREUZBERGPageInfo) [*c]u8;
pub extern fn kreuzberg_page_info_image_count(ptr: ?*const KREUZBERGPageInfo) usize;
pub extern fn kreuzberg_page_info_table_count(ptr: ?*const KREUZBERGPageInfo) usize;
pub extern fn kreuzberg_page_info_hidden(ptr: ?*const KREUZBERGPageInfo) i32;
pub extern fn kreuzberg_page_info_is_blank(ptr: ?*const KREUZBERGPageInfo) i32;
pub extern fn kreuzberg_page_info_has_vector_graphics(ptr: ?*const KREUZBERGPageInfo) i32;
pub extern fn kreuzberg_page_content_from_json(json: [*c]const u8) ?*KREUZBERGPageContent;
pub extern fn kreuzberg_page_content_to_json(ptr: ?*const KREUZBERGPageContent) [*c]u8;
pub extern fn kreuzberg_page_content_free(ptr: ?*KREUZBERGPageContent) void;
pub extern fn kreuzberg_page_content_page_number(ptr: ?*const KREUZBERGPageContent) usize;
pub extern fn kreuzberg_page_content_content(ptr: ?*const KREUZBERGPageContent) [*c]u8;
pub extern fn kreuzberg_page_content_tables(ptr: ?*const KREUZBERGPageContent) [*c]u8;
pub extern fn kreuzberg_page_content_images(ptr: ?*const KREUZBERGPageContent) [*c]u8;
pub extern fn kreuzberg_page_content_hierarchy(ptr: ?*const KREUZBERGPageContent) ?*KREUZBERGPageHierarchy;
pub extern fn kreuzberg_page_content_is_blank(ptr: ?*const KREUZBERGPageContent) i32;
pub extern fn kreuzberg_page_content_layout_regions(ptr: ?*const KREUZBERGPageContent) [*c]u8;
pub extern fn kreuzberg_layout_region_from_json(json: [*c]const u8) ?*KREUZBERGLayoutRegion;
pub extern fn kreuzberg_layout_region_to_json(ptr: ?*const KREUZBERGLayoutRegion) [*c]u8;
pub extern fn kreuzberg_layout_region_free(ptr: ?*KREUZBERGLayoutRegion) void;
pub extern fn kreuzberg_layout_region_class_name(ptr: ?*const KREUZBERGLayoutRegion) [*c]u8;
pub extern fn kreuzberg_layout_region_confidence(ptr: ?*const KREUZBERGLayoutRegion) f64;
pub extern fn kreuzberg_layout_region_area_fraction(ptr: ?*const KREUZBERGLayoutRegion) f64;
pub extern fn kreuzberg_page_hierarchy_from_json(json: [*c]const u8) ?*KREUZBERGPageHierarchy;
pub extern fn kreuzberg_page_hierarchy_to_json(ptr: ?*const KREUZBERGPageHierarchy) [*c]u8;
pub extern fn kreuzberg_page_hierarchy_free(ptr: ?*KREUZBERGPageHierarchy) void;
pub extern fn kreuzberg_page_hierarchy_block_count(ptr: ?*const KREUZBERGPageHierarchy) usize;
pub extern fn kreuzberg_page_hierarchy_blocks(ptr: ?*const KREUZBERGPageHierarchy) [*c]u8;
pub extern fn kreuzberg_hierarchical_block_from_json(json: [*c]const u8) ?*KREUZBERGHierarchicalBlock;
pub extern fn kreuzberg_hierarchical_block_to_json(ptr: ?*const KREUZBERGHierarchicalBlock) [*c]u8;
pub extern fn kreuzberg_hierarchical_block_free(ptr: ?*KREUZBERGHierarchicalBlock) void;
pub extern fn kreuzberg_hierarchical_block_text(ptr: ?*const KREUZBERGHierarchicalBlock) [*c]u8;
pub extern fn kreuzberg_hierarchical_block_font_size(ptr: ?*const KREUZBERGHierarchicalBlock) f32;
pub extern fn kreuzberg_hierarchical_block_level(ptr: ?*const KREUZBERGHierarchicalBlock) [*c]u8;
pub extern fn kreuzberg_table_from_json(json: [*c]const u8) ?*KREUZBERGTable;
pub extern fn kreuzberg_table_to_json(ptr: ?*const KREUZBERGTable) [*c]u8;
pub extern fn kreuzberg_table_free(ptr: ?*KREUZBERGTable) void;
pub extern fn kreuzberg_table_cells(ptr: ?*const KREUZBERGTable) [*c]u8;
pub extern fn kreuzberg_table_markdown(ptr: ?*const KREUZBERGTable) [*c]u8;
pub extern fn kreuzberg_table_page_number(ptr: ?*const KREUZBERGTable) usize;
pub extern fn kreuzberg_table_cell_from_json(json: [*c]const u8) ?*KREUZBERGTableCell;
pub extern fn kreuzberg_table_cell_to_json(ptr: ?*const KREUZBERGTableCell) [*c]u8;
pub extern fn kreuzberg_table_cell_free(ptr: ?*KREUZBERGTableCell) void;
pub extern fn kreuzberg_table_cell_content(ptr: ?*const KREUZBERGTableCell) [*c]u8;
pub extern fn kreuzberg_table_cell_row_span(ptr: ?*const KREUZBERGTableCell) usize;
pub extern fn kreuzberg_table_cell_col_span(ptr: ?*const KREUZBERGTableCell) usize;
pub extern fn kreuzberg_table_cell_is_header(ptr: ?*const KREUZBERGTableCell) i32;
pub extern fn kreuzberg_uri_from_json(json: [*c]const u8) ?*KREUZBERGUri;
pub extern fn kreuzberg_uri_to_json(ptr: ?*const KREUZBERGUri) [*c]u8;
pub extern fn kreuzberg_uri_free(ptr: ?*KREUZBERGUri) void;
pub extern fn kreuzberg_uri_url(ptr: ?*const KREUZBERGUri) [*c]u8;
pub extern fn kreuzberg_uri_label(ptr: ?*const KREUZBERGUri) [*c]u8;
pub extern fn kreuzberg_uri_page(ptr: ?*const KREUZBERGUri) u32;
pub extern fn kreuzberg_uri_kind(ptr: ?*const KREUZBERGUri) ?*KREUZBERGUriKind;
pub extern fn kreuzberg_string_buffer_pool_free(ptr: ?*KREUZBERGStringBufferPool) void;
pub extern fn kreuzberg_byte_buffer_pool_free(ptr: ?*KREUZBERGByteBufferPool) void;
pub extern fn kreuzberg_tracing_layer_free(ptr: ?*KREUZBERGTracingLayer) void;
pub extern fn kreuzberg_api_doc_free(ptr: ?*KREUZBERGApiDoc) void;
pub extern fn kreuzberg_info_response_from_json(json: [*c]const u8) ?*KREUZBERGInfoResponse;
pub extern fn kreuzberg_info_response_to_json(ptr: ?*const KREUZBERGInfoResponse) [*c]u8;
pub extern fn kreuzberg_info_response_free(ptr: ?*KREUZBERGInfoResponse) void;
pub extern fn kreuzberg_info_response_version(ptr: ?*const KREUZBERGInfoResponse) [*c]u8;
pub extern fn kreuzberg_info_response_rust_backend(ptr: ?*const KREUZBERGInfoResponse) i32;
pub extern fn kreuzberg_extract_response_free(ptr: ?*KREUZBERGExtractResponse) void;
pub extern fn kreuzberg_embed_request_from_json(json: [*c]const u8) ?*KREUZBERGEmbedRequest;
pub extern fn kreuzberg_embed_request_to_json(ptr: ?*const KREUZBERGEmbedRequest) [*c]u8;
pub extern fn kreuzberg_embed_request_free(ptr: ?*KREUZBERGEmbedRequest) void;
pub extern fn kreuzberg_embed_request_texts(ptr: ?*const KREUZBERGEmbedRequest) [*c]u8;
pub extern fn kreuzberg_embed_request_config(ptr: ?*const KREUZBERGEmbedRequest) ?*KREUZBERGEmbeddingConfig;
pub extern fn kreuzberg_embed_response_from_json(json: [*c]const u8) ?*KREUZBERGEmbedResponse;
pub extern fn kreuzberg_embed_response_to_json(ptr: ?*const KREUZBERGEmbedResponse) [*c]u8;
pub extern fn kreuzberg_embed_response_free(ptr: ?*KREUZBERGEmbedResponse) void;
pub extern fn kreuzberg_embed_response_embeddings(ptr: ?*const KREUZBERGEmbedResponse) [*c]u8;
pub extern fn kreuzberg_embed_response_model(ptr: ?*const KREUZBERGEmbedResponse) [*c]u8;
pub extern fn kreuzberg_embed_response_dimensions(ptr: ?*const KREUZBERGEmbedResponse) usize;
pub extern fn kreuzberg_embed_response_count(ptr: ?*const KREUZBERGEmbedResponse) usize;
pub extern fn kreuzberg_chunk_request_from_json(json: [*c]const u8) ?*KREUZBERGChunkRequest;
pub extern fn kreuzberg_chunk_request_to_json(ptr: ?*const KREUZBERGChunkRequest) [*c]u8;
pub extern fn kreuzberg_chunk_request_free(ptr: ?*KREUZBERGChunkRequest) void;
pub extern fn kreuzberg_chunk_request_text(ptr: ?*const KREUZBERGChunkRequest) [*c]u8;
pub extern fn kreuzberg_chunk_request_chunker_type(ptr: ?*const KREUZBERGChunkRequest) [*c]u8;
pub extern fn kreuzberg_chunk_response_from_json(json: [*c]const u8) ?*KREUZBERGChunkResponse;
pub extern fn kreuzberg_chunk_response_to_json(ptr: ?*const KREUZBERGChunkResponse) [*c]u8;
pub extern fn kreuzberg_chunk_response_free(ptr: ?*KREUZBERGChunkResponse) void;
pub extern fn kreuzberg_chunk_response_chunk_count(ptr: ?*const KREUZBERGChunkResponse) usize;
pub extern fn kreuzberg_chunk_response_input_size_bytes(ptr: ?*const KREUZBERGChunkResponse) usize;
pub extern fn kreuzberg_chunk_response_chunker_type(ptr: ?*const KREUZBERGChunkResponse) [*c]u8;
pub extern fn kreuzberg_detect_response_from_json(json: [*c]const u8) ?*KREUZBERGDetectResponse;
pub extern fn kreuzberg_detect_response_to_json(ptr: ?*const KREUZBERGDetectResponse) [*c]u8;
pub extern fn kreuzberg_detect_response_free(ptr: ?*KREUZBERGDetectResponse) void;
pub extern fn kreuzberg_detect_response_mime_type(ptr: ?*const KREUZBERGDetectResponse) [*c]u8;
pub extern fn kreuzberg_detect_response_filename(ptr: ?*const KREUZBERGDetectResponse) [*c]u8;
pub extern fn kreuzberg_manifest_entry_response_from_json(json: [*c]const u8) ?*KREUZBERGManifestEntryResponse;
pub extern fn kreuzberg_manifest_entry_response_to_json(ptr: ?*const KREUZBERGManifestEntryResponse) [*c]u8;
pub extern fn kreuzberg_manifest_entry_response_free(ptr: ?*KREUZBERGManifestEntryResponse) void;
pub extern fn kreuzberg_manifest_entry_response_relative_path(ptr: ?*const KREUZBERGManifestEntryResponse) [*c]u8;
pub extern fn kreuzberg_manifest_entry_response_sha256(ptr: ?*const KREUZBERGManifestEntryResponse) [*c]u8;
pub extern fn kreuzberg_manifest_entry_response_size_bytes(ptr: ?*const KREUZBERGManifestEntryResponse) u64;
pub extern fn kreuzberg_manifest_entry_response_source_url(ptr: ?*const KREUZBERGManifestEntryResponse) [*c]u8;
pub extern fn kreuzberg_manifest_response_from_json(json: [*c]const u8) ?*KREUZBERGManifestResponse;
pub extern fn kreuzberg_manifest_response_to_json(ptr: ?*const KREUZBERGManifestResponse) [*c]u8;
pub extern fn kreuzberg_manifest_response_free(ptr: ?*KREUZBERGManifestResponse) void;
pub extern fn kreuzberg_manifest_response_kreuzberg_version(ptr: ?*const KREUZBERGManifestResponse) [*c]u8;
pub extern fn kreuzberg_manifest_response_total_size_bytes(ptr: ?*const KREUZBERGManifestResponse) u64;
pub extern fn kreuzberg_manifest_response_model_count(ptr: ?*const KREUZBERGManifestResponse) usize;
pub extern fn kreuzberg_manifest_response_models(ptr: ?*const KREUZBERGManifestResponse) [*c]u8;
pub extern fn kreuzberg_warm_response_from_json(json: [*c]const u8) ?*KREUZBERGWarmResponse;
pub extern fn kreuzberg_warm_response_to_json(ptr: ?*const KREUZBERGWarmResponse) [*c]u8;
pub extern fn kreuzberg_warm_response_free(ptr: ?*KREUZBERGWarmResponse) void;
pub extern fn kreuzberg_warm_response_cache_dir(ptr: ?*const KREUZBERGWarmResponse) [*c]u8;
pub extern fn kreuzberg_warm_response_downloaded(ptr: ?*const KREUZBERGWarmResponse) [*c]u8;
pub extern fn kreuzberg_warm_response_already_cached(ptr: ?*const KREUZBERGWarmResponse) [*c]u8;
pub extern fn kreuzberg_structured_extraction_response_from_json(json: [*c]const u8) ?*KREUZBERGStructuredExtractionResponse;
pub extern fn kreuzberg_structured_extraction_response_to_json(ptr: ?*const KREUZBERGStructuredExtractionResponse) [*c]u8;
pub extern fn kreuzberg_structured_extraction_response_free(ptr: ?*KREUZBERGStructuredExtractionResponse) void;
pub extern fn kreuzberg_structured_extraction_response_structured_output(ptr: ?*const KREUZBERGStructuredExtractionResponse) [*c]u8;
pub extern fn kreuzberg_structured_extraction_response_content(ptr: ?*const KREUZBERGStructuredExtractionResponse) [*c]u8;
pub extern fn kreuzberg_structured_extraction_response_mime_type(ptr: ?*const KREUZBERGStructuredExtractionResponse) [*c]u8;
pub extern fn kreuzberg_open_web_document_response_from_json(json: [*c]const u8) ?*KREUZBERGOpenWebDocumentResponse;
pub extern fn kreuzberg_open_web_document_response_to_json(ptr: ?*const KREUZBERGOpenWebDocumentResponse) [*c]u8;
pub extern fn kreuzberg_open_web_document_response_free(ptr: ?*KREUZBERGOpenWebDocumentResponse) void;
pub extern fn kreuzberg_open_web_document_response_page_content(ptr: ?*const KREUZBERGOpenWebDocumentResponse) [*c]u8;
pub extern fn kreuzberg_docling_compat_response_from_json(json: [*c]const u8) ?*KREUZBERGDoclingCompatResponse;
pub extern fn kreuzberg_docling_compat_response_to_json(ptr: ?*const KREUZBERGDoclingCompatResponse) [*c]u8;
pub extern fn kreuzberg_docling_compat_response_free(ptr: ?*KREUZBERGDoclingCompatResponse) void;
pub extern fn kreuzberg_docling_compat_response_status(ptr: ?*const KREUZBERGDoclingCompatResponse) [*c]u8;
pub extern fn kreuzberg_detect_mime_type_params_from_json(json: [*c]const u8) ?*KREUZBERGDetectMimeTypeParams;
pub extern fn kreuzberg_detect_mime_type_params_to_json(ptr: ?*const KREUZBERGDetectMimeTypeParams) [*c]u8;
pub extern fn kreuzberg_detect_mime_type_params_free(ptr: ?*KREUZBERGDetectMimeTypeParams) void;
pub extern fn kreuzberg_detect_mime_type_params_path(ptr: ?*const KREUZBERGDetectMimeTypeParams) [*c]u8;
pub extern fn kreuzberg_detect_mime_type_params_use_content(ptr: ?*const KREUZBERGDetectMimeTypeParams) i32;
pub extern fn kreuzberg_cache_warm_params_from_json(json: [*c]const u8) ?*KREUZBERGCacheWarmParams;
pub extern fn kreuzberg_cache_warm_params_to_json(ptr: ?*const KREUZBERGCacheWarmParams) [*c]u8;
pub extern fn kreuzberg_cache_warm_params_free(ptr: ?*KREUZBERGCacheWarmParams) void;
pub extern fn kreuzberg_cache_warm_params_all_embeddings(ptr: ?*const KREUZBERGCacheWarmParams) i32;
pub extern fn kreuzberg_cache_warm_params_embedding_model(ptr: ?*const KREUZBERGCacheWarmParams) [*c]u8;
pub extern fn kreuzberg_embed_text_params_from_json(json: [*c]const u8) ?*KREUZBERGEmbedTextParams;
pub extern fn kreuzberg_embed_text_params_to_json(ptr: ?*const KREUZBERGEmbedTextParams) [*c]u8;
pub extern fn kreuzberg_embed_text_params_free(ptr: ?*KREUZBERGEmbedTextParams) void;
pub extern fn kreuzberg_embed_text_params_texts(ptr: ?*const KREUZBERGEmbedTextParams) [*c]u8;
pub extern fn kreuzberg_embed_text_params_preset(ptr: ?*const KREUZBERGEmbedTextParams) [*c]u8;
pub extern fn kreuzberg_embed_text_params_model(ptr: ?*const KREUZBERGEmbedTextParams) [*c]u8;
pub extern fn kreuzberg_embed_text_params_api_key(ptr: ?*const KREUZBERGEmbedTextParams) [*c]u8;
pub extern fn kreuzberg_embed_text_params_embedding_plugin(ptr: ?*const KREUZBERGEmbedTextParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_from_json(json: [*c]const u8) ?*KREUZBERGExtractStructuredParams;
pub extern fn kreuzberg_extract_structured_params_to_json(ptr: ?*const KREUZBERGExtractStructuredParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_free(ptr: ?*KREUZBERGExtractStructuredParams) void;
pub extern fn kreuzberg_extract_structured_params_path(ptr: ?*const KREUZBERGExtractStructuredParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_schema(ptr: ?*const KREUZBERGExtractStructuredParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_model(ptr: ?*const KREUZBERGExtractStructuredParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_schema_name(ptr: ?*const KREUZBERGExtractStructuredParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_schema_description(ptr: ?*const KREUZBERGExtractStructuredParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_prompt(ptr: ?*const KREUZBERGExtractStructuredParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_api_key(ptr: ?*const KREUZBERGExtractStructuredParams) [*c]u8;
pub extern fn kreuzberg_extract_structured_params_strict(ptr: ?*const KREUZBERGExtractStructuredParams) i32;
pub extern fn kreuzberg_chunk_text_params_from_json(json: [*c]const u8) ?*KREUZBERGChunkTextParams;
pub extern fn kreuzberg_chunk_text_params_to_json(ptr: ?*const KREUZBERGChunkTextParams) [*c]u8;
pub extern fn kreuzberg_chunk_text_params_free(ptr: ?*KREUZBERGChunkTextParams) void;
pub extern fn kreuzberg_chunk_text_params_text(ptr: ?*const KREUZBERGChunkTextParams) [*c]u8;
pub extern fn kreuzberg_chunk_text_params_max_characters(ptr: ?*const KREUZBERGChunkTextParams) usize;
pub extern fn kreuzberg_chunk_text_params_overlap(ptr: ?*const KREUZBERGChunkTextParams) usize;
pub extern fn kreuzberg_chunk_text_params_chunker_type(ptr: ?*const KREUZBERGChunkTextParams) [*c]u8;
pub extern fn kreuzberg_chunk_text_params_topic_threshold(ptr: ?*const KREUZBERGChunkTextParams) f32;
pub extern fn kreuzberg_detected_boundary_from_json(json: [*c]const u8) ?*KREUZBERGDetectedBoundary;
pub extern fn kreuzberg_detected_boundary_to_json(ptr: ?*const KREUZBERGDetectedBoundary) [*c]u8;
pub extern fn kreuzberg_detected_boundary_free(ptr: ?*KREUZBERGDetectedBoundary) void;
pub extern fn kreuzberg_detected_boundary_byte_offset(ptr: ?*const KREUZBERGDetectedBoundary) usize;
pub extern fn kreuzberg_detected_boundary_is_header(ptr: ?*const KREUZBERGDetectedBoundary) i32;
pub extern fn kreuzberg_chunking_result_from_json(json: [*c]const u8) ?*KREUZBERGChunkingResult;
pub extern fn kreuzberg_chunking_result_to_json(ptr: ?*const KREUZBERGChunkingResult) [*c]u8;
pub extern fn kreuzberg_chunking_result_free(ptr: ?*KREUZBERGChunkingResult) void;
pub extern fn kreuzberg_chunking_result_chunks(ptr: ?*const KREUZBERGChunkingResult) [*c]u8;
pub extern fn kreuzberg_chunking_result_chunk_count(ptr: ?*const KREUZBERGChunkingResult) usize;
pub extern fn kreuzberg_merged_chunk_free(ptr: ?*KREUZBERGMergedChunk) void;
pub extern fn kreuzberg_merged_chunk_text(ptr: ?*const KREUZBERGMergedChunk) [*c]u8;
pub extern fn kreuzberg_merged_chunk_byte_start(ptr: ?*const KREUZBERGMergedChunk) usize;
pub extern fn kreuzberg_merged_chunk_byte_end(ptr: ?*const KREUZBERGMergedChunk) usize;
pub extern fn kreuzberg_embedding_preset_from_json(json: [*c]const u8) ?*KREUZBERGEmbeddingPreset;
pub extern fn kreuzberg_embedding_preset_to_json(ptr: ?*const KREUZBERGEmbeddingPreset) [*c]u8;
pub extern fn kreuzberg_embedding_preset_free(ptr: ?*KREUZBERGEmbeddingPreset) void;
pub extern fn kreuzberg_embedding_preset_name(ptr: ?*const KREUZBERGEmbeddingPreset) [*c]u8;
pub extern fn kreuzberg_embedding_preset_chunk_size(ptr: ?*const KREUZBERGEmbeddingPreset) usize;
pub extern fn kreuzberg_embedding_preset_overlap(ptr: ?*const KREUZBERGEmbeddingPreset) usize;
pub extern fn kreuzberg_embedding_preset_model_repo(ptr: ?*const KREUZBERGEmbeddingPreset) [*c]u8;
pub extern fn kreuzberg_embedding_preset_pooling(ptr: ?*const KREUZBERGEmbeddingPreset) [*c]u8;
pub extern fn kreuzberg_embedding_preset_model_file(ptr: ?*const KREUZBERGEmbeddingPreset) [*c]u8;
pub extern fn kreuzberg_embedding_preset_dimensions(ptr: ?*const KREUZBERGEmbeddingPreset) usize;
pub extern fn kreuzberg_embedding_preset_description(ptr: ?*const KREUZBERGEmbeddingPreset) [*c]u8;
pub extern fn kreuzberg_yake_params_from_json(json: [*c]const u8) ?*KREUZBERGYakeParams;
pub extern fn kreuzberg_yake_params_to_json(ptr: ?*const KREUZBERGYakeParams) [*c]u8;
pub extern fn kreuzberg_yake_params_free(ptr: ?*KREUZBERGYakeParams) void;
pub extern fn kreuzberg_yake_params_window_size(ptr: ?*const KREUZBERGYakeParams) usize;
pub extern fn kreuzberg_yake_params_default() ?*KREUZBERGYakeParams;
pub extern fn kreuzberg_rake_params_from_json(json: [*c]const u8) ?*KREUZBERGRakeParams;
pub extern fn kreuzberg_rake_params_to_json(ptr: ?*const KREUZBERGRakeParams) [*c]u8;
pub extern fn kreuzberg_rake_params_free(ptr: ?*KREUZBERGRakeParams) void;
pub extern fn kreuzberg_rake_params_min_word_length(ptr: ?*const KREUZBERGRakeParams) usize;
pub extern fn kreuzberg_rake_params_max_words_per_phrase(ptr: ?*const KREUZBERGRakeParams) usize;
pub extern fn kreuzberg_rake_params_default() ?*KREUZBERGRakeParams;
pub extern fn kreuzberg_keyword_config_from_json(json: [*c]const u8) ?*KREUZBERGKeywordConfig;
pub extern fn kreuzberg_keyword_config_to_json(ptr: ?*const KREUZBERGKeywordConfig) [*c]u8;
pub extern fn kreuzberg_keyword_config_free(ptr: ?*KREUZBERGKeywordConfig) void;
pub extern fn kreuzberg_keyword_config_algorithm(ptr: ?*const KREUZBERGKeywordConfig) ?*KREUZBERGKeywordAlgorithm;
pub extern fn kreuzberg_keyword_config_max_keywords(ptr: ?*const KREUZBERGKeywordConfig) usize;
pub extern fn kreuzberg_keyword_config_min_score(ptr: ?*const KREUZBERGKeywordConfig) f32;
pub extern fn kreuzberg_keyword_config_language(ptr: ?*const KREUZBERGKeywordConfig) [*c]u8;
pub extern fn kreuzberg_keyword_config_yake_params(ptr: ?*const KREUZBERGKeywordConfig) ?*KREUZBERGYakeParams;
pub extern fn kreuzberg_keyword_config_rake_params(ptr: ?*const KREUZBERGKeywordConfig) ?*KREUZBERGRakeParams;
pub extern fn kreuzberg_keyword_config_default() ?*KREUZBERGKeywordConfig;
pub extern fn kreuzberg_keyword_from_json(json: [*c]const u8) ?*KREUZBERGKeyword;
pub extern fn kreuzberg_keyword_to_json(ptr: ?*const KREUZBERGKeyword) [*c]u8;
pub extern fn kreuzberg_keyword_free(ptr: ?*KREUZBERGKeyword) void;
pub extern fn kreuzberg_keyword_text(ptr: ?*const KREUZBERGKeyword) [*c]u8;
pub extern fn kreuzberg_keyword_score(ptr: ?*const KREUZBERGKeyword) f32;
pub extern fn kreuzberg_keyword_algorithm(ptr: ?*const KREUZBERGKeyword) ?*KREUZBERGKeywordAlgorithm;
pub extern fn kreuzberg_keyword_positions(ptr: ?*const KREUZBERGKeyword) [*c]u8;
pub extern fn kreuzberg_ocr_cache_stats_free(ptr: ?*KREUZBERGOcrCacheStats) void;
pub extern fn kreuzberg_ocr_cache_stats_total_files(ptr: ?*const KREUZBERGOcrCacheStats) usize;
pub extern fn kreuzberg_ocr_cache_stats_total_size_mb(ptr: ?*const KREUZBERGOcrCacheStats) f64;
pub extern fn kreuzberg_recognized_table_from_json(json: [*c]const u8) ?*KREUZBERGRecognizedTable;
pub extern fn kreuzberg_recognized_table_to_json(ptr: ?*const KREUZBERGRecognizedTable) [*c]u8;
pub extern fn kreuzberg_recognized_table_free(ptr: ?*KREUZBERGRecognizedTable) void;
pub extern fn kreuzberg_recognized_table_detection_bbox(ptr: ?*const KREUZBERGRecognizedTable) ?*KREUZBERGBBox;
pub extern fn kreuzberg_recognized_table_cells(ptr: ?*const KREUZBERGRecognizedTable) [*c]u8;
pub extern fn kreuzberg_recognized_table_markdown(ptr: ?*const KREUZBERGRecognizedTable) [*c]u8;
pub extern fn kreuzberg_tessdata_manager_free(ptr: ?*KREUZBERGTessdataManager) void;
pub extern fn kreuzberg_tessdata_manager_cache_dir(this_: ?*const KREUZBERGTessdataManager) [*c]u8;
pub extern fn kreuzberg_tessdata_manager_is_language_cached(this_: ?*const KREUZBERGTessdataManager, lang: [*c]const u8) i32;
pub extern fn kreuzberg_tessdata_manager_ensure_all_languages(this_: ?*const KREUZBERGTessdataManager) usize;
pub extern fn kreuzberg_paddle_ocr_config_from_json(json: [*c]const u8) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_to_json(ptr: ?*const KREUZBERGPaddleOcrConfig) [*c]u8;
pub extern fn kreuzberg_paddle_ocr_config_free(ptr: ?*KREUZBERGPaddleOcrConfig) void;
pub extern fn kreuzberg_paddle_ocr_config_language(ptr: ?*const KREUZBERGPaddleOcrConfig) [*c]u8;
pub extern fn kreuzberg_paddle_ocr_config_cache_dir(ptr: ?*const KREUZBERGPaddleOcrConfig) [*c]u8;
pub extern fn kreuzberg_paddle_ocr_config_use_angle_cls(ptr: ?*const KREUZBERGPaddleOcrConfig) i32;
pub extern fn kreuzberg_paddle_ocr_config_enable_table_detection(ptr: ?*const KREUZBERGPaddleOcrConfig) i32;
pub extern fn kreuzberg_paddle_ocr_config_det_db_thresh(ptr: ?*const KREUZBERGPaddleOcrConfig) f32;
pub extern fn kreuzberg_paddle_ocr_config_det_db_box_thresh(ptr: ?*const KREUZBERGPaddleOcrConfig) f32;
pub extern fn kreuzberg_paddle_ocr_config_det_db_unclip_ratio(ptr: ?*const KREUZBERGPaddleOcrConfig) f32;
pub extern fn kreuzberg_paddle_ocr_config_det_limit_side_len(ptr: ?*const KREUZBERGPaddleOcrConfig) u32;
pub extern fn kreuzberg_paddle_ocr_config_rec_batch_num(ptr: ?*const KREUZBERGPaddleOcrConfig) u32;
pub extern fn kreuzberg_paddle_ocr_config_padding(ptr: ?*const KREUZBERGPaddleOcrConfig) u32;
pub extern fn kreuzberg_paddle_ocr_config_drop_score(ptr: ?*const KREUZBERGPaddleOcrConfig) f32;
pub extern fn kreuzberg_paddle_ocr_config_model_tier(ptr: ?*const KREUZBERGPaddleOcrConfig) [*c]u8;
pub extern fn kreuzberg_paddle_ocr_config_with_cache_dir(this_: ?*KREUZBERGPaddleOcrConfig, path: [*c]const u8) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_table_detection(this_: ?*KREUZBERGPaddleOcrConfig, enable: i32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_angle_cls(this_: ?*KREUZBERGPaddleOcrConfig, enable: i32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_det_db_thresh(this_: ?*KREUZBERGPaddleOcrConfig, threshold: f32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_det_db_box_thresh(this_: ?*KREUZBERGPaddleOcrConfig, threshold: f32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_det_db_unclip_ratio(this_: ?*KREUZBERGPaddleOcrConfig, ratio: f32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_det_limit_side_len(this_: ?*KREUZBERGPaddleOcrConfig, length: u32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_rec_batch_num(this_: ?*KREUZBERGPaddleOcrConfig, batch_size: u32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_drop_score(this_: ?*KREUZBERGPaddleOcrConfig, score: f32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_padding(this_: ?*KREUZBERGPaddleOcrConfig, padding: u32) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_with_model_tier(this_: ?*KREUZBERGPaddleOcrConfig, tier: [*c]const u8) ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_paddle_ocr_config_default() ?*KREUZBERGPaddleOcrConfig;
pub extern fn kreuzberg_model_paths_from_json(json: [*c]const u8) ?*KREUZBERGModelPaths;
pub extern fn kreuzberg_model_paths_to_json(ptr: ?*const KREUZBERGModelPaths) [*c]u8;
pub extern fn kreuzberg_model_paths_free(ptr: ?*KREUZBERGModelPaths) void;
pub extern fn kreuzberg_model_paths_det_model(ptr: ?*const KREUZBERGModelPaths) [*c]u8;
pub extern fn kreuzberg_model_paths_cls_model(ptr: ?*const KREUZBERGModelPaths) [*c]u8;
pub extern fn kreuzberg_model_paths_rec_model(ptr: ?*const KREUZBERGModelPaths) [*c]u8;
pub extern fn kreuzberg_model_paths_dict_file(ptr: ?*const KREUZBERGModelPaths) [*c]u8;
pub extern fn kreuzberg_orientation_result_from_json(json: [*c]const u8) ?*KREUZBERGOrientationResult;
pub extern fn kreuzberg_orientation_result_to_json(ptr: ?*const KREUZBERGOrientationResult) [*c]u8;
pub extern fn kreuzberg_orientation_result_free(ptr: ?*KREUZBERGOrientationResult) void;
pub extern fn kreuzberg_orientation_result_degrees(ptr: ?*const KREUZBERGOrientationResult) u32;
pub extern fn kreuzberg_orientation_result_confidence(ptr: ?*const KREUZBERGOrientationResult) f32;
pub extern fn kreuzberg_b_box_from_json(json: [*c]const u8) ?*KREUZBERGBBox;
pub extern fn kreuzberg_b_box_to_json(ptr: ?*const KREUZBERGBBox) [*c]u8;
pub extern fn kreuzberg_b_box_free(ptr: ?*KREUZBERGBBox) void;
pub extern fn kreuzberg_b_box_x1(ptr: ?*const KREUZBERGBBox) f32;
pub extern fn kreuzberg_b_box_y1(ptr: ?*const KREUZBERGBBox) f32;
pub extern fn kreuzberg_b_box_x2(ptr: ?*const KREUZBERGBBox) f32;
pub extern fn kreuzberg_b_box_y2(ptr: ?*const KREUZBERGBBox) f32;
pub extern fn kreuzberg_layout_detection_from_json(json: [*c]const u8) ?*KREUZBERGLayoutDetection;
pub extern fn kreuzberg_layout_detection_to_json(ptr: ?*const KREUZBERGLayoutDetection) [*c]u8;
pub extern fn kreuzberg_layout_detection_free(ptr: ?*KREUZBERGLayoutDetection) void;
pub extern fn kreuzberg_layout_detection_class_name(ptr: ?*const KREUZBERGLayoutDetection) ?*KREUZBERGLayoutClass;
pub extern fn kreuzberg_layout_detection_confidence(ptr: ?*const KREUZBERGLayoutDetection) f32;
pub extern fn kreuzberg_layout_detection_bbox(ptr: ?*const KREUZBERGLayoutDetection) ?*KREUZBERGBBox;
pub extern fn kreuzberg_detection_result_from_json(json: [*c]const u8) ?*KREUZBERGDetectionResult;
pub extern fn kreuzberg_detection_result_to_json(ptr: ?*const KREUZBERGDetectionResult) [*c]u8;
pub extern fn kreuzberg_detection_result_free(ptr: ?*KREUZBERGDetectionResult) void;
pub extern fn kreuzberg_detection_result_page_width(ptr: ?*const KREUZBERGDetectionResult) u32;
pub extern fn kreuzberg_detection_result_page_height(ptr: ?*const KREUZBERGDetectionResult) u32;
pub extern fn kreuzberg_detection_result_detections(ptr: ?*const KREUZBERGDetectionResult) [*c]u8;
pub extern fn kreuzberg_embedded_file_free(ptr: ?*KREUZBERGEmbeddedFile) void;
pub extern fn kreuzberg_embedded_file_name(ptr: ?*const KREUZBERGEmbeddedFile) [*c]u8;
pub extern fn kreuzberg_embedded_file_data(ptr: ?*const KREUZBERGEmbeddedFile, out_len: [*c]usize) [*c]u8;
pub extern fn kreuzberg_embedded_file_mime_type(ptr: ?*const KREUZBERGEmbeddedFile) [*c]u8;
pub extern fn kreuzberg_execution_provider_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_execution_provider_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_output_format_from_i32(value: i32) i32;
pub extern fn kreuzberg_output_format_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_html_theme_from_i32(value: i32) i32;
pub extern fn kreuzberg_html_theme_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_table_model_from_i32(value: i32) i32;
pub extern fn kreuzberg_table_model_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_chunker_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_chunker_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_chunk_sizing_from_i32(value: i32) i32;
pub extern fn kreuzberg_chunk_sizing_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_embedding_model_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_embedding_model_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_code_content_mode_from_i32(value: i32) i32;
pub extern fn kreuzberg_code_content_mode_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_frac_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_frac_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_ocr_backend_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_ocr_backend_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_processing_stage_from_i32(value: i32) i32;
pub extern fn kreuzberg_processing_stage_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_reduction_level_from_i32(value: i32) i32;
pub extern fn kreuzberg_reduction_level_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_pdf_annotation_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_pdf_annotation_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_block_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_block_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_inline_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_inline_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_relationship_kind_from_i32(value: i32) i32;
pub extern fn kreuzberg_relationship_kind_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_content_layer_from_i32(value: i32) i32;
pub extern fn kreuzberg_content_layer_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_node_content_from_i32(value: i32) i32;
pub extern fn kreuzberg_node_content_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_annotation_kind_from_i32(value: i32) i32;
pub extern fn kreuzberg_annotation_kind_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_extraction_method_from_i32(value: i32) i32;
pub extern fn kreuzberg_extraction_method_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_chunk_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_chunk_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_image_kind_from_i32(value: i32) i32;
pub extern fn kreuzberg_image_kind_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_result_format_from_i32(value: i32) i32;
pub extern fn kreuzberg_result_format_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_element_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_element_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_format_metadata_from_i32(value: i32) i32;
pub extern fn kreuzberg_format_metadata_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_text_direction_from_i32(value: i32) i32;
pub extern fn kreuzberg_text_direction_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_link_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_link_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_image_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_image_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_structured_data_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_structured_data_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_ocr_bounding_geometry_from_i32(value: i32) i32;
pub extern fn kreuzberg_ocr_bounding_geometry_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_ocr_element_level_from_i32(value: i32) i32;
pub extern fn kreuzberg_ocr_element_level_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_page_unit_type_from_i32(value: i32) i32;
pub extern fn kreuzberg_page_unit_type_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_uri_kind_from_i32(value: i32) i32;
pub extern fn kreuzberg_uri_kind_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_pool_error_from_i32(value: i32) i32;
pub extern fn kreuzberg_pool_error_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_keyword_algorithm_from_i32(value: i32) i32;
pub extern fn kreuzberg_keyword_algorithm_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_psm_mode_from_i32(value: i32) i32;
pub extern fn kreuzberg_psm_mode_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_paddle_language_from_i32(value: i32) i32;
pub extern fn kreuzberg_paddle_language_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_layout_class_from_i32(value: i32) i32;
pub extern fn kreuzberg_layout_class_from_str(name: [*c]const u8) i32;
pub extern fn kreuzberg_extract_bytes(content: [*c]const u8, content_len: usize, mime_type: [*c]const u8, config: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGExtractionResult;
pub extern fn kreuzberg_extract_file(path: [*c]const u8, mime_type: [*c]const u8, config: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGExtractionResult;
pub extern fn kreuzberg_extract_file_sync(path: [*c]const u8, mime_type: [*c]const u8, config: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGExtractionResult;
pub extern fn kreuzberg_extract_bytes_sync(content: [*c]const u8, content_len: usize, mime_type: [*c]const u8, config: ?*const KREUZBERGExtractionConfig) ?*KREUZBERGExtractionResult;
pub extern fn kreuzberg_batch_extract_files_sync(items: [*c]const u8, config: ?*const KREUZBERGExtractionConfig) [*c]u8;
pub extern fn kreuzberg_batch_extract_bytes_sync(items: [*c]const u8, config: ?*const KREUZBERGExtractionConfig) [*c]u8;
pub extern fn kreuzberg_batch_extract_files(items: [*c]const u8, config: ?*const KREUZBERGExtractionConfig) [*c]u8;
pub extern fn kreuzberg_batch_extract_bytes(items: [*c]const u8, config: ?*const KREUZBERGExtractionConfig) [*c]u8;
pub extern fn kreuzberg_detect_mime_type_from_bytes(content: [*c]const u8, content_len: usize) [*c]u8;
pub extern fn kreuzberg_get_extensions_for_mime(mime_type: [*c]const u8) [*c]u8;
pub extern fn kreuzberg_list_document_extractors() [*c]u8;
pub extern fn kreuzberg_list_ocr_backends() [*c]u8;
pub extern fn kreuzberg_clear_ocr_backends() i32;
pub extern fn kreuzberg_list_post_processors() [*c]u8;
pub extern fn kreuzberg_clear_post_processors() i32;
pub extern fn kreuzberg_list_validators() [*c]u8;
pub extern fn kreuzberg_clear_validators() i32;
pub extern fn kreuzberg_embed_texts_async(texts: [*c]const u8, config: ?*const KREUZBERGEmbeddingConfig) [*c]u8;
pub extern fn kreuzberg_detect_mime_type(path: [*c]const u8, check_exists: i32) [*c]u8;
pub extern fn kreuzberg_embed_texts(texts: [*c]const u8, config: ?*const KREUZBERGEmbeddingConfig) [*c]u8;
pub extern fn kreuzberg_get_embedding_preset(name: [*c]const u8) ?*KREUZBERGEmbeddingPreset;
pub extern fn kreuzberg_list_embedding_presets() [*c]u8;
pub extern fn kreuzberg_register_ocr_backend(name: [*c]const u8, vtable: struct_KREUZBERGKreuzbergOcrBackendVTable, user_data: ?*const anyopaque, out_error: [*c][*c]u8) i32;
pub extern fn kreuzberg_unregister_ocr_backend(name: [*c]const u8, out_error: [*c][*c]u8) i32;
pub extern fn kreuzberg_register_post_processor(name: [*c]const u8, vtable: struct_KREUZBERGKreuzbergPostProcessorVTable, user_data: ?*const anyopaque, out_error: [*c][*c]u8) i32;
pub extern fn kreuzberg_unregister_post_processor(name: [*c]const u8, out_error: [*c][*c]u8) i32;
pub extern fn kreuzberg_register_validator(name: [*c]const u8, vtable: struct_KREUZBERGKreuzbergValidatorVTable, user_data: ?*const anyopaque, out_error: [*c][*c]u8) i32;
pub extern fn kreuzberg_unregister_validator(name: [*c]const u8, out_error: [*c][*c]u8) i32;
pub extern fn kreuzberg_register_embedding_backend(name: [*c]const u8, vtable: struct_KREUZBERGKreuzbergEmbeddingBackendVTable, user_data: ?*const anyopaque, out_error: [*c][*c]u8) i32;
pub extern fn kreuzberg_unregister_embedding_backend(name: [*c]const u8, out_error: [*c][*c]u8) i32;

pub const __VERSION__ = "Aro aro-zig";
pub const __Aro__ = "";
pub const __STDC__ = @as(c_int, 1);
pub const __STDC_HOSTED__ = @as(c_int, 1);
pub const __STDC_UTF_16__ = @as(c_int, 1);
pub const __STDC_UTF_32__ = @as(c_int, 1);
pub const __STDC_EMBED_NOT_FOUND__ = @as(c_int, 0);
pub const __STDC_EMBED_FOUND__ = @as(c_int, 1);
pub const __STDC_EMBED_EMPTY__ = @as(c_int, 2);
pub const __STDC_NO_THREADS__ = @as(c_int, 1);
pub const __STDC_VERSION__ = @as(c_long, 201710);
pub const __GNUC__ = @as(c_int, 4);
pub const __GNUC_MINOR__ = @as(c_int, 2);
pub const __GNUC_PATCHLEVEL__ = @as(c_int, 1);
pub const __ARO_EMULATE_NO__ = @as(c_int, 0);
pub const __ARO_EMULATE_CLANG__ = @as(c_int, 1);
pub const __ARO_EMULATE_GCC__ = @as(c_int, 2);
pub const __ARO_EMULATE_MSVC__ = @as(c_int, 3);
pub const __ARO_EMULATE__ = __ARO_EMULATE_CLANG__;
pub inline fn __building_module(x: anytype) @TypeOf(@as(c_int, 0)) {
    _ = &x;
    return @as(c_int, 0);
}
pub const __APPLE__ = @as(c_int, 1);
pub const __APPLE_CC__ = @as(c_int, 6000);
pub const __ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ = __helpers.promoteIntLiteral(c_int, 260301, .decimal);
pub const __ENVIRONMENT_OS_VERSION_MIN_REQUIRED__ = __helpers.promoteIntLiteral(c_int, 260301, .decimal);
pub const __aarch64__ = @as(c_int, 1);
pub const __AARCH64EL__ = @as(c_int, 1);
pub const __AARCH64_SIMD__ = @as(c_int, 1);
pub const __ARM64_ARCH_8__ = @as(c_int, 1);
pub const __ARM_NEON__ = @as(c_int, 1);
pub const __arm64 = @as(c_int, 1);
pub const __arm64__ = @as(c_int, 1);
pub const __AARCH64_CMODEL_SMALL__ = @as(c_int, 1);
pub const __ARM_FP = @as(c_int, 0xE);
pub const __ARM_NEON = @as(c_int, 1);
pub const __ARM_NEON_FP = @as(c_int, 0xE);
pub const __ARM_FEATURE_BF16 = @as(c_int, 1);
pub const __ARM_FEATURE_BF16_VECTOR_ARITHMETIC = @as(c_int, 1);
pub const __ARM_BF16_FORMAT_ALTERNATIVE = @as(c_int, 1);
pub const __ARM_FEATURE_BF16_SCALAR_ARITHMETIC = @as(c_int, 1);
pub const __ARM_FEATURE_SME = @as(c_int, 1);
pub const __ARM_FEATURE_LOCALLY_STREAMING = @as(c_int, 1);
pub const __ARM_FEATURE_SHA3 = @as(c_int, 1);
pub const __ARM_FEATURE_SHA512 = @as(c_int, 1);
pub const __ARM_FEATURE_UNALIGNED = @as(c_int, 1);
pub const __ARM_FEATURE_FP16_VECTOR_ARITHMETIC = @as(c_int, 1);
pub const __ARM_FEATURE_RCPC = @as(c_int, 1);
pub const __ARM_FEATURE_SME2 = @as(c_int, 1);
pub const __ARM_FEATURE_CRC32 = @as(c_int, 1);
pub const __ARM_FEATURE_AES = @as(c_int, 1);
pub const __ARM_FEATURE_SHA2 = @as(c_int, 1);
pub const __ARM_FEATURE_PAUTH = @as(c_int, 1);
pub const __ARM_FEATURE_BTI = @as(c_int, 1);
pub const __ARM_FEATURE_FP16_SCALAR_ARITHMETIC = @as(c_int, 1);
pub const __ARM_FEATURE_DOTPROD = @as(c_int, 1);
pub const __ARM_FEATURE_MATMUL_INT8 = @as(c_int, 1);
pub const __ARM_FEATURE_ATOMICS = @as(c_int, 1);
pub const __ARM_FEATURE_SVE_MATMUL_INT8 = @as(c_int, 1);
pub const __ARM_FEATURE_FP16_FML = @as(c_int, 1);
pub const _LP64 = @as(c_int, 1);
pub const __LP64__ = @as(c_int, 1);
pub const __ORDER_LITTLE_ENDIAN__ = @as(c_int, 1234);
pub const __ORDER_BIG_ENDIAN__ = @as(c_int, 4321);
pub const __ORDER_PDP_ENDIAN__ = @as(c_int, 3412);
pub const __BYTE_ORDER__ = __ORDER_LITTLE_ENDIAN__;
pub const __LITTLE_ENDIAN__ = @as(c_int, 1);
pub const __MACH__ = @as(c_int, 1);
pub const __nonnull = @compileError("unable to translate C expr: unexpected token '_Nonnull'"); // <builtin>:67:9
pub const __null_unspecified = @compileError("unable to translate C expr: unexpected token '_Null_unspecified'"); // <builtin>:68:9
pub const __nullable = @compileError("unable to translate C expr: unexpected token '_Nullable'"); // <builtin>:69:9
pub const __ATOMIC_RELAXED = @as(c_int, 0);
pub const __ATOMIC_CONSUME = @as(c_int, 1);
pub const __ATOMIC_ACQUIRE = @as(c_int, 2);
pub const __ATOMIC_RELEASE = @as(c_int, 3);
pub const __ATOMIC_ACQ_REL = @as(c_int, 4);
pub const __ATOMIC_SEQ_CST = @as(c_int, 5);
pub const __ATOMIC_BOOL_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_CHAR_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_CHAR16_T_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_CHAR32_T_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_WCHAR_T_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_WINT_T_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_SHORT_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_INT_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_LONG_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_LLONG_LOCK_FREE = @as(c_int, 1);
pub const __ATOMIC_POINTER_LOCK_FREE = @as(c_int, 1);
pub const __CHAR_BIT__ = @as(c_int, 8);
pub const __BOOL_WIDTH__ = @as(c_int, 8);
pub const __SCHAR_MAX__ = @as(c_int, 127);
pub const __SCHAR_WIDTH__ = @as(c_int, 8);
pub const __SHRT_MAX__ = @as(c_int, 32767);
pub const __SHRT_WIDTH__ = @as(c_int, 16);
pub const __INT_MAX__ = __helpers.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __INT_WIDTH__ = @as(c_int, 32);
pub const __LONG_MAX__ = __helpers.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __LONG_WIDTH__ = @as(c_int, 64);
pub const __LONG_LONG_MAX__ = @as(c_longlong, 9223372036854775807);
pub const __LONG_LONG_WIDTH__ = @as(c_int, 64);
pub const __WCHAR_MAX__ = __helpers.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __WCHAR_WIDTH__ = @as(c_int, 32);
pub const __WINT_MAX__ = __helpers.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __WINT_WIDTH__ = @as(c_int, 32);
pub const __INTMAX_MAX__ = __helpers.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __INTMAX_WIDTH__ = @as(c_int, 64);
pub const __SIZE_MAX__ = __helpers.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __SIZE_WIDTH__ = @as(c_int, 64);
pub const __UINTMAX_MAX__ = __helpers.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __UINTMAX_WIDTH__ = @as(c_int, 64);
pub const __PTRDIFF_MAX__ = __helpers.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __PTRDIFF_WIDTH__ = @as(c_int, 64);
pub const __INTPTR_MAX__ = __helpers.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const __INTPTR_WIDTH__ = @as(c_int, 64);
pub const __UINTPTR_MAX__ = __helpers.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const __UINTPTR_WIDTH__ = @as(c_int, 64);
pub const __SIG_ATOMIC_MAX__ = __helpers.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __SIG_ATOMIC_WIDTH__ = @as(c_int, 32);
pub const __BITINT_MAXWIDTH__ = __helpers.promoteIntLiteral(c_int, 65535, .decimal);
pub const __SIZEOF_FLOAT__ = @as(c_int, 4);
pub const __SIZEOF_DOUBLE__ = @as(c_int, 8);
pub const __SIZEOF_LONG_DOUBLE__ = @as(c_int, 8);
pub const __SIZEOF_SHORT__ = @as(c_int, 2);
pub const __SIZEOF_INT__ = @as(c_int, 4);
pub const __SIZEOF_LONG__ = @as(c_int, 8);
pub const __SIZEOF_LONG_LONG__ = @as(c_int, 8);
pub const __SIZEOF_POINTER__ = @as(c_int, 8);
pub const __SIZEOF_PTRDIFF_T__ = @as(c_int, 8);
pub const __SIZEOF_SIZE_T__ = @as(c_int, 8);
pub const __SIZEOF_WCHAR_T__ = @as(c_int, 4);
pub const __SIZEOF_WINT_T__ = @as(c_int, 4);
pub const __SIZEOF_INT128__ = @as(c_int, 16);
pub const __INTPTR_TYPE__ = c_long;
pub const __UINTPTR_TYPE__ = c_ulong;
pub const __INTMAX_TYPE__ = c_long;
pub const __INTMAX_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `L`"); // <builtin>:134:9
pub const __INTMAX_C = __helpers.L_SUFFIX;
pub const __UINTMAX_TYPE__ = c_ulong;
pub const __UINTMAX_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `UL`"); // <builtin>:137:9
pub const __UINTMAX_C = __helpers.UL_SUFFIX;
pub const __PTRDIFF_TYPE__ = c_long;
pub const __SIZE_TYPE__ = c_ulong;
pub const __WCHAR_TYPE__ = c_int;
pub const __WINT_TYPE__ = c_int;
pub const __CHAR16_TYPE__ = c_ushort;
pub const __CHAR32_TYPE__ = c_uint;
pub const __INT8_TYPE__ = i8;
pub const __INT8_FMTd__ = "hhd";
pub const __INT8_FMTi__ = "hhi";
pub const __INT8_C_SUFFIX__ = "";
pub inline fn __INT8_C(c: anytype) @TypeOf(c) {
    _ = &c;
    return c;
}
pub const __INT16_TYPE__ = c_short;
pub const __INT16_FMTd__ = "hd";
pub const __INT16_FMTi__ = "hi";
pub const __INT16_C_SUFFIX__ = "";
pub inline fn __INT16_C(c: anytype) @TypeOf(c) {
    _ = &c;
    return c;
}
pub const __INT32_TYPE__ = c_int;
pub const __INT32_FMTd__ = "d";
pub const __INT32_FMTi__ = "i";
pub const __INT32_C_SUFFIX__ = "";
pub inline fn __INT32_C(c: anytype) @TypeOf(c) {
    _ = &c;
    return c;
}
pub const __INT64_TYPE__ = c_longlong;
pub const __INT64_FMTd__ = "lld";
pub const __INT64_FMTi__ = "lli";
pub const __INT64_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `LL`"); // <builtin>:163:9
pub const __INT64_C = __helpers.LL_SUFFIX;
pub const __UINT8_TYPE__ = u8;
pub const __UINT8_FMTo__ = "hho";
pub const __UINT8_FMTu__ = "hhu";
pub const __UINT8_FMTx__ = "hhx";
pub const __UINT8_FMTX__ = "hhX";
pub const __UINT8_C_SUFFIX__ = "";
pub inline fn __UINT8_C(c: anytype) @TypeOf(c) {
    _ = &c;
    return c;
}
pub const __UINT8_MAX__ = @as(c_int, 255);
pub const __INT8_MAX__ = @as(c_int, 127);
pub const __UINT16_TYPE__ = c_ushort;
pub const __UINT16_FMTo__ = "ho";
pub const __UINT16_FMTu__ = "hu";
pub const __UINT16_FMTx__ = "hx";
pub const __UINT16_FMTX__ = "hX";
pub const __UINT16_C_SUFFIX__ = "";
pub inline fn __UINT16_C(c: anytype) @TypeOf(c) {
    _ = &c;
    return c;
}
pub const __UINT16_MAX__ = __helpers.promoteIntLiteral(c_int, 65535, .decimal);
pub const __INT16_MAX__ = @as(c_int, 32767);
pub const __UINT32_TYPE__ = c_uint;
pub const __UINT32_FMTo__ = "o";
pub const __UINT32_FMTu__ = "u";
pub const __UINT32_FMTx__ = "x";
pub const __UINT32_FMTX__ = "X";
pub const __UINT32_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `U`"); // <builtin>:188:9
pub const __UINT32_C = __helpers.U_SUFFIX;
pub const __UINT32_MAX__ = __helpers.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const __INT32_MAX__ = __helpers.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __UINT64_TYPE__ = c_ulonglong;
pub const __UINT64_FMTo__ = "llo";
pub const __UINT64_FMTu__ = "llu";
pub const __UINT64_FMTx__ = "llx";
pub const __UINT64_FMTX__ = "llX";
pub const __UINT64_C_SUFFIX__ = @compileError("unable to translate macro: undefined identifier `ULL`"); // <builtin>:197:9
pub const __UINT64_C = __helpers.ULL_SUFFIX;
pub const __UINT64_MAX__ = @as(c_ulonglong, 18446744073709551615);
pub const __INT64_MAX__ = @as(c_longlong, 9223372036854775807);
pub const __INT_LEAST8_TYPE__ = i8;
pub const __INT_LEAST8_MAX__ = @as(c_int, 127);
pub const __INT_LEAST8_WIDTH__ = @as(c_int, 8);
pub const INT_LEAST8_FMTd__ = "hhd";
pub const INT_LEAST8_FMTi__ = "hhi";
pub const __UINT_LEAST8_TYPE__ = u8;
pub const __UINT_LEAST8_MAX__ = @as(c_int, 255);
pub const UINT_LEAST8_FMTo__ = "hho";
pub const UINT_LEAST8_FMTu__ = "hhu";
pub const UINT_LEAST8_FMTx__ = "hhx";
pub const UINT_LEAST8_FMTX__ = "hhX";
pub const __INT_FAST8_TYPE__ = i8;
pub const __INT_FAST8_MAX__ = @as(c_int, 127);
pub const __INT_FAST8_WIDTH__ = @as(c_int, 8);
pub const INT_FAST8_FMTd__ = "hhd";
pub const INT_FAST8_FMTi__ = "hhi";
pub const __UINT_FAST8_TYPE__ = u8;
pub const __UINT_FAST8_MAX__ = @as(c_int, 255);
pub const UINT_FAST8_FMTo__ = "hho";
pub const UINT_FAST8_FMTu__ = "hhu";
pub const UINT_FAST8_FMTx__ = "hhx";
pub const UINT_FAST8_FMTX__ = "hhX";
pub const __INT_LEAST16_TYPE__ = c_short;
pub const __INT_LEAST16_MAX__ = @as(c_int, 32767);
pub const __INT_LEAST16_WIDTH__ = @as(c_int, 16);
pub const INT_LEAST16_FMTd__ = "hd";
pub const INT_LEAST16_FMTi__ = "hi";
pub const __UINT_LEAST16_TYPE__ = c_ushort;
pub const __UINT_LEAST16_MAX__ = __helpers.promoteIntLiteral(c_int, 65535, .decimal);
pub const UINT_LEAST16_FMTo__ = "ho";
pub const UINT_LEAST16_FMTu__ = "hu";
pub const UINT_LEAST16_FMTx__ = "hx";
pub const UINT_LEAST16_FMTX__ = "hX";
pub const __INT_FAST16_TYPE__ = c_short;
pub const __INT_FAST16_MAX__ = @as(c_int, 32767);
pub const __INT_FAST16_WIDTH__ = @as(c_int, 16);
pub const INT_FAST16_FMTd__ = "hd";
pub const INT_FAST16_FMTi__ = "hi";
pub const __UINT_FAST16_TYPE__ = c_ushort;
pub const __UINT_FAST16_MAX__ = __helpers.promoteIntLiteral(c_int, 65535, .decimal);
pub const UINT_FAST16_FMTo__ = "ho";
pub const UINT_FAST16_FMTu__ = "hu";
pub const UINT_FAST16_FMTx__ = "hx";
pub const UINT_FAST16_FMTX__ = "hX";
pub const __INT_LEAST32_TYPE__ = c_int;
pub const __INT_LEAST32_MAX__ = __helpers.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __INT_LEAST32_WIDTH__ = @as(c_int, 32);
pub const INT_LEAST32_FMTd__ = "d";
pub const INT_LEAST32_FMTi__ = "i";
pub const __UINT_LEAST32_TYPE__ = c_uint;
pub const __UINT_LEAST32_MAX__ = __helpers.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const UINT_LEAST32_FMTo__ = "o";
pub const UINT_LEAST32_FMTu__ = "u";
pub const UINT_LEAST32_FMTx__ = "x";
pub const UINT_LEAST32_FMTX__ = "X";
pub const __INT_FAST32_TYPE__ = c_int;
pub const __INT_FAST32_MAX__ = __helpers.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const __INT_FAST32_WIDTH__ = @as(c_int, 32);
pub const INT_FAST32_FMTd__ = "d";
pub const INT_FAST32_FMTi__ = "i";
pub const __UINT_FAST32_TYPE__ = c_uint;
pub const __UINT_FAST32_MAX__ = __helpers.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const UINT_FAST32_FMTo__ = "o";
pub const UINT_FAST32_FMTu__ = "u";
pub const UINT_FAST32_FMTx__ = "x";
pub const UINT_FAST32_FMTX__ = "X";
pub const __INT_LEAST64_TYPE__ = c_longlong;
pub const __INT_LEAST64_MAX__ = @as(c_longlong, 9223372036854775807);
pub const __INT_LEAST64_WIDTH__ = @as(c_int, 64);
pub const INT_LEAST64_FMTd__ = "lld";
pub const INT_LEAST64_FMTi__ = "lli";
pub const __UINT_LEAST64_TYPE__ = c_ulonglong;
pub const __UINT_LEAST64_MAX__ = @as(c_ulonglong, 18446744073709551615);
pub const UINT_LEAST64_FMTo__ = "llo";
pub const UINT_LEAST64_FMTu__ = "llu";
pub const UINT_LEAST64_FMTx__ = "llx";
pub const UINT_LEAST64_FMTX__ = "llX";
pub const __INT_FAST64_TYPE__ = c_longlong;
pub const __INT_FAST64_MAX__ = @as(c_longlong, 9223372036854775807);
pub const __INT_FAST64_WIDTH__ = @as(c_int, 64);
pub const INT_FAST64_FMTd__ = "lld";
pub const INT_FAST64_FMTi__ = "lli";
pub const __UINT_FAST64_TYPE__ = c_ulonglong;
pub const __UINT_FAST64_MAX__ = @as(c_ulonglong, 18446744073709551615);
pub const UINT_FAST64_FMTo__ = "llo";
pub const UINT_FAST64_FMTu__ = "llu";
pub const UINT_FAST64_FMTx__ = "llx";
pub const UINT_FAST64_FMTX__ = "llX";
pub const __FLT16_DENORM_MIN__ = @as(f16, 5.9604644775390625e-8);
pub const __FLT16_HAS_DENORM__ = "";
pub const __FLT16_DIG__ = @as(c_int, 3);
pub const __FLT16_DECIMAL_DIG__ = @as(c_int, 5);
pub const __FLT16_EPSILON__ = @as(f16, 9.765625e-4);
pub const __FLT16_HAS_INFINITY__ = "";
pub const __FLT16_HAS_QUIET_NAN__ = "";
pub const __FLT16_MANT_DIG__ = @as(c_int, 11);
pub const __FLT16_MAX_10_EXP__ = @as(c_int, 4);
pub const __FLT16_MAX_EXP__ = @as(c_int, 16);
pub const __FLT16_MAX__ = @as(f16, 6.5504e+4);
pub const __FLT16_MIN_10_EXP__ = -@as(c_int, 4);
pub const __FLT16_MIN_EXP__ = -@as(c_int, 13);
pub const __FLT16_MIN__ = @as(f16, 6.103515625e-5);
pub const __FLT_DENORM_MIN__ = @as(f32, 1.40129846e-45);
pub const __FLT_HAS_DENORM__ = "";
pub const __FLT_DIG__ = @as(c_int, 6);
pub const __FLT_DECIMAL_DIG__ = @as(c_int, 9);
pub const __FLT_EPSILON__ = @as(f32, 1.19209290e-7);
pub const __FLT_HAS_INFINITY__ = "";
pub const __FLT_HAS_QUIET_NAN__ = "";
pub const __FLT_MANT_DIG__ = @as(c_int, 24);
pub const __FLT_MAX_10_EXP__ = @as(c_int, 38);
pub const __FLT_MAX_EXP__ = @as(c_int, 128);
pub const __FLT_MAX__ = @as(f32, 3.40282347e+38);
pub const __FLT_MIN_10_EXP__ = -@as(c_int, 37);
pub const __FLT_MIN_EXP__ = -@as(c_int, 125);
pub const __FLT_MIN__ = @as(f32, 1.17549435e-38);
pub const __DBL_DENORM_MIN__ = @as(f64, 4.9406564584124654e-324);
pub const __DBL_HAS_DENORM__ = "";
pub const __DBL_DIG__ = @as(c_int, 15);
pub const __DBL_DECIMAL_DIG__ = @as(c_int, 17);
pub const __DBL_EPSILON__ = @as(f64, 2.2204460492503131e-16);
pub const __DBL_HAS_INFINITY__ = "";
pub const __DBL_HAS_QUIET_NAN__ = "";
pub const __DBL_MANT_DIG__ = @as(c_int, 53);
pub const __DBL_MAX_10_EXP__ = @as(c_int, 308);
pub const __DBL_MAX_EXP__ = @as(c_int, 1024);
pub const __DBL_MAX__ = @as(f64, 1.7976931348623157e+308);
pub const __DBL_MIN_10_EXP__ = -@as(c_int, 307);
pub const __DBL_MIN_EXP__ = -@as(c_int, 1021);
pub const __DBL_MIN__ = @as(f64, 2.2250738585072014e-308);
pub const __LDBL_DENORM_MIN__ = @as(c_longdouble, 4.9406564584124654e-324);
pub const __LDBL_HAS_DENORM__ = "";
pub const __LDBL_DIG__ = @as(c_int, 15);
pub const __LDBL_DECIMAL_DIG__ = @as(c_int, 17);
pub const __LDBL_EPSILON__ = @as(c_longdouble, 2.2204460492503131e-16);
pub const __LDBL_HAS_INFINITY__ = "";
pub const __LDBL_HAS_QUIET_NAN__ = "";
pub const __LDBL_MANT_DIG__ = @as(c_int, 53);
pub const __LDBL_MAX_10_EXP__ = @as(c_int, 308);
pub const __LDBL_MAX_EXP__ = @as(c_int, 1024);
pub const __LDBL_MAX__ = @as(c_longdouble, 1.7976931348623157e+308);
pub const __LDBL_MIN_10_EXP__ = -@as(c_int, 307);
pub const __LDBL_MIN_EXP__ = -@as(c_int, 1021);
pub const __LDBL_MIN__ = @as(c_longdouble, 2.2250738585072014e-308);
pub const __FLT_EVAL_METHOD__ = @as(c_int, 0);
pub const __FLT_RADIX__ = @as(c_int, 2);
pub const __DECIMAL_DIG__ = __LDBL_DECIMAL_DIG__;
pub const __pic__ = @as(c_int, 2);
pub const __PIC__ = @as(c_int, 2);
pub const KREUZBERG_H = "";
pub const __STDC_VERSION_STDARG_H__ = @as(c_int, 0);
pub const va_start = @compileError("unable to translate macro: undefined identifier `__builtin_va_start`"); // /opt/homebrew/Cellar/zig/0.16.0_1/lib/zig/compiler/aro/include/stdarg.h:12:9
pub const va_end = @compileError("unable to translate macro: undefined identifier `__builtin_va_end`"); // /opt/homebrew/Cellar/zig/0.16.0_1/lib/zig/compiler/aro/include/stdarg.h:14:9
pub const va_arg = @compileError("unable to translate macro: undefined identifier `__builtin_va_arg`"); // /opt/homebrew/Cellar/zig/0.16.0_1/lib/zig/compiler/aro/include/stdarg.h:15:9
pub const __va_copy = @compileError("unable to translate macro: undefined identifier `__builtin_va_copy`"); // /opt/homebrew/Cellar/zig/0.16.0_1/lib/zig/compiler/aro/include/stdarg.h:18:9
pub const va_copy = @compileError("unable to translate macro: undefined identifier `__builtin_va_copy`"); // /opt/homebrew/Cellar/zig/0.16.0_1/lib/zig/compiler/aro/include/stdarg.h:22:9
pub const __GNUC_VA_LIST = @as(c_int, 1);
pub const @"bool" = bool;
pub const @"true" = @as(c_int, 1);
pub const @"false" = @as(c_int, 0);
pub const __bool_true_false_are_defined = @as(c_int, 1);
pub const __CLANG_STDINT_H = "";
pub const _STDINT_H_ = "";
pub const __WORDSIZE = @as(c_int, 64);
pub const _INT8_T = "";
pub const _INT16_T = "";
pub const _INT32_T = "";
pub const _INT64_T = "";
pub const _UINT8_T = "";
pub const _UINT16_T = "";
pub const _UINT32_T = "";
pub const _UINT64_T = "";
pub const _SYS__TYPES_H_ = "";
pub const _CDEFS_H_ = "";
pub const __BEGIN_DECLS = "";
pub const __END_DECLS = "";
pub inline fn __has_cpp_attribute(x: anytype) @TypeOf(@as(c_int, 0)) {
    _ = &x;
    return @as(c_int, 0);
}
pub inline fn __P(protos: anytype) @TypeOf(protos) {
    _ = &protos;
    return protos;
}
pub const __CONCAT = @compileError("unable to translate C expr: unexpected token '##'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:116:9
pub const __STRING = @compileError("unable to translate C expr: unexpected token ''"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:117:9
pub const __const = @compileError("unable to translate C expr: unexpected token 'const'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:119:9
pub const __signed = c_int;
pub const __volatile = @compileError("unable to translate C expr: unexpected token 'volatile'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:121:9
pub const __dead2 = @compileError("unable to translate macro: undefined identifier `__noreturn__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:165:9
pub const __pure2 = @compileError("unable to translate C expr: unexpected token '__attribute__'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:166:9
pub const __stateful_pure = @compileError("unable to translate macro: undefined identifier `__pure__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:167:9
pub const __unused = @compileError("unable to translate macro: undefined identifier `__unused__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:172:9
pub const __used = @compileError("unable to translate macro: undefined identifier `__used__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:177:9
pub const __cold = @compileError("unable to translate macro: undefined identifier `__cold__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:183:9
pub const __returns_nonnull = @compileError("unable to translate macro: undefined identifier `returns_nonnull`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:190:9
pub const __exported = @compileError("unable to translate macro: undefined identifier `__visibility__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:200:9
pub const __exported_push = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:201:9
pub const __exported_push_hidden = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:203:9
pub const __exported_pop = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:204:9
pub const __exported_hidden = @compileError("unable to translate macro: undefined identifier `__private_extern__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:205:9
pub const __deprecated = @compileError("unable to translate macro: undefined identifier `__deprecated__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:223:9
pub const __deprecated_msg = @compileError("unable to translate macro: undefined identifier `__deprecated__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:229:10
pub inline fn __deprecated_enum_msg(_msg: anytype) void {
    _ = &_msg;
    return;
}
pub inline fn __kpi_deprecated(_msg: anytype) void {
    _ = &_msg;
    return;
}
pub const __unavailable = @compileError("unable to translate macro: undefined identifier `__unavailable__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:244:9
pub const __kpi_unavailable = "";
pub const __kpi_deprecated_arm64_macos_unavailable = "";
pub const __dead = "";
pub const __pure = "";
pub const __restrict = @compileError("unable to translate C expr: unexpected token 'restrict'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:266:9
pub const __disable_tail_calls = "";
pub const __not_tail_called = "";
pub const __result_use_check = @compileError("unable to translate macro: undefined identifier `__warn_unused_result__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:322:9
pub const __swift_unavailable = @compileError("unable to translate macro: undefined identifier `__availability__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:332:9
pub inline fn __swift_unavailable_from_async(_msg: anytype) void {
    _ = &_msg;
    return;
}
pub const __swift_nonisolated = "";
pub const __swift_nonisolated_unsafe = "";
pub const __abortlike = __dead2 ++ __cold;
pub const __header_inline = @compileError("unable to translate C expr: unexpected token 'extern'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:383:10
pub const __header_always_inline = @compileError("unable to translate macro: undefined identifier `__always_inline__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:392:10
pub const __unreachable_ok_push = "";
pub const __unreachable_ok_pop = "";
pub const __printflike = @compileError("unable to translate macro: undefined identifier `__format__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:429:9
pub const __printf0like = @compileError("unable to translate macro: undefined identifier `__format__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:431:9
pub const __scanflike = @compileError("unable to translate macro: undefined identifier `__format__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:433:9
pub const __osloglike = @compileError("unable to translate macro: undefined identifier `__format__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:435:9
pub const __IDSTRING = @compileError("unable to translate C expr: unexpected token 'static'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:438:9
pub const __COPYRIGHT = @compileError("unable to translate macro: undefined identifier `copyright`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:441:9
pub const __RCSID = @compileError("unable to translate macro: undefined identifier `rcsid`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:445:9
pub const __SCCSID = @compileError("unable to translate macro: undefined identifier `sccsid`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:449:9
pub const __PROJECT_VERSION = @compileError("unable to translate macro: undefined identifier `project_version`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:453:9
pub inline fn __FBSDID(s: anytype) void {
    _ = &s;
    return;
}
pub const __DECONST = @compileError("unable to translate C expr: unexpected token 'const'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:462:9
pub const __DEVOLATILE = @compileError("unable to translate C expr: unexpected token 'volatile'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:466:9
pub const __DEQUALIFY = @compileError("unable to translate C expr: unexpected token 'const'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:470:9
pub const __alloc_align = @compileError("unable to translate macro: undefined identifier `alloc_align`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:479:9
pub const __alloc_size = @compileError("unable to translate macro: undefined identifier `alloc_size`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:500:9
pub const __has_safe_buffers = @as(c_int, 0);
pub const __unsafe_buffer_usage = "";
pub const __unsafe_buffer_usage_begin = "";
pub const __unsafe_buffer_usage_end = "";
pub const __DARWIN_ONLY_64_BIT_INO_T = @as(c_int, 1);
pub const __DARWIN_ONLY_UNIX_CONFORMANCE = @as(c_int, 1);
pub const __DARWIN_ONLY_VERS_1050 = @as(c_int, 1);
pub const __DARWIN_UNIX03 = @as(c_int, 1);
pub const __DARWIN_64_BIT_INO_T = @as(c_int, 1);
pub const __DARWIN_VERS_1050 = @as(c_int, 1);
pub const __DARWIN_NON_CANCELABLE = @as(c_int, 0);
pub const __DARWIN_SUF_UNIX03 = "";
pub const __DARWIN_SUF_64_BIT_INO_T = "";
pub const __DARWIN_SUF_1050 = "";
pub const __DARWIN_SUF_NON_CANCELABLE = "";
pub const __DARWIN_SUF_EXTSN = "$DARWIN_EXTSN";
pub const __DARWIN_ALIAS = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:790:9
pub const __DARWIN_ALIAS_C = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:791:9
pub const __DARWIN_ALIAS_I = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:792:9
pub const __DARWIN_NOCANCEL = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:793:9
pub const __DARWIN_INODE64 = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:794:9
pub const __DARWIN_1050 = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:796:9
pub const __DARWIN_1050ALIAS = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:797:9
pub const __DARWIN_1050ALIAS_C = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:798:9
pub const __DARWIN_1050ALIAS_I = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:799:9
pub const __DARWIN_1050INODE64 = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:800:9
pub const __DARWIN_EXTSN = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:802:9
pub const __DARWIN_EXTSN_C = @compileError("unable to translate C expr: unexpected token '__asm'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:803:9
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_2_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_2_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_2_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_3_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_3_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_3_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_4_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_4_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_4_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_4_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_5_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_5_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_6_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_6_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_7_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_7_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_8_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_8_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_8_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_8_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_8_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_9_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_9_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_9_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_9_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_10_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_10_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_10_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_10_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_11_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_11_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_11_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_11_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_11_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_12_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_12_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_12_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_12_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_12_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_13_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_13_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_13_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_13_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_13_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_13_5(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_13_6(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_13_7(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_14_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_14_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_14_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_14_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_14_5(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_14_6(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_14_7(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_14_8(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_5(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_6(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_7(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_15_8(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_16_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_16_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_16_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_16_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_16_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_16_5(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_16_6(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_16_7(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_17_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_17_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_17_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_17_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_17_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_17_5(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_17_6(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_17_7(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_18_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_18_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_18_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_18_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_18_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_18_5(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_18_6(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_19_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_26_0(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_26_1(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_26_2(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_26_3(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_IPHONE___IPHONE_26_4(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_0(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_5(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_6(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_7(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_8(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_9(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_10(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_10_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_10_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_11(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_11_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_11_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_11_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_12(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_12_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_12_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_12_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_13(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_13_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_13_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_13_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_14(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_14_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_14_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_14_5(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_14_6(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_15(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_15_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_15_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_10_16(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_11_0(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_11_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_11_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_11_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_11_5(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_11_6(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_12_0(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_12_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_12_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_12_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_12_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_12_5(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_12_6(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_12_7(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_13_0(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_13_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_13_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_13_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_13_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_13_5(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_13_6(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_13_7(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_14_0(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_14_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_14_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_14_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_14_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_14_5(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_14_6(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_14_7(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_15_0(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_15_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_15_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_15_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_15_4(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_15_5(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_15_6(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_16_0(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_26_0(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_26_1(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_26_2(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_26_3(x: anytype) @TypeOf(x) {
    _ = &x;
    return x;
}
pub inline fn __DARWIN_ALIAS_STARTING_MAC___MAC_26_4(x: anytype) void {
    _ = &x;
    return;
}
pub const __DARWIN_ALIAS_STARTING = @compileError("unable to translate macro: undefined identifier `__DARWIN_ALIAS_STARTING_MAC_`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:813:9
pub const ___POSIX_C_DEPRECATED_STARTING_198808L = "";
pub const ___POSIX_C_DEPRECATED_STARTING_199009L = "";
pub const ___POSIX_C_DEPRECATED_STARTING_199209L = "";
pub const ___POSIX_C_DEPRECATED_STARTING_199309L = "";
pub const ___POSIX_C_DEPRECATED_STARTING_199506L = "";
pub const ___POSIX_C_DEPRECATED_STARTING_200112L = "";
pub const ___POSIX_C_DEPRECATED_STARTING_200809L = "";
pub const __POSIX_C_DEPRECATED = @compileError("unable to translate macro: undefined identifier `___POSIX_C_DEPRECATED_STARTING_`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:876:9
pub const __DARWIN_C_ANSI = @as(c_long, 0o10000);
pub const __DARWIN_C_FULL = @as(c_long, 900000);
pub const __DARWIN_C_LEVEL = __DARWIN_C_FULL;
pub const __STDC_WANT_LIB_EXT1__ = @as(c_int, 1);
pub const __DARWIN_NO_LONG_LONG = @as(c_int, 0);
pub const _DARWIN_FEATURE_64_BIT_INODE = @as(c_int, 1);
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE = @as(c_int, 1);
pub const _DARWIN_FEATURE_ONLY_VERS_1050 = @as(c_int, 1);
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE = @as(c_int, 1);
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE = @as(c_int, 3);
pub const __CAST_AWAY_QUALIFIER = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:974:9
pub const __XNU_PRIVATE_EXTERN = @compileError("unable to translate macro: undefined identifier `visibility`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:988:9
pub const __has_ptrcheck = @as(c_int, 0);
pub const __single = "";
pub const __unsafe_indexable = "";
pub inline fn __counted_by(N: anytype) void {
    _ = &N;
    return;
}
pub inline fn __counted_by_or_null(N: anytype) void {
    _ = &N;
    return;
}
pub inline fn __sized_by(N: anytype) void {
    _ = &N;
    return;
}
pub inline fn __sized_by_or_null(N: anytype) void {
    _ = &N;
    return;
}
pub inline fn __ended_by(E: anytype) void {
    _ = &E;
    return;
}
pub inline fn __terminated_by(T: anytype) void {
    _ = &T;
    return;
}
pub const __null_terminated = "";
pub inline fn __ptrcheck_abi_assume_single() void {
    return;
}
pub inline fn __ptrcheck_abi_assume_unsafe_indexable() void {
    return;
}
pub inline fn __unsafe_forge_bidi_indexable(T: anytype, P: anytype, S: anytype) @TypeOf(T(P)) {
    _ = &T;
    _ = &P;
    _ = &S;
    return T(P);
}
pub const __unsafe_forge_single = __helpers.CAST_OR_CALL;
pub inline fn __unsafe_forge_terminated_by(T: anytype, P: anytype, E: anytype) @TypeOf(T(P)) {
    _ = &T;
    _ = &P;
    _ = &E;
    return T(P);
}
pub const __unsafe_forge_null_terminated = __helpers.CAST_OR_CALL;
pub inline fn __terminated_by_to_indexable(P: anytype) @TypeOf(P) {
    _ = &P;
    return P;
}
pub inline fn __unsafe_terminated_by_to_indexable(P: anytype) @TypeOf(P) {
    _ = &P;
    return P;
}
pub inline fn __null_terminated_to_indexable(P: anytype) @TypeOf(P) {
    _ = &P;
    return P;
}
pub inline fn __unsafe_null_terminated_to_indexable(P: anytype) @TypeOf(P) {
    _ = &P;
    return P;
}
pub inline fn __unsafe_terminated_by_from_indexable(T: anytype, P: anytype) @TypeOf(P) {
    _ = &T;
    _ = &P;
    return P;
}
pub inline fn __unsafe_null_terminated_from_indexable(P: anytype) @TypeOf(P) {
    _ = &P;
    return P;
}
pub const __array_decay_dicards_count_in_parameters = "";
pub const __ptrcheck_unavailable = "";
pub inline fn __ptrcheck_unavailable_r(REPLACEMENT: anytype) void {
    _ = &REPLACEMENT;
    return;
}
pub const __ASSUME_PTR_ABI_SINGLE_BEGIN = __ptrcheck_abi_assume_single();
pub const __ASSUME_PTR_ABI_SINGLE_END = __ptrcheck_abi_assume_unsafe_indexable();
pub const __header_indexable = "";
pub const __header_bidi_indexable = "";
pub const __compiler_barrier = @compileError("unable to translate C expr: unexpected token '__asm__'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:1073:9
pub const __enum_open = "";
pub const __enum_closed = "";
pub const __enum_options = "";
pub const __enum_decl = @compileError("unable to translate C expr: unexpected token 'typedef'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:1106:9
pub const __enum_closed_decl = @compileError("unable to translate C expr: unexpected token 'typedef'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:1108:9
pub const __options_decl = @compileError("unable to translate C expr: unexpected token 'typedef'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:1110:9
pub const __options_closed_decl = @compileError("unable to translate C expr: unexpected token 'typedef'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/cdefs.h:1112:9
pub const __kernel_ptr_semantics = "";
pub const __kernel_data_semantics = "";
pub const __kernel_dual_semantics = "";
pub const __xnu_data_size = "";
pub const __xnu_returns_data_pointer = "";
pub const _BSD_MACHINE__TYPES_H_ = "";
pub const _BSD_ARM__TYPES_H_ = "";
pub const __DARWIN_NULL = __helpers.cast(?*anyopaque, @as(c_int, 0));
pub const _SYS__PTHREAD_TYPES_H_ = "";
pub const __PTHREAD_SIZE__ = @as(c_int, 8176);
pub const __PTHREAD_ATTR_SIZE__ = @as(c_int, 56);
pub const __PTHREAD_MUTEXATTR_SIZE__ = @as(c_int, 8);
pub const __PTHREAD_MUTEX_SIZE__ = @as(c_int, 56);
pub const __PTHREAD_CONDATTR_SIZE__ = @as(c_int, 8);
pub const __PTHREAD_COND_SIZE__ = @as(c_int, 40);
pub const __PTHREAD_ONCE_SIZE__ = @as(c_int, 8);
pub const __PTHREAD_RWLOCK_SIZE__ = @as(c_int, 192);
pub const __PTHREAD_RWLOCKATTR_SIZE__ = @as(c_int, 16);
pub const __offsetof = @compileError("unable to translate macro: undefined identifier `__builtin_offsetof`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/_types.h:97:9
pub const _INTPTR_T = "";
pub const _UINTPTR_T = "";
pub const _INTMAX_T = "";
pub const _UINTMAX_T = "";
pub inline fn INT8_C(v: anytype) @TypeOf(v) {
    _ = &v;
    return v;
}
pub inline fn INT16_C(v: anytype) @TypeOf(v) {
    _ = &v;
    return v;
}
pub inline fn INT32_C(v: anytype) @TypeOf(v) {
    _ = &v;
    return v;
}
pub const INT64_C = __helpers.LL_SUFFIX;
pub inline fn UINT8_C(v: anytype) @TypeOf(v) {
    _ = &v;
    return v;
}
pub inline fn UINT16_C(v: anytype) @TypeOf(v) {
    _ = &v;
    return v;
}
pub const UINT32_C = __helpers.U_SUFFIX;
pub const UINT64_C = __helpers.ULL_SUFFIX;
pub const INTMAX_C = __helpers.L_SUFFIX;
pub const UINTMAX_C = __helpers.UL_SUFFIX;
pub const INT8_MAX = @as(c_int, 127);
pub const INT16_MAX = @as(c_int, 32767);
pub const INT32_MAX = __helpers.promoteIntLiteral(c_int, 2147483647, .decimal);
pub const INT64_MAX = @as(c_longlong, 9223372036854775807);
pub const INT8_MIN = -@as(c_int, 128);
pub const INT16_MIN = -__helpers.promoteIntLiteral(c_int, 32768, .decimal);
pub const INT32_MIN = -INT32_MAX - @as(c_int, 1);
pub const INT64_MIN = -INT64_MAX - @as(c_int, 1);
pub const UINT8_MAX = @as(c_int, 255);
pub const UINT16_MAX = __helpers.promoteIntLiteral(c_int, 65535, .decimal);
pub const UINT32_MAX = __helpers.promoteIntLiteral(c_uint, 4294967295, .decimal);
pub const UINT64_MAX = @as(c_ulonglong, 18446744073709551615);
pub const INT_LEAST8_MIN = INT8_MIN;
pub const INT_LEAST16_MIN = INT16_MIN;
pub const INT_LEAST32_MIN = INT32_MIN;
pub const INT_LEAST64_MIN = INT64_MIN;
pub const INT_LEAST8_MAX = INT8_MAX;
pub const INT_LEAST16_MAX = INT16_MAX;
pub const INT_LEAST32_MAX = INT32_MAX;
pub const INT_LEAST64_MAX = INT64_MAX;
pub const UINT_LEAST8_MAX = UINT8_MAX;
pub const UINT_LEAST16_MAX = UINT16_MAX;
pub const UINT_LEAST32_MAX = UINT32_MAX;
pub const UINT_LEAST64_MAX = UINT64_MAX;
pub const INT_FAST8_MIN = INT8_MIN;
pub const INT_FAST16_MIN = INT16_MIN;
pub const INT_FAST32_MIN = INT32_MIN;
pub const INT_FAST64_MIN = INT64_MIN;
pub const INT_FAST8_MAX = INT8_MAX;
pub const INT_FAST16_MAX = INT16_MAX;
pub const INT_FAST32_MAX = INT32_MAX;
pub const INT_FAST64_MAX = INT64_MAX;
pub const UINT_FAST8_MAX = UINT8_MAX;
pub const UINT_FAST16_MAX = UINT16_MAX;
pub const UINT_FAST32_MAX = UINT32_MAX;
pub const UINT_FAST64_MAX = UINT64_MAX;
pub const INTPTR_MAX = __helpers.promoteIntLiteral(c_long, 9223372036854775807, .decimal);
pub const INTPTR_MIN = -INTPTR_MAX - @as(c_int, 1);
pub const UINTPTR_MAX = __helpers.promoteIntLiteral(c_ulong, 18446744073709551615, .decimal);
pub const INTMAX_MAX = INTMAX_C(__helpers.promoteIntLiteral(c_int, 9223372036854775807, .decimal));
pub const UINTMAX_MAX = UINTMAX_C(__helpers.promoteIntLiteral(c_int, 18446744073709551615, .decimal));
pub const INTMAX_MIN = -INTMAX_MAX - @as(c_int, 1);
pub const PTRDIFF_MIN = INTMAX_MIN;
pub const PTRDIFF_MAX = INTMAX_MAX;
pub const SIZE_MAX = UINTPTR_MAX;
pub const RSIZE_MAX = SIZE_MAX >> @as(c_int, 1);
pub const WCHAR_MAX = __WCHAR_MAX__;
pub const WCHAR_MIN = -WCHAR_MAX - @as(c_int, 1);
pub const WINT_MIN = INT32_MIN;
pub const WINT_MAX = INT32_MAX;
pub const SIG_ATOMIC_MIN = INT32_MIN;
pub const SIG_ATOMIC_MAX = INT32_MAX;
pub const _STDLIB_H_ = "";
pub const _LIBC_COUNT__MB_LEN_MAX = _LIBC_UNSAFE_INDEXABLE;
pub const _LIBC_COUNT__PATH_MAX = _LIBC_UNSAFE_INDEXABLE;
pub const __AVAILABILITY__ = "";
pub const __API_TO_BE_DEPRECATED = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_MACOS = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_MACOSAPPLICATIONEXTENSION = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_IOS = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_IOSAPPLICATIONEXTENSION = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_MACCATALYST = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_MACCATALYSTAPPLICATIONEXTENSION = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_WATCHOS = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_WATCHOSAPPLICATIONEXTENSION = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_TVOS = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_TVOSAPPLICATIONEXTENSION = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_DRIVERKIT = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_VISIONOS = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_VISIONOSAPPLICATIONEXTENSION = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __API_TO_BE_DEPRECATED_KERNELKIT = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __AVAILABILITY_VERSIONS__ = "";
pub const __MAC_10_0 = @as(c_int, 1000);
pub const __MAC_10_1 = @as(c_int, 1010);
pub const __MAC_10_2 = @as(c_int, 1020);
pub const __MAC_10_3 = @as(c_int, 1030);
pub const __MAC_10_4 = @as(c_int, 1040);
pub const __MAC_10_5 = @as(c_int, 1050);
pub const __MAC_10_6 = @as(c_int, 1060);
pub const __MAC_10_7 = @as(c_int, 1070);
pub const __MAC_10_8 = @as(c_int, 1080);
pub const __MAC_10_9 = @as(c_int, 1090);
pub const __MAC_10_10 = __helpers.promoteIntLiteral(c_int, 101000, .decimal);
pub const __MAC_10_10_2 = __helpers.promoteIntLiteral(c_int, 101002, .decimal);
pub const __MAC_10_10_3 = __helpers.promoteIntLiteral(c_int, 101003, .decimal);
pub const __MAC_10_11 = __helpers.promoteIntLiteral(c_int, 101100, .decimal);
pub const __MAC_10_11_2 = __helpers.promoteIntLiteral(c_int, 101102, .decimal);
pub const __MAC_10_11_3 = __helpers.promoteIntLiteral(c_int, 101103, .decimal);
pub const __MAC_10_11_4 = __helpers.promoteIntLiteral(c_int, 101104, .decimal);
pub const __MAC_10_12 = __helpers.promoteIntLiteral(c_int, 101200, .decimal);
pub const __MAC_10_12_1 = __helpers.promoteIntLiteral(c_int, 101201, .decimal);
pub const __MAC_10_12_2 = __helpers.promoteIntLiteral(c_int, 101202, .decimal);
pub const __MAC_10_12_4 = __helpers.promoteIntLiteral(c_int, 101204, .decimal);
pub const __MAC_10_13 = __helpers.promoteIntLiteral(c_int, 101300, .decimal);
pub const __MAC_10_13_1 = __helpers.promoteIntLiteral(c_int, 101301, .decimal);
pub const __MAC_10_13_2 = __helpers.promoteIntLiteral(c_int, 101302, .decimal);
pub const __MAC_10_13_4 = __helpers.promoteIntLiteral(c_int, 101304, .decimal);
pub const __MAC_10_14 = __helpers.promoteIntLiteral(c_int, 101400, .decimal);
pub const __MAC_10_14_1 = __helpers.promoteIntLiteral(c_int, 101401, .decimal);
pub const __MAC_10_14_4 = __helpers.promoteIntLiteral(c_int, 101404, .decimal);
pub const __MAC_10_14_5 = __helpers.promoteIntLiteral(c_int, 101405, .decimal);
pub const __MAC_10_14_6 = __helpers.promoteIntLiteral(c_int, 101406, .decimal);
pub const __MAC_10_15 = __helpers.promoteIntLiteral(c_int, 101500, .decimal);
pub const __MAC_10_15_1 = __helpers.promoteIntLiteral(c_int, 101501, .decimal);
pub const __MAC_10_15_4 = __helpers.promoteIntLiteral(c_int, 101504, .decimal);
pub const __MAC_10_16 = __helpers.promoteIntLiteral(c_int, 101600, .decimal);
pub const __MAC_11_0 = __helpers.promoteIntLiteral(c_int, 110000, .decimal);
pub const __MAC_11_1 = __helpers.promoteIntLiteral(c_int, 110100, .decimal);
pub const __MAC_11_3 = __helpers.promoteIntLiteral(c_int, 110300, .decimal);
pub const __MAC_11_4 = __helpers.promoteIntLiteral(c_int, 110400, .decimal);
pub const __MAC_11_5 = __helpers.promoteIntLiteral(c_int, 110500, .decimal);
pub const __MAC_11_6 = __helpers.promoteIntLiteral(c_int, 110600, .decimal);
pub const __MAC_12_0 = __helpers.promoteIntLiteral(c_int, 120000, .decimal);
pub const __MAC_12_1 = __helpers.promoteIntLiteral(c_int, 120100, .decimal);
pub const __MAC_12_2 = __helpers.promoteIntLiteral(c_int, 120200, .decimal);
pub const __MAC_12_3 = __helpers.promoteIntLiteral(c_int, 120300, .decimal);
pub const __MAC_12_4 = __helpers.promoteIntLiteral(c_int, 120400, .decimal);
pub const __MAC_12_5 = __helpers.promoteIntLiteral(c_int, 120500, .decimal);
pub const __MAC_12_6 = __helpers.promoteIntLiteral(c_int, 120600, .decimal);
pub const __MAC_12_7 = __helpers.promoteIntLiteral(c_int, 120700, .decimal);
pub const __MAC_13_0 = __helpers.promoteIntLiteral(c_int, 130000, .decimal);
pub const __MAC_13_1 = __helpers.promoteIntLiteral(c_int, 130100, .decimal);
pub const __MAC_13_2 = __helpers.promoteIntLiteral(c_int, 130200, .decimal);
pub const __MAC_13_3 = __helpers.promoteIntLiteral(c_int, 130300, .decimal);
pub const __MAC_13_4 = __helpers.promoteIntLiteral(c_int, 130400, .decimal);
pub const __MAC_13_5 = __helpers.promoteIntLiteral(c_int, 130500, .decimal);
pub const __MAC_13_6 = __helpers.promoteIntLiteral(c_int, 130600, .decimal);
pub const __MAC_13_7 = __helpers.promoteIntLiteral(c_int, 130700, .decimal);
pub const __MAC_14_0 = __helpers.promoteIntLiteral(c_int, 140000, .decimal);
pub const __MAC_14_1 = __helpers.promoteIntLiteral(c_int, 140100, .decimal);
pub const __MAC_14_2 = __helpers.promoteIntLiteral(c_int, 140200, .decimal);
pub const __MAC_14_3 = __helpers.promoteIntLiteral(c_int, 140300, .decimal);
pub const __MAC_14_4 = __helpers.promoteIntLiteral(c_int, 140400, .decimal);
pub const __MAC_14_5 = __helpers.promoteIntLiteral(c_int, 140500, .decimal);
pub const __MAC_14_6 = __helpers.promoteIntLiteral(c_int, 140600, .decimal);
pub const __MAC_14_7 = __helpers.promoteIntLiteral(c_int, 140700, .decimal);
pub const __MAC_15_0 = __helpers.promoteIntLiteral(c_int, 150000, .decimal);
pub const __MAC_15_1 = __helpers.promoteIntLiteral(c_int, 150100, .decimal);
pub const __MAC_15_2 = __helpers.promoteIntLiteral(c_int, 150200, .decimal);
pub const __MAC_15_3 = __helpers.promoteIntLiteral(c_int, 150300, .decimal);
pub const __MAC_15_4 = __helpers.promoteIntLiteral(c_int, 150400, .decimal);
pub const __MAC_15_5 = __helpers.promoteIntLiteral(c_int, 150500, .decimal);
pub const __MAC_15_6 = __helpers.promoteIntLiteral(c_int, 150600, .decimal);
pub const __MAC_16_0 = __helpers.promoteIntLiteral(c_int, 160000, .decimal);
pub const __MAC_26_0 = __helpers.promoteIntLiteral(c_int, 260000, .decimal);
pub const __MAC_26_1 = __helpers.promoteIntLiteral(c_int, 260100, .decimal);
pub const __MAC_26_2 = __helpers.promoteIntLiteral(c_int, 260200, .decimal);
pub const __MAC_26_3 = __helpers.promoteIntLiteral(c_int, 260300, .decimal);
pub const __MAC_26_4 = __helpers.promoteIntLiteral(c_int, 260400, .decimal);
pub const __IPHONE_2_0 = @as(c_int, 20000);
pub const __IPHONE_2_1 = @as(c_int, 20100);
pub const __IPHONE_2_2 = @as(c_int, 20200);
pub const __IPHONE_3_0 = @as(c_int, 30000);
pub const __IPHONE_3_1 = @as(c_int, 30100);
pub const __IPHONE_3_2 = @as(c_int, 30200);
pub const __IPHONE_4_0 = __helpers.promoteIntLiteral(c_int, 40000, .decimal);
pub const __IPHONE_4_1 = __helpers.promoteIntLiteral(c_int, 40100, .decimal);
pub const __IPHONE_4_2 = __helpers.promoteIntLiteral(c_int, 40200, .decimal);
pub const __IPHONE_4_3 = __helpers.promoteIntLiteral(c_int, 40300, .decimal);
pub const __IPHONE_5_0 = __helpers.promoteIntLiteral(c_int, 50000, .decimal);
pub const __IPHONE_5_1 = __helpers.promoteIntLiteral(c_int, 50100, .decimal);
pub const __IPHONE_6_0 = __helpers.promoteIntLiteral(c_int, 60000, .decimal);
pub const __IPHONE_6_1 = __helpers.promoteIntLiteral(c_int, 60100, .decimal);
pub const __IPHONE_7_0 = __helpers.promoteIntLiteral(c_int, 70000, .decimal);
pub const __IPHONE_7_1 = __helpers.promoteIntLiteral(c_int, 70100, .decimal);
pub const __IPHONE_8_0 = __helpers.promoteIntLiteral(c_int, 80000, .decimal);
pub const __IPHONE_8_1 = __helpers.promoteIntLiteral(c_int, 80100, .decimal);
pub const __IPHONE_8_2 = __helpers.promoteIntLiteral(c_int, 80200, .decimal);
pub const __IPHONE_8_3 = __helpers.promoteIntLiteral(c_int, 80300, .decimal);
pub const __IPHONE_8_4 = __helpers.promoteIntLiteral(c_int, 80400, .decimal);
pub const __IPHONE_9_0 = __helpers.promoteIntLiteral(c_int, 90000, .decimal);
pub const __IPHONE_9_1 = __helpers.promoteIntLiteral(c_int, 90100, .decimal);
pub const __IPHONE_9_2 = __helpers.promoteIntLiteral(c_int, 90200, .decimal);
pub const __IPHONE_9_3 = __helpers.promoteIntLiteral(c_int, 90300, .decimal);
pub const __IPHONE_10_0 = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __IPHONE_10_1 = __helpers.promoteIntLiteral(c_int, 100100, .decimal);
pub const __IPHONE_10_2 = __helpers.promoteIntLiteral(c_int, 100200, .decimal);
pub const __IPHONE_10_3 = __helpers.promoteIntLiteral(c_int, 100300, .decimal);
pub const __IPHONE_11_0 = __helpers.promoteIntLiteral(c_int, 110000, .decimal);
pub const __IPHONE_11_1 = __helpers.promoteIntLiteral(c_int, 110100, .decimal);
pub const __IPHONE_11_2 = __helpers.promoteIntLiteral(c_int, 110200, .decimal);
pub const __IPHONE_11_3 = __helpers.promoteIntLiteral(c_int, 110300, .decimal);
pub const __IPHONE_11_4 = __helpers.promoteIntLiteral(c_int, 110400, .decimal);
pub const __IPHONE_12_0 = __helpers.promoteIntLiteral(c_int, 120000, .decimal);
pub const __IPHONE_12_1 = __helpers.promoteIntLiteral(c_int, 120100, .decimal);
pub const __IPHONE_12_2 = __helpers.promoteIntLiteral(c_int, 120200, .decimal);
pub const __IPHONE_12_3 = __helpers.promoteIntLiteral(c_int, 120300, .decimal);
pub const __IPHONE_12_4 = __helpers.promoteIntLiteral(c_int, 120400, .decimal);
pub const __IPHONE_13_0 = __helpers.promoteIntLiteral(c_int, 130000, .decimal);
pub const __IPHONE_13_1 = __helpers.promoteIntLiteral(c_int, 130100, .decimal);
pub const __IPHONE_13_2 = __helpers.promoteIntLiteral(c_int, 130200, .decimal);
pub const __IPHONE_13_3 = __helpers.promoteIntLiteral(c_int, 130300, .decimal);
pub const __IPHONE_13_4 = __helpers.promoteIntLiteral(c_int, 130400, .decimal);
pub const __IPHONE_13_5 = __helpers.promoteIntLiteral(c_int, 130500, .decimal);
pub const __IPHONE_13_6 = __helpers.promoteIntLiteral(c_int, 130600, .decimal);
pub const __IPHONE_13_7 = __helpers.promoteIntLiteral(c_int, 130700, .decimal);
pub const __IPHONE_14_0 = __helpers.promoteIntLiteral(c_int, 140000, .decimal);
pub const __IPHONE_14_1 = __helpers.promoteIntLiteral(c_int, 140100, .decimal);
pub const __IPHONE_14_2 = __helpers.promoteIntLiteral(c_int, 140200, .decimal);
pub const __IPHONE_14_3 = __helpers.promoteIntLiteral(c_int, 140300, .decimal);
pub const __IPHONE_14_5 = __helpers.promoteIntLiteral(c_int, 140500, .decimal);
pub const __IPHONE_14_6 = __helpers.promoteIntLiteral(c_int, 140600, .decimal);
pub const __IPHONE_14_7 = __helpers.promoteIntLiteral(c_int, 140700, .decimal);
pub const __IPHONE_14_8 = __helpers.promoteIntLiteral(c_int, 140800, .decimal);
pub const __IPHONE_15_0 = __helpers.promoteIntLiteral(c_int, 150000, .decimal);
pub const __IPHONE_15_1 = __helpers.promoteIntLiteral(c_int, 150100, .decimal);
pub const __IPHONE_15_2 = __helpers.promoteIntLiteral(c_int, 150200, .decimal);
pub const __IPHONE_15_3 = __helpers.promoteIntLiteral(c_int, 150300, .decimal);
pub const __IPHONE_15_4 = __helpers.promoteIntLiteral(c_int, 150400, .decimal);
pub const __IPHONE_15_5 = __helpers.promoteIntLiteral(c_int, 150500, .decimal);
pub const __IPHONE_15_6 = __helpers.promoteIntLiteral(c_int, 150600, .decimal);
pub const __IPHONE_15_7 = __helpers.promoteIntLiteral(c_int, 150700, .decimal);
pub const __IPHONE_15_8 = __helpers.promoteIntLiteral(c_int, 150800, .decimal);
pub const __IPHONE_16_0 = __helpers.promoteIntLiteral(c_int, 160000, .decimal);
pub const __IPHONE_16_1 = __helpers.promoteIntLiteral(c_int, 160100, .decimal);
pub const __IPHONE_16_2 = __helpers.promoteIntLiteral(c_int, 160200, .decimal);
pub const __IPHONE_16_3 = __helpers.promoteIntLiteral(c_int, 160300, .decimal);
pub const __IPHONE_16_4 = __helpers.promoteIntLiteral(c_int, 160400, .decimal);
pub const __IPHONE_16_5 = __helpers.promoteIntLiteral(c_int, 160500, .decimal);
pub const __IPHONE_16_6 = __helpers.promoteIntLiteral(c_int, 160600, .decimal);
pub const __IPHONE_16_7 = __helpers.promoteIntLiteral(c_int, 160700, .decimal);
pub const __IPHONE_17_0 = __helpers.promoteIntLiteral(c_int, 170000, .decimal);
pub const __IPHONE_17_1 = __helpers.promoteIntLiteral(c_int, 170100, .decimal);
pub const __IPHONE_17_2 = __helpers.promoteIntLiteral(c_int, 170200, .decimal);
pub const __IPHONE_17_3 = __helpers.promoteIntLiteral(c_int, 170300, .decimal);
pub const __IPHONE_17_4 = __helpers.promoteIntLiteral(c_int, 170400, .decimal);
pub const __IPHONE_17_5 = __helpers.promoteIntLiteral(c_int, 170500, .decimal);
pub const __IPHONE_17_6 = __helpers.promoteIntLiteral(c_int, 170600, .decimal);
pub const __IPHONE_17_7 = __helpers.promoteIntLiteral(c_int, 170700, .decimal);
pub const __IPHONE_18_0 = __helpers.promoteIntLiteral(c_int, 180000, .decimal);
pub const __IPHONE_18_1 = __helpers.promoteIntLiteral(c_int, 180100, .decimal);
pub const __IPHONE_18_2 = __helpers.promoteIntLiteral(c_int, 180200, .decimal);
pub const __IPHONE_18_3 = __helpers.promoteIntLiteral(c_int, 180300, .decimal);
pub const __IPHONE_18_4 = __helpers.promoteIntLiteral(c_int, 180400, .decimal);
pub const __IPHONE_18_5 = __helpers.promoteIntLiteral(c_int, 180500, .decimal);
pub const __IPHONE_18_6 = __helpers.promoteIntLiteral(c_int, 180600, .decimal);
pub const __IPHONE_19_0 = __helpers.promoteIntLiteral(c_int, 190000, .decimal);
pub const __IPHONE_26_0 = __helpers.promoteIntLiteral(c_int, 260000, .decimal);
pub const __IPHONE_26_1 = __helpers.promoteIntLiteral(c_int, 260100, .decimal);
pub const __IPHONE_26_2 = __helpers.promoteIntLiteral(c_int, 260200, .decimal);
pub const __IPHONE_26_3 = __helpers.promoteIntLiteral(c_int, 260300, .decimal);
pub const __IPHONE_26_4 = __helpers.promoteIntLiteral(c_int, 260400, .decimal);
pub const __WATCHOS_1_0 = @as(c_int, 10000);
pub const __WATCHOS_2_0 = @as(c_int, 20000);
pub const __WATCHOS_2_1 = @as(c_int, 20100);
pub const __WATCHOS_2_2 = @as(c_int, 20200);
pub const __WATCHOS_3_0 = @as(c_int, 30000);
pub const __WATCHOS_3_1 = @as(c_int, 30100);
pub const __WATCHOS_3_1_1 = @as(c_int, 30101);
pub const __WATCHOS_3_2 = @as(c_int, 30200);
pub const __WATCHOS_4_0 = __helpers.promoteIntLiteral(c_int, 40000, .decimal);
pub const __WATCHOS_4_1 = __helpers.promoteIntLiteral(c_int, 40100, .decimal);
pub const __WATCHOS_4_2 = __helpers.promoteIntLiteral(c_int, 40200, .decimal);
pub const __WATCHOS_4_3 = __helpers.promoteIntLiteral(c_int, 40300, .decimal);
pub const __WATCHOS_5_0 = __helpers.promoteIntLiteral(c_int, 50000, .decimal);
pub const __WATCHOS_5_1 = __helpers.promoteIntLiteral(c_int, 50100, .decimal);
pub const __WATCHOS_5_2 = __helpers.promoteIntLiteral(c_int, 50200, .decimal);
pub const __WATCHOS_5_3 = __helpers.promoteIntLiteral(c_int, 50300, .decimal);
pub const __WATCHOS_6_0 = __helpers.promoteIntLiteral(c_int, 60000, .decimal);
pub const __WATCHOS_6_1 = __helpers.promoteIntLiteral(c_int, 60100, .decimal);
pub const __WATCHOS_6_2 = __helpers.promoteIntLiteral(c_int, 60200, .decimal);
pub const __WATCHOS_7_0 = __helpers.promoteIntLiteral(c_int, 70000, .decimal);
pub const __WATCHOS_7_1 = __helpers.promoteIntLiteral(c_int, 70100, .decimal);
pub const __WATCHOS_7_2 = __helpers.promoteIntLiteral(c_int, 70200, .decimal);
pub const __WATCHOS_7_3 = __helpers.promoteIntLiteral(c_int, 70300, .decimal);
pub const __WATCHOS_7_4 = __helpers.promoteIntLiteral(c_int, 70400, .decimal);
pub const __WATCHOS_7_5 = __helpers.promoteIntLiteral(c_int, 70500, .decimal);
pub const __WATCHOS_7_6 = __helpers.promoteIntLiteral(c_int, 70600, .decimal);
pub const __WATCHOS_8_0 = __helpers.promoteIntLiteral(c_int, 80000, .decimal);
pub const __WATCHOS_8_1 = __helpers.promoteIntLiteral(c_int, 80100, .decimal);
pub const __WATCHOS_8_3 = __helpers.promoteIntLiteral(c_int, 80300, .decimal);
pub const __WATCHOS_8_4 = __helpers.promoteIntLiteral(c_int, 80400, .decimal);
pub const __WATCHOS_8_5 = __helpers.promoteIntLiteral(c_int, 80500, .decimal);
pub const __WATCHOS_8_6 = __helpers.promoteIntLiteral(c_int, 80600, .decimal);
pub const __WATCHOS_8_7 = __helpers.promoteIntLiteral(c_int, 80700, .decimal);
pub const __WATCHOS_8_8 = __helpers.promoteIntLiteral(c_int, 80800, .decimal);
pub const __WATCHOS_9_0 = __helpers.promoteIntLiteral(c_int, 90000, .decimal);
pub const __WATCHOS_9_1 = __helpers.promoteIntLiteral(c_int, 90100, .decimal);
pub const __WATCHOS_9_2 = __helpers.promoteIntLiteral(c_int, 90200, .decimal);
pub const __WATCHOS_9_3 = __helpers.promoteIntLiteral(c_int, 90300, .decimal);
pub const __WATCHOS_9_4 = __helpers.promoteIntLiteral(c_int, 90400, .decimal);
pub const __WATCHOS_9_5 = __helpers.promoteIntLiteral(c_int, 90500, .decimal);
pub const __WATCHOS_9_6 = __helpers.promoteIntLiteral(c_int, 90600, .decimal);
pub const __WATCHOS_10_0 = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __WATCHOS_10_1 = __helpers.promoteIntLiteral(c_int, 100100, .decimal);
pub const __WATCHOS_10_2 = __helpers.promoteIntLiteral(c_int, 100200, .decimal);
pub const __WATCHOS_10_3 = __helpers.promoteIntLiteral(c_int, 100300, .decimal);
pub const __WATCHOS_10_4 = __helpers.promoteIntLiteral(c_int, 100400, .decimal);
pub const __WATCHOS_10_5 = __helpers.promoteIntLiteral(c_int, 100500, .decimal);
pub const __WATCHOS_10_6 = __helpers.promoteIntLiteral(c_int, 100600, .decimal);
pub const __WATCHOS_10_7 = __helpers.promoteIntLiteral(c_int, 100700, .decimal);
pub const __WATCHOS_11_0 = __helpers.promoteIntLiteral(c_int, 110000, .decimal);
pub const __WATCHOS_11_1 = __helpers.promoteIntLiteral(c_int, 110100, .decimal);
pub const __WATCHOS_11_2 = __helpers.promoteIntLiteral(c_int, 110200, .decimal);
pub const __WATCHOS_11_3 = __helpers.promoteIntLiteral(c_int, 110300, .decimal);
pub const __WATCHOS_11_4 = __helpers.promoteIntLiteral(c_int, 110400, .decimal);
pub const __WATCHOS_11_5 = __helpers.promoteIntLiteral(c_int, 110500, .decimal);
pub const __WATCHOS_11_6 = __helpers.promoteIntLiteral(c_int, 110600, .decimal);
pub const __WATCHOS_12_0 = __helpers.promoteIntLiteral(c_int, 120000, .decimal);
pub const __WATCHOS_26_0 = __helpers.promoteIntLiteral(c_int, 260000, .decimal);
pub const __WATCHOS_26_1 = __helpers.promoteIntLiteral(c_int, 260100, .decimal);
pub const __WATCHOS_26_2 = __helpers.promoteIntLiteral(c_int, 260200, .decimal);
pub const __WATCHOS_26_3 = __helpers.promoteIntLiteral(c_int, 260300, .decimal);
pub const __WATCHOS_26_4 = __helpers.promoteIntLiteral(c_int, 260400, .decimal);
pub const __TVOS_9_0 = __helpers.promoteIntLiteral(c_int, 90000, .decimal);
pub const __TVOS_9_1 = __helpers.promoteIntLiteral(c_int, 90100, .decimal);
pub const __TVOS_9_2 = __helpers.promoteIntLiteral(c_int, 90200, .decimal);
pub const __TVOS_10_0 = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __TVOS_10_0_1 = __helpers.promoteIntLiteral(c_int, 100001, .decimal);
pub const __TVOS_10_1 = __helpers.promoteIntLiteral(c_int, 100100, .decimal);
pub const __TVOS_10_2 = __helpers.promoteIntLiteral(c_int, 100200, .decimal);
pub const __TVOS_11_0 = __helpers.promoteIntLiteral(c_int, 110000, .decimal);
pub const __TVOS_11_1 = __helpers.promoteIntLiteral(c_int, 110100, .decimal);
pub const __TVOS_11_2 = __helpers.promoteIntLiteral(c_int, 110200, .decimal);
pub const __TVOS_11_3 = __helpers.promoteIntLiteral(c_int, 110300, .decimal);
pub const __TVOS_11_4 = __helpers.promoteIntLiteral(c_int, 110400, .decimal);
pub const __TVOS_12_0 = __helpers.promoteIntLiteral(c_int, 120000, .decimal);
pub const __TVOS_12_1 = __helpers.promoteIntLiteral(c_int, 120100, .decimal);
pub const __TVOS_12_2 = __helpers.promoteIntLiteral(c_int, 120200, .decimal);
pub const __TVOS_12_3 = __helpers.promoteIntLiteral(c_int, 120300, .decimal);
pub const __TVOS_12_4 = __helpers.promoteIntLiteral(c_int, 120400, .decimal);
pub const __TVOS_13_0 = __helpers.promoteIntLiteral(c_int, 130000, .decimal);
pub const __TVOS_13_2 = __helpers.promoteIntLiteral(c_int, 130200, .decimal);
pub const __TVOS_13_3 = __helpers.promoteIntLiteral(c_int, 130300, .decimal);
pub const __TVOS_13_4 = __helpers.promoteIntLiteral(c_int, 130400, .decimal);
pub const __TVOS_14_0 = __helpers.promoteIntLiteral(c_int, 140000, .decimal);
pub const __TVOS_14_1 = __helpers.promoteIntLiteral(c_int, 140100, .decimal);
pub const __TVOS_14_2 = __helpers.promoteIntLiteral(c_int, 140200, .decimal);
pub const __TVOS_14_3 = __helpers.promoteIntLiteral(c_int, 140300, .decimal);
pub const __TVOS_14_5 = __helpers.promoteIntLiteral(c_int, 140500, .decimal);
pub const __TVOS_14_6 = __helpers.promoteIntLiteral(c_int, 140600, .decimal);
pub const __TVOS_14_7 = __helpers.promoteIntLiteral(c_int, 140700, .decimal);
pub const __TVOS_15_0 = __helpers.promoteIntLiteral(c_int, 150000, .decimal);
pub const __TVOS_15_1 = __helpers.promoteIntLiteral(c_int, 150100, .decimal);
pub const __TVOS_15_2 = __helpers.promoteIntLiteral(c_int, 150200, .decimal);
pub const __TVOS_15_3 = __helpers.promoteIntLiteral(c_int, 150300, .decimal);
pub const __TVOS_15_4 = __helpers.promoteIntLiteral(c_int, 150400, .decimal);
pub const __TVOS_15_5 = __helpers.promoteIntLiteral(c_int, 150500, .decimal);
pub const __TVOS_15_6 = __helpers.promoteIntLiteral(c_int, 150600, .decimal);
pub const __TVOS_16_0 = __helpers.promoteIntLiteral(c_int, 160000, .decimal);
pub const __TVOS_16_1 = __helpers.promoteIntLiteral(c_int, 160100, .decimal);
pub const __TVOS_16_2 = __helpers.promoteIntLiteral(c_int, 160200, .decimal);
pub const __TVOS_16_3 = __helpers.promoteIntLiteral(c_int, 160300, .decimal);
pub const __TVOS_16_4 = __helpers.promoteIntLiteral(c_int, 160400, .decimal);
pub const __TVOS_16_5 = __helpers.promoteIntLiteral(c_int, 160500, .decimal);
pub const __TVOS_16_6 = __helpers.promoteIntLiteral(c_int, 160600, .decimal);
pub const __TVOS_17_0 = __helpers.promoteIntLiteral(c_int, 170000, .decimal);
pub const __TVOS_17_1 = __helpers.promoteIntLiteral(c_int, 170100, .decimal);
pub const __TVOS_17_2 = __helpers.promoteIntLiteral(c_int, 170200, .decimal);
pub const __TVOS_17_3 = __helpers.promoteIntLiteral(c_int, 170300, .decimal);
pub const __TVOS_17_4 = __helpers.promoteIntLiteral(c_int, 170400, .decimal);
pub const __TVOS_17_5 = __helpers.promoteIntLiteral(c_int, 170500, .decimal);
pub const __TVOS_17_6 = __helpers.promoteIntLiteral(c_int, 170600, .decimal);
pub const __TVOS_18_0 = __helpers.promoteIntLiteral(c_int, 180000, .decimal);
pub const __TVOS_18_1 = __helpers.promoteIntLiteral(c_int, 180100, .decimal);
pub const __TVOS_18_2 = __helpers.promoteIntLiteral(c_int, 180200, .decimal);
pub const __TVOS_18_3 = __helpers.promoteIntLiteral(c_int, 180300, .decimal);
pub const __TVOS_18_4 = __helpers.promoteIntLiteral(c_int, 180400, .decimal);
pub const __TVOS_18_5 = __helpers.promoteIntLiteral(c_int, 180500, .decimal);
pub const __TVOS_18_6 = __helpers.promoteIntLiteral(c_int, 180600, .decimal);
pub const __TVOS_19_0 = __helpers.promoteIntLiteral(c_int, 190000, .decimal);
pub const __TVOS_26_0 = __helpers.promoteIntLiteral(c_int, 260000, .decimal);
pub const __TVOS_26_1 = __helpers.promoteIntLiteral(c_int, 260100, .decimal);
pub const __TVOS_26_2 = __helpers.promoteIntLiteral(c_int, 260200, .decimal);
pub const __TVOS_26_3 = __helpers.promoteIntLiteral(c_int, 260300, .decimal);
pub const __TVOS_26_4 = __helpers.promoteIntLiteral(c_int, 260400, .decimal);
pub const __BRIDGEOS_2_0 = @as(c_int, 20000);
pub const __BRIDGEOS_3_0 = @as(c_int, 30000);
pub const __BRIDGEOS_3_1 = @as(c_int, 30100);
pub const __BRIDGEOS_3_4 = @as(c_int, 30400);
pub const __BRIDGEOS_4_0 = __helpers.promoteIntLiteral(c_int, 40000, .decimal);
pub const __BRIDGEOS_4_1 = __helpers.promoteIntLiteral(c_int, 40100, .decimal);
pub const __BRIDGEOS_5_0 = __helpers.promoteIntLiteral(c_int, 50000, .decimal);
pub const __BRIDGEOS_5_1 = __helpers.promoteIntLiteral(c_int, 50100, .decimal);
pub const __BRIDGEOS_5_3 = __helpers.promoteIntLiteral(c_int, 50300, .decimal);
pub const __BRIDGEOS_6_0 = __helpers.promoteIntLiteral(c_int, 60000, .decimal);
pub const __BRIDGEOS_6_2 = __helpers.promoteIntLiteral(c_int, 60200, .decimal);
pub const __BRIDGEOS_6_4 = __helpers.promoteIntLiteral(c_int, 60400, .decimal);
pub const __BRIDGEOS_6_5 = __helpers.promoteIntLiteral(c_int, 60500, .decimal);
pub const __BRIDGEOS_6_6 = __helpers.promoteIntLiteral(c_int, 60600, .decimal);
pub const __BRIDGEOS_7_0 = __helpers.promoteIntLiteral(c_int, 70000, .decimal);
pub const __BRIDGEOS_7_1 = __helpers.promoteIntLiteral(c_int, 70100, .decimal);
pub const __BRIDGEOS_7_2 = __helpers.promoteIntLiteral(c_int, 70200, .decimal);
pub const __BRIDGEOS_7_3 = __helpers.promoteIntLiteral(c_int, 70300, .decimal);
pub const __BRIDGEOS_7_4 = __helpers.promoteIntLiteral(c_int, 70400, .decimal);
pub const __BRIDGEOS_7_6 = __helpers.promoteIntLiteral(c_int, 70600, .decimal);
pub const __BRIDGEOS_8_0 = __helpers.promoteIntLiteral(c_int, 80000, .decimal);
pub const __BRIDGEOS_8_1 = __helpers.promoteIntLiteral(c_int, 80100, .decimal);
pub const __BRIDGEOS_8_2 = __helpers.promoteIntLiteral(c_int, 80200, .decimal);
pub const __BRIDGEOS_8_3 = __helpers.promoteIntLiteral(c_int, 80300, .decimal);
pub const __BRIDGEOS_8_4 = __helpers.promoteIntLiteral(c_int, 80400, .decimal);
pub const __BRIDGEOS_8_5 = __helpers.promoteIntLiteral(c_int, 80500, .decimal);
pub const __BRIDGEOS_8_6 = __helpers.promoteIntLiteral(c_int, 80600, .decimal);
pub const __BRIDGEOS_9_0 = __helpers.promoteIntLiteral(c_int, 90000, .decimal);
pub const __BRIDGEOS_9_1 = __helpers.promoteIntLiteral(c_int, 90100, .decimal);
pub const __BRIDGEOS_9_2 = __helpers.promoteIntLiteral(c_int, 90200, .decimal);
pub const __BRIDGEOS_9_3 = __helpers.promoteIntLiteral(c_int, 90300, .decimal);
pub const __BRIDGEOS_9_4 = __helpers.promoteIntLiteral(c_int, 90400, .decimal);
pub const __BRIDGEOS_9_5 = __helpers.promoteIntLiteral(c_int, 90500, .decimal);
pub const __BRIDGEOS_9_6 = __helpers.promoteIntLiteral(c_int, 90600, .decimal);
pub const __BRIDGEOS_10_0 = __helpers.promoteIntLiteral(c_int, 100000, .decimal);
pub const __BRIDGEOS_10_1 = __helpers.promoteIntLiteral(c_int, 100100, .decimal);
pub const __BRIDGEOS_10_2 = __helpers.promoteIntLiteral(c_int, 100200, .decimal);
pub const __BRIDGEOS_10_3 = __helpers.promoteIntLiteral(c_int, 100300, .decimal);
pub const __BRIDGEOS_10_4 = __helpers.promoteIntLiteral(c_int, 100400, .decimal);
pub const __DRIVERKIT_19_0 = __helpers.promoteIntLiteral(c_int, 190000, .decimal);
pub const __DRIVERKIT_20_0 = __helpers.promoteIntLiteral(c_int, 200000, .decimal);
pub const __DRIVERKIT_21_0 = __helpers.promoteIntLiteral(c_int, 210000, .decimal);
pub const __DRIVERKIT_22_0 = __helpers.promoteIntLiteral(c_int, 220000, .decimal);
pub const __DRIVERKIT_22_4 = __helpers.promoteIntLiteral(c_int, 220400, .decimal);
pub const __DRIVERKIT_22_5 = __helpers.promoteIntLiteral(c_int, 220500, .decimal);
pub const __DRIVERKIT_22_6 = __helpers.promoteIntLiteral(c_int, 220600, .decimal);
pub const __DRIVERKIT_23_0 = __helpers.promoteIntLiteral(c_int, 230000, .decimal);
pub const __DRIVERKIT_23_1 = __helpers.promoteIntLiteral(c_int, 230100, .decimal);
pub const __DRIVERKIT_23_2 = __helpers.promoteIntLiteral(c_int, 230200, .decimal);
pub const __DRIVERKIT_23_3 = __helpers.promoteIntLiteral(c_int, 230300, .decimal);
pub const __DRIVERKIT_23_4 = __helpers.promoteIntLiteral(c_int, 230400, .decimal);
pub const __DRIVERKIT_23_5 = __helpers.promoteIntLiteral(c_int, 230500, .decimal);
pub const __DRIVERKIT_23_6 = __helpers.promoteIntLiteral(c_int, 230600, .decimal);
pub const __DRIVERKIT_24_0 = __helpers.promoteIntLiteral(c_int, 240000, .decimal);
pub const __DRIVERKIT_24_1 = __helpers.promoteIntLiteral(c_int, 240100, .decimal);
pub const __DRIVERKIT_24_2 = __helpers.promoteIntLiteral(c_int, 240200, .decimal);
pub const __DRIVERKIT_24_3 = __helpers.promoteIntLiteral(c_int, 240300, .decimal);
pub const __DRIVERKIT_24_4 = __helpers.promoteIntLiteral(c_int, 240400, .decimal);
pub const __DRIVERKIT_24_5 = __helpers.promoteIntLiteral(c_int, 240500, .decimal);
pub const __DRIVERKIT_24_6 = __helpers.promoteIntLiteral(c_int, 240600, .decimal);
pub const __DRIVERKIT_25_0 = __helpers.promoteIntLiteral(c_int, 250000, .decimal);
pub const __DRIVERKIT_25_1 = __helpers.promoteIntLiteral(c_int, 250100, .decimal);
pub const __DRIVERKIT_25_2 = __helpers.promoteIntLiteral(c_int, 250200, .decimal);
pub const __DRIVERKIT_25_3 = __helpers.promoteIntLiteral(c_int, 250300, .decimal);
pub const __DRIVERKIT_25_4 = __helpers.promoteIntLiteral(c_int, 250400, .decimal);
pub const __VISIONOS_1_0 = @as(c_int, 10000);
pub const __VISIONOS_1_1 = @as(c_int, 10100);
pub const __VISIONOS_1_2 = @as(c_int, 10200);
pub const __VISIONOS_1_3 = @as(c_int, 10300);
pub const __VISIONOS_2_0 = @as(c_int, 20000);
pub const __VISIONOS_2_1 = @as(c_int, 20100);
pub const __VISIONOS_2_2 = @as(c_int, 20200);
pub const __VISIONOS_2_3 = @as(c_int, 20300);
pub const __VISIONOS_2_4 = @as(c_int, 20400);
pub const __VISIONOS_2_5 = @as(c_int, 20500);
pub const __VISIONOS_2_6 = @as(c_int, 20600);
pub const __VISIONOS_3_0 = @as(c_int, 30000);
pub const __VISIONOS_26_0 = __helpers.promoteIntLiteral(c_int, 260000, .decimal);
pub const __VISIONOS_26_1 = __helpers.promoteIntLiteral(c_int, 260100, .decimal);
pub const __VISIONOS_26_2 = __helpers.promoteIntLiteral(c_int, 260200, .decimal);
pub const __VISIONOS_26_3 = __helpers.promoteIntLiteral(c_int, 260300, .decimal);
pub const __VISIONOS_26_4 = __helpers.promoteIntLiteral(c_int, 260400, .decimal);
pub const MAC_OS_X_VERSION_10_0 = __MAC_10_0;
pub const MAC_OS_X_VERSION_10_1 = __MAC_10_1;
pub const MAC_OS_X_VERSION_10_2 = __MAC_10_2;
pub const MAC_OS_X_VERSION_10_3 = __MAC_10_3;
pub const MAC_OS_X_VERSION_10_4 = __MAC_10_4;
pub const MAC_OS_X_VERSION_10_5 = __MAC_10_5;
pub const MAC_OS_X_VERSION_10_6 = __MAC_10_6;
pub const MAC_OS_X_VERSION_10_7 = __MAC_10_7;
pub const MAC_OS_X_VERSION_10_8 = __MAC_10_8;
pub const MAC_OS_X_VERSION_10_9 = __MAC_10_9;
pub const MAC_OS_X_VERSION_10_10 = __MAC_10_10;
pub const MAC_OS_X_VERSION_10_10_2 = __MAC_10_10_2;
pub const MAC_OS_X_VERSION_10_10_3 = __MAC_10_10_3;
pub const MAC_OS_X_VERSION_10_11 = __MAC_10_11;
pub const MAC_OS_X_VERSION_10_11_2 = __MAC_10_11_2;
pub const MAC_OS_X_VERSION_10_11_3 = __MAC_10_11_3;
pub const MAC_OS_X_VERSION_10_11_4 = __MAC_10_11_4;
pub const MAC_OS_X_VERSION_10_12 = __MAC_10_12;
pub const MAC_OS_X_VERSION_10_12_1 = __MAC_10_12_1;
pub const MAC_OS_X_VERSION_10_12_2 = __MAC_10_12_2;
pub const MAC_OS_X_VERSION_10_12_4 = __MAC_10_12_4;
pub const MAC_OS_X_VERSION_10_13 = __MAC_10_13;
pub const MAC_OS_X_VERSION_10_13_1 = __MAC_10_13_1;
pub const MAC_OS_X_VERSION_10_13_2 = __MAC_10_13_2;
pub const MAC_OS_X_VERSION_10_13_4 = __MAC_10_13_4;
pub const MAC_OS_X_VERSION_10_14 = __MAC_10_14;
pub const MAC_OS_X_VERSION_10_14_1 = __MAC_10_14_1;
pub const MAC_OS_X_VERSION_10_14_4 = __MAC_10_14_4;
pub const MAC_OS_X_VERSION_10_14_5 = __MAC_10_14_5;
pub const MAC_OS_X_VERSION_10_14_6 = __MAC_10_14_6;
pub const MAC_OS_X_VERSION_10_15 = __MAC_10_15;
pub const MAC_OS_X_VERSION_10_15_1 = __MAC_10_15_1;
pub const MAC_OS_X_VERSION_10_15_4 = __MAC_10_15_4;
pub const MAC_OS_X_VERSION_10_16 = __MAC_10_16;
pub const MAC_OS_VERSION_11_0 = __MAC_11_0;
pub const MAC_OS_VERSION_11_1 = __MAC_11_1;
pub const MAC_OS_VERSION_11_3 = __MAC_11_3;
pub const MAC_OS_VERSION_11_4 = __MAC_11_4;
pub const MAC_OS_VERSION_11_5 = __MAC_11_5;
pub const MAC_OS_VERSION_11_6 = __MAC_11_6;
pub const MAC_OS_VERSION_12_0 = __MAC_12_0;
pub const MAC_OS_VERSION_12_1 = __MAC_12_1;
pub const MAC_OS_VERSION_12_2 = __MAC_12_2;
pub const MAC_OS_VERSION_12_3 = __MAC_12_3;
pub const MAC_OS_VERSION_12_4 = __MAC_12_4;
pub const MAC_OS_VERSION_12_5 = __MAC_12_5;
pub const MAC_OS_VERSION_12_6 = __MAC_12_6;
pub const MAC_OS_VERSION_12_7 = __MAC_12_7;
pub const MAC_OS_VERSION_13_0 = __MAC_13_0;
pub const MAC_OS_VERSION_13_1 = __MAC_13_1;
pub const MAC_OS_VERSION_13_2 = __MAC_13_2;
pub const MAC_OS_VERSION_13_3 = __MAC_13_3;
pub const MAC_OS_VERSION_13_4 = __MAC_13_4;
pub const MAC_OS_VERSION_13_5 = __MAC_13_5;
pub const MAC_OS_VERSION_13_6 = __MAC_13_6;
pub const MAC_OS_VERSION_13_7 = __MAC_13_7;
pub const MAC_OS_VERSION_14_0 = __MAC_14_0;
pub const MAC_OS_VERSION_14_1 = __MAC_14_1;
pub const MAC_OS_VERSION_14_2 = __MAC_14_2;
pub const MAC_OS_VERSION_14_3 = __MAC_14_3;
pub const MAC_OS_VERSION_14_4 = __MAC_14_4;
pub const MAC_OS_VERSION_14_5 = __MAC_14_5;
pub const MAC_OS_VERSION_14_6 = __MAC_14_6;
pub const MAC_OS_VERSION_14_7 = __MAC_14_7;
pub const MAC_OS_VERSION_15_0 = __MAC_15_0;
pub const MAC_OS_VERSION_15_1 = __MAC_15_1;
pub const MAC_OS_VERSION_15_2 = __MAC_15_2;
pub const MAC_OS_VERSION_15_3 = __MAC_15_3;
pub const MAC_OS_VERSION_15_4 = __MAC_15_4;
pub const MAC_OS_VERSION_15_5 = __MAC_15_5;
pub const MAC_OS_VERSION_15_6 = __MAC_15_6;
pub const MAC_OS_VERSION_16_0 = __MAC_16_0;
pub const MAC_OS_VERSION_26_0 = __MAC_26_0;
pub const MAC_OS_VERSION_26_1 = __MAC_26_1;
pub const MAC_OS_VERSION_26_2 = __MAC_26_2;
pub const MAC_OS_VERSION_26_3 = __MAC_26_3;
pub const MAC_OS_VERSION_26_4 = __MAC_26_4;
pub const __AVAILABILITY_VERSIONS_VERSION_HASH = __helpers.promoteIntLiteral(c_uint, 93585900, .decimal);
pub const __AVAILABILITY_VERSIONS_VERSION_STRING = "Local";
pub const __AVAILABILITY_FILE = "AvailabilityVersions.h";
pub const __AVAILABILITY_INTERNAL__ = "";
pub const __MAC_OS_X_VERSION_MIN_REQUIRED = __ENVIRONMENT_OS_VERSION_MIN_REQUIRED__;
pub const __MAC_OS_X_VERSION_MAX_ALLOWED = __MAC_26_4;
pub const __IPHONE_OS_VERSION_MIN_REQUIRED = __IPHONE_17_1;
pub const __IPHONE_OS_VERSION_MAX_ALLOWED = __IPHONE_26_4;
pub const __WATCH_OS_VERSION_MIN_REQUIRED = __ENVIRONMENT_OS_VERSION_MIN_REQUIRED__;
pub const __WATCH_OS_VERSION_MAX_ALLOWED = __WATCHOS_26_4;
pub const __TV_OS_VERSION_MIN_REQUIRED = __ENVIRONMENT_OS_VERSION_MIN_REQUIRED__;
pub const __TV_OS_VERSION_MAX_ALLOWED = __TVOS_26_4;
pub const __DRIVERKIT_VERSION_MIN_REQUIRED = __ENVIRONMENT_OS_VERSION_MIN_REQUIRED__;
pub const __DRIVERKIT_VERSION_MAX_ALLOWED = __DRIVERKIT_25_4;
pub const __VISION_OS_VERSION_MIN_REQUIRED = __ENVIRONMENT_OS_VERSION_MIN_REQUIRED__;
pub const __VISION_OS_VERSION_MAX_ALLOWED = __VISIONOS_26_4;
pub const __AVAILABILITY_INTERNAL_DEPRECATED = @compileError("unable to translate macro: undefined identifier `deprecated`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:142:9
pub const __AVAILABILITY_INTERNAL_DEPRECATED_MSG = @compileError("unable to translate macro: undefined identifier `deprecated`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:145:17
pub const __AVAILABILITY_INTERNAL_UNAVAILABLE = @compileError("unable to translate macro: undefined identifier `unavailable`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:154:9
pub const __AVAILABILITY_INTERNAL_WEAK_IMPORT = @compileError("unable to translate macro: undefined identifier `weak_import`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:155:9
pub const __AVAILABILITY_INTERNAL_REGULAR = "";
pub const __API_AVAILABLE_PLATFORM_macos = @compileError("unable to translate macro: undefined identifier `macos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:160:12
pub const __API_DEPRECATED_PLATFORM_macos = @compileError("unable to translate macro: undefined identifier `macos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:161:12
pub const __API_OBSOLETED_PLATFORM_macos = @compileError("unable to translate macro: undefined identifier `macos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:162:12
pub const __API_UNAVAILABLE_PLATFORM_macos = @compileError("unable to translate macro: undefined identifier `macos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:163:12
pub const __API_AVAILABLE_PLATFORM_macosx = @compileError("unable to translate macro: undefined identifier `macos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:164:12
pub const __API_DEPRECATED_PLATFORM_macosx = @compileError("unable to translate macro: undefined identifier `macos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:165:12
pub const __API_OBSOLETED_PLATFORM_macosx = @compileError("unable to translate macro: undefined identifier `macos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:166:12
pub const __API_UNAVAILABLE_PLATFORM_macosx = @compileError("unable to translate macro: undefined identifier `macos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:167:12
pub const __API_AVAILABLE_PLATFORM_macOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `macOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:168:12
pub const __API_DEPRECATED_PLATFORM_macOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `macOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:169:12
pub const __API_OBSOLETED_PLATFORM_macOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `macOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:170:12
pub const __API_UNAVAILABLE_PLATFORM_macOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `macOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:171:12
pub const __API_AVAILABLE_PLATFORM_ios = @compileError("unable to translate macro: undefined identifier `ios`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:172:12
pub const __API_DEPRECATED_PLATFORM_ios = @compileError("unable to translate macro: undefined identifier `ios`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:173:12
pub const __API_OBSOLETED_PLATFORM_ios = @compileError("unable to translate macro: undefined identifier `ios`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:174:12
pub const __API_UNAVAILABLE_PLATFORM_ios = @compileError("unable to translate macro: undefined identifier `ios`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:175:12
pub const __API_AVAILABLE_PLATFORM_iOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `iOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:176:12
pub const __API_DEPRECATED_PLATFORM_iOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `iOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:177:12
pub const __API_OBSOLETED_PLATFORM_iOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `iOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:178:12
pub const __API_UNAVAILABLE_PLATFORM_iOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `iOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:179:12
pub const __API_AVAILABLE_PLATFORM_macCatalyst = @compileError("unable to translate macro: undefined identifier `macCatalyst`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:180:12
pub const __API_DEPRECATED_PLATFORM_macCatalyst = @compileError("unable to translate macro: undefined identifier `macCatalyst`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:181:12
pub const __API_OBSOLETED_PLATFORM_macCatalyst = @compileError("unable to translate macro: undefined identifier `macCatalyst`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:182:12
pub const __API_UNAVAILABLE_PLATFORM_macCatalyst = @compileError("unable to translate macro: undefined identifier `macCatalyst`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:183:12
pub const __API_AVAILABLE_PLATFORM_macCatalystApplicationExtension = @compileError("unable to translate macro: undefined identifier `macCatalystApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:184:12
pub const __API_DEPRECATED_PLATFORM_macCatalystApplicationExtension = @compileError("unable to translate macro: undefined identifier `macCatalystApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:185:12
pub const __API_OBSOLETED_PLATFORM_macCatalystApplicationExtension = @compileError("unable to translate macro: undefined identifier `macCatalystApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:186:12
pub const __API_UNAVAILABLE_PLATFORM_macCatalystApplicationExtension = @compileError("unable to translate macro: undefined identifier `macCatalystApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:187:12
pub const __API_AVAILABLE_PLATFORM_watchos = @compileError("unable to translate macro: undefined identifier `watchos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:188:12
pub const __API_DEPRECATED_PLATFORM_watchos = @compileError("unable to translate macro: undefined identifier `watchos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:189:12
pub const __API_OBSOLETED_PLATFORM_watchos = @compileError("unable to translate macro: undefined identifier `watchos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:190:12
pub const __API_UNAVAILABLE_PLATFORM_watchos = @compileError("unable to translate macro: undefined identifier `watchos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:191:12
pub const __API_AVAILABLE_PLATFORM_watchOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `watchOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:192:12
pub const __API_DEPRECATED_PLATFORM_watchOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `watchOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:193:12
pub const __API_OBSOLETED_PLATFORM_watchOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `watchOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:194:12
pub const __API_UNAVAILABLE_PLATFORM_watchOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `watchOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:195:12
pub const __API_AVAILABLE_PLATFORM_tvos = @compileError("unable to translate macro: undefined identifier `tvos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:196:12
pub const __API_DEPRECATED_PLATFORM_tvos = @compileError("unable to translate macro: undefined identifier `tvos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:197:12
pub const __API_OBSOLETED_PLATFORM_tvos = @compileError("unable to translate macro: undefined identifier `tvos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:198:12
pub const __API_UNAVAILABLE_PLATFORM_tvos = @compileError("unable to translate macro: undefined identifier `tvos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:199:12
pub const __API_AVAILABLE_PLATFORM_tvOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `tvOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:200:12
pub const __API_DEPRECATED_PLATFORM_tvOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `tvOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:201:12
pub const __API_OBSOLETED_PLATFORM_tvOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `tvOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:202:12
pub const __API_UNAVAILABLE_PLATFORM_tvOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `tvOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:203:12
pub const __API_AVAILABLE_PLATFORM_driverkit = @compileError("unable to translate macro: undefined identifier `driverkit`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:205:12
pub const __API_DEPRECATED_PLATFORM_driverkit = @compileError("unable to translate macro: undefined identifier `driverkit`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:206:12
pub const __API_OBSOLETED_PLATFORM_driverkit = @compileError("unable to translate macro: undefined identifier `driverkit`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:207:12
pub const __API_UNAVAILABLE_PLATFORM_driverkit = @compileError("unable to translate macro: undefined identifier `driverkit`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:208:12
pub const __API_AVAILABLE_PLATFORM_visionos = @compileError("unable to translate macro: undefined identifier `visionos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:209:12
pub const __API_DEPRECATED_PLATFORM_visionos = @compileError("unable to translate macro: undefined identifier `visionos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:210:12
pub const __API_OBSOLETED_PLATFORM_visionos = @compileError("unable to translate macro: undefined identifier `visionos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:211:12
pub const __API_UNAVAILABLE_PLATFORM_visionos = @compileError("unable to translate macro: undefined identifier `visionos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:212:12
pub const __API_AVAILABLE_PLATFORM_visionOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `visionOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:213:12
pub const __API_DEPRECATED_PLATFORM_visionOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `visionOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:214:12
pub const __API_OBSOLETED_PLATFORM_visionOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `visionOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:215:12
pub const __API_UNAVAILABLE_PLATFORM_visionOSApplicationExtension = @compileError("unable to translate macro: undefined identifier `visionOSApplicationExtension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:216:12
pub const __API_UNAVAILABLE_PLATFORM_kernelkit = @compileError("unable to translate macro: undefined identifier `kernelkit`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:218:12
pub const __API_APPLY_TO = @compileError("unable to translate macro: undefined identifier `any`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:228:11
pub inline fn __API_RANGE_STRINGIFY(x: anytype) @TypeOf(__API_RANGE_STRINGIFY2(x)) {
    _ = &x;
    return __API_RANGE_STRINGIFY2(x);
}
pub const __API_RANGE_STRINGIFY2 = @compileError("unable to translate C expr: unexpected token ''"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:230:11
pub const __API_A = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:244:13
pub inline fn __API_AVAILABLE0(arg0: anytype) @TypeOf(__API_A(arg0)) {
    _ = &arg0;
    return __API_A(arg0);
}
pub inline fn __API_AVAILABLE1(arg0: anytype, arg1: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1)) {
    _ = &arg0;
    _ = &arg1;
    return __API_A(arg0) ++ __API_A(arg1);
}
pub inline fn __API_AVAILABLE2(arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2);
}
pub inline fn __API_AVAILABLE3(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3);
}
pub inline fn __API_AVAILABLE4(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4);
}
pub inline fn __API_AVAILABLE5(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5);
}
pub inline fn __API_AVAILABLE6(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6);
}
pub inline fn __API_AVAILABLE7(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7);
}
pub inline fn __API_AVAILABLE8(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8);
}
pub inline fn __API_AVAILABLE9(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9);
}
pub inline fn __API_AVAILABLE10(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10);
}
pub inline fn __API_AVAILABLE11(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11);
}
pub inline fn __API_AVAILABLE12(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11) ++ __API_A(arg12)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11) ++ __API_A(arg12);
}
pub inline fn __API_AVAILABLE13(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11) ++ __API_A(arg12) ++ __API_A(arg13)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11) ++ __API_A(arg12) ++ __API_A(arg13);
}
pub inline fn __API_AVAILABLE14(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11) ++ __API_A(arg12) ++ __API_A(arg13) ++ __API_A(arg14)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11) ++ __API_A(arg12) ++ __API_A(arg13) ++ __API_A(arg14);
}
pub inline fn __API_AVAILABLE15(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11) ++ __API_A(arg12) ++ __API_A(arg13) ++ __API_A(arg14) ++ __API_A(arg15)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_A(arg0) ++ __API_A(arg1) ++ __API_A(arg2) ++ __API_A(arg3) ++ __API_A(arg4) ++ __API_A(arg5) ++ __API_A(arg6) ++ __API_A(arg7) ++ __API_A(arg8) ++ __API_A(arg9) ++ __API_A(arg10) ++ __API_A(arg11) ++ __API_A(arg12) ++ __API_A(arg13) ++ __API_A(arg14) ++ __API_A(arg15);
}
pub inline fn __API_AVAILABLE_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &NAME;
    return NAME;
}
pub const __API_A_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:264:13
pub inline fn __API_AVAILABLE_BEGIN0(arg0: anytype) @TypeOf(__API_A_BEGIN(arg0)) {
    _ = &arg0;
    return __API_A_BEGIN(arg0);
}
pub inline fn __API_AVAILABLE_BEGIN1(arg0: anytype, arg1: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1)) {
    _ = &arg0;
    _ = &arg1;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1);
}
pub inline fn __API_AVAILABLE_BEGIN2(arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2);
}
pub inline fn __API_AVAILABLE_BEGIN3(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3);
}
pub inline fn __API_AVAILABLE_BEGIN4(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4);
}
pub inline fn __API_AVAILABLE_BEGIN5(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5);
}
pub inline fn __API_AVAILABLE_BEGIN6(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6);
}
pub inline fn __API_AVAILABLE_BEGIN7(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7);
}
pub inline fn __API_AVAILABLE_BEGIN8(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8);
}
pub inline fn __API_AVAILABLE_BEGIN9(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9);
}
pub inline fn __API_AVAILABLE_BEGIN10(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10);
}
pub inline fn __API_AVAILABLE_BEGIN11(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11);
}
pub inline fn __API_AVAILABLE_BEGIN12(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11) ++ __API_A_BEGIN(arg12)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11) ++ __API_A_BEGIN(arg12);
}
pub inline fn __API_AVAILABLE_BEGIN13(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11) ++ __API_A_BEGIN(arg12) ++ __API_A_BEGIN(arg13)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11) ++ __API_A_BEGIN(arg12) ++ __API_A_BEGIN(arg13);
}
pub inline fn __API_AVAILABLE_BEGIN14(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11) ++ __API_A_BEGIN(arg12) ++ __API_A_BEGIN(arg13) ++ __API_A_BEGIN(arg14)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11) ++ __API_A_BEGIN(arg12) ++ __API_A_BEGIN(arg13) ++ __API_A_BEGIN(arg14);
}
pub inline fn __API_AVAILABLE_BEGIN15(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11) ++ __API_A_BEGIN(arg12) ++ __API_A_BEGIN(arg13) ++ __API_A_BEGIN(arg14) ++ __API_A_BEGIN(arg15)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_A_BEGIN(arg0) ++ __API_A_BEGIN(arg1) ++ __API_A_BEGIN(arg2) ++ __API_A_BEGIN(arg3) ++ __API_A_BEGIN(arg4) ++ __API_A_BEGIN(arg5) ++ __API_A_BEGIN(arg6) ++ __API_A_BEGIN(arg7) ++ __API_A_BEGIN(arg8) ++ __API_A_BEGIN(arg9) ++ __API_A_BEGIN(arg10) ++ __API_A_BEGIN(arg11) ++ __API_A_BEGIN(arg12) ++ __API_A_BEGIN(arg13) ++ __API_A_BEGIN(arg14) ++ __API_A_BEGIN(arg15);
}
pub inline fn __API_AVAILABLE_BEGIN_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &NAME;
    return NAME;
}
pub const __API_D = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:286:13
pub inline fn __API_DEPRECATED_MSG0(msg: anytype, arg0: anytype) @TypeOf(__API_D(msg, arg0)) {
    _ = &msg;
    _ = &arg0;
    return __API_D(msg, arg0);
}
pub inline fn __API_DEPRECATED_MSG1(msg: anytype, arg0: anytype, arg1: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1);
}
pub inline fn __API_DEPRECATED_MSG2(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2);
}
pub inline fn __API_DEPRECATED_MSG3(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3);
}
pub inline fn __API_DEPRECATED_MSG4(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4);
}
pub inline fn __API_DEPRECATED_MSG5(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5);
}
pub inline fn __API_DEPRECATED_MSG6(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6);
}
pub inline fn __API_DEPRECATED_MSG7(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7);
}
pub inline fn __API_DEPRECATED_MSG8(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8);
}
pub inline fn __API_DEPRECATED_MSG9(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9);
}
pub inline fn __API_DEPRECATED_MSG10(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10);
}
pub inline fn __API_DEPRECATED_MSG11(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11);
}
pub inline fn __API_DEPRECATED_MSG12(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11) ++ __API_D(msg, arg12)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11) ++ __API_D(msg, arg12);
}
pub inline fn __API_DEPRECATED_MSG13(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11) ++ __API_D(msg, arg12) ++ __API_D(msg, arg13)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11) ++ __API_D(msg, arg12) ++ __API_D(msg, arg13);
}
pub inline fn __API_DEPRECATED_MSG14(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11) ++ __API_D(msg, arg12) ++ __API_D(msg, arg13) ++ __API_D(msg, arg14)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11) ++ __API_D(msg, arg12) ++ __API_D(msg, arg13) ++ __API_D(msg, arg14);
}
pub inline fn __API_DEPRECATED_MSG15(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11) ++ __API_D(msg, arg12) ++ __API_D(msg, arg13) ++ __API_D(msg, arg14) ++ __API_D(msg, arg15)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_D(msg, arg0) ++ __API_D(msg, arg1) ++ __API_D(msg, arg2) ++ __API_D(msg, arg3) ++ __API_D(msg, arg4) ++ __API_D(msg, arg5) ++ __API_D(msg, arg6) ++ __API_D(msg, arg7) ++ __API_D(msg, arg8) ++ __API_D(msg, arg9) ++ __API_D(msg, arg10) ++ __API_D(msg, arg11) ++ __API_D(msg, arg12) ++ __API_D(msg, arg13) ++ __API_D(msg, arg14) ++ __API_D(msg, arg15);
}
pub inline fn __API_DEPRECATED_MSG_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, _16: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &_16;
    _ = &NAME;
    return NAME;
}
pub const __API_D_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:306:13
pub inline fn __API_DEPRECATED_BEGIN0(msg: anytype, arg0: anytype) @TypeOf(__API_D_BEGIN(msg, arg0)) {
    _ = &msg;
    _ = &arg0;
    return __API_D_BEGIN(msg, arg0);
}
pub inline fn __API_DEPRECATED_BEGIN1(msg: anytype, arg0: anytype, arg1: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1);
}
pub inline fn __API_DEPRECATED_BEGIN2(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2);
}
pub inline fn __API_DEPRECATED_BEGIN3(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3);
}
pub inline fn __API_DEPRECATED_BEGIN4(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4);
}
pub inline fn __API_DEPRECATED_BEGIN5(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5);
}
pub inline fn __API_DEPRECATED_BEGIN6(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6);
}
pub inline fn __API_DEPRECATED_BEGIN7(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7);
}
pub inline fn __API_DEPRECATED_BEGIN8(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8);
}
pub inline fn __API_DEPRECATED_BEGIN9(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9);
}
pub inline fn __API_DEPRECATED_BEGIN10(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10);
}
pub inline fn __API_DEPRECATED_BEGIN11(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11);
}
pub inline fn __API_DEPRECATED_BEGIN12(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11) ++ __API_D_BEGIN(msg, arg12)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11) ++ __API_D_BEGIN(msg, arg12);
}
pub inline fn __API_DEPRECATED_BEGIN13(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11) ++ __API_D_BEGIN(msg, arg12) ++ __API_D_BEGIN(msg, arg13)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11) ++ __API_D_BEGIN(msg, arg12) ++ __API_D_BEGIN(msg, arg13);
}
pub inline fn __API_DEPRECATED_BEGIN14(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11) ++ __API_D_BEGIN(msg, arg12) ++ __API_D_BEGIN(msg, arg13) ++ __API_D_BEGIN(msg, arg14)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11) ++ __API_D_BEGIN(msg, arg12) ++ __API_D_BEGIN(msg, arg13) ++ __API_D_BEGIN(msg, arg14);
}
pub inline fn __API_DEPRECATED_BEGIN15(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11) ++ __API_D_BEGIN(msg, arg12) ++ __API_D_BEGIN(msg, arg13) ++ __API_D_BEGIN(msg, arg14) ++ __API_D_BEGIN(msg, arg15)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_D_BEGIN(msg, arg0) ++ __API_D_BEGIN(msg, arg1) ++ __API_D_BEGIN(msg, arg2) ++ __API_D_BEGIN(msg, arg3) ++ __API_D_BEGIN(msg, arg4) ++ __API_D_BEGIN(msg, arg5) ++ __API_D_BEGIN(msg, arg6) ++ __API_D_BEGIN(msg, arg7) ++ __API_D_BEGIN(msg, arg8) ++ __API_D_BEGIN(msg, arg9) ++ __API_D_BEGIN(msg, arg10) ++ __API_D_BEGIN(msg, arg11) ++ __API_D_BEGIN(msg, arg12) ++ __API_D_BEGIN(msg, arg13) ++ __API_D_BEGIN(msg, arg14) ++ __API_D_BEGIN(msg, arg15);
}
pub inline fn __API_DEPRECATED_BEGIN_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, _16: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &_16;
    _ = &NAME;
    return NAME;
}
pub const __API_DR = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:327:17
pub inline fn __API_DEPRECATED_REP0(msg: anytype, arg0: anytype) @TypeOf(__API_DR(msg, arg0)) {
    _ = &msg;
    _ = &arg0;
    return __API_DR(msg, arg0);
}
pub inline fn __API_DEPRECATED_REP1(msg: anytype, arg0: anytype, arg1: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1);
}
pub inline fn __API_DEPRECATED_REP2(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2);
}
pub inline fn __API_DEPRECATED_REP3(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3);
}
pub inline fn __API_DEPRECATED_REP4(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4);
}
pub inline fn __API_DEPRECATED_REP5(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5);
}
pub inline fn __API_DEPRECATED_REP6(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6);
}
pub inline fn __API_DEPRECATED_REP7(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7);
}
pub inline fn __API_DEPRECATED_REP8(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8);
}
pub inline fn __API_DEPRECATED_REP9(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9);
}
pub inline fn __API_DEPRECATED_REP10(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10);
}
pub inline fn __API_DEPRECATED_REP11(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11);
}
pub inline fn __API_DEPRECATED_REP12(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11) ++ __API_DR(msg, arg12)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11) ++ __API_DR(msg, arg12);
}
pub inline fn __API_DEPRECATED_REP13(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11) ++ __API_DR(msg, arg12) ++ __API_DR(msg, arg13)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11) ++ __API_DR(msg, arg12) ++ __API_DR(msg, arg13);
}
pub inline fn __API_DEPRECATED_REP14(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11) ++ __API_DR(msg, arg12) ++ __API_DR(msg, arg13) ++ __API_DR(msg, arg14)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11) ++ __API_DR(msg, arg12) ++ __API_DR(msg, arg13) ++ __API_DR(msg, arg14);
}
pub inline fn __API_DEPRECATED_REP15(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11) ++ __API_DR(msg, arg12) ++ __API_DR(msg, arg13) ++ __API_DR(msg, arg14) ++ __API_DR(msg, arg15)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_DR(msg, arg0) ++ __API_DR(msg, arg1) ++ __API_DR(msg, arg2) ++ __API_DR(msg, arg3) ++ __API_DR(msg, arg4) ++ __API_DR(msg, arg5) ++ __API_DR(msg, arg6) ++ __API_DR(msg, arg7) ++ __API_DR(msg, arg8) ++ __API_DR(msg, arg9) ++ __API_DR(msg, arg10) ++ __API_DR(msg, arg11) ++ __API_DR(msg, arg12) ++ __API_DR(msg, arg13) ++ __API_DR(msg, arg14) ++ __API_DR(msg, arg15);
}
pub inline fn __API_DEPRECATED_REP_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, _16: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &_16;
    _ = &NAME;
    return NAME;
}
pub const __API_DR_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:351:17
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN0(msg: anytype, arg0: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0)) {
    _ = &msg;
    _ = &arg0;
    return __API_DR_BEGIN(msg, arg0);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN1(msg: anytype, arg0: anytype, arg1: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN2(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN3(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN4(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN5(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN6(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN7(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN8(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN9(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN10(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN11(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN12(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11) ++ __API_DR_BEGIN(msg, arg12)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11) ++ __API_DR_BEGIN(msg, arg12);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN13(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11) ++ __API_DR_BEGIN(msg, arg12) ++ __API_DR_BEGIN(msg, arg13)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11) ++ __API_DR_BEGIN(msg, arg12) ++ __API_DR_BEGIN(msg, arg13);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN14(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11) ++ __API_DR_BEGIN(msg, arg12) ++ __API_DR_BEGIN(msg, arg13) ++ __API_DR_BEGIN(msg, arg14)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11) ++ __API_DR_BEGIN(msg, arg12) ++ __API_DR_BEGIN(msg, arg13) ++ __API_DR_BEGIN(msg, arg14);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN15(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11) ++ __API_DR_BEGIN(msg, arg12) ++ __API_DR_BEGIN(msg, arg13) ++ __API_DR_BEGIN(msg, arg14) ++ __API_DR_BEGIN(msg, arg15)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_DR_BEGIN(msg, arg0) ++ __API_DR_BEGIN(msg, arg1) ++ __API_DR_BEGIN(msg, arg2) ++ __API_DR_BEGIN(msg, arg3) ++ __API_DR_BEGIN(msg, arg4) ++ __API_DR_BEGIN(msg, arg5) ++ __API_DR_BEGIN(msg, arg6) ++ __API_DR_BEGIN(msg, arg7) ++ __API_DR_BEGIN(msg, arg8) ++ __API_DR_BEGIN(msg, arg9) ++ __API_DR_BEGIN(msg, arg10) ++ __API_DR_BEGIN(msg, arg11) ++ __API_DR_BEGIN(msg, arg12) ++ __API_DR_BEGIN(msg, arg13) ++ __API_DR_BEGIN(msg, arg14) ++ __API_DR_BEGIN(msg, arg15);
}
pub inline fn __API_DEPRECATED_WITH_REPLACEMENT_BEGIN_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, _16: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &_16;
    _ = &NAME;
    return NAME;
}
pub const __API_O = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:376:9
pub inline fn __API_OBSOLETED_MSG0(msg: anytype, arg0: anytype) @TypeOf(__API_O(msg, arg0)) {
    _ = &msg;
    _ = &arg0;
    return __API_O(msg, arg0);
}
pub inline fn __API_OBSOLETED_MSG1(msg: anytype, arg0: anytype, arg1: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1);
}
pub inline fn __API_OBSOLETED_MSG2(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2);
}
pub inline fn __API_OBSOLETED_MSG3(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3);
}
pub inline fn __API_OBSOLETED_MSG4(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4);
}
pub inline fn __API_OBSOLETED_MSG5(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5);
}
pub inline fn __API_OBSOLETED_MSG6(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6);
}
pub inline fn __API_OBSOLETED_MSG7(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7);
}
pub inline fn __API_OBSOLETED_MSG8(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8);
}
pub inline fn __API_OBSOLETED_MSG9(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9);
}
pub inline fn __API_OBSOLETED_MSG10(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10);
}
pub inline fn __API_OBSOLETED_MSG11(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11);
}
pub inline fn __API_OBSOLETED_MSG12(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11) ++ __API_O(msg, arg12)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11) ++ __API_O(msg, arg12);
}
pub inline fn __API_OBSOLETED_MSG13(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11) ++ __API_O(msg, arg12) ++ __API_O(msg, arg13)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11) ++ __API_O(msg, arg12) ++ __API_O(msg, arg13);
}
pub inline fn __API_OBSOLETED_MSG14(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11) ++ __API_O(msg, arg12) ++ __API_O(msg, arg13) ++ __API_O(msg, arg14)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11) ++ __API_O(msg, arg12) ++ __API_O(msg, arg13) ++ __API_O(msg, arg14);
}
pub inline fn __API_OBSOLETED_MSG15(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11) ++ __API_O(msg, arg12) ++ __API_O(msg, arg13) ++ __API_O(msg, arg14) ++ __API_O(msg, arg15)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_O(msg, arg0) ++ __API_O(msg, arg1) ++ __API_O(msg, arg2) ++ __API_O(msg, arg3) ++ __API_O(msg, arg4) ++ __API_O(msg, arg5) ++ __API_O(msg, arg6) ++ __API_O(msg, arg7) ++ __API_O(msg, arg8) ++ __API_O(msg, arg9) ++ __API_O(msg, arg10) ++ __API_O(msg, arg11) ++ __API_O(msg, arg12) ++ __API_O(msg, arg13) ++ __API_O(msg, arg14) ++ __API_O(msg, arg15);
}
pub inline fn __API_OBSOLETED_MSG_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, _16: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &_16;
    _ = &NAME;
    return NAME;
}
pub const __API_O_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:396:9
pub inline fn __API_OBSOLETED_BEGIN0(msg: anytype, arg0: anytype) @TypeOf(__API_O_BEGIN(msg, arg0)) {
    _ = &msg;
    _ = &arg0;
    return __API_O_BEGIN(msg, arg0);
}
pub inline fn __API_OBSOLETED_BEGIN1(msg: anytype, arg0: anytype, arg1: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1);
}
pub inline fn __API_OBSOLETED_BEGIN2(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2);
}
pub inline fn __API_OBSOLETED_BEGIN3(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3);
}
pub inline fn __API_OBSOLETED_BEGIN4(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4);
}
pub inline fn __API_OBSOLETED_BEGIN5(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5);
}
pub inline fn __API_OBSOLETED_BEGIN6(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6);
}
pub inline fn __API_OBSOLETED_BEGIN7(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7);
}
pub inline fn __API_OBSOLETED_BEGIN8(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8);
}
pub inline fn __API_OBSOLETED_BEGIN9(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9);
}
pub inline fn __API_OBSOLETED_BEGIN10(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10);
}
pub inline fn __API_OBSOLETED_BEGIN11(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11);
}
pub inline fn __API_OBSOLETED_BEGIN12(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11) ++ __API_O_BEGIN(msg, arg12)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11) ++ __API_O_BEGIN(msg, arg12);
}
pub inline fn __API_OBSOLETED_BEGIN13(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11) ++ __API_O_BEGIN(msg, arg12) ++ __API_O_BEGIN(msg, arg13)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11) ++ __API_O_BEGIN(msg, arg12) ++ __API_O_BEGIN(msg, arg13);
}
pub inline fn __API_OBSOLETED_BEGIN14(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11) ++ __API_O_BEGIN(msg, arg12) ++ __API_O_BEGIN(msg, arg13) ++ __API_O_BEGIN(msg, arg14)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11) ++ __API_O_BEGIN(msg, arg12) ++ __API_O_BEGIN(msg, arg13) ++ __API_O_BEGIN(msg, arg14);
}
pub inline fn __API_OBSOLETED_BEGIN15(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11) ++ __API_O_BEGIN(msg, arg12) ++ __API_O_BEGIN(msg, arg13) ++ __API_O_BEGIN(msg, arg14) ++ __API_O_BEGIN(msg, arg15)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_O_BEGIN(msg, arg0) ++ __API_O_BEGIN(msg, arg1) ++ __API_O_BEGIN(msg, arg2) ++ __API_O_BEGIN(msg, arg3) ++ __API_O_BEGIN(msg, arg4) ++ __API_O_BEGIN(msg, arg5) ++ __API_O_BEGIN(msg, arg6) ++ __API_O_BEGIN(msg, arg7) ++ __API_O_BEGIN(msg, arg8) ++ __API_O_BEGIN(msg, arg9) ++ __API_O_BEGIN(msg, arg10) ++ __API_O_BEGIN(msg, arg11) ++ __API_O_BEGIN(msg, arg12) ++ __API_O_BEGIN(msg, arg13) ++ __API_O_BEGIN(msg, arg14) ++ __API_O_BEGIN(msg, arg15);
}
pub inline fn __API_OBSOLETED_BEGIN_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, _16: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &_16;
    _ = &NAME;
    return NAME;
}
pub const __API_OR = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:417:13
pub inline fn __API_OBSOLETED_REP0(msg: anytype, arg0: anytype) @TypeOf(__API_OR(msg, arg0)) {
    _ = &msg;
    _ = &arg0;
    return __API_OR(msg, arg0);
}
pub inline fn __API_OBSOLETED_REP1(msg: anytype, arg0: anytype, arg1: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1);
}
pub inline fn __API_OBSOLETED_REP2(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2);
}
pub inline fn __API_OBSOLETED_REP3(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3);
}
pub inline fn __API_OBSOLETED_REP4(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4);
}
pub inline fn __API_OBSOLETED_REP5(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5);
}
pub inline fn __API_OBSOLETED_REP6(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6);
}
pub inline fn __API_OBSOLETED_REP7(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7);
}
pub inline fn __API_OBSOLETED_REP8(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8);
}
pub inline fn __API_OBSOLETED_REP9(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9);
}
pub inline fn __API_OBSOLETED_REP10(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10);
}
pub inline fn __API_OBSOLETED_REP11(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11);
}
pub inline fn __API_OBSOLETED_REP12(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11) ++ __API_OR(msg, arg12)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11) ++ __API_OR(msg, arg12);
}
pub inline fn __API_OBSOLETED_REP13(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11) ++ __API_OR(msg, arg12) ++ __API_OR(msg, arg13)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11) ++ __API_OR(msg, arg12) ++ __API_OR(msg, arg13);
}
pub inline fn __API_OBSOLETED_REP14(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11) ++ __API_OR(msg, arg12) ++ __API_OR(msg, arg13) ++ __API_OR(msg, arg14)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11) ++ __API_OR(msg, arg12) ++ __API_OR(msg, arg13) ++ __API_OR(msg, arg14);
}
pub inline fn __API_OBSOLETED_REP15(msg: anytype, arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11) ++ __API_OR(msg, arg12) ++ __API_OR(msg, arg13) ++ __API_OR(msg, arg14) ++ __API_OR(msg, arg15)) {
    _ = &msg;
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_OR(msg, arg0) ++ __API_OR(msg, arg1) ++ __API_OR(msg, arg2) ++ __API_OR(msg, arg3) ++ __API_OR(msg, arg4) ++ __API_OR(msg, arg5) ++ __API_OR(msg, arg6) ++ __API_OR(msg, arg7) ++ __API_OR(msg, arg8) ++ __API_OR(msg, arg9) ++ __API_OR(msg, arg10) ++ __API_OR(msg, arg11) ++ __API_OR(msg, arg12) ++ __API_OR(msg, arg13) ++ __API_OR(msg, arg14) ++ __API_OR(msg, arg15);
}
pub inline fn __API_OBSOLETED_REP_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, _16: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &_16;
    _ = &NAME;
    return NAME;
}
pub const __API_OR_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:441:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN0 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:446:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN1 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:447:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN2 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:448:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN3 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:449:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN4 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:450:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN5 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:451:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN6 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:452:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN7 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:453:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN8 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:454:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN9 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:455:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN10 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:456:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN11 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:457:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN12 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:458:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN13 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:459:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN14 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:460:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN15 = @compileError("unable to translate macro: undefined identifier `__API_R_BEGIN`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:461:13
pub inline fn __API_OBSOLETED_WITH_REPLACEMENT_BEGIN_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, _16: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &_16;
    _ = &NAME;
    return NAME;
}
pub const __API_U = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:473:13
pub inline fn __API_UNAVAILABLE0(arg0: anytype) @TypeOf(__API_U(arg0)) {
    _ = &arg0;
    return __API_U(arg0);
}
pub inline fn __API_UNAVAILABLE1(arg0: anytype, arg1: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1)) {
    _ = &arg0;
    _ = &arg1;
    return __API_U(arg0) ++ __API_U(arg1);
}
pub inline fn __API_UNAVAILABLE2(arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2);
}
pub inline fn __API_UNAVAILABLE3(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3);
}
pub inline fn __API_UNAVAILABLE4(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4);
}
pub inline fn __API_UNAVAILABLE5(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5);
}
pub inline fn __API_UNAVAILABLE6(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6);
}
pub inline fn __API_UNAVAILABLE7(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7);
}
pub inline fn __API_UNAVAILABLE8(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8);
}
pub inline fn __API_UNAVAILABLE9(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9);
}
pub inline fn __API_UNAVAILABLE10(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10);
}
pub inline fn __API_UNAVAILABLE11(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11);
}
pub inline fn __API_UNAVAILABLE12(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11) ++ __API_U(arg12)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11) ++ __API_U(arg12);
}
pub inline fn __API_UNAVAILABLE13(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11) ++ __API_U(arg12) ++ __API_U(arg13)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11) ++ __API_U(arg12) ++ __API_U(arg13);
}
pub inline fn __API_UNAVAILABLE14(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11) ++ __API_U(arg12) ++ __API_U(arg13) ++ __API_U(arg14)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11) ++ __API_U(arg12) ++ __API_U(arg13) ++ __API_U(arg14);
}
pub inline fn __API_UNAVAILABLE15(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11) ++ __API_U(arg12) ++ __API_U(arg13) ++ __API_U(arg14) ++ __API_U(arg15)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_U(arg0) ++ __API_U(arg1) ++ __API_U(arg2) ++ __API_U(arg3) ++ __API_U(arg4) ++ __API_U(arg5) ++ __API_U(arg6) ++ __API_U(arg7) ++ __API_U(arg8) ++ __API_U(arg9) ++ __API_U(arg10) ++ __API_U(arg11) ++ __API_U(arg12) ++ __API_U(arg13) ++ __API_U(arg14) ++ __API_U(arg15);
}
pub inline fn __API_UNAVAILABLE_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &NAME;
    return NAME;
}
pub const __API_U_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternal.h:493:13
pub inline fn __API_UNAVAILABLE_BEGIN0(arg0: anytype) @TypeOf(__API_U_BEGIN(arg0)) {
    _ = &arg0;
    return __API_U_BEGIN(arg0);
}
pub inline fn __API_UNAVAILABLE_BEGIN1(arg0: anytype, arg1: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1)) {
    _ = &arg0;
    _ = &arg1;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1);
}
pub inline fn __API_UNAVAILABLE_BEGIN2(arg0: anytype, arg1: anytype, arg2: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2);
}
pub inline fn __API_UNAVAILABLE_BEGIN3(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3);
}
pub inline fn __API_UNAVAILABLE_BEGIN4(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4);
}
pub inline fn __API_UNAVAILABLE_BEGIN5(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5);
}
pub inline fn __API_UNAVAILABLE_BEGIN6(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6);
}
pub inline fn __API_UNAVAILABLE_BEGIN7(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7);
}
pub inline fn __API_UNAVAILABLE_BEGIN8(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8);
}
pub inline fn __API_UNAVAILABLE_BEGIN9(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9);
}
pub inline fn __API_UNAVAILABLE_BEGIN10(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10);
}
pub inline fn __API_UNAVAILABLE_BEGIN11(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11);
}
pub inline fn __API_UNAVAILABLE_BEGIN12(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11) ++ __API_U_BEGIN(arg12)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11) ++ __API_U_BEGIN(arg12);
}
pub inline fn __API_UNAVAILABLE_BEGIN13(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11) ++ __API_U_BEGIN(arg12) ++ __API_U_BEGIN(arg13)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11) ++ __API_U_BEGIN(arg12) ++ __API_U_BEGIN(arg13);
}
pub inline fn __API_UNAVAILABLE_BEGIN14(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11) ++ __API_U_BEGIN(arg12) ++ __API_U_BEGIN(arg13) ++ __API_U_BEGIN(arg14)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11) ++ __API_U_BEGIN(arg12) ++ __API_U_BEGIN(arg13) ++ __API_U_BEGIN(arg14);
}
pub inline fn __API_UNAVAILABLE_BEGIN15(arg0: anytype, arg1: anytype, arg2: anytype, arg3: anytype, arg4: anytype, arg5: anytype, arg6: anytype, arg7: anytype, arg8: anytype, arg9: anytype, arg10: anytype, arg11: anytype, arg12: anytype, arg13: anytype, arg14: anytype, arg15: anytype) @TypeOf(__API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11) ++ __API_U_BEGIN(arg12) ++ __API_U_BEGIN(arg13) ++ __API_U_BEGIN(arg14) ++ __API_U_BEGIN(arg15)) {
    _ = &arg0;
    _ = &arg1;
    _ = &arg2;
    _ = &arg3;
    _ = &arg4;
    _ = &arg5;
    _ = &arg6;
    _ = &arg7;
    _ = &arg8;
    _ = &arg9;
    _ = &arg10;
    _ = &arg11;
    _ = &arg12;
    _ = &arg13;
    _ = &arg14;
    _ = &arg15;
    return __API_U_BEGIN(arg0) ++ __API_U_BEGIN(arg1) ++ __API_U_BEGIN(arg2) ++ __API_U_BEGIN(arg3) ++ __API_U_BEGIN(arg4) ++ __API_U_BEGIN(arg5) ++ __API_U_BEGIN(arg6) ++ __API_U_BEGIN(arg7) ++ __API_U_BEGIN(arg8) ++ __API_U_BEGIN(arg9) ++ __API_U_BEGIN(arg10) ++ __API_U_BEGIN(arg11) ++ __API_U_BEGIN(arg12) ++ __API_U_BEGIN(arg13) ++ __API_U_BEGIN(arg14) ++ __API_U_BEGIN(arg15);
}
pub inline fn __API_UNAVAILABLE_BEGIN_GET_MACRO_93585900(_0: anytype, _1: anytype, _2: anytype, _3: anytype, _4: anytype, _5: anytype, _6: anytype, _7: anytype, _8: anytype, _9: anytype, _10: anytype, _11: anytype, _12: anytype, _13: anytype, _14: anytype, _15: anytype, NAME: anytype) @TypeOf(NAME) {
    _ = &_0;
    _ = &_1;
    _ = &_2;
    _ = &_3;
    _ = &_4;
    _ = &_5;
    _ = &_6;
    _ = &_7;
    _ = &_8;
    _ = &_9;
    _ = &_10;
    _ = &_11;
    _ = &_12;
    _ = &_13;
    _ = &_14;
    _ = &_15;
    _ = &NAME;
    return NAME;
}
pub inline fn __swift_compiler_version_at_least() @TypeOf(@as(c_int, 1)) {
    return @as(c_int, 1);
}
pub const __AVAILABILITY_INTERNAL_LEGACY__ = "";
pub const __ENABLE_LEGACY_IPHONE_AVAILABILITY = @as(c_int, 1);
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:67:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:68:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:70:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:74:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:76:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:80:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:82:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:86:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:88:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_11_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:92:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_2_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:93:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_2_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:95:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_2_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:99:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_2_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:101:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_2_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:105:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_2_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:107:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_3_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:111:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_3_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:113:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_3_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:117:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_3_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:119:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_3_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:123:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_3_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:125:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_4_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:129:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_4_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:131:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:135:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_4_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:137:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:141:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:143:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:147:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:149:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:153:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:155:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:159:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:161:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:165:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:167:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:171:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:173:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:177:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:179:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:183:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:185:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:189:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:191:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:195:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:197:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:201:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:203:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:207:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:209:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:213:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:215:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:219:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:221:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:225:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:227:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:231:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:233:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:237:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:239:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:243:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:244:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:245:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:246:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:248:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:252:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:254:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:258:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:260:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:264:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:266:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_2_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:270:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_2_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:272:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_2_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:276:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_2_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:278:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_3_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:282:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_3_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:284:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_3_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:288:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_3_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:290:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_3_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:294:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_3_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:296:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_4_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:300:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_4_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:302:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:306:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_4_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:308:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:312:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:314:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:318:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:320:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:324:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:326:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:330:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:332:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:336:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:338:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:342:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:344:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:348:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:350:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:354:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:356:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:360:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:362:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:366:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:368:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:372:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:374:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:378:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:380:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:384:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:386:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:390:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:392:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:396:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:398:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:402:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:404:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:408:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:410:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:414:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:415:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:416:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:417:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:419:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:423:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:425:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:429:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:431:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:435:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:437:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_2_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:441:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_2_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:443:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_3_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:447:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_3_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:449:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_3_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:453:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_3_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:455:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_3_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:459:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_3_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:461:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_4_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:465:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_4_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:467:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:471:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_4_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:473:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:477:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:479:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:483:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:485:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:489:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:491:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:495:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:497:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:501:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:503:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:507:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:509:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:513:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:515:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:519:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:521:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:525:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:527:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:531:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:533:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:537:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:539:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:543:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:545:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:549:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:551:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:555:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:557:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:561:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:563:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:567:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:569:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:573:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:575:25
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:579:21
pub const __AVAILABILITY_INTERNAL__IPHONE_2_2_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:580:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:581:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:582:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:584:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:588:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:590:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:594:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:596:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:600:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:602:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_3_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:606:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_3_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:608:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_3_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:612:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_3_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:614:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_3_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:618:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_3_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:620:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_4_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:624:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_4_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:626:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:630:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_4_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:632:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:636:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:638:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:642:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:644:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:648:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:650:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:654:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:656:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:660:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:662:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:666:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:668:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:672:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:674:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:678:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:680:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:684:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:686:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:690:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:692:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:696:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:698:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:702:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:704:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:708:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:710:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:714:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:716:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:720:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:722:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:726:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:728:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:732:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:734:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:738:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:739:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:740:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:741:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:743:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:747:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:749:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:753:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:755:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:759:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:761:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_3_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:765:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_3_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:767:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_3_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:771:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_3_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:773:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_4_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:777:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_4_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:779:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:783:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_4_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:785:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:789:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:791:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:795:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:797:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:801:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:803:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:807:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:809:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:813:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:815:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:819:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:821:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:825:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:827:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:831:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:833:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:837:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:839:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:843:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:845:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:849:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:851:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:855:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:857:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:861:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:863:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:867:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:869:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:873:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:875:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:879:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:881:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:885:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:887:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:891:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:892:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:893:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:894:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:896:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:900:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:902:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:906:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:908:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:912:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:914:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_3_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:918:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_3_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:920:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_4_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:924:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_4_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:926:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:930:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_4_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:932:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:936:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:938:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:942:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:944:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:948:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:950:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:954:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:956:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:960:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:962:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:966:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:968:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:972:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:974:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:978:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:980:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:984:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:986:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:990:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:992:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:996:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:998:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1002:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1004:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1008:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1010:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1014:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1016:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1020:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1022:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1026:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1028:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1032:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1034:25
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1038:21
pub const __AVAILABILITY_INTERNAL__IPHONE_3_2_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1039:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1040:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1041:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1043:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1047:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1049:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1053:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1055:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1059:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1061:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_12_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1066:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_4_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1070:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_4_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1072:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1076:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_4_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1078:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1082:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1084:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1088:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1090:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1094:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1096:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1100:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1102:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1106:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1108:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1112:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1114:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1118:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1120:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1124:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1126:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1130:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1132:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1136:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1138:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1142:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1144:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1148:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1150:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1154:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1156:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1160:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1162:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1166:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1168:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1172:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1174:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1178:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1180:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1184:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1185:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1186:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1187:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1189:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1193:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1195:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1199:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1201:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1205:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1207:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_4_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1211:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_4_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1213:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1217:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1219:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1223:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1225:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1229:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1231:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1235:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1237:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1241:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1243:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1247:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1249:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1253:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1255:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1259:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1261:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1265:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1267:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1271:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1273:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1277:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1279:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1283:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1285:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1289:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1291:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1295:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1297:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1301:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1303:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1307:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1309:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1313:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1315:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1319:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1320:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1321:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1322:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1324:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1328:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1330:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1334:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1336:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1340:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1342:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_4_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1346:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_4_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1348:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1352:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1354:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1358:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1360:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1364:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1366:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1370:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1372:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1376:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1378:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1382:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1384:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1388:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1390:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1394:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1396:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1400:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1402:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1406:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1408:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1412:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1414:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1418:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1420:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1424:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1426:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1430:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1432:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1436:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1438:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1442:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1444:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1448:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_2_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1449:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1450:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1451:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1453:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1457:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1459:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1463:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1465:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1469:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1471:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_4_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1475:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_4_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1477:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1481:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1483:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1487:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1489:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1493:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1495:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1499:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1501:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1505:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1507:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1511:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1513:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1517:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1519:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1523:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1525:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1529:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1531:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1535:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1537:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1541:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1543:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1547:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1549:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1553:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1555:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1559:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1561:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1565:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1567:25
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1571:21
pub const __AVAILABILITY_INTERNAL__IPHONE_4_3_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1572:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1573:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1574:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1576:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1580:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1582:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1586:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1588:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1592:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1594:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_11_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1598:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_5_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1599:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_5_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1601:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1605:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1607:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1611:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1613:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1617:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1619:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1623:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1625:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1629:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1631:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1635:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1637:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1641:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1643:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1647:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1649:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1653:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1655:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1659:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1661:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1665:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1667:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1671:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1673:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1677:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1679:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1683:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1685:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1689:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1690:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1691:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1692:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1694:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1698:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1700:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1704:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1706:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1710:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1712:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_5_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1716:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_5_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1718:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1722:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1724:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1728:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1730:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1734:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1736:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1740:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1742:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1746:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1748:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1752:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1754:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1758:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1760:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1764:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1766:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1770:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1772:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1776:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1778:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1782:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1784:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1788:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1790:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1794:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1796:25
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1800:21
pub const __AVAILABILITY_INTERNAL__IPHONE_5_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1801:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1802:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1803:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1805:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1809:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1811:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1815:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1817:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1821:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1823:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_6_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1827:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_6_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1829:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1833:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1835:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1839:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1841:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1845:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1847:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1851:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1853:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1857:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1859:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1863:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1865:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1869:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1871:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1875:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1877:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1881:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1883:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1887:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1889:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1893:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1895:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1899:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1901:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1905:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1906:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1907:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1908:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1910:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1914:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1916:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1920:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1922:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1926:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1928:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_6_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1932:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_6_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1934:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1938:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1940:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1944:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1946:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1950:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1952:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1956:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1958:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1962:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1964:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1968:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1970:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1974:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1976:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1980:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1982:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1986:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1988:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1992:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1994:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:1998:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2000:25
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2004:21
pub const __AVAILABILITY_INTERNAL__IPHONE_6_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2005:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2006:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2007:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2009:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2013:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2015:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2019:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2021:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2025:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2027:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_11_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2031:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_11_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2032:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_12_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2034:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_7_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2038:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_7_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2040:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2044:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2046:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2050:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2052:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2056:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2058:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2062:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2064:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2068:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2070:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2074:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2076:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2080:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2082:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2086:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2088:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2092:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2094:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2098:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2100:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2104:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2105:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2106:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2107:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2109:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2113:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2115:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2119:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2121:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2125:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2127:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_7_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2131:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_7_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2133:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2137:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2139:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2143:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2145:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2149:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2151:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2155:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2157:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2161:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2163:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2167:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2169:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2173:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2175:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2179:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2181:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2185:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2187:25
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2191:21
pub const __AVAILABILITY_INTERNAL__IPHONE_7_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2192:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2193:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2194:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2196:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2200:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2202:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2206:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2208:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2212:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2214:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_11_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2219:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_11_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2223:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_12_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2224:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2225:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2227:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2231:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2233:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2237:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2239:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2243:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2245:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2249:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2251:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2255:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2257:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2261:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2263:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2267:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2269:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2273:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2275:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2279:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2280:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2281:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2282:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2284:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2288:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2290:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2294:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2296:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2300:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2302:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_8_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2306:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_8_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2308:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2312:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2314:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2318:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2320:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2324:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2326:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2330:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2332:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2336:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2338:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2342:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2344:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2348:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2350:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2354:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2355:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2356:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2357:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2359:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2363:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2365:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2369:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2371:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2375:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2377:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_8_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2381:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_8_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2383:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2387:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2389:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2393:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2395:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2399:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2401:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2405:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2407:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2411:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2413:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2417:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2419:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2423:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_2_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2424:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2425:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2426:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2428:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2432:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2434:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2438:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2440:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2444:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2446:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_8_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2450:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_8_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2452:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2456:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2458:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2462:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2464:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2468:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2470:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2474:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2476:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2480:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2482:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2486:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_3_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2487:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2488:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2489:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2491:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2495:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2497:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2501:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2503:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2507:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2509:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_8_4 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2513:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_8_4_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2515:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2519:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2521:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2525:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2527:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2531:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2533:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2537:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2539:25
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2543:21
pub const __AVAILABILITY_INTERNAL__IPHONE_8_4_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2544:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2545:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2546:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2548:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2552:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2554:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2558:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2560:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2564:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2566:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_9_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2570:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_9_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2572:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2576:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2578:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2582:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2584:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2588:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2590:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2594:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2595:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2596:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2597:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2599:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2603:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2605:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2609:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2611:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2615:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2617:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_9_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2621:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_9_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2623:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2627:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2629:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2633:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2635:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2639:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2640:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2641:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2642:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2644:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2648:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2650:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2654:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2656:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2660:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2662:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_9_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2666:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_9_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2668:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2672:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2674:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2678:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_2_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2679:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2680:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2681:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2683:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2687:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2689:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2693:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2695:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2699:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2701:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_9_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2705:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_9_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2707:25
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2711:21
pub const __AVAILABILITY_INTERNAL__IPHONE_9_3_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2712:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2713:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_10_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2714:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_10_0_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2716:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2720:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2722:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2726:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2728:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2732:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2734:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_11_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2738:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_12_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2739:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2740:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_0_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2741:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2742:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1_DEP__IPHONE_10_1 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2743:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1_DEP__IPHONE_10_1_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2745:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2749:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2751:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2755:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2757:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2761:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_1_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2762:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2763:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_2_DEP__IPHONE_10_2 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2764:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_2_DEP__IPHONE_10_2_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2766:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_2_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2770:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_2_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2772:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_2_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2776:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_2_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2777:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2778:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_3_DEP__IPHONE_10_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2779:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_3_DEP__IPHONE_10_3_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2781:25
pub const __AVAILABILITY_INTERNAL__IPHONE_10_3_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2785:21
pub const __AVAILABILITY_INTERNAL__IPHONE_10_3_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2786:21
pub const __AVAILABILITY_INTERNAL__IPHONE_11 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2787:21
pub const __AVAILABILITY_INTERNAL__IPHONE_11_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2788:21
pub const __AVAILABILITY_INTERNAL__IPHONE_11_3 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2789:21
pub const __AVAILABILITY_INTERNAL__IPHONE_12_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2790:21
pub const __AVAILABILITY_INTERNAL__IPHONE_13_0 = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2791:21
pub const __AVAILABILITY_INTERNAL__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2793:21
pub const __AVAILABILITY_INTERNAL__IPHONE_NA__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2794:21
pub const __AVAILABILITY_INTERNAL__IPHONE_NA_DEP__IPHONE_NA = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2795:21
pub const __AVAILABILITY_INTERNAL__IPHONE_NA_DEP__IPHONE_NA_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2796:21
pub const __AVAILABILITY_INTERNAL__IPHONE_COMPAT_VERSION = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2817:25
pub const __AVAILABILITY_INTERNAL__IPHONE_COMPAT_VERSION_DEP__IPHONE_COMPAT_VERSION = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2818:25
pub const __AVAILABILITY_INTERNAL__IPHONE_COMPAT_VERSION_DEP__IPHONE_COMPAT_VERSION_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/AvailabilityInternalLegacy.h:2820:25
pub const __OSX_AVAILABLE_STARTING = @compileError("unable to translate macro: undefined identifier `__AVAILABILITY_INTERNAL`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:204:13
pub const __OSX_AVAILABLE_BUT_DEPRECATED = @compileError("unable to translate macro: undefined identifier `__AVAILABILITY_INTERNAL`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:205:13
pub const __OSX_AVAILABLE_BUT_DEPRECATED_MSG = @compileError("unable to translate macro: undefined identifier `__AVAILABILITY_INTERNAL`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:207:13
pub const __OS_AVAILABILITY = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:263:13
pub const __OS_AVAILABILITY_MSG = @compileError("unable to translate macro: undefined identifier `availability`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:264:13
pub const __OSX_EXTENSION_UNAVAILABLE = @compileError("unable to translate macro: undefined identifier `macosx_app_extension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:281:13
pub const __IOS_EXTENSION_UNAVAILABLE = @compileError("unable to translate macro: undefined identifier `ios_app_extension`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:282:13
pub inline fn __OS_EXTENSION_UNAVAILABLE(_msg: anytype) @TypeOf(__OSX_EXTENSION_UNAVAILABLE(_msg) ++ __IOS_EXTENSION_UNAVAILABLE(_msg)) {
    _ = &_msg;
    return __OSX_EXTENSION_UNAVAILABLE(_msg) ++ __IOS_EXTENSION_UNAVAILABLE(_msg);
}
pub const __OSX_UNAVAILABLE = @compileError("unable to translate macro: undefined identifier `macosx`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:299:13
pub const __OSX_AVAILABLE = @compileError("unable to translate macro: undefined identifier `macosx`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:300:13
pub const __OSX_DEPRECATED = @compileError("unable to translate macro: undefined identifier `macosx`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:301:13
pub const __IOS_UNAVAILABLE = @compileError("unable to translate macro: undefined identifier `ios`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:325:13
pub const __IOS_PROHIBITED = @compileError("unable to translate macro: undefined identifier `ios`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:327:15
pub const __IOS_AVAILABLE = @compileError("unable to translate macro: undefined identifier `ios`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:329:13
pub const __IOS_DEPRECATED = @compileError("unable to translate macro: undefined identifier `ios`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:330:13
pub const __TVOS_UNAVAILABLE = @compileError("unable to translate macro: undefined identifier `tvos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:354:13
pub const __TVOS_PROHIBITED = @compileError("unable to translate macro: undefined identifier `tvos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:356:15
pub const __TVOS_AVAILABLE = @compileError("unable to translate macro: undefined identifier `tvos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:358:13
pub const __TVOS_DEPRECATED = @compileError("unable to translate macro: undefined identifier `tvos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:359:13
pub const __WATCHOS_UNAVAILABLE = @compileError("unable to translate macro: undefined identifier `watchos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:383:13
pub const __WATCHOS_PROHIBITED = @compileError("unable to translate macro: undefined identifier `watchos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:385:15
pub const __WATCHOS_AVAILABLE = @compileError("unable to translate macro: undefined identifier `watchos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:387:13
pub const __WATCHOS_DEPRECATED = @compileError("unable to translate macro: undefined identifier `watchos`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:388:13
pub const __SWIFT_UNAVAILABLE = @compileError("unable to translate macro: undefined identifier `swift`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:411:13
pub const __SWIFT_UNAVAILABLE_MSG = @compileError("unable to translate macro: undefined identifier `swift`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:412:13
pub const __API_AVAILABLE = @compileError("unable to translate C expr: unexpected token '__VA_ARGS__'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:457:13
pub const __API_AVAILABLE_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:459:13
pub const __API_AVAILABLE_END = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:460:13
pub const __API_DEPRECATED = @compileError("unable to translate C expr: unexpected token '__VA_ARGS__'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:483:13
pub const __API_DEPRECATED_WITH_REPLACEMENT = @compileError("unable to translate C expr: unexpected token '__VA_ARGS__'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:484:13
pub const __API_DEPRECATED_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:486:13
pub const __API_DEPRECATED_END = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:487:13
pub const __API_DEPRECATED_WITH_REPLACEMENT_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:489:13
pub const __API_DEPRECATED_WITH_REPLACEMENT_END = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:490:13
pub const __API_OBSOLETED = @compileError("unable to translate C expr: unexpected token '__VA_ARGS__'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:494:13
pub const __API_OBSOLETED_WITH_REPLACEMENT = @compileError("unable to translate C expr: unexpected token '__VA_ARGS__'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:495:13
pub const __API_OBSOLETED_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:497:13
pub const __API_OBSOLETED_END = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:498:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:500:13
pub const __API_OBSOLETED_WITH_REPLACEMENT_END = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:501:13
pub const __API_UNAVAILABLE = @compileError("unable to translate C expr: unexpected token '__VA_ARGS__'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:512:13
pub const __API_UNAVAILABLE_BEGIN = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:514:13
pub const __API_UNAVAILABLE_END = @compileError("unable to translate macro: undefined identifier `_Pragma`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/Availability.h:515:13
pub inline fn __SPI_AVAILABLE() void {
    return;
}
pub inline fn __SPI_AVAILABLE_BEGIN() void {
    return;
}
pub const __SPI_AVAILABLE_END = "";
pub inline fn __SPI_DEPRECATED() void {
    return;
}
pub inline fn __SPI_DEPRECATED_WITH_REPLACEMENT() void {
    return;
}
pub const _LIBC_BOUNDS_H_ = "";
pub inline fn _LIBC_COUNT(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn _LIBC_COUNT_OR_NULL(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn _LIBC_SIZE(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn _LIBC_SIZE_OR_NULL(x: anytype) void {
    _ = &x;
    return;
}
pub inline fn _LIBC_ENDED_BY(x: anytype) void {
    _ = &x;
    return;
}
pub const _LIBC_SINGLE = "";
pub const _LIBC_UNSAFE_INDEXABLE = "";
pub const _LIBC_CSTR = "";
pub const _LIBC_NULL_TERMINATED = "";
pub inline fn _LIBC_FLEX_COUNT(FIELD: anytype, INTCOUNT: anytype) @TypeOf(INTCOUNT) {
    _ = &FIELD;
    _ = &INTCOUNT;
    return INTCOUNT;
}
pub inline fn _LIBC_SINGLE_BY_DEFAULT() void {
    return;
}
pub inline fn _LIBC_PTRCHECK_REPLACED(R: anytype) void {
    _ = &R;
    return;
}
pub inline fn _LIBC_FORGE_PTR(P: anytype, S: anytype) @TypeOf(P) {
    _ = &P;
    _ = &S;
    return P;
}
pub const __TYPES_H_ = "";
pub const __strfmonlike = @compileError("unable to translate macro: undefined identifier `__format__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/_types.h:34:9
pub const __strftimelike = @compileError("unable to translate macro: undefined identifier `__format__`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/_types.h:36:9
pub const __DARWIN_WCHAR_MAX = __WCHAR_MAX__;
pub const __DARWIN_WCHAR_MIN = -__helpers.promoteIntLiteral(c_int, 0x7fffffff, .hex) - @as(c_int, 1);
pub const __DARWIN_WEOF = __helpers.cast(__darwin_wint_t, -@as(c_int, 1));
pub const _FORTIFY_SOURCE = @as(c_int, 2);
pub const _SYS_WAIT_H_ = "";
pub const _PID_T = "";
pub const _ID_T = "";
pub const _SYS_SIGNAL_H_ = "";
pub const __SYS_APPLEAPIOPTS_H__ = "";
pub const __APPLE_API_STANDARD = "";
pub const __APPLE_API_STABLE = "";
pub const __APPLE_API_EVOLVING = "";
pub const __APPLE_API_UNSTABLE = "";
pub const __APPLE_API_PRIVATE = "";
pub const __APPLE_API_OBSOLETE = "";
pub const __DARWIN_NSIG = @as(c_int, 32);
pub const NSIG = __DARWIN_NSIG;
pub const _BSD_MACHINE_SIGNAL_H_ = "";
pub const _ARM_SIGNAL_ = @as(c_int, 1);
pub const SIGHUP = @as(c_int, 1);
pub const SIGINT = @as(c_int, 2);
pub const SIGQUIT = @as(c_int, 3);
pub const SIGILL = @as(c_int, 4);
pub const SIGTRAP = @as(c_int, 5);
pub const SIGABRT = @as(c_int, 6);
pub const SIGIOT = SIGABRT;
pub const SIGEMT = @as(c_int, 7);
pub const SIGFPE = @as(c_int, 8);
pub const SIGKILL = @as(c_int, 9);
pub const SIGBUS = @as(c_int, 10);
pub const SIGSEGV = @as(c_int, 11);
pub const SIGSYS = @as(c_int, 12);
pub const SIGPIPE = @as(c_int, 13);
pub const SIGALRM = @as(c_int, 14);
pub const SIGTERM = @as(c_int, 15);
pub const SIGURG = @as(c_int, 16);
pub const SIGSTOP = @as(c_int, 17);
pub const SIGTSTP = @as(c_int, 18);
pub const SIGCONT = @as(c_int, 19);
pub const SIGCHLD = @as(c_int, 20);
pub const SIGTTIN = @as(c_int, 21);
pub const SIGTTOU = @as(c_int, 22);
pub const SIGIO = @as(c_int, 23);
pub const SIGXCPU = @as(c_int, 24);
pub const SIGXFSZ = @as(c_int, 25);
pub const SIGVTALRM = @as(c_int, 26);
pub const SIGPROF = @as(c_int, 27);
pub const SIGWINCH = @as(c_int, 28);
pub const SIGINFO = @as(c_int, 29);
pub const SIGUSR1 = @as(c_int, 30);
pub const SIGUSR2 = @as(c_int, 31);
pub const SIG_DFL = @compileError("unable to translate C expr: expected ')' instead got '('"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/signal.h:131:9
pub const SIG_IGN = @compileError("unable to translate C expr: expected ')' instead got '('"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/signal.h:132:9
pub const SIG_HOLD = @compileError("unable to translate C expr: expected ')' instead got '('"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/signal.h:133:9
pub const SIG_ERR = @compileError("unable to translate C expr: expected ')' instead got '('"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/signal.h:134:9
pub const _BSD_MACHINE__MCONTEXT_H_ = "";
pub const __ARM_MCONTEXT_H_ = "";
pub const _MACH_MACHINE__STRUCTS_H_ = "";
pub const _MACH_ARM__STRUCTS_H_ = "";
pub const _BSD_MACHINE_TYPES_H_ = "";
pub const _ARM_MACHTYPES_H_ = "";
pub const _MACHTYPES_H_ = "";
pub const _U_INT8_T = "";
pub const _U_INT16_T = "";
pub const _U_INT32_T = "";
pub const _U_INT64_T = "";
pub const USER_ADDR_NULL = __helpers.cast(user_addr_t, @as(c_int, 0));
pub inline fn CAST_USER_ADDR_T(a_ptr: anytype) user_addr_t {
    _ = &a_ptr;
    return __helpers.cast(user_addr_t, __helpers.cast(usize, a_ptr));
}
pub const _STRUCT_ARM_EXCEPTION_STATE = struct___darwin_arm_exception_state;
pub const _STRUCT_ARM_EXCEPTION_STATE64 = struct___darwin_arm_exception_state64;
pub const _STRUCT_ARM_EXCEPTION_STATE64_V2 = struct___darwin_arm_exception_state64_v2;
pub const _STRUCT_ARM_THREAD_STATE = struct___darwin_arm_thread_state;
pub const __DARWIN_OPAQUE_ARM_THREAD_STATE64 = @as(c_int, 0);
pub const _STRUCT_ARM_THREAD_STATE64 = struct___darwin_arm_thread_state64;
pub inline fn __darwin_arm_thread_state64_get_pc(ts: anytype) @TypeOf(ts.__pc) {
    _ = &ts;
    return ts.__pc;
}
pub inline fn __darwin_arm_thread_state64_get_pc_fptr(ts: anytype) ?*anyopaque {
    _ = &ts;
    return __helpers.cast(?*anyopaque, __helpers.cast(usize, ts.__pc));
}
pub const __darwin_arm_thread_state64_set_pc_fptr = @compileError("unable to translate C expr: expected ')' instead got '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/mach/arm/_structs.h:437:9
pub const __darwin_arm_thread_state64_set_pc_presigned_fptr = @compileError("unable to translate C expr: expected ')' instead got '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/mach/arm/_structs.h:440:9
pub inline fn __darwin_arm_thread_state64_get_lr(ts: anytype) @TypeOf(ts.__lr) {
    _ = &ts;
    return ts.__lr;
}
pub inline fn __darwin_arm_thread_state64_get_lr_fptr(ts: anytype) ?*anyopaque {
    _ = &ts;
    return __helpers.cast(?*anyopaque, __helpers.cast(usize, ts.__lr));
}
pub const __darwin_arm_thread_state64_set_lr_fptr = @compileError("unable to translate C expr: expected ')' instead got '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/mach/arm/_structs.h:449:9
pub const __darwin_arm_thread_state64_set_lr_presigned_fptr = @compileError("unable to translate C expr: expected ')' instead got '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/mach/arm/_structs.h:452:9
pub inline fn __darwin_arm_thread_state64_get_sp(ts: anytype) @TypeOf(ts.__sp) {
    _ = &ts;
    return ts.__sp;
}
pub const __darwin_arm_thread_state64_set_sp = @compileError("unable to translate C expr: expected ')' instead got '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/mach/arm/_structs.h:458:9
pub inline fn __darwin_arm_thread_state64_get_fp(ts: anytype) @TypeOf(ts.__fp) {
    _ = &ts;
    return ts.__fp;
}
pub const __darwin_arm_thread_state64_set_fp = @compileError("unable to translate C expr: expected ')' instead got '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/mach/arm/_structs.h:464:9
pub const __darwin_arm_thread_state64_ptrauth_strip = __helpers.DISCARD;
pub const _STRUCT_ARM_VFP_STATE = struct___darwin_arm_vfp_state;
pub const _STRUCT_ARM_NEON_STATE64 = struct___darwin_arm_neon_state64;
pub const _STRUCT_ARM_NEON_STATE = struct___darwin_arm_neon_state;
pub const _STRUCT_ARM_PAGEIN_STATE = struct___arm_pagein_state;
pub const _STRUCT_ARM_SME_STATE = struct___darwin_arm_sme_state;
pub const _STRUCT_ARM_SVE_Z_STATE = struct___darwin_arm_sve_z_state;
pub const _STRUCT_ARM_SVE_P_STATE = struct___darwin_arm_sve_p_state;
pub const _STRUCT_ARM_SME_ZA_STATE = struct___darwin_arm_sme_za_state;
pub const _STRUCT_ARM_SME2_STATE = struct___darwin_arm_sme2_state;
pub const _STRUCT_ARM_LEGACY_DEBUG_STATE = struct___arm_legacy_debug_state;
pub const _STRUCT_ARM_DEBUG_STATE32 = struct___darwin_arm_debug_state32;
pub const _STRUCT_ARM_DEBUG_STATE64 = struct___darwin_arm_debug_state64;
pub const _STRUCT_ARM_CPMU_STATE64 = struct___darwin_arm_cpmu_state64;
pub const _STRUCT_MCONTEXT32 = struct___darwin_mcontext32;
pub const _STRUCT_MCONTEXT64 = struct___darwin_mcontext64;
pub const _MCONTEXT_T = "";
pub const _STRUCT_MCONTEXT = _STRUCT_MCONTEXT64;
pub const _PTHREAD_ATTR_T = "";
pub const _STRUCT_SIGALTSTACK = struct___darwin_sigaltstack;
pub const _STRUCT_UCONTEXT = struct___darwin_ucontext;
pub const _SIGSET_T = "";
pub const _SIZE_T = "";
pub const _UID_T = "";
pub const SIGEV_NONE = @as(c_int, 0);
pub const SIGEV_SIGNAL = @as(c_int, 1);
pub const SIGEV_THREAD = @as(c_int, 3);
pub const SIGEV_KEVENT = @as(c_int, 4);
pub const ILL_NOOP = @as(c_int, 0);
pub const ILL_ILLOPC = @as(c_int, 1);
pub const ILL_ILLTRP = @as(c_int, 2);
pub const ILL_PRVOPC = @as(c_int, 3);
pub const ILL_ILLOPN = @as(c_int, 4);
pub const ILL_ILLADR = @as(c_int, 5);
pub const ILL_PRVREG = @as(c_int, 6);
pub const ILL_COPROC = @as(c_int, 7);
pub const ILL_BADSTK = @as(c_int, 8);
pub const FPE_NOOP = @as(c_int, 0);
pub const FPE_FLTDIV = @as(c_int, 1);
pub const FPE_FLTOVF = @as(c_int, 2);
pub const FPE_FLTUND = @as(c_int, 3);
pub const FPE_FLTRES = @as(c_int, 4);
pub const FPE_FLTINV = @as(c_int, 5);
pub const FPE_FLTSUB = @as(c_int, 6);
pub const FPE_INTDIV = @as(c_int, 7);
pub const FPE_INTOVF = @as(c_int, 8);
pub const SEGV_NOOP = @as(c_int, 0);
pub const SEGV_MAPERR = @as(c_int, 1);
pub const SEGV_ACCERR = @as(c_int, 2);
pub const BUS_NOOP = @as(c_int, 0);
pub const BUS_ADRALN = @as(c_int, 1);
pub const BUS_ADRERR = @as(c_int, 2);
pub const BUS_OBJERR = @as(c_int, 3);
pub const TRAP_BRKPT = @as(c_int, 1);
pub const TRAP_TRACE = @as(c_int, 2);
pub const CLD_NOOP = @as(c_int, 0);
pub const CLD_EXITED = @as(c_int, 1);
pub const CLD_KILLED = @as(c_int, 2);
pub const CLD_DUMPED = @as(c_int, 3);
pub const CLD_TRAPPED = @as(c_int, 4);
pub const CLD_STOPPED = @as(c_int, 5);
pub const CLD_CONTINUED = @as(c_int, 6);
pub const POLL_IN = @as(c_int, 1);
pub const POLL_OUT = @as(c_int, 2);
pub const POLL_MSG = @as(c_int, 3);
pub const POLL_ERR = @as(c_int, 4);
pub const POLL_PRI = @as(c_int, 5);
pub const POLL_HUP = @as(c_int, 6);
pub const sa_handler = __sigaction_u.__sa_handler;
pub const sa_sigaction = __sigaction_u.__sa_sigaction;
pub const SA_ONSTACK = @as(c_int, 0x0001);
pub const SA_RESTART = @as(c_int, 0x0002);
pub const SA_RESETHAND = @as(c_int, 0x0004);
pub const SA_NOCLDSTOP = @as(c_int, 0x0008);
pub const SA_NODEFER = @as(c_int, 0x0010);
pub const SA_NOCLDWAIT = @as(c_int, 0x0020);
pub const SA_SIGINFO = @as(c_int, 0x0040);
pub const SA_USERTRAMP = @as(c_int, 0x0100);
pub const SA_64REGSET = @as(c_int, 0x0200);
pub const SA_USERSPACE_MASK = (((((SA_ONSTACK | SA_RESTART) | SA_RESETHAND) | SA_NOCLDSTOP) | SA_NODEFER) | SA_NOCLDWAIT) | SA_SIGINFO;
pub const SIG_BLOCK = @as(c_int, 1);
pub const SIG_UNBLOCK = @as(c_int, 2);
pub const SIG_SETMASK = @as(c_int, 3);
pub const SI_USER = __helpers.promoteIntLiteral(c_int, 0x10001, .hex);
pub const SI_QUEUE = __helpers.promoteIntLiteral(c_int, 0x10002, .hex);
pub const SI_TIMER = __helpers.promoteIntLiteral(c_int, 0x10003, .hex);
pub const SI_ASYNCIO = __helpers.promoteIntLiteral(c_int, 0x10004, .hex);
pub const SI_MESGQ = __helpers.promoteIntLiteral(c_int, 0x10005, .hex);
pub const SS_ONSTACK = @as(c_int, 0x0001);
pub const SS_DISABLE = @as(c_int, 0x0004);
pub const MINSIGSTKSZ = __helpers.promoteIntLiteral(c_int, 32768, .decimal);
pub const SIGSTKSZ = __helpers.promoteIntLiteral(c_int, 131072, .decimal);
pub const SV_ONSTACK = SA_ONSTACK;
pub const SV_INTERRUPT = SA_RESTART;
pub const SV_RESETHAND = SA_RESETHAND;
pub const SV_NODEFER = SA_NODEFER;
pub const SV_NOCLDSTOP = SA_NOCLDSTOP;
pub const SV_SIGINFO = SA_SIGINFO;
pub const sv_onstack = @compileError("unable to translate macro: undefined identifier `sv_flags`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/signal.h:362:9
pub inline fn sigmask(m: anytype) @TypeOf(@as(c_int, 1) << (m - @as(c_int, 1))) {
    _ = &m;
    return @as(c_int, 1) << (m - @as(c_int, 1));
}
pub const BADSIG = SIG_ERR;
pub const _SYS_RESOURCE_H_ = "";
pub const _STRUCT_TIMEVAL = struct_timeval;
pub const PRIO_PROCESS = @as(c_int, 0);
pub const PRIO_PGRP = @as(c_int, 1);
pub const PRIO_USER = @as(c_int, 2);
pub const PRIO_DARWIN_THREAD = @as(c_int, 3);
pub const PRIO_DARWIN_PROCESS = @as(c_int, 4);
pub const PRIO_MIN = -@as(c_int, 20);
pub const PRIO_MAX = @as(c_int, 20);
pub const PRIO_DARWIN_BG = @as(c_int, 0x1000);
pub const PRIO_DARWIN_NONUI = @as(c_int, 0x1001);
pub const RUSAGE_SELF = @as(c_int, 0);
pub const RUSAGE_CHILDREN = -@as(c_int, 1);
pub const ru_first = @compileError("unable to translate macro: undefined identifier `ru_ixrss`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/resource.h:164:9
pub const ru_last = @compileError("unable to translate macro: undefined identifier `ru_nivcsw`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/resource.h:178:9
pub const RUSAGE_INFO_V0 = @as(c_int, 0);
pub const RUSAGE_INFO_V1 = @as(c_int, 1);
pub const RUSAGE_INFO_V2 = @as(c_int, 2);
pub const RUSAGE_INFO_V3 = @as(c_int, 3);
pub const RUSAGE_INFO_V4 = @as(c_int, 4);
pub const RUSAGE_INFO_V5 = @as(c_int, 5);
pub const RUSAGE_INFO_V6 = @as(c_int, 6);
pub const RUSAGE_INFO_CURRENT = RUSAGE_INFO_V6;
pub const RU_PROC_RUNS_RESLIDE = @as(c_int, 0x00000001);
pub const RLIM_INFINITY = (__helpers.cast(__uint64_t, @as(c_int, 1)) << @as(c_int, 63)) - @as(c_int, 1);
pub const RLIM_SAVED_MAX = RLIM_INFINITY;
pub const RLIM_SAVED_CUR = RLIM_INFINITY;
pub const RLIMIT_CPU = @as(c_int, 0);
pub const RLIMIT_FSIZE = @as(c_int, 1);
pub const RLIMIT_DATA = @as(c_int, 2);
pub const RLIMIT_STACK = @as(c_int, 3);
pub const RLIMIT_CORE = @as(c_int, 4);
pub const RLIMIT_AS = @as(c_int, 5);
pub const RLIMIT_RSS = RLIMIT_AS;
pub const RLIMIT_MEMLOCK = @as(c_int, 6);
pub const RLIMIT_NPROC = @as(c_int, 7);
pub const RLIMIT_NOFILE = @as(c_int, 8);
pub const RLIM_NLIMITS = @as(c_int, 9);
pub const _RLIMIT_POSIX_FLAG = @as(c_int, 0x1000);
pub const RLIMIT_WAKEUPS_MONITOR = @as(c_int, 0x1);
pub const RLIMIT_CPU_USAGE_MONITOR = @as(c_int, 0x2);
pub const RLIMIT_THREAD_CPULIMITS = @as(c_int, 0x3);
pub const RLIMIT_FOOTPRINT_INTERVAL = @as(c_int, 0x4);
pub const WAKEMON_ENABLE = @as(c_int, 0x01);
pub const WAKEMON_DISABLE = @as(c_int, 0x02);
pub const WAKEMON_GET_PARAMS = @as(c_int, 0x04);
pub const WAKEMON_SET_DEFAULTS = @as(c_int, 0x08);
pub const WAKEMON_MAKE_FATAL = @as(c_int, 0x10);
pub const CPUMON_MAKE_FATAL = @as(c_int, 0x1000);
pub const FOOTPRINT_INTERVAL_RESET = @as(c_int, 0x1);
pub const IOPOL_TYPE_DISK = @as(c_int, 0);
pub const IOPOL_TYPE_VFS_ATIME_UPDATES = @as(c_int, 2);
pub const IOPOL_TYPE_VFS_MATERIALIZE_DATALESS_FILES = @as(c_int, 3);
pub const IOPOL_TYPE_VFS_STATFS_NO_DATA_VOLUME = @as(c_int, 4);
pub const IOPOL_TYPE_VFS_TRIGGER_RESOLVE = @as(c_int, 5);
pub const IOPOL_TYPE_VFS_IGNORE_CONTENT_PROTECTION = @as(c_int, 6);
pub const IOPOL_TYPE_VFS_IGNORE_PERMISSIONS = @as(c_int, 7);
pub const IOPOL_TYPE_VFS_SKIP_MTIME_UPDATE = @as(c_int, 8);
pub const IOPOL_TYPE_VFS_ALLOW_LOW_SPACE_WRITES = @as(c_int, 9);
pub const IOPOL_TYPE_VFS_DISALLOW_RW_FOR_O_EVTONLY = @as(c_int, 10);
pub const IOPOL_TYPE_VFS_ENTITLED_RESERVE_ACCESS = @as(c_int, 14);
pub const IOPOL_SCOPE_PROCESS = @as(c_int, 0);
pub const IOPOL_SCOPE_THREAD = @as(c_int, 1);
pub const IOPOL_SCOPE_DARWIN_BG = @as(c_int, 2);
pub const IOPOL_DEFAULT = @as(c_int, 0);
pub const IOPOL_IMPORTANT = @as(c_int, 1);
pub const IOPOL_PASSIVE = @as(c_int, 2);
pub const IOPOL_THROTTLE = @as(c_int, 3);
pub const IOPOL_UTILITY = @as(c_int, 4);
pub const IOPOL_STANDARD = @as(c_int, 5);
pub const IOPOL_APPLICATION = IOPOL_STANDARD;
pub const IOPOL_NORMAL = IOPOL_IMPORTANT;
pub const IOPOL_ATIME_UPDATES_DEFAULT = @as(c_int, 0);
pub const IOPOL_ATIME_UPDATES_OFF = @as(c_int, 1);
pub const IOPOL_MATERIALIZE_DATALESS_FILES_DEFAULT = @as(c_int, 0);
pub const IOPOL_MATERIALIZE_DATALESS_FILES_OFF = @as(c_int, 1);
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ON = @as(c_int, 2);
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ORIG = @as(c_int, 4);
pub const IOPOL_MATERIALIZE_DATALESS_FILES_BASIC_MASK = @as(c_int, 3);
pub const IOPOL_VFS_STATFS_NO_DATA_VOLUME_DEFAULT = @as(c_int, 0);
pub const IOPOL_VFS_STATFS_FORCE_NO_DATA_VOLUME = @as(c_int, 1);
pub const IOPOL_VFS_TRIGGER_RESOLVE_DEFAULT = @as(c_int, 0);
pub const IOPOL_VFS_TRIGGER_RESOLVE_OFF = @as(c_int, 1);
pub const IOPOL_VFS_CONTENT_PROTECTION_DEFAULT = @as(c_int, 0);
pub const IOPOL_VFS_CONTENT_PROTECTION_IGNORE = @as(c_int, 1);
pub const IOPOL_VFS_IGNORE_PERMISSIONS_OFF = @as(c_int, 0);
pub const IOPOL_VFS_IGNORE_PERMISSIONS_ON = @as(c_int, 1);
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_OFF = @as(c_int, 0);
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_ON = @as(c_int, 1);
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_IGNORE = @as(c_int, 2);
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_OFF = @as(c_int, 0);
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_ON = @as(c_int, 1);
pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_DEFAULT = @as(c_int, 0);
pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_ON = @as(c_int, 1);
pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_DEFAULT = @as(c_int, 0);
pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_ON = @as(c_int, 1);
pub const IOPOL_VFS_ENTITLED_RESERVE_ACCESS_OFF = @as(c_int, 0);
pub const IOPOL_VFS_ENTITLED_RESERVE_ACCESS_ON = @as(c_int, 1);
pub const WNOHANG = @as(c_int, 0x00000001);
pub const WUNTRACED = @as(c_int, 0x00000002);
pub inline fn _W_INT(w: anytype) @TypeOf(__helpers.cast([*c]c_int, &w).*) {
    _ = &w;
    return __helpers.cast([*c]c_int, &w).*;
}
pub const WCOREFLAG = @as(c_int, 0o200);
pub inline fn _WSTATUS(x: anytype) @TypeOf(_W_INT(x) & @as(c_int, 0o177)) {
    _ = &x;
    return _W_INT(x) & @as(c_int, 0o177);
}
pub const _WSTOPPED = @as(c_int, 0o177);
pub inline fn WEXITSTATUS(x: anytype) @TypeOf((_W_INT(x) >> @as(c_int, 8)) & @as(c_int, 0x000000ff)) {
    _ = &x;
    return (_W_INT(x) >> @as(c_int, 8)) & @as(c_int, 0x000000ff);
}
pub inline fn WSTOPSIG(x: anytype) @TypeOf(_W_INT(x) >> @as(c_int, 8)) {
    _ = &x;
    return _W_INT(x) >> @as(c_int, 8);
}
pub inline fn WIFCONTINUED(x: anytype) @TypeOf((_WSTATUS(x) == _WSTOPPED) and (WSTOPSIG(x) == @as(c_int, 0x13))) {
    _ = &x;
    return (_WSTATUS(x) == _WSTOPPED) and (WSTOPSIG(x) == @as(c_int, 0x13));
}
pub inline fn WIFSTOPPED(x: anytype) @TypeOf((_WSTATUS(x) == _WSTOPPED) and (WSTOPSIG(x) != @as(c_int, 0x13))) {
    _ = &x;
    return (_WSTATUS(x) == _WSTOPPED) and (WSTOPSIG(x) != @as(c_int, 0x13));
}
pub inline fn WIFEXITED(x: anytype) @TypeOf(_WSTATUS(x) == @as(c_int, 0)) {
    _ = &x;
    return _WSTATUS(x) == @as(c_int, 0);
}
pub inline fn WIFSIGNALED(x: anytype) @TypeOf((_WSTATUS(x) != _WSTOPPED) and (_WSTATUS(x) != @as(c_int, 0))) {
    _ = &x;
    return (_WSTATUS(x) != _WSTOPPED) and (_WSTATUS(x) != @as(c_int, 0));
}
pub inline fn WTERMSIG(x: anytype) @TypeOf(_WSTATUS(x)) {
    _ = &x;
    return _WSTATUS(x);
}
pub inline fn WCOREDUMP(x: anytype) @TypeOf(_W_INT(x) & WCOREFLAG) {
    _ = &x;
    return _W_INT(x) & WCOREFLAG;
}
pub inline fn W_EXITCODE(ret: anytype, sig: anytype) @TypeOf((ret << @as(c_int, 8)) | sig) {
    _ = &ret;
    _ = &sig;
    return (ret << @as(c_int, 8)) | sig;
}
pub inline fn W_STOPCODE(sig: anytype) @TypeOf((sig << @as(c_int, 8)) | _WSTOPPED) {
    _ = &sig;
    return (sig << @as(c_int, 8)) | _WSTOPPED;
}
pub const WEXITED = @as(c_int, 0x00000004);
pub const WSTOPPED = @as(c_int, 0x00000008);
pub const WCONTINUED = @as(c_int, 0x00000010);
pub const WNOWAIT = @as(c_int, 0x00000020);
pub const WAIT_ANY = -@as(c_int, 1);
pub const WAIT_MYPGRP = @as(c_int, 0);
pub const _BSD_MACHINE_ENDIAN_H_ = "";
pub const _ARM__ENDIAN_H_ = "";
pub const _QUAD_HIGHWORD = @as(c_int, 1);
pub const _QUAD_LOWWORD = @as(c_int, 0);
pub const _SYS__ENDIAN_H_ = "";
pub const _BSD_MACHINE__ENDIAN_H_ = "";
pub const _ARM___ENDIAN_H_ = "";
pub const _SYS___ENDIAN_H_ = "";
pub const __DARWIN_LITTLE_ENDIAN = @as(c_int, 1234);
pub const __DARWIN_BIG_ENDIAN = @as(c_int, 4321);
pub const __DARWIN_PDP_ENDIAN = @as(c_int, 3412);
pub const LITTLE_ENDIAN = __DARWIN_LITTLE_ENDIAN;
pub const BIG_ENDIAN = __DARWIN_BIG_ENDIAN;
pub const PDP_ENDIAN = __DARWIN_PDP_ENDIAN;
pub const __DARWIN_BYTE_ORDER = __DARWIN_LITTLE_ENDIAN;
pub const BYTE_ORDER = __DARWIN_BYTE_ORDER;
pub const _OS__OSBYTEORDER_H = "";
pub inline fn __DARWIN_OSSwapConstInt16(x: anytype) __uint16_t {
    _ = &x;
    return __helpers.cast(__uint16_t, ((__helpers.cast(__uint16_t, x) & @as(c_uint, 0xff00)) >> @as(c_int, 8)) | ((__helpers.cast(__uint16_t, x) & @as(c_uint, 0x00ff)) << @as(c_int, 8)));
}
pub inline fn __DARWIN_OSSwapConstInt32(x: anytype) __uint32_t {
    _ = &x;
    return __helpers.cast(__uint32_t, ((((__helpers.cast(__uint32_t, x) & __helpers.promoteIntLiteral(c_uint, 0xff000000, .hex)) >> @as(c_int, 24)) | ((__helpers.cast(__uint32_t, x) & __helpers.promoteIntLiteral(c_uint, 0x00ff0000, .hex)) >> @as(c_int, 8))) | ((__helpers.cast(__uint32_t, x) & @as(c_uint, 0x0000ff00)) << @as(c_int, 8))) | ((__helpers.cast(__uint32_t, x) & @as(c_uint, 0x000000ff)) << @as(c_int, 24)));
}
pub inline fn __DARWIN_OSSwapConstInt64(x: anytype) __uint64_t {
    _ = &x;
    return __helpers.cast(__uint64_t, ((((((((__helpers.cast(__uint64_t, x) & @as(c_ulonglong, 0xff00000000000000)) >> @as(c_int, 56)) | ((__helpers.cast(__uint64_t, x) & @as(c_ulonglong, 0x00ff000000000000)) >> @as(c_int, 40))) | ((__helpers.cast(__uint64_t, x) & @as(c_ulonglong, 0x0000ff0000000000)) >> @as(c_int, 24))) | ((__helpers.cast(__uint64_t, x) & @as(c_ulonglong, 0x000000ff00000000)) >> @as(c_int, 8))) | ((__helpers.cast(__uint64_t, x) & @as(c_ulonglong, 0x00000000ff000000)) << @as(c_int, 8))) | ((__helpers.cast(__uint64_t, x) & @as(c_ulonglong, 0x0000000000ff0000)) << @as(c_int, 24))) | ((__helpers.cast(__uint64_t, x) & @as(c_ulonglong, 0x000000000000ff00)) << @as(c_int, 40))) | ((__helpers.cast(__uint64_t, x) & @as(c_ulonglong, 0x00000000000000ff)) << @as(c_int, 56)));
}
pub const _OS__OSBYTEORDERARM_H = "";
pub const __DARWIN_OS_INLINE = @compileError("unable to translate C expr: unexpected token 'static'"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/libkern/arm/_OSByteOrder.h:38:17
pub inline fn __DARWIN_OSSwapInt16(x: anytype) __uint16_t {
    _ = &x;
    return __helpers.cast(__uint16_t, if (__helpers.cast(bool, __builtin.constant_p(x))) __DARWIN_OSSwapConstInt16(x) else _OSSwapInt16(x));
}
pub inline fn __DARWIN_OSSwapInt32(x: anytype) @TypeOf(if (__helpers.cast(bool, __builtin.constant_p(x))) __DARWIN_OSSwapConstInt32(x) else _OSSwapInt32(x)) {
    _ = &x;
    return if (__helpers.cast(bool, __builtin.constant_p(x))) __DARWIN_OSSwapConstInt32(x) else _OSSwapInt32(x);
}
pub inline fn __DARWIN_OSSwapInt64(x: anytype) @TypeOf(if (__helpers.cast(bool, __builtin.constant_p(x))) __DARWIN_OSSwapConstInt64(x) else _OSSwapInt64(x)) {
    _ = &x;
    return if (__helpers.cast(bool, __builtin.constant_p(x))) __DARWIN_OSSwapConstInt64(x) else _OSSwapInt64(x);
}
pub inline fn ntohs(x: anytype) @TypeOf(__DARWIN_OSSwapInt16(x)) {
    _ = &x;
    return __DARWIN_OSSwapInt16(x);
}
pub inline fn htons(x: anytype) @TypeOf(__DARWIN_OSSwapInt16(x)) {
    _ = &x;
    return __DARWIN_OSSwapInt16(x);
}
pub inline fn ntohl(x: anytype) @TypeOf(__DARWIN_OSSwapInt32(x)) {
    _ = &x;
    return __DARWIN_OSSwapInt32(x);
}
pub inline fn htonl(x: anytype) @TypeOf(__DARWIN_OSSwapInt32(x)) {
    _ = &x;
    return __DARWIN_OSSwapInt32(x);
}
pub inline fn ntohll(x: anytype) @TypeOf(__DARWIN_OSSwapInt64(x)) {
    _ = &x;
    return __DARWIN_OSSwapInt64(x);
}
pub inline fn htonll(x: anytype) @TypeOf(__DARWIN_OSSwapInt64(x)) {
    _ = &x;
    return __DARWIN_OSSwapInt64(x);
}
pub const NTOHL = @compileError("unable to translate C expr: unexpected token '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/_endian.h:144:9
pub const NTOHS = @compileError("unable to translate C expr: unexpected token '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/_endian.h:145:9
pub const NTOHLL = @compileError("unable to translate C expr: unexpected token '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/_endian.h:146:9
pub const HTONL = @compileError("unable to translate C expr: unexpected token '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/_endian.h:147:9
pub const HTONS = @compileError("unable to translate C expr: unexpected token '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/_endian.h:148:9
pub const HTONLL = @compileError("unable to translate C expr: unexpected token '='"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/_endian.h:149:9
pub const w_termsig = @compileError("unable to translate macro: undefined identifier `w_T`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/wait.h:229:9
pub const w_coredump = @compileError("unable to translate macro: undefined identifier `w_T`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/wait.h:230:9
pub const w_retcode = @compileError("unable to translate macro: undefined identifier `w_T`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/wait.h:231:9
pub const w_stopval = @compileError("unable to translate macro: undefined identifier `w_S`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/wait.h:232:9
pub const w_stopsig = @compileError("unable to translate macro: undefined identifier `w_S`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/sys/wait.h:233:9
pub const _ALLOCA_H_ = "";
pub const __alloca = @compileError("unable to translate macro: undefined identifier `__builtin_alloca`"); // /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/alloca.h:43:9
pub const _CT_RUNE_T = "";
pub const _RUNE_T = "";
pub const _WCHAR_T = "";
pub const NULL = __DARWIN_NULL;
pub const EXIT_FAILURE = @as(c_int, 1);
pub const EXIT_SUCCESS = @as(c_int, 0);
pub const RAND_MAX = __helpers.promoteIntLiteral(c_int, 0x7fffffff, .hex);
// /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.4.sdk/usr/include/_stdlib.h:139:9: warning: macro 'MB_CUR_MAX' contains a runtime value, translated to function
pub inline fn MB_CUR_MAX() @TypeOf(__mb_cur_max) {
    return __mb_cur_max;
}
pub const _MALLOC_UNDERSCORE_MALLOC_H_ = "";
pub const _MALLOC_UNDERSCORE_MALLOC_TYPE_H_ = "";
pub const _MALLOC_UNDERSCORE_PTRCHECK_H_ = "";
pub const _MALLOC_TYPE_MALLOC_BACKDEPLOY_PUBLIC = @as(c_int, 1);
pub inline fn _MALLOC_TYPED(override: anytype, type_param_pos: anytype) void {
    _ = &override;
    _ = &type_param_pos;
    return;
}
pub const __ABORT_H_ = "";
pub const _DEV_T = "";
pub const _MODE_T = "";
pub const __darwin_pthread_handler_rec = struct___darwin_pthread_handler_rec;
pub const _opaque_pthread_attr_t = struct__opaque_pthread_attr_t;
pub const _opaque_pthread_cond_t = struct__opaque_pthread_cond_t;
pub const _opaque_pthread_condattr_t = struct__opaque_pthread_condattr_t;
pub const _opaque_pthread_mutex_t = struct__opaque_pthread_mutex_t;
pub const _opaque_pthread_mutexattr_t = struct__opaque_pthread_mutexattr_t;
pub const _opaque_pthread_once_t = struct__opaque_pthread_once_t;
pub const _opaque_pthread_rwlock_t = struct__opaque_pthread_rwlock_t;
pub const _opaque_pthread_rwlockattr_t = struct__opaque_pthread_rwlockattr_t;
pub const _opaque_pthread_t = struct__opaque_pthread_t;
pub const __darwin_arm_exception_state = struct___darwin_arm_exception_state;
pub const __darwin_arm_exception_state64 = struct___darwin_arm_exception_state64;
pub const __darwin_arm_exception_state64_v2 = struct___darwin_arm_exception_state64_v2;
pub const __darwin_arm_thread_state = struct___darwin_arm_thread_state;
pub const __darwin_arm_thread_state64 = struct___darwin_arm_thread_state64;
pub const __darwin_arm_vfp_state = struct___darwin_arm_vfp_state;
pub const __darwin_arm_neon_state64 = struct___darwin_arm_neon_state64;
pub const __darwin_arm_neon_state = struct___darwin_arm_neon_state;
pub const __arm_pagein_state = struct___arm_pagein_state;
pub const __darwin_arm_sme_state = struct___darwin_arm_sme_state;
pub const __darwin_arm_sve_z_state = struct___darwin_arm_sve_z_state;
pub const __darwin_arm_sve_p_state = struct___darwin_arm_sve_p_state;
pub const __darwin_arm_sme_za_state = struct___darwin_arm_sme_za_state;
pub const __darwin_arm_sme2_state = struct___darwin_arm_sme2_state;
pub const __arm_legacy_debug_state = struct___arm_legacy_debug_state;
pub const __darwin_arm_debug_state32 = struct___darwin_arm_debug_state32;
pub const __darwin_arm_debug_state64 = struct___darwin_arm_debug_state64;
pub const __darwin_arm_cpmu_state64 = struct___darwin_arm_cpmu_state64;
pub const __darwin_mcontext32 = struct___darwin_mcontext32;
pub const __darwin_mcontext64 = struct___darwin_mcontext64;
pub const __darwin_sigaltstack = struct___darwin_sigaltstack;
pub const __darwin_ucontext = struct___darwin_ucontext;
pub const sigval = union_sigval;
pub const sigevent = struct_sigevent;
pub const __siginfo = struct___siginfo;
pub const __sigaction_u = union___sigaction_u;
pub const __sigaction = struct___sigaction;
pub const sigaction = struct_sigaction;
pub const sigvec = struct_sigvec;
pub const sigstack = struct_sigstack;
pub const timeval = struct_timeval;
pub const rusage = struct_rusage;
pub const rusage_info_v0 = struct_rusage_info_v0;
pub const rusage_info_v1 = struct_rusage_info_v1;
pub const rusage_info_v2 = struct_rusage_info_v2;
pub const rusage_info_v3 = struct_rusage_info_v3;
pub const rusage_info_v4 = struct_rusage_info_v4;
pub const rusage_info_v5 = struct_rusage_info_v5;
pub const rusage_info_v6 = struct_rusage_info_v6;
pub const rlimit = struct_rlimit;
pub const proc_rlimit_control_wakeupmon = struct_proc_rlimit_control_wakeupmon;
pub const _malloc_zone_t = struct__malloc_zone_t;
