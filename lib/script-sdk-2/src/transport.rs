use std::cell::RefCell;

use script_sdk::{Context, Function, Reply, Request, Value};

/// Where a script's requests go. The host implements the real one; tests use [`MockTransport`].
pub trait Transport {
    fn request(&self, request: Request) -> Reply;
}

/// Sends requests to the game host through the `rust_ro::invoke` import.
pub struct WasmTransport;

impl Transport for WasmTransport {
    fn request(&self, request: Request) -> Reply {
        Context.request(request)
    }
}

/// Answers requests with a closure and records them, so scripts run without a host. The requests of a batch are answered
/// and recorded one by one.
pub struct MockTransport {
    handler: Box<dyn Fn(&Request) -> Reply>,
    requests: RefCell<Vec<Request>>,
    batches: RefCell<usize>,
}

impl MockTransport {
    pub fn new(handler: impl Fn(&Request) -> Reply + 'static) -> Self {
        Self {
            handler: Box::new(handler),
            requests: RefCell::new(Vec::new()),
            batches: RefCell::new(0),
        }
    }

    /// Answers every request with the default value, `0`.
    pub fn silent() -> Self {
        Self::new(|_| Ok(Value::default()))
    }

    /// How many batches the script sent.
    pub fn batches(&self) -> usize {
        *self.batches.borrow()
    }

    pub fn requests(&self) -> Vec<Request> {
        self.requests.borrow().clone()
    }

    /// The argument lists of every call to `function`, in the order they were made.
    pub fn calls(&self, function: Function) -> Vec<Vec<Value>> {
        self.requests
            .borrow()
            .iter()
            .filter_map(|request| match request {
                Request::Call {
                    function: called,
                    arguments,
                } if *called == function => Some(arguments.clone()),
                _ => None,
            })
            .collect()
    }
}

impl Transport for MockTransport {
    fn request(&self, request: Request) -> Reply {
        if let Request::Batch(requests) = request {
            *self.batches.borrow_mut() += 1;
            return requests.into_iter().map(|request| self.request(request)).collect::<Result<Vec<_>, _>>().map(Value::Array);
        }
        let reply = (self.handler)(&request);
        self.requests.borrow_mut().push(request);
        reply
    }
}
