use std::ffi::CStr;
use std::io::{stdin, stdout, Read, Write};
use std::ptr::null_mut;

use llvm_quick::disassembler::DisasmContext;
use llvm_quick::target::{
    initialize_all_disassemblers, initialize_all_target_infos, initialize_all_target_mcs,
};

fn main() -> std::io::Result<()> {
    initialize_all_target_infos();
    initialize_all_target_mcs();
    initialize_all_disassemblers();

    let disasm = DisasmContext::create(c"x86_64", null_mut(), 0, None, None)
        .expect("Failed to create disassembler");

    let mut data = Vec::<u8>::new();
    stdin().read_to_end(&mut data)?;
    disassemble_bytes(&mut data, &*disasm, stdout())
}

const PC_BASE_ADDR: u64 = 0;

fn disassemble_bytes<W: Write>(
    mut x: &mut [u8],
    disasm: &DisasmContext,
    mut out: W,
) -> std::io::Result<()> {
    let mut pc = PC_BASE_ADDR;

    loop {
        let mut sbuf = [0; 128];
        let sz = disasm.instruction(x, pc, &mut sbuf[..]);

        if sz == 0 {
            break;
        }

        let instr_str = unsafe { CStr::from_ptr(sbuf.as_ptr()) };
        write!(out, "{}\n", instr_str.to_string_lossy())?;

        pc += sz as u64;
        x = &mut x[sz..];
    }

    Ok(())
}
