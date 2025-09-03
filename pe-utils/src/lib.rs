use goblin::container;
use goblin::pe::header::Header;
use goblin::pe::import::ImportData;
use goblin::pe::optional_header::OptionalHeader;
use goblin::pe::section_table::SectionTable;
use goblin::pe::{import, options};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Goblin Error: {0}")]
    Goblin(#[from] goblin::error::Error),
    #[error("Optional header missing")]
    NoOptionalHeader,
    #[error("Offset: {0} not found in any section")]
    NotInSection(usize),
}

pub fn take_hint_bytes(bytes: &[u8]) -> Option<&[u8; 16]> {
    bytes
        .get(0..16)
        .and_then(|hint_bytes_slice| hint_bytes_slice.try_into().ok())
}

pub trait MemAlignedAddress<T> {
    fn get_mem_aligned_address(address: T, alignment: T) -> T;
}

impl MemAlignedAddress<Self> for u32 {
    fn get_mem_aligned_address(address: Self, alignment: Self) -> Self {
        let remainder = address % alignment;
        if remainder != 0 {
            return address + alignment - remainder;
        }
        address
    }
}

impl MemAlignedAddress<Self> for u64 {
    fn get_mem_aligned_address(address: Self, alignment: Self) -> Self {
        let remainder = address % alignment;
        if remainder != 0 {
            return address + alignment - remainder;
        }
        address
    }
}

pub fn parse_headers(dump: &[u8]) -> Result<Header, Error> {
    let result = if let Some(hint_bytes) = take_hint_bytes(dump) {
        match goblin::peek_bytes(hint_bytes)? {
            goblin::Hint::PE => Ok(Header::parse(dump)?),
            _ => Err(goblin::error::Error::Malformed(
                "We were expecting a PE and it's not a PE".to_string(),
            )),
        }
    } else {
        Err(goblin::error::Error::Malformed(
            "Object is too small.".to_string(),
        ))
    }?;
    Ok(result)
}

pub fn get_optional_headers(header: &Header) -> Result<OptionalHeader, Error> {
    match header.optional_header {
        None => Err(Error::NoOptionalHeader),
        Some(optional_header) => Ok(optional_header),
    }
}

pub fn get_sections(header: &Header, dump: &[u8]) -> Result<Vec<SectionTable>, Error> {
    let optional_header_offset = header.dos_header.pe_pointer as usize
        + goblin::pe::header::SIZEOF_PE_MAGIC
        + goblin::pe::header::SIZEOF_COFF_HEADER;
    let offset =
        &mut (optional_header_offset + header.coff_header.size_of_optional_header as usize);
    Ok(header.coff_header.sections(dump, offset)?)
}

pub fn resolve_symbol(
    image_base: usize,
    sections: &[SectionTable],
    addr: usize,
) -> Result<usize, Error> {
    for section in sections {
        if (addr > section.pointer_to_raw_data as usize)
            && (addr < (section.pointer_to_raw_data + section.size_of_raw_data) as usize)
        {
            return Ok(image_base
                + (section.virtual_address - section.pointer_to_raw_data) as usize
                + addr);
        }
    }
    Err(Error::NotInSection(addr))
}

pub fn get_imports<'a>(
    bytes: &'a [u8],
    optional_header: &OptionalHeader,
    sections: &[SectionTable],
) -> Result<Option<ImportData<'a>>, Error> {
    let opts = &options::ParseOptions::default();
    let file_alignment = optional_header.windows_fields.file_alignment;
    let is_64 = optional_header.container()? == container::Container::Big;
    let mut _imports = vec![];
    let mut import_data = None;

    if let Some(&import_table) = optional_header.data_directories.get_import_table() {
        let id = if is_64 {
            ImportData::parse_with_opts::<u64>(
                bytes,
                import_table,
                &sections,
                file_alignment,
                opts,
            )?
        } else {
            ImportData::parse_with_opts::<u32>(
                bytes,
                import_table,
                &sections,
                file_alignment,
                opts,
            )?
        };

        if is_64 {
            _imports = import::Import::parse::<u64>(bytes, &id, &sections)?
        } else {
            _imports = import::Import::parse::<u32>(bytes, &id, &sections)?
        }

        let mut libraries = id
            .import_data
            .iter()
            .map(|data| data.name)
            .collect::<Vec<&'a str>>();
        libraries.sort();
        libraries.dedup();

        import_data = Some(id);
    }

    Ok(import_data)
}
