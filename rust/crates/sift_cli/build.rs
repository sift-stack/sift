/// Build descriptor's so that the Black Hole gRPC server can
/// stand up the reflection service.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .file_descriptor_set_path("descriptor.bin")
        .build_client(false)
        .build_server(false)
        .compile_protos(
            &[
                "/tmp/exported-protos/sift/assets/v1/assets.proto",
                "/tmp/exported-protos/sift/ping/v1/ping.proto",
                "/tmp/exported-protos/sift/ingest/v1/ingest.proto",
                "/tmp/exported-protos/sift/ingestion_configs/v2/ingestion_configs.proto",
            ],
            // Run the following command to generate exported-protos:
            // buf export ../../../protos --output /tmp/exported-protos
            &["/tmp/exported-protos"],
        )?;

    Ok(())
}
