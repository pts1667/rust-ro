use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Stop;
use crate::value::Val;

impl Ctx<'_> {
    /// The class mask (`constants::EAJ_*` and `EAJL_*` bits) of job `job`, or of the attached player's job when `None`,
    /// as rathena's `eaclass`.
    pub fn ea_class(&self, job: Option<i32>) -> Result<i32, Stop> {
        let mut arguments = args![];
        arguments.extend(job.map(Val::from));
        self.call(Function::EaClass, arguments)?.number()
    }

    /// The job of class mask `mask`, as rathena's `roclass`. `-1` when no job has that mask.
    pub fn ro_class(&self, mask: i32) -> Result<i32, Stop> {
        self.call(Function::RoClass, args![mask])?.number()
    }

    /// The display name of job `job`, such as `"Swordman"`.
    pub fn job_name(&self, job: i32) -> Result<String, Stop> {
        self.call(Function::JobName, args![job]).map(|value| value.text())
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn ea_class_names_the_job_only_when_given() {
        let transport = MockTransport::new(|_| Ok(Value::Number(1)));
        let ctx = Ctx::new(&transport);
        ctx.ea_class(None).unwrap();
        ctx.ea_class(Some(7)).unwrap();
        assert_eq!(transport.calls(Function::EaClass), vec![vec![], vec![Value::new_number(7)]]);
    }
}
