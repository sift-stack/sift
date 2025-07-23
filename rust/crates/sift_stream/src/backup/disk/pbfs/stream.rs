use super::BackupsDecoder;
use prost::Message as PbMessage;
use sift_error::prelude::*;
use std::{
    fs::File,
    io::BufReader,
    iter::Iterator,
    marker::PhantomData,
    path::{Path, PathBuf},
};

/// Takes in a path to a backup file and returns an instance of [BackupsDecoder] which is an
/// iterator over the protobuf messages found in the backup file. The iterator will terminate when
/// reaching an EOF or it hits a corrupt message; in this case the error returned by the item will
/// be an `Err` whose kind if [ErrorKind::BackupIntegrityError].
pub fn decode_backup<P, M>(path: P) -> Result<BackupsDecoder<M, BufReader<File>>>
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

pub struct BackupsStream<M>
where
    M: PbMessage + Default + 'static,
{
    backups: Vec<PathBuf>,
    message_type: PhantomData<M>,
}

impl<M> BackupsStream<M>
where
    M: PbMessage + Default + 'static,
{
    pub fn new(backups: &[PathBuf]) -> Self {
        Self {
            backups: backups.to_vec(),
            message_type: PhantomData,
        }
    }
}

impl<M> IntoIterator for BackupsStream<M>
where
    M: PbMessage + Default + 'static,
{
    type Item = M;
    type IntoIter = BackupsIter<M>;

    fn into_iter(self) -> Self::IntoIter {
        BackupsIter::new(self.backups)
    }
}

pub struct BackupsIter<M>
where
    M: PbMessage + Default + 'static,
{
    backups: Vec<PathBuf>,
    current_backup_idx: usize,
    current_backup: Option<BackupsDecoder<M, BufReader<File>>>,
    error: Option<Error>,
}

impl<M> Iterator for BackupsIter<M>
where
    M: PbMessage + Default + 'static,
{
    type Item = M;

    fn next(&mut self) -> Option<Self::Item> {
        let current_backup = match self.current_backup.as_mut() {
            Some(bu_mut) => bu_mut,
            None => {
                if self.current_backup_idx == self.backups.len() {
                    return None;
                }

                let current_backup_res = self
                    .backups
                    .get(self.current_backup_idx)
                    .map(|p| decode_backup::<_, M>(p))
                    .transpose();

                match current_backup_res {
                    Ok(None) => return None,

                    Ok(Some(current_backup)) => {
                        self.current_backup = Some(current_backup);
                        self.current_backup.as_mut().unwrap()
                    }
                    Err(err) => {
                        self.error = Some(err);
                        return None;
                    }
                }
            }
        };

        match current_backup.next() {
            Some(Ok(point)) => Some(point),
            Some(Err(err)) => {
                self.error = Some(err);
                None
            }
            None => {
                self.current_backup_idx += 1;
                self.next()
            }
        }
    }
}

impl<M> BackupsIter<M>
where
    M: PbMessage + Default + 'static,
{
    pub fn new(backups: Vec<PathBuf>) -> Self {
        let current_backup_idx = 0;
        Self {
            backups,
            current_backup_idx,
            current_backup: None,
            error: None,
        }
    }
}

#[cfg(feature = "tracing")]
impl<M> Drop for BackupsIter<M>
where
    M: PbMessage + Default + 'static,
{
    fn drop(&mut self) {
        if let Some(err) = self.error.as_ref() {
            tracing::warn!(
                error = format!("{err:?}"),
                "error encountered while reingesting backups - backup will remain on disk"
            )
        }
    }
}
