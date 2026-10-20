#[macro_export]
macro_rules! read_lock {
    ($rw_lock:expr) => {
        $rw_lock.read().unwrap()
    };
}
#[macro_export]
macro_rules! write_lock {
    ($rw_lock:expr) => {
        $rw_lock.write().unwrap()
    };
}
/// Script failure diagnostics, only emitted when the server runs with `--debug-log`.
#[macro_export]
macro_rules! script_debug {
    ($($argument:tt)+) => {
        tracing::debug!(target: "script_debug", $($argument)+)
    };
}
#[macro_export]
macro_rules! mutex_lock {
    ($mutex:expr) => {
        $mutex.lock().unwrap()
    };
}

#[macro_export]
macro_rules! cast {
    ($packet:expr, $type:ty) => {
        $packet.as_any().downcast_ref::<$type>().unwrap()
    };
}

#[macro_export]
macro_rules! socket_send {
    ($context:expr, $packet:expr) => {
        $context.socket().send(std::mem::take($packet.raw_mut()));
    };
}
#[macro_export]
macro_rules! socket_send_raw {
    ($context:expr, $data:expr) => {
        $context.socket().send($data);
    };
}

#[macro_export]
macro_rules! elapsed {
    ($instant:expr) => {
        info!("{} ms", $instant.elapsed().as_nanos() as f64 / 1000.0 / 1000.0);
    };
}
