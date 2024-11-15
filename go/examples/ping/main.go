package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"github.com/sift-stack/sift/go/gen/sift/ping/v1"
	"github.com/sift-stack/sift/go/grpc"
)

func main() {
	ctx := context.Background()
	channelConfig := grpc.SiftChannelConfig{
		Uri:    os.Getenv("SIFT_URI"),
		Apikey: os.Getenv("SIFT_API_KEY"),
	}
	conn, err := grpc.UseSiftChannel(ctx, channelConfig)
	if err != nil {
		log.Fatalln(err)
	}
	pingClient := pingv1.NewPingServiceClient(conn)
	res, err := pingClient.Ping(ctx, &pingv1.PingRequest{})
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println(res.Response)
}
