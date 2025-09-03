use std::io::Write;

use colored::*;
use goblin::pe::optional_header::OptionalHeader;
use goblin::pe::section_table::SectionTable;
use log::{info, trace};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("PE Utils Error: {0}")]
    PEUtils(#[from] pe_utils::Error),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn restore_from_ptr<A: AsRef<str>, B: AsRef<str>>(
    name: A,
    module_base: usize,
    restored_filename: Option<B>,
) -> Result<Vec<u8>, Error> {
    let data = unsafe { std::slice::from_raw_parts(module_base as *const u8, 0x1000) };

    let header = pe_utils::parse_headers(data)?;
    trace!("{:#?}", header);
    let optional_headers = pe_utils::get_optional_headers(&header)?;
    let sections = pe_utils::get_sections(&header, data)?;

    let mut vaddr_end: u32 = 0;
    for section in &sections {
        let virtual_end = section.virtual_address + section.size_of_raw_data;
        if virtual_end > vaddr_end {
            vaddr_end = virtual_end;
        }
    }
    let data = unsafe { std::slice::from_raw_parts(module_base as *const u8, vaddr_end as usize) };
    restore_raw(name, data, optional_headers, &sections, restored_filename)
}

pub fn restore_from_dump<A: AsRef<str>, B: AsRef<str>>(
    name: A,
    dump: &[u8],
    restored_filename: Option<B>,
) -> Result<Vec<u8>, Error> {
    let header = pe_utils::parse_headers(dump)?;
    trace!("{:#?}", header);
    let optional_headers = pe_utils::get_optional_headers(&header)?;
    let sections = pe_utils::get_sections(&header, dump)?;
    restore_raw(name, dump, optional_headers, &sections, restored_filename)
}

pub fn restore_raw<A: AsRef<str>, B: AsRef<str>>(
    name: A,
    dump: &[u8],
    optional_headers: OptionalHeader,
    sections: &[SectionTable],
    restored_filename: Option<B>,
) -> Result<Vec<u8>, Error> {
    let mut output = vec![0; dump.len()];
    output[0..optional_headers.windows_fields.size_of_headers as usize]
        .copy_from_slice(&dump[0..optional_headers.windows_fields.size_of_headers as usize]);

    let mut eof: u32 = 0;
    for section in sections {
        let phys_start = section.pointer_to_raw_data as usize;
        let phys_end = phys_start + section.size_of_raw_data as usize;
        let virt_start = section.virtual_address as usize;
        let virt_end = virt_start + section.size_of_raw_data as usize;

        if phys_start <= output.len()
            && phys_end <= output.len()
            && virt_start <= dump.len()
            && virt_end <= dump.len()
        {
            let source_slice = &dump[virt_start..virt_end];
            let dest_slice = &mut output[phys_start..phys_end];

            if source_slice.len() == dest_slice.len() {
                dest_slice.copy_from_slice(source_slice);
            } else {
                eprintln!(
                    "Skipping section {} due to slice length mismatch (source: {}, destination: {})",
                    String::from_utf8_lossy(&section.name).bright_red(),
                    source_slice.len(),
                    dest_slice.len()
                );
            }
        } else {
            eprintln!(
                "Skipping section {} due to out-of-bounds access",
                String::from_utf8_lossy(&section.name).bright_red()
            );
        }

        if phys_end as u32 > eof {
            eof = phys_end as u32;
        }
    }

    match restored_filename {
        None => info!("Since no restored_filename was provided, the restored output will not be saved to a file"),
        Some(filename) => {
            let mut data_file = std::fs::File::create(filename.as_ref())?;
            data_file.write_all(&output[0..eof as usize])?;
            info!("Restored executable saved to: {}", filename.as_ref());
        }
    }

    info!("Executable {} restored successfully", name.as_ref());
    Ok(output)
}
