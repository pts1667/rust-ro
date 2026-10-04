use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use async_trait::async_trait;
use script_runtime::{Host, Limits, WasmRuntime};
use script_sdk::{Reply, Request, Value};

struct CountingHost {
    count: Arc<AtomicUsize>,
}

#[async_trait]
impl Host for CountingHost {
    async fn invoke(&mut self, _request: Request) -> Reply {
        self.count.fetch_add(1, Ordering::Relaxed);
        Ok(Value::default())
    }
}

fn module(body: &str, abi: u32) -> Vec<u8> {
    format!(
        r#"(module
        (import "rust_ro" "invoke" (func $invoke (param i32 i32 i32 i32) (result i32)))
        (memory (export "memory") 1)
        (data (i32.const 0) "\22Arguments\22")
        (func (export "script_abi") (result i32) i32.const {abi})
        (func (export "run_npc") (param i32) (result i32) {body})
    )"#
    )
    .into_bytes()
}

async fn execute(bytes: Vec<u8>, limits: Limits) -> (usize, Result<(), String>) {
    let runtime = WasmRuntime::from_bytes(&bytes, limits).unwrap();
    let count = Arc::new(AtomicUsize::new(0));
    let (_, result) = runtime.execute(CountingHost { count: count.clone() }, "run_npc", 0).await;
    (count.load(Ordering::Relaxed), result)
}

#[tokio::test]
async fn links_typed_host_requests() {
    let (count, result) = execute(
        module(
            "i32.const 0 i32.const 11 i32.const 64 i32.const 256 call $invoke drop i32.const 0",
            1,
        ),
        Limits::default(),
    )
    .await;
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(count, 1);
}

#[tokio::test]
async fn invalid_response_memory_never_calls_host() {
    let (count, result) = execute(
        module(
            "i32.const 0 i32.const 11 i32.const 65530 i32.const 256 call $invoke drop i32.const 0",
            1,
        ),
        Limits::default(),
    )
    .await;
    assert!(result.unwrap_err().contains("Invalid response buffer"));
    assert_eq!(count, 0);
}

#[tokio::test]
async fn rejects_wrong_abi_before_host_actions() {
    let (count, result) = execute(
        module(
            "i32.const 0 i32.const 11 i32.const 64 i32.const 256 call $invoke drop i32.const 0",
            9,
        ),
        Limits::default(),
    )
    .await;
    assert!(result.unwrap_err().contains("Unsupported script ABI"));
    assert_eq!(count, 0);
}

#[tokio::test]
async fn terminates_infinite_guest_loop() {
    let (_, result) = execute(module("(loop $forever br $forever) i32.const 0", 1), Limits {
        fuel: 1000,
        ..Limits::default()
    })
    .await;
    assert!(result.unwrap_err().contains("fuel"));
}

#[tokio::test]
async fn limits_host_call_count() {
    let body = "i32.const 0 i32.const 11 i32.const 64 i32.const 256 call $invoke drop i32.const 0 i32.const 11 i32.const 64 i32.const 256 \
                call $invoke drop i32.const 0";
    let (count, result) = execute(module(body, 1), Limits {
        host_calls: 1,
        ..Limits::default()
    })
    .await;
    assert!(result.unwrap_err().contains("host call budget"));
    assert_eq!(count, 1);
}

#[tokio::test]
async fn denies_guest_memory_growth_past_limit() {
    let (_, result) = execute(module("i32.const 1 memory.grow i32.const -1 i32.ne", 1), Limits {
        memory_bytes: 65_536,
        ..Limits::default()
    })
    .await;
    assert!(result.is_ok(), "{result:?}");
}

#[tokio::test]
async fn guest_start_function_cannot_call_game_api() {
    let bytes = String::from_utf8(module("i32.const 0", 1)).unwrap();
    let start = "(func $start i32.const 0 i32.const 11 i32.const 64 i32.const 256 call $invoke drop) (start $start)";
    let bytes = format!("{}{start})", bytes.trim_end().strip_suffix(')').unwrap()).into_bytes();
    let (count, result) = execute(bytes, Limits::default()).await;
    assert!(result.unwrap_err().contains("before ABI verification"));
    assert_eq!(count, 0);
}

#[tokio::test]
async fn asynchronous_host_call_suspends_without_blocking_executor() {
    struct WaitingHost(tokio::sync::oneshot::Receiver<()>);
    #[async_trait]
    impl Host for WaitingHost {
        async fn invoke(&mut self, _request: Request) -> Reply {
            (&mut self.0).await.map_err(|_| "Cancelled".to_string())?;
            Ok(Value::default())
        }
    }
    let runtime = WasmRuntime::from_bytes(
        &module(
            "i32.const 0 i32.const 11 i32.const 64 i32.const 256 call $invoke drop i32.const 0",
            1,
        ),
        Limits::default(),
    )
    .unwrap();
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move { runtime.execute(WaitingHost(receiver), "run_npc", 0).await.1 });
    tokio::task::yield_now().await;
    assert!(!task.is_finished());
    sender.send(()).unwrap();
    assert!(task.await.unwrap().is_ok());
}
