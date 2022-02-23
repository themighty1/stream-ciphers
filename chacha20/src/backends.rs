use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(chacha20_force_soft______)] {
        pub(crate) mod soft;
    } else {
        pub(crate) mod sse2;
    }
}
