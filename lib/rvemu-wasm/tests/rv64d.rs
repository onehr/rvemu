#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use rvemu_core::bus::DRAM_BASE;

wasm_bindgen_test_configure!(run_in_browser);

const DEFAULT_SP: i64 = 1048000 + 0x8000_0000;

#[wasm_bindgen_test]
pub fn fcvtld_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x2f, 0xc2, // fcvt.l.d x31, f31 (rm: 000)
    ];

    cpu.fregs.write(31, -4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, -4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -4.2,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtlud_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x3f, 0xc2, // fcvt.lu.d x31, f31 (rm: 000)
    ];

    cpu.fregs.write(31, 4.2);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.2,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtdl_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x2f, 0xd2, // fcvt.d.l x31, f31 (rm: 000)
    ];

    cpu.xregs.write(31, -4);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, -4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -4.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fcvtdlu_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x3f, 0xd2, // fcvt.d.lu x31, f31 (rm: 000)
    ];

    cpu.xregs.write(31, 4);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fmvxd_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x0f, 0xe2, // fmv.x.d x31, f31
    ];

    cpu.fregs.write(31, 4.0);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}

#[wasm_bindgen_test]
pub fn fmvdx_rd_rs1_rs2() {
    let mut emu = Emulator::new();
    let mut bus = rvemu_core::bus::Bus::new();
    let data = vec![
        0xd3, 0x8f, 0x0f, 0xf2, // fmv.d.x x31, f31
    ];

    cpu.xregs.write(31, 4);

    bus.dram.dram.splice(..data.len(), data.iter().cloned());

    cpu.pc = DRAM_BASE;
    cpu.start(&mut bus);

    // x0-x31
    let expected_x = [
        0, 0, DEFAULT_SP, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 4,
    ];
    // f0-f31
    let expected_f = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0,
    ];
    for (i, e) in expected_x.iter().enumerate() {
        assert_eq!(*e, cpu.xregs.read(i));
    }
    for (i, e) in expected_f.iter().enumerate() {
        assert_eq!(*e, cpu.fregs.read(i));
    }
}
