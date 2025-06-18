/*
 * Example application to ping Sift.
 */
#include <grpcpp/grpcpp.h>

#include <iostream>
#include <memory>
#include <string>

#include "sift/ping/v1/ping.grpc.pb.h"
#include "sift/ping/v1/ping.pb.h"

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

int main()
{
    const char *apiUrl = std::getenv("BASE_URI");
    const char *apiKey = std::getenv("SIFT_API_KEY");
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

    std::shared_ptr<Channel> channel = CreateChannel(apiUrl, apiKey);
    std::unique_ptr<sift::ping::v1::PingService::Stub> service = sift::ping::v1::PingService::NewStub(channel);

    ClientContext context;
    sift::ping::v1::PingRequest request;
    sift::ping::v1::PingResponse response;

    ExitOnError("Failed to send ping", service->Ping(&context, request, &response));

    std::cout << response.response() << std::endl;

    return 0;
}
