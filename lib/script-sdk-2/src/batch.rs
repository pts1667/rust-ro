//! Requests sent together, so a script driving many units waits for the game loop once instead of once per request.

use script_sdk::{Function, MAX_BATCH_REQUESTS, MAX_MESSAGE_BYTES, Request, Value};

use crate::ctx::Ctx;
use crate::flow::Stop;
use crate::value::Val;

/// Room kept in each message for the batch's own encoding.
const MESSAGE_MARGIN: usize = 1_024;

/// Built by [`Ctx::batch`]. Requests that wait (`Sleep`, dialogue, unit data) are refused by the host.
pub struct Batch<'c, 'a> {
    ctx: &'c Ctx<'a>,
    requests: Vec<Request>,
}

impl<'c, 'a> Batch<'c, 'a> {
    pub(crate) fn new(ctx: &'c Ctx<'a>) -> Self {
        Self { ctx, requests: Vec::new() }
    }

    pub fn call(&mut self, function: Function, arguments: Vec<Val>) -> &mut Self {
        self.request(Request::Call { function, arguments: arguments.into_iter().map(Value::from).collect() })
    }

    pub fn request(&mut self, request: Request) -> &mut Self {
        self.requests.push(request);
        self
    }

    pub fn len(&self) -> usize {
        self.requests.len()
    }

    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    /// Sends the requests, as several batches when they do not fit in one message, and returns their replies in order.
    pub fn send(self) -> Result<Vec<Val>, Stop> {
        let mut replies = Vec::with_capacity(self.requests.len());
        let mut chunk = Vec::new();
        let mut size = 0;
        for request in self.requests {
            let length = serde_json::to_vec(&request).map_err(|error| Stop::from(error.to_string()))?.len() + 1;
            if !chunk.is_empty() && (chunk.len() == MAX_BATCH_REQUESTS || size + length > MAX_MESSAGE_BYTES - MESSAGE_MARGIN) {
                replies.extend(Self::send_chunk(self.ctx, std::mem::take(&mut chunk))?);
                size = 0;
            }
            size += length;
            chunk.push(request);
        }
        if !chunk.is_empty() {
            replies.extend(Self::send_chunk(self.ctx, chunk)?);
        }
        Ok(replies)
    }

    fn send_chunk(ctx: &Ctx, chunk: Vec<Request>) -> Result<Vec<Val>, Stop> {
        let count = chunk.len();
        let replies = ctx.request(Request::Batch(chunk))?.into_array().ok_or_else(|| Stop::from("Batch reply is not an array"))?;
        if replies.len() != count {
            return Err(Stop::from("Batch reply does not answer every request"));
        }
        Ok(replies)
    }
}

impl<'a> Ctx<'a> {
    /// Collects requests to send in one go with [`Batch::send`].
    pub fn batch(&self) -> Batch<'_, 'a> {
        Batch::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::MockTransport;

    #[test]
    fn a_batch_is_one_request_answered_in_order() {
        let transport = MockTransport::new(|request| match request {
            Request::Call { function: Function::GetMapUsers, arguments } => Ok(Value::Number(arguments[0].text().len() as i32)),
            _ => Ok(Value::default()),
        });
        let ctx = Ctx::new(&transport);
        let mut batch = ctx.batch();
        batch.call(Function::GetMapUsers, vec!["payon".into()]).call(Function::Emotion, vec![1.into(), 7.into()]).call(Function::GetMapUsers, vec!["prontera".into()]);
        assert_eq!(batch.send().unwrap(), vec![Val::from(5), Val::from(0), Val::from(8)]);
        assert_eq!(transport.batches(), 1);
        assert_eq!(transport.calls(Function::Emotion), [vec![Value::Number(1), Value::Number(7)]], "the inner requests are recorded");
    }

    #[test]
    fn a_long_batch_is_split_to_fit_the_limits() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        let mut batch = ctx.batch();
        for index in 0..MAX_BATCH_REQUESTS + 10 {
            batch.call(Function::Emotion, vec![1.into(), (index as i32).into()]);
        }
        assert_eq!(batch.send().unwrap().len(), MAX_BATCH_REQUESTS + 10);
        assert_eq!(transport.batches(), 2);

        let mut batch = ctx.batch();
        for _ in 0..20 {
            batch.call(Function::NpcTalk, vec!["x".repeat(4_000).into()]);
        }
        batch.send().unwrap();
        assert_eq!(transport.batches(), 4, "80 KB of text needs two more messages");
    }
}
