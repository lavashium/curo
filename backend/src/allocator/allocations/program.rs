use super::*;
use crate::asm::*;
use common::*;

impl<'scp, 'ctx> AllocatorAllocations<'scp, 'ctx> {
    pub fn allocate_program(&mut self, program: &mut AsmProgram) -> Vec<i32> {
        <Self as Factory<Vec<i32>, Self, AsmProgram>>::run(self, program)
    }
}

impl<'scp, 'ctx> Factory<Vec<i32>, Self, AsmProgram> for AllocatorAllocations<'scp, 'ctx> {
    fn run(driver: &mut Self, program: &mut AsmProgram) -> Vec<i32> {
        let mut stack_offsets = vec![];
        for function in program.function_definitions_mut() {
            driver.allocate_function(function);
            stack_offsets.push(-driver.ctx.next_offset);
        }
        stack_offsets
    }

}