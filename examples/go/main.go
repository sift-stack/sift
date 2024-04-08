package main

import (
	"context"
	"crypto/tls"
	"fmt"
	"log"
	"os"

	"github.com/joho/godotenv"
	"github.com/sift-go-cli/gen/protos/go/sift/annotations/v1"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/metadata"
)

func main() {
    err := godotenv.Load()
    if err != nil {
        log.Fatalf("Error loading .env file: %v", err)
    }

    siftApiKey := os.Getenv("SIFT_API_KEY")

	if siftApiKey == "" {
        log.Fatalln("Expected 'SIFT_API_KEY' to exist in .env")
	}

    baseUri := os.Getenv("BASE_URI")

	if baseUri == "" {
		log.Fatalln("Expected 'BASE_URI' to exist in .env")
	}

	name := os.Args[1]

	if name == "" {
		log.Fatalln("Expected a name to be provided")
	}

	unaryInterceptor := func(
		ctx context.Context,
		method string,
		req, reply interface{},
		cc *grpc.ClientConn,
		invoker grpc.UnaryInvoker,
		opts ...grpc.CallOption,
	) error {
		ctx = metadata.AppendToOutgoingContext(ctx, "authorization", fmt.Sprintf("Bearer %s", siftApiKey))
		return invoker(ctx, method, req, reply, cc, opts...)
	}

	ctx := context.Background()

	conn, err := grpc.DialContext(
		ctx,
		baseUri,
		grpc.WithTransportCredentials(credentials.NewTLS(&tls.Config{})),
		grpc.WithUnaryInterceptor(unaryInterceptor),
	)

	if err != nil {
		log.Fatalf("Failed to dial: %v", err)
	}

	client := annotationsv1.NewAnnotationServiceClient(conn)

	req := annotationsv1.ListAnnotationsRequest{
		Filter: fmt.Sprintf("name.matches(\"(?i)%s\")", name),
		PageSize: 10,
		PageToken: "",
	}

	res, err := client.ListAnnotations(ctx, &req)

	if err != nil {
		log.Fatalln("Failed to get annotations with err: %v", err)
	}

	out := ""

	if len(res.Annotations) == 0 {
		log.Printf("No annotations found whose name matches '%s'\n", name)
		return
	}

	for _, annotation := range res.Annotations {
		out += fmt.Sprintf(`
Annotation ID: %s
	Name: %s
	Description: %s
	State: %s
	Type: %s
	Created at: %v
	Modified at: %v
	Created by rule condition ID: %s
		`,
		annotation.GetAnnotationId(),
		annotation.GetName(),
		annotation.GetDescription(),
		annotation.GetState(),
		annotation.GetAnnotationType(),	
		annotation.GetCreatedDate().AsTime(),
		annotation.GetModifiedDate().AsTime(),
		annotation.GetCreatedByConditionId(),
	)
	}

	fmt.Println(out)
}
