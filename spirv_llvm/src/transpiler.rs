use rspirv;
use rspirv::mr::*;
use spirv_headers::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use trans::*;
use TranspilerError;

const MAGIC_NUMBER: u32 = 0x07230203;

/// LLVM interface for Brainfuck programs
pub struct SpirvTranspiler<'a> {
    spirv_mod: &'a rspirv::mr::Module,

    ctx: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
}

impl<'a> SpirvTranspiler<'a> {
    pub fn new(spirv_mod: &rspirv::mr::Module) -> Result<SpirvTranspiler, TranspilerError> {
        let header = spirv_mod.header.as_ref().ok_or(TranspilerError::NoHeader)?;
        if header.magic_number != MAGIC_NUMBER {
            return Err(TranspilerError::InvalidMagicNumber);
        }
        // TODO check version, generator, capabilities, memorymodel, executionmode

        let (ctx, module, builder) = unsafe {
            let ctx = LLVMContextCreate();
            let module =
                LLVMModuleCreateWithNameInContext(b"spirv_llvm\0".as_ptr() as *const _, ctx);
            let builder = LLVMCreateBuilderInContext(ctx);

            (ctx, module, builder)
        };

        Ok(SpirvTranspiler {
            spirv_mod: spirv_mod,
            ctx: ctx,
            module: module,
            builder: builder,
        })
    }

    pub fn transpile(&mut self) -> Result<(), TranspilerError> {
        // https://github.com/KhronosGroup/SPIRV-LLVM/blob/0d6cd12d350bcaed0634bcb1f260bc3925dfdc23/lib/SPIRV/SPIRVReader.cpp#L2262
        self.trans_addressing_model()?;
        self.trans_types_global_values()?;
        unimplemented!()
    }

    pub fn trans_addressing_model(&mut self) -> Result<(), TranspilerError> {
        let memory_model = self.spirv_mod.memory_model.as_ref().ok_or(
            TranspilerError::NoMemoryModelProvided,
        )?;
        match *memory_model.operands.get(0).unwrap() {
            Operand::AddressingModel(addr_model) => {
                match addr_model {
                    // Vulkan only uses Logical, according to somewhere on the internet
                    AddressingModel::Logical => return Ok(()),
                    AddressingModel::Physical32 => {
                        return Err(TranspilerError::UnsupportedAddressingModel)
                    }
                    AddressingModel::Physical64 => {
                        return Err(TranspilerError::UnsupportedAddressingModel)
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn trans_types_global_values() -> Result<(), TranspilerError> {
        for type_const_global in &self.spirv_mod.types_global_values {
            // Iterated value is either a type, constant or global value
            self.trans_value(type_const_global, None, None, true)?;
        }
    }

    pub fn trans_value(
        inst: Instruction,
        function: Option<LLVMValueRef>,
        bb: Option<LLVMBasicBlockRef>,
        create_placeholder: bool,
    ) -> Result<(), TranspilerError> {
        
    }
}

impl<'a> Drop for SpirvTranspiler<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMDisposeModule(self.module);
            LLVMContextDispose(self.ctx);
        }
    }
}
