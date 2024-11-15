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
	assetName = "SiftLV426"
	clientKey = "sift-lv426-v1"
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

	// RNG to simulate real data
	rng := rand.New(rand.NewSource(time.Now().UnixNano()))

	start := time.Now()
	duration := 60 * time.Second

	readingsFrequencyHz := 1.5
	logsFrequencyHz := 2.0
	readingsInterval := 1.0 / readingsFrequencyHz
	logsInterval := 1.0 / logsFrequencyHz

	lastReading := time.Now()
	lastLog := time.Now()

	for time.Since(start) < duration {
		current := time.Now()

		if current.Sub(lastReading) >= time.Duration(readingsInterval) {
			req := &ingestv1.IngestWithConfigDataStreamRequest{
				IngestionConfigId: ingestionConfig.IngestionConfigId,
				RunId:             run.RunId,
				Flow:              "reading",
				Timestamp:         timestamppb.New(current),
				ChannelValues: []*ingestv1.IngestWithConfigDataChannelValue{
					// velocity channel
					{Type: &ingestv1.IngestWithConfigDataChannelValue_Double{Double: rng.Float64()}},

					// voltage channel
					{Type: &ingestv1.IngestWithConfigDataChannelValue_Double{Double: rng.Float64()}},
				},
				// Use this flag only for debugging purposes to get real-time data validation from
				// the Sift API. Do not use in production as it will hurt performance.
				EndStreamOnValidationError: true,
			}
			if err := siftStream.Send(req); err != nil {
				log.Fatalln(err)
			}
			log.Println("ingested a reading flow")
		}

		if current.Sub(lastLog) >= time.Duration(logsInterval) {
			req := &ingestv1.IngestWithConfigDataStreamRequest{
				IngestionConfigId: ingestionConfig.IngestionConfigId,
				RunId:             run.RunId,
				Flow:              "log",
				Timestamp:         timestamppb.New(current),
				ChannelValues: []*ingestv1.IngestWithConfigDataChannelValue{
					// log channel
					{Type: &ingestv1.IngestWithConfigDataChannelValue_String_{String_: "test log emission"}},
				},
				// Use this flag only for debugging purposes to get real-time data validation from
				// the Sift API. Do not use in production as it will hurt performance.
				EndStreamOnValidationError: true,
			}
			if err := siftStream.Send(req); err != nil {
				log.Fatalln(err)
			}
			log.Println("ingested a log flow")
		}
	}
	log.Println("done.")
}

// Flow and channel configuration
func config() []*ingestion_configsv1.FlowConfig {
	return []*ingestion_configsv1.FlowConfig{
		{
			Name: "reading",
			Channels: []*ingestion_configsv1.ChannelConfig{
				{
					Name:        "velocity",
					Component:   "mainmotor",
					Unit:        "km/hr",
					Description: "vehicle speed",
					DataType:    typev1.ChannelDataType_CHANNEL_DATA_TYPE_DOUBLE,
				},
				{
					Name:        "voltage",
					Unit:        "kV",
					Description: "potential difference",
					DataType:    typev1.ChannelDataType_CHANNEL_DATA_TYPE_DOUBLE,
				},
			},
		},
		{
			Name: "log",
			Channels: []*ingestion_configsv1.ChannelConfig{
				{
					Name:        "log",
					Description: "log",
					DataType:    typev1.ChannelDataType_CHANNEL_DATA_TYPE_STRING,
				},
			},
		},
	}
}

// / Retrieves an existing ingestion config or creates it
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
