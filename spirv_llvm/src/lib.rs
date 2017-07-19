extern crate rspirv;
extern crate llvm_sys;
extern crate spirv_headers;

mod transpiler;
mod trans;

use transpiler::SpirvTranspiler;

pub fn spirv_to_llvm(spirv_mod: &rspirv::mr::Module) -> Result<(), TranspilerError> {
    let transpiler = SpirvTranspiler::new(spirv_mod)?;
    unimplemented!()
}

pub enum TranspilerError {
    NoHeader,
    InvalidMagicNumber,
    NoMemoryModelProvided,
    UnsupportedAddressingModel,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
