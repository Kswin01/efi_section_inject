use std::{fs, path::Path};
use goblin::pe::{
    section_table::{IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_CNT_INITIALIZED_DATA, IMAGE_SCN_MEM_READ},
    PE,
};
use ifrit::writer::{PEWriter, Section};

use pe_parser::pe::parse_portable_executable;

fn main() {
    // Read the file of the UEFI wrapper that we have already compiled.
    let file = fs::read("/home/kwinter/TS/orin/rust_pe/pe_test/src/uefi-test.efi").unwrap();
    let file = &file[..];
    // Read the image that we want to append to the above UEFI wrapper.
    let loader_img = fs::read("/home/kwinter/TS/orin/rust_pe/pe_test/src/loader.img").expect("failed to read loader image");

    // Parse the UEFI wrapper using goblin.
    let pe = PE::parse(file).unwrap();
    println!("{}", file.len());
    let mut pe_writer = PEWriter::new(pe).expect("Failed to create a wrapper");

    // We will call the section ".mloader" to avoid conflicts with the section already called
    // ".loader" in the EFI image.
    let section_name: [u8; 8] = *b".mloader";

    // Using "ifrit" to append a section to the UEFI wrapper. Labellign this as initialized data and readable.
    pe_writer
    .insert_section(
        Section::new(
            &section_name,
            Some(loader_img),
            IMAGE_SCN_CNT_INITIALIZED_DATA | IMAGE_SCN_MEM_READ,
        )
        .expect("Failed to create a section"),
    )
    .unwrap();

    let new_pe = pe_writer.write_into().unwrap();
    std::fs::write("/home/kwinter/TS/orin/rust_pe/pe_test/src/test.efi", &new_pe[..]).unwrap();
}
