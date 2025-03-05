package grpc

import (
	"context"
	"crypto/x509"
	"net"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
)

// Configuration for [SiftChannel].
type SiftChannelConfig struct {
	Uri            string
	Apikey         string
	UseSystemCerts bool
}

// Type alias for a gRPC channel configured specifically to communicate with the Sift API.
type SiftChannel = *grpc.ClientConn

// Initializes a gRPC connection to Sift.
func UseSiftChannel(ctx context.Context, config SiftChannelConfig) (SiftChannel, error) {
	var creds credentials.TransportCredentials
	if config.UseSystemCerts {
		certPool, err := x509.SystemCertPool()
		if err != nil {
			return nil, err
		}
		creds = credentials.NewClientTLSFromCert(certPool, "")

	} else if useInsecure(config.Uri) {
		creds = insecure.NewCredentials()
	} else {
		creds = credentials.NewClientTLSFromCert(nil, "")
	}
	transportCred := grpc.WithTransportCredentials(creds)

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

func useInsecure(uri string) bool {
	host, _, err := net.SplitHostPort(uri)
	if err != nil {
		host = uri
	}
	return host == "localhost" || host == "127.0.0.1" || host == "::1"
}
