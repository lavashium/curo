use super::*;
use crate::asm::*;
use common::*;

impl<'scp, 'ctx> AllocatorAllocations<'scp, 'ctx> {
    pub fn allocate_program(&mut self, program: &mut AsmProgram) -> i32 {
        <Self as Factory<i32, Self, AsmProgram>>::run(self, program)
    }
}

impl<'scp, 'ctx> Factory<i32, Self, AsmProgram> for AllocatorAllocations<'scp, 'ctx> {
    fn run(driver: &mut Self, program: &mut AsmProgram) -> i32 {
        for function in program.function_definitions_mut() {
            driver.allocate_function(function);
        }
        -driver.ctx.next_offset
    }

}