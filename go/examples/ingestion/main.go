package main

import (
	"context"
	"fmt"
	"log"
	"math/rand"
	"os"
	"time"

	"github.com/sift-stack/sift/go/gen/sift/common/type/v1"
	ingestv1 "github.com/sift-stack/sift/go/gen/sift/ingest/v1"
	"github.com/sift-stack/sift/go/gen/sift/ingestion_configs/v1"
	"github.com/sift-stack/sift/go/gen/sift/runs/v2"
	"github.com/sift-stack/sift/go/grpc"
	"google.golang.org/protobuf/types/known/timestamppb"
)

const (
	assetName = "Sift-LV-426"
	clientKey = "sift-lv-426-v1"
)

func main() {
	ctx := context.Background()

	grpcChannel, err := grpc.UseSiftChannel(ctx, grpc.SiftChannelConfig{
		Uri:    os.Getenv("SIFT_URI"),
		Apikey: os.Getenv("SIFT_API_KEY"),
	})
	if err != nil {
		log.Fatalln(err)
	}

	ingestionConfig, err := getOrCreateIngestionConfig(ctx, grpcChannel, assetName, clientKey)
	if err != nil {
		log.Fatalln(err)
	}
	log.Printf("initialized ingestion config %s\n", ingestionConfig.ClientKey)

	run, err := createRun(ctx, grpcChannel, assetName)
	if err != nil {
		log.Fatalln(err)
	}
	log.Printf("initialized run %s\n", run.Name)

	siftStream, err := ingestv1.NewIngestServiceClient(grpcChannel).IngestWithConfigDataStream(ctx)
	if err != nil {
		log.Fatalln(err)
	}

	dataStream := dataSource()

	for data := range dataStream {
		req := &ingestv1.IngestWithConfigDataStreamRequest{
			IngestionConfigId: ingestionConfig.IngestionConfigId,
			RunId:             run.RunId,
			Flow:              "velocity_reading",
			Timestamp:         timestamppb.New(data.Timestamp),
			ChannelValues: []*ingestv1.IngestWithConfigDataChannelValue{
				{Type: &ingestv1.IngestWithConfigDataChannelValue_Double{Double: data.Value}},
			},
			// Set this flag to `true` only for debugging purposes to get real-time data validation from
			// the Sift API. Do not use in production as it will hurt performance.
			EndStreamOnValidationError: false,
		}
		if err := siftStream.Send(req); err != nil {
			log.Fatalln(err)
		}
		log.Println("ingested a velocity_reading flow")
	}

	// Close the stream when finished and check if there are any errors
	if _, err := siftStream.CloseAndRecv(); err != nil {
		log.Fatalln(err)
	}

	log.Println("done.")
}

type dataPoint struct {
	Timestamp time.Time
	Value     float64
}

func dataSource() <-chan dataPoint {
	dataChannel := make(chan dataPoint)
	go func() {
		rng := rand.New(rand.NewSource(time.Now().UnixNano()))
		duration := 60 * time.Second
		start := time.Now()

		for time.Since(start) < duration {
			dataChannel <- dataPoint{
				Timestamp: time.Now(),
				Value:     rng.Float64(),
			}
			time.Sleep(500 * time.Millisecond)
		}
	}()
	return dataChannel
}

// Flow and channel configuration
func config() []*ingestion_configsv1.FlowConfig {
	return []*ingestion_configsv1.FlowConfig{
		{
			Name: "velocity_reading",
			Channels: []*ingestion_configsv1.ChannelConfig{
				{
					Name:        "velocity",
					Component:   "mainmotor",
					Unit:        "km/hr",
					Description: "vehicle speed",
					DataType:    typev1.ChannelDataType_CHANNEL_DATA_TYPE_DOUBLE,
				},
			},
		},
	}
}

// Retrieves an existing ingestion config or create it.
func getOrCreateIngestionConfig(
	ctx context.Context,
	grpcChannel grpc.SiftChannel,
	assetName,
	clientKey string,
) (*ingestion_configsv1.IngestionConfig, error) {
	svc := ingestion_configsv1.NewIngestionConfigServiceClient(grpcChannel)

	listRes, err := svc.ListIngestionConfigs(ctx, &ingestion_configsv1.ListIngestionConfigsRequest{
		Filter: fmt.Sprintf("client_key == '%s'", clientKey),
	})
	if err != nil {
		return nil, err
	}
	if listRes != nil && len(listRes.IngestionConfigs) > 0 {
		return listRes.IngestionConfigs[0], nil
	}

	createRes, err := svc.CreateIngestionConfig(ctx, &ingestion_configsv1.CreateIngestionConfigRequest{
		AssetName: assetName,
		ClientKey: clientKey,
		Flows:     config(),
	})
	if err != nil {
		return nil, err
	}
	return createRes.IngestionConfig, nil
}

// Create a run to use to group all the data ingested during this period.
func createRun(
	ctx context.Context,
	grpcChannel grpc.SiftChannel,
	runName string,
) (*runsv2.Run, error) {
	svc := runsv2.NewRunServiceClient(grpcChannel)
	ts := timestamppb.Now()

	createRes, err := svc.CreateRun(ctx, &runsv2.CreateRunRequest{
		Name:      fmt.Sprintf("[%s].%d", runName, ts.Seconds),
		StartTime: ts,
	})
	if err != nil {
		return nil, err
	}
	return createRes.Run, nil
}
