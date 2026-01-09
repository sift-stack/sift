use prost::Message as PbMessage;
use sift_error::prelude::*;
use sift_pbfs::BackupsDecoder;
use std::{fs::File, io::BufReader, path::Path};

mod async_manager;
pub(crate) use async_manager::{AsyncBackupsManager, BackupIngestTask};

pub(crate) mod file_writer;

mod policy;
pub use policy::{DiskBackupPolicy, RollingFilePolicy};

/// Takes in a path to a backup file and returns an instance of [BackupsDecoder] which is an
/// iterator over the protobuf messages found in the backup file. The iterator will terminate when
/// reaching an EOF or it hits a corrupt message; in this case the error returned by the item will
/// be an `Err` whose kind if [ErrorKind::BackupIntegrityError].
pub(crate) fn decode_backup<P, M>(path: P) -> Result<BackupsDecoder<M, BufReader<File>>>
where
    P: AsRef<Path>,
    M: PbMessage + Default + 'static,
{
    File::open(path.as_ref())
        .map(BufReader::new)
        .map(BackupsDecoder::new)
        .map_err(|e| Error::new(ErrorKind::IoError, e))
        .context("failed to open backup")
        .help("contact Sift")
}
