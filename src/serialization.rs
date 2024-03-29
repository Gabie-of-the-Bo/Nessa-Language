use std::{fs, path::Path};

use crate::{types::TypeTemplate, interfaces::InterfaceImpl, compilation::{CompiledNessaExpr, NessaInstruction, NessaError}, context::{NessaContext, standard_ctx, NUM_STD_TYPES, NUM_STD_INT_IMPL}, execution::ExecutionInfo};

use serde::{Serialize, Deserialize};
use bitcode;

#[derive(Clone, Serialize, Deserialize)]
pub struct CompiledNessaModule {
    pub hash: String,
    type_templates: Vec<TypeTemplate>,
    interface_impls: Vec<InterfaceImpl>,
    instructions: Vec<CompiledNessaExpr>
}

impl NessaContext {
    pub fn get_serializable_module(&self, hash: String, instructions: &[NessaInstruction]) -> CompiledNessaModule {
        return CompiledNessaModule {
            hash, 
            type_templates: self.type_templates[NUM_STD_TYPES..].to_vec(), 
            interface_impls: self.interface_impls[NUM_STD_INT_IMPL..].to_vec(), 
            instructions: instructions.iter().map(|i| i.instruction.clone()).collect()
        };
    }
}

impl CompiledNessaModule {
    pub fn deserialize(data: &[u8]) -> Self {
        bitcode::deserialize(data).expect("Unable to deserialize code")
    }

    pub fn serialize(&self) -> Vec<u8> {
        bitcode::serialize(self).expect("Unable to serialize code")
    }

    pub fn from_file(path: &Path) -> Self {
        let data = fs::read(path).expect("Unable to read serialized code from file");
        CompiledNessaModule::deserialize(&data)
    }

    pub fn write_to_file(&self, path: &Path) {
        fs::write(path, self.serialize()).expect("Unable to write serialized code to file");
    }

    pub fn execute<const DEBUG: bool>(&mut self, program_input: &[String]) -> Result<ExecutionInfo, NessaError> {
        let mut ctx = standard_ctx();

        ctx.type_templates.append(&mut self.type_templates);
        ctx.interface_impls.append(&mut self.interface_impls);

        ctx.program_input = program_input.to_vec();

        ctx.execute_compiled_code::<DEBUG>(&self.instructions, &[])
    }
}