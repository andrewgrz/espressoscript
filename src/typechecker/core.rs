use crate::error::CompileError;

type ID = usize;

#[derive(Copy, Clone, Debug)]
pub struct Value(ID);
#[derive(Copy, Clone, Debug)]
pub struct Use(ID);


pub struct TypeCheckerCore;

impl TypeCheckerCore {
    pub fn var(&mut self) -> (Value, Use) {
        unimplemented!()
    }

    pub fn bool(&mut self) -> Value {
        unimplemented!()
    }
    pub fn bool_use(&mut self) -> Use {
        unimplemented!()
    }

    pub fn integer(&mut self) -> Value {
        unimplemented!()
    }
    pub fn integer_use(&mut self) -> Use {
        unimplemented!()
    }

    pub fn func(&mut self, args: Vec<Use>, ret: Value) -> Value {
        unimplemented!()
    }
    pub fn func_use(&mut self, args: Vec<Value>, ret: Use) -> Use {
        unimplemented!()
    }

    pub fn flow(&mut self, lhs: Value, rhs: Use) -> Result<(), CompileError> {
        unimplemented!()
    }

    pub fn new() -> TypeCheckerCore {
        TypeCheckerCore
    }
}
