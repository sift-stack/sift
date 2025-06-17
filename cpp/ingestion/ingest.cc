#include <google/protobuf/timestamp.pb.h>
#include <grpcpp/grpcpp.h>

#include <chrono>
#include <iostream>
#include <memory>
#include <random>
#include <string>
#include <thread>
#include <vector>

#include "sift/ingest/v1/ingest.grpc.pb.h"
#include "sift/ingest/v1/ingest.pb.h"
#include "sift/ingestion_configs/v1/ingestion_configs.grpc.pb.h"
#include "sift/ingestion_configs/v1/ingestion_configs.pb.h"
#include "sift/runs/v2/runs.grpc.pb.h"
#include "sift/runs/v2/runs.pb.h"

using grpc::Channel;
using grpc::ClientContext;
using grpc::Status;

/**
 * Used to authenticate each call with the API key.
 */
class ApiKeyAuthenticator : public grpc::MetadataCredentialsPlugin
{
  public:
    explicit ApiKeyAuthenticator(const std::string &api_key) : api_key_(api_key)
    {
    }

    grpc::Status GetMetadata(grpc::string_ref service_url, grpc::string_ref method_name,
                             const grpc::AuthContext &channel_auth_context,
                             std::multimap<grpc::string, grpc::string> *metadata) override
    {
        metadata->insert(std::make_pair("authorization", "Bearer " + api_key_));
        return grpc::Status::OK;
    }

  private:
    std::string api_key_;
};

/**
 * Create and return a secure channel.
 */
std::shared_ptr<Channel> CreateChannel(const std::string &apiUrl, const std::string &apiKey)
{
    grpc::ChannelArguments args;
    args.SetInt(GRPC_ARG_ENABLE_RETRIES, 1);

    grpc::SslCredentialsOptions ssl_opts;
    auto ssl_creds = grpc::SslCredentials(ssl_opts);

    auto call_creds = grpc::MetadataCredentialsFromPlugin(
        std::unique_ptr<grpc::MetadataCredentialsPlugin>(new ApiKeyAuthenticator(apiKey)));

    auto channel_creds = grpc::CompositeChannelCredentials(ssl_creds, call_creds);

    return grpc::CreateCustomChannel(apiUrl, channel_creds, args);
}

/*
 * Prints and exits on an invalid status.
 */
void ExitOnError(const std::string &msg, const Status &status)
{
    if (!status.ok())
    {
        std::cerr << msg << ": " << status.error_message() << std::endl;
        exit(1);
    }
}

/**
 * Helper function to create a data message coming the data for the kinematic flow.
 */
void SetKinematicMessage(const std::string runId, const std::string ingestionConfigId, const std::string organizationId,
                         google::protobuf::Timestamp &timestamp, const double position, const double speed,
                         sift::ingest::v1::IngestWithConfigDataStreamRequest &message)
{
    message.set_ingestion_config_id(ingestionConfigId);
    message.set_flow("kinematics");
    message.set_organization_id(organizationId);

    /*
     * Should be set to false if validation is no longer required for better performance.
     */
    message.set_end_stream_on_validation_error(true);

    message.set_run_id(runId);
    google::protobuf::Timestamp *t = new google::protobuf::Timestamp(timestamp);
    message.set_allocated_timestamp(t);

    /*
     * Values should be added in the same order as when created in the ingestion config.
     */
    message.add_channel_values()->set_double_(position);
    message.add_channel_values()->set_double_(speed);
}

/**
 * Helper function to create a data message coming the data for the electrical flow.
 */
void SetElectricalMessage(const std::string runId, const std::string ingestionConfigId,
                          const std::string organizationId, google::protobuf::Timestamp &timestamp,
                          const double voltage, const double current,
                          sift::ingest::v1::IngestWithConfigDataStreamRequest &message)
{
    message.set_ingestion_config_id(ingestionConfigId);
    message.set_flow("electrical");
    message.set_organization_id(organizationId);

    /*
     * Should be set to false if validation is no longer required for better performance.
     */
    message.set_end_stream_on_validation_error(true);

    message.set_run_id(runId);
    google::protobuf::Timestamp *t = new google::protobuf::Timestamp(timestamp);
    message.set_allocated_timestamp(t);

    /*
     * Values should be added in the same order as when created in the ingestion config.
     */
    message.add_channel_values()->set_double_(voltage);
    message.add_channel_values()->set_double_(current);
}

/*
 * Convert the time_point into a protobuf Timestamp.
 */
void GetTimestamp(std::chrono::time_point<std::chrono::system_clock> time, google::protobuf::Timestamp &timestamp)
{
    auto epoch_seconds = std::chrono::duration_cast<std::chrono::seconds>(time.time_since_epoch()).count();
    auto epoch_nanos = std::chrono::duration_cast<std::chrono::nanoseconds>(time.time_since_epoch()).count();

    timestamp.set_seconds(epoch_seconds);
    timestamp.set_nanos(epoch_nanos % 1000000000);
}

int main()
{
    const char *apiUrl = std::getenv("BASE_URI");
    const char *apiKey = std::getenv("SIFT_API_KEY");
    const char *organizationId = std::getenv("SIFT_ORGANIZATION_ID");
    if (!apiUrl)
    {
        std::cerr << "Run with BASE_URI environment variable" << std::endl;
        exit(1);
    }
    if (!apiKey)
    {
        std::cerr << "Run with SIFT_API_KEY environment variable" << std::endl;
        exit(1);
    }
    if (!organizationId)
    {
        std::cerr << "Run with SIFT_ORGANIZATION_ID environment variable" << std::endl;
        exit(1);
    }

    auto channel = CreateChannel(apiUrl, apiKey);
    auto ingestionConfigService = sift::ingestion_configs::v1::IngestionConfigService::NewStub(channel);
    auto runService = sift::runs::v2::RunService::NewStub(channel);
    auto ingestService = sift::ingest::v1::IngestService::NewStub(channel);

    /*
     * Step 1:
     * Create and send ingestion config.
     */
    sift::ingestion_configs::v1::CreateIngestionConfigRequest ingestionConfigRequest;
    ingestionConfigRequest.set_asset_name("train_1");
    ingestionConfigRequest.set_organization_id(organizationId);
    // Optionally set a user defined client_key.
    ingestionConfigRequest.set_client_key("example_client_key");

    /*
     * Create the first 10Hz flow.
     */
    auto kinematics_flow = ingestionConfigRequest.add_flows();
    kinematics_flow->set_name("kinematics");

    auto position = kinematics_flow->add_channels();
    position->set_name("position");
    position->set_unit("m");
    position->set_description("Position of the train");
    position->set_data_type(sift::common::type::v1::CHANNEL_DATA_TYPE_DOUBLE);

    auto speed = kinematics_flow->add_channels();
    speed->set_name("speed");
    speed->set_unit("m/s");
    speed->set_description("Speed of the train");
    speed->set_data_type(sift::common::type::v1::CHANNEL_DATA_TYPE_DOUBLE);

    /*
     * Create the second 1Hz flow.
     */
    auto electrical_flow = ingestionConfigRequest.add_flows();
    electrical_flow->set_name("electrical");

    auto voltage = electrical_flow->add_channels();
    voltage->set_name("voltage");
    voltage->set_unit("V");
    voltage->set_description("Bus voltage");
    voltage->set_data_type(sift::common::type::v1::CHANNEL_DATA_TYPE_DOUBLE);

    auto current = electrical_flow->add_channels();
    current->set_name("current");
    current->set_unit("A");
    current->set_description("Bus current");
    current->set_data_type(sift::common::type::v1::CHANNEL_DATA_TYPE_DOUBLE);

    /*
     * Before sending check if it exists already.
     */
    ClientContext context1;
    sift::ingestion_configs::v1::ListIngestionConfigsRequest listConfigRequest;
    sift::ingestion_configs::v1::ListIngestionConfigsResponse listConfigResponse;
    listConfigRequest.set_filter("client_key==\"example_client_key\"");
    ExitOnError("Failed to check is config exists",
                ingestionConfigService->ListIngestionConfigs(&context1, listConfigRequest, &listConfigResponse));
    std::string ingestionConfigId;
    if (listConfigResponse.ingestion_configs_size() == 0)
    {
        /*
         * Create the config.
         */
        ClientContext context1;
        sift::ingestion_configs::v1::CreateIngestionConfigResponse ingestionConfigResponse;
        auto status =
            ingestionConfigService->CreateIngestionConfig(&context1, ingestionConfigRequest, &ingestionConfigResponse);
        ExitOnError("Error creating ingestion config", status);
        ingestionConfigId = ingestionConfigResponse.ingestion_config().ingestion_config_id();
        std::cout << "Created ingestion config (id=" << ingestionConfigId << ")" << std::endl;
    }
    else
    {
        /*
         * Use the existing config.
         */
        std::cout << "This config exists already, not creating." << std::endl;
        ingestionConfigId = listConfigResponse.ingestion_configs(0).ingestion_config_id();
    }

    /*
     * Step 2:
     * Create a Run.
     * This is optional. You can stream to an asset without specifying a run.
     */
    auto now = std::chrono::system_clock::now();
    std::time_t now_time = std::chrono::system_clock::to_time_t(now);
    std::string run_title = "CPP Example: Train Run ";
    run_title.append(std::ctime(&now_time));

    sift::runs::v2::CreateRunRequest createRunRequest;
    createRunRequest.set_name(run_title);
    createRunRequest.set_description("Example run generated from CPP example.");
    createRunRequest.set_organization_id(organizationId);

    ClientContext context2;
    sift::runs::v2::CreateRunResponse createRunResponse;
    ExitOnError("Failed to create run", runService->CreateRun(&context2, createRunRequest, &createRunResponse));
    std::string runId = createRunResponse.run().run_id();
    std::cout << "Created Run (title=" << run_title << ")" << std::endl;

    /**
     * Step 3.
     * Simulate and stream data.
     * We batch the stream requests and send data at 5s intervals to reduce the overhead
     * of setting up/tearing down connections. In this example, we are generating and streaming
     * data in batches from the same application which is generally not a good idea if your
     * application needs to run in real-time or at a consistent rate. The recommendation is to
     * send data to a dedicated proxy application that can handle batching the data and sending it to Sift.
     */
    const int total_iterations = 1000;

    /*
     * Telemetered variables.
     */
    double position_value;
    double speed_value;
    double voltage_value;
    double current_value;

    /**
     * Buffer data and send in batches (every 5 seconds).
     */
    std::vector<sift::ingest::v1::IngestWithConfigDataStreamRequest> buffer;

    const auto start_time = std::chrono::system_clock::now();

    /*
     * The simulated time that will be used to timestamp the data points.
     */
    auto simulated_time = start_time;

    /*
     * The next time to flush data to Sift, based on real time.
     */
    auto next_flush_time = start_time + std::chrono::seconds(5);

    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<> dis(-2, 2);

    std::cout << "Streaming data for 100s" << std::endl;
    for (int i = 0; i < total_iterations; i++)
    {
        auto now = std::chrono::system_clock::now();

        speed_value = 2.5;
        position_value += speed_value;
        voltage_value = 36 + dis(gen);
        current_value = 10 + dis(gen);

        google::protobuf::Timestamp timestamp;
        GetTimestamp(simulated_time, timestamp);

        /* Send electrical flow at 10 Hz*/
        sift::ingest::v1::IngestWithConfigDataStreamRequest kinematic_message;
        SetKinematicMessage(runId, ingestionConfigId, organizationId, timestamp, position_value, speed_value,
                            kinematic_message);
        buffer.push_back(kinematic_message);

        /* Send electrical flow at 1 Hz*/
        if (i % 10 == 0)
        {
            sift::ingest::v1::IngestWithConfigDataStreamRequest electrical_message;
            SetElectricalMessage(runId, ingestionConfigId, organizationId, timestamp, voltage_value, current_value,
                                 electrical_message);
            buffer.push_back(electrical_message);
        }

        if (now > next_flush_time)
        {
            std::cout << "Flushing data to Sift" << std::endl;

            grpc::ClientContext stream_context;
            sift::ingest::v1::IngestWithConfigDataStreamResponse stream_response;
            auto writer = ingestService->IngestWithConfigDataStream(&stream_context, &stream_response);
            for (const auto &message : buffer)
            {
                if (!writer->Write(message))
                {
                    std::cout << "Issue writing to stream" << std::endl;
                }
            }
            writer->WritesDone();
            ExitOnError("Failed writing to stream", writer->Finish());

            next_flush_time = std::chrono::system_clock::now() + std::chrono::seconds(5);
            buffer.clear();
        }

        /*
         * Sleep here so the sim runs similar to real time.
         */
        std::this_thread::sleep_for(std::chrono::milliseconds(100));

        simulated_time += std::chrono::milliseconds(100);
    }

    std::cout << "Done!" << std::endl;
    return 0;
}
