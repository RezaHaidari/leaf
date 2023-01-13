use jni::{
    objects::{JClass, JString},
    JNIEnv,
};

/// No error.
pub const ERR_OK: i32 = 0;
/// Config path error.
pub const ERR_CONFIG_PATH: i32 = 1;
/// Config parsing error.
pub const ERR_CONFIG: i32 = 2;
/// IO error.
pub const ERR_IO: i32 = 3;
/// Config file watcher error.
pub const ERR_WATCHER: i32 = 4;
/// Async channel send error.
pub const ERR_ASYNC_CHANNEL_SEND: i32 = 5;
/// Sync channel receive error.
pub const ERR_SYNC_CHANNEL_RECV: i32 = 6;
/// Runtime manager error.
pub const ERR_RUNTIME_MANAGER: i32 = 7;
/// No associated config file.
pub const ERR_NO_CONFIG_FILE: i32 = 8;

fn to_errno(e: leaf::Error) -> i32 {
    match e {
        leaf::Error::Config(..) => ERR_CONFIG,
        leaf::Error::NoConfigFile => ERR_NO_CONFIG_FILE,
        leaf::Error::Io(..) => ERR_IO,
        #[cfg(feature = "auto-reload")]
        leaf::Error::Watcher(..) => ERR_WATCHER,
        leaf::Error::AsyncChannelSend(..) => ERR_ASYNC_CHANNEL_SEND,
        leaf::Error::SyncChannelRecv(..) => ERR_SYNC_CHANNEL_RECV,
        leaf::Error::RuntimeManager => ERR_RUNTIME_MANAGER,
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_sail_1tunnel_sail_services_TunnelInstance_leafRun(
    env: JNIEnv,
    _: JClass,
    rt_id: u16,
    config_path: JString,
) -> i32 {
    let config_path = env
        .get_string(config_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    if !config_path.is_empty() {
        let opts = leaf::StartOptions {
            config: leaf::Config::File(config_path),
            #[cfg(feature = "auto-reload")]
            auto_reload: false,
            runtime_opt: leaf::RuntimeOption::SingleThread,
        };
        if let Err(e) = leaf::start(rt_id, opts) {
            return to_errno(e);
        }
        ERR_OK
    } else {
        ERR_CONFIG_PATH
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_sail_1tunnel_sail_services_TunnelInstance_leafReload(
    _: JNIEnv,
    _: JClass,
    rt_id: u16,
) -> i32 {
    if let Err(e) = leaf::reload(rt_id) {
        return to_errno(e);
    }
    ERR_OK
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_sail_1tunnel_sail_services_TunnelInstance_leafShutdown(
    _: JNIEnv,
    _: JClass,
    rt_id: u16,
) {
    leaf::shutdown(rt_id);
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_com_sail_1tunnel_sail_services_TunnelInstance_leafTestConfig(
    env: JNIEnv,
    _: JClass,
    config_path: JString,
) -> i32 {
    let config_path = env
        .get_string(config_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    if !config_path.is_empty() {
        if let Err(e) = leaf::test_config(&config_path) {
            return to_errno(e);
        }
        ERR_OK
    } else {
        ERR_CONFIG_PATH
    }
}
