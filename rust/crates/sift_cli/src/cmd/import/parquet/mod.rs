#[cfg(not(target_os = "windows"))]
use unix::{FooterMetadata, get_footer};

#[cfg(target_os = "windows")]
use windows::{FooterMetadata, get_footer};

pub mod flat_dataset;

#[cfg(not(target_os = "windows"))]
mod unix {
    use anyhow::{Result, anyhow};
    use std::{
        fs::File,
        io::{Read, Seek, SeekFrom},
        os::unix::fs::FileExt,
    };

    /// Metadata about the Parquet's footer
    #[derive(Copy, Clone)]
    pub struct FooterMetadata {
        /// Offset to the Parquet file's footer
        pub offset: u64,
        /// Length of the Parquet file's footer
        pub length: u64,
    }

    pub fn get_footer(file: &mut File, footer_metadata: FooterMetadata) -> Result<Vec<u8>> {
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
}

#[cfg(target_os = "windows")]
mod windows {
    use anyhow::{Result, anyhow};
    use std::{
        fs::File,
        io::{Read, Seek, SeekFrom},
        os::windows::fs::FileExt,
    };

    /// Metadata about the Parquet's footer
    #[derive(Copy, Clone)]
    pub struct FooterMetadata {
        pub offset: u64,
        pub length: u64,
    }

    pub fn get_footer(file: &mut File, footer_metadata: FooterMetadata) -> Result<Vec<u8>> {
        let FooterMetadata { length, offset } = footer_metadata;
        let mut buf = vec![0u8; length as usize];
        // On Windows, use `seek_read` to read without moving the main file cursor
        let bytes_read = file.seek_read(&mut buf, offset)?;
        if bytes_read != buf.len() {
            return Err(anyhow!("incomplete footer read"));
        }
        Ok(buf)
    }

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
}
