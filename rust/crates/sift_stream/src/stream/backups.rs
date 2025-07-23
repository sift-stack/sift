/// Concerned with transmitting backups for ingestion-config based streaming.
pub mod ingestion_config {
    use crate::backup::{BackupsTransmitter, disk::pbfs::stream::BackupsStream};
    use sift_error::prelude::*;
    use sift_rs::{
        SiftChannel,
        ingest::v1::{
            IngestWithConfigDataStreamRequest as IngestRequest,
            ingest_service_client::IngestServiceClient,
        },
    };

    pub struct BackupsTransmitterDisk {
        sift_channel: SiftChannel,
    }

    impl BackupsTransmitterDisk {
        pub fn new(sift_channel: SiftChannel) -> Self {
            Self { sift_channel }
        }
    }

    impl BackupsTransmitter<IngestRequest, BackupsStream<IngestRequest>> for BackupsTransmitterDisk {
        async fn transmit(&mut self, data: BackupsStream<IngestRequest>) -> Result<()> {
            let mut insc = IngestServiceClient::new(self.sift_channel.clone());

            insc.ingest_with_config_data_stream(tokio_stream::iter(data))
                .await
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))
                .context("error occurred while transmitting backups")?;

            Ok(())
        }
    }

    pub struct BackupsTransmitterInMemory {
        sift_channel: SiftChannel,
    }

    impl BackupsTransmitterInMemory {
        pub fn new(sift_channel: SiftChannel) -> Self {
            Self { sift_channel }
        }
    }

    impl BackupsTransmitter<IngestRequest, Vec<IngestRequest>> for BackupsTransmitterInMemory {
        async fn transmit(&mut self, data: Vec<IngestRequest>) -> Result<()> {
            let mut insc = IngestServiceClient::new(self.sift_channel.clone());

            insc.ingest_with_config_data_stream(tokio_stream::iter(data))
                .await
                .map_err(|e| Error::new(ErrorKind::BackupsError, e))
                .context("error occurred while transmitting backups")?;

            Ok(())
        }
    }
}
