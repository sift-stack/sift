package grpc

import (
	"context"
	"crypto/tls"
	"net/url"
	"strings"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
)

type SiftChannelConfig struct {
	Uri    string
	Apikey string
}

// Type alias for a gRPC channel configured specifically to communicate with the Sift API.
type SiftChannel = *grpc.ClientConn

// Initializes a gRPC connection to Sift.
func UseSiftChannel(ctx context.Context, config SiftChannelConfig) (SiftChannel, error) {
	url, err := url.Parse(config.Uri)
	if err != nil {
		return nil, err
	}

	transportCred := grpc.WithTransportCredentials(credentials.NewTLS(&tls.Config{}))
	if !strings.Contains(url.Scheme, "https") {
		transportCred = grpc.WithTransportCredentials(insecure.NewCredentials())
	}

	// When adding new interceptors keep in mind that from top to bottom it's outermost
	// to innermost.
	unaryInterceptors := grpc.WithChainUnaryInterceptor(
		initAuthUnaryInterceptor(config.Apikey),
	)

	// When adding new interceptors keep in mind that from top to bottom it's outermost
	// to innermost.
	streamInterceptors := grpc.WithChainStreamInterceptor(
		initAuthStreamInterceptor(config.Apikey),
	)

	return grpc.DialContext(
		ctx,
		config.Uri,
		transportCred,
		unaryInterceptors,
		streamInterceptors,
	)
}
