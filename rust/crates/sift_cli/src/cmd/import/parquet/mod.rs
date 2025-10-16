use anyhow::{Result, anyhow};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    os::unix::fs::FileExt,
};

pub mod flat_dataset;

/// Metadata about the Parquet's footer
#[derive(Copy, Clone)]
struct FooterMetadata {
    /// Offset to the Parquet file's footer
    offset: u64,
    /// Length of the Parquet file's footer
    length: u64,
}

fn get_footer(file: &mut File, footer_metadata: FooterMetadata) -> Result<Vec<u8>> {
    let FooterMetadata { length, offset } = footer_metadata;
    let mut buf = vec![0u8; length as usize];
    file.read_exact_at(&mut buf, offset)?;
    Ok(buf)
}

/// Note that this will advance the cursor.
impl TryFrom<&mut File> for FooterMetadata {
    type Error = anyhow::Error;

    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        file.seek(SeekFrom::End(-8))?;

        let footer_len = {
            let mut buf = [0u8; 4];
            file.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;
        if &magic != b"PAR1" {
            return Err(anyhow!("invalid Parquet magic bytes"));
        }

        let file_len = file.metadata()?.len();
        if u64::from(footer_len) + 8 > file_len {
            return Err(anyhow!(
                "footer length ({footer_len}) exceeds file size ({file_len})",
            ));
        }

        Ok(Self {
            offset: file_len - u64::from(footer_len) - 8,
            length: u64::from(footer_len),
        })
    }
}
