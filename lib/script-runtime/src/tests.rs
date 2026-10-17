use std::time::Duration;

use async_trait::async_trait;
use script_sdk::{Capability, Entry, Function, Interception, Reply, Request, Value, Verdict};

use super::*;

struct NoHost;

#[async_trait]
impl Host for NoHost {
    async fn invoke(&mut self, _request: Request) -> Reply {
        Ok(Value::default())
    }
}

fn module(wat: &str, cpu_ms: u64) -> WasmRuntime {
    WasmRuntime::from_bytes(wat.as_bytes(), Limits { cpu_ms, ..Limits::default() }).unwrap()
}

const SPIN: &str = r#"(module
    (func (export "script_abi") (result i32) i32.const 2)
    (func (export "script_run") (result i32) (loop $forever br $forever) i32.const 0))"#;

#[tokio::test(flavor = "current_thread")]
async fn a_guest_that_never_calls_the_host_traps_after_its_cpu_budget() {
    let started = std::time::Instant::now();
    let (_, result) = module(SPIN, 50).execute_named(NoHost, &Entry::Npc("spin".into())).await;
    let error = result.unwrap_err();
    assert!(error.contains("CPU budget of 50 ms"), "{error}");
    assert!(started.elapsed() < Duration::from_secs(5), "took {:?}", started.elapsed());
}

#[tokio::test(flavor = "current_thread")]
async fn a_long_run_yields_its_thread_to_other_tasks() {
    let timer = tokio::spawn(async { tokio::time::sleep(Duration::from_millis(10)).await; std::time::Instant::now() });
    let runtime = module(SPIN, 300);
    let (_, result) = runtime.execute_named(NoHost, &Entry::Npc("spin".into())).await;
    let ended = std::time::Instant::now();
    assert!(result.is_err());
    let fired = timer.await.unwrap();
    assert!(fired + Duration::from_millis(100) < ended, "the timer only ran once the guest had trapped");
}

fn policy(capabilities: &[Capability]) -> PluginPolicy {
    PluginPolicy { name: "p".into(), capabilities: capabilities.iter().copied().collect(), config: r#"{"multiplier":2}"#.into() }
}

#[test]
fn plugins_are_refused_what_they_did_not_declare_and_answered_their_config_locally() {
    let warp = Request::Call { function: Function::Warp, arguments: vec![] };
    let config = Request::Call { function: Function::PluginConfig, arguments: vec![] };
    let refused = WasmRuntime::admit(Some(&policy(&[Capability::Rates])), warp.clone()).unwrap_err().unwrap_err();
    assert!(refused.contains("World"), "{refused}");
    assert_eq!(WasmRuntime::admit(Some(&policy(&[Capability::World])), warp.clone()), Ok(warp.clone()));
    assert_eq!(WasmRuntime::admit(None, warp.clone()), Ok(warp), "base modules hold every capability");
    assert_eq!(WasmRuntime::admit(Some(&policy(&[])), config.clone()), Err(Ok(Value::String(r#"{"multiplier":2}"#.into()))));
    assert!(WasmRuntime::admit(None, config).unwrap_err().is_err());
    assert_eq!(WasmRuntime::admit(Some(&policy(&[])), Request::Read("$x".into())), Ok(Request::Read("$p::x".into())));
}

/// A guest whose interceptor answers what its data segment holds at offset 0, after spinning `spin` times through `$work`.
fn interceptor(body: &str) -> String {
    format!(
        r#"(module
            (import "rust_ro" "invoke" (func $invoke (param i32 i32 i32 i32) (result i32)))
            (memory (export "memory") 1)
            (data (i32.const 1024) "\"Allow\"")
            (func (export "plugin_buffer") (result i32) i32.const 0)
            (func (export "plugin_intercept") (param $length i32) (result i32) {body}))"#
    )
}

fn exp_gain() -> Interception {
    Interception::ExpGain { char_id: 1, mob_id: 1002, base: 10, job: 5 }
}

#[test]
fn an_interceptor_answer_is_decoded() {
    let wat = interceptor("(memory.copy (i32.const 0) (i32.const 1024) (i32.const 7)) i32.const 7");
    let runtime = module(&wat, 50);
    let mut sandbox = PluginSandbox::new(&runtime);
    assert_eq!(sandbox.intercept(&exp_gain(), Duration::from_millis(50)), Ok(Verdict::Allow));
    assert_eq!(sandbox.intercept(&exp_gain(), Duration::from_millis(50)), Ok(Verdict::Allow), "the instance stays warm");
}

#[test]
fn an_interceptor_that_loops_is_stopped_by_the_epoch() {
    let runtime = module(&interceptor("(loop $forever br $forever) i32.const 0"), 50);
    let mut sandbox = PluginSandbox::new(&runtime);
    let started = std::time::Instant::now();
    let error = sandbox.intercept(&exp_gain(), Duration::from_micros(200)).unwrap_err();
    assert!(started.elapsed() < Duration::from_millis(500), "took {:?}: {error}", started.elapsed());
}

#[test]
fn an_interceptor_cannot_call_the_host() {
    let runtime = module(&interceptor("(call $invoke (i32.const 0) (i32.const 0) (i32.const 0) (i32.const 0))"), 50);
    let mut sandbox = PluginSandbox::new(&runtime);
    assert!(sandbox.intercept(&exp_gain(), Duration::from_millis(50)).is_err());
}
