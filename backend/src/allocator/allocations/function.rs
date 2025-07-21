use super::*;
use crate::asm::*;
use common::*;

impl<'scp, 'ctx> AllocatorAllocations<'scp, 'ctx> {
    pub fn allocate_function(&mut self, function: &mut AsmFunction) {
        <Self as Factory<(), Self, AsmFunction>>::run(self, function)
    }
}

impl<'scp, 'ctx> Factory<(), Self, AsmFunction> for AllocatorAllocations<'scp, 'ctx> {
    fn run(driver: &mut Self, function: &mut AsmFunction) {
        for instr in function.instructions_mut() {
            driver.allocate_instruction(instr);
        }
    }
}