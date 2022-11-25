package main

import (
	"context"
	"flag"
	"fmt"
	"log"

	"github.com/chirpstack/chirpstack/api/go/v4/integration"
	"github.com/go-redis/redis/v8"
	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
)

var (
	server string
	key    string
)

func init() {
	flag.StringVar(&server, "server", "localhost:6379", "Redis hostname:port")
	flag.StringVar(&key, "key", "device:stream:event", "Redis Streams key to read from")
	flag.Parse()
}

func main() {
	rdb := redis.NewClient(&redis.Options{
		Addr: server,
	})
	ctx := context.Background()

	lastID := "0"

	for {
		resp, err := rdb.XRead(ctx, &redis.XReadArgs{
			Streams: []string{key, lastID},
			Count:   10,
			Block:   0,
		}).Result()
		if err != nil {
			log.Fatal(err)
		}

		if len(resp) != 1 {
			log.Fatal("Exactly one stream response is expected")
		}

		for _, msg := range resp[0].Messages {
			lastID = msg.ID

			if b, ok := msg.Values["up"].(string); ok {
				var pl integration.UplinkEvent
				if err := proto.Unmarshal([]byte(b), &pl); err != nil {
					log.Fatal(err)
				}

				fmt.Println("=== UP ===")
				fmt.Println(protojson.Format(&pl))
				fmt.Println("==========")
			}

			if b, ok := msg.Values["join"].(string); ok {
				var pl integration.JoinEvent
				if err := proto.Unmarshal([]byte(b), &pl); err != nil {
					log.Fatal(err)
				}

				fmt.Println("=== JOIN ===")
				fmt.Println(protojson.Format(&pl))
				fmt.Println("============")
			}

			if b, ok := msg.Values["ack"].(string); ok {
				var pl integration.AckEvent
				if err := proto.Unmarshal([]byte(b), &pl); err != nil {
					log.Fatal(err)
				}

				fmt.Println("=== ACK ===")
				fmt.Println(protojson.Format(&pl))
				fmt.Println("===========")
			}

			if b, ok := msg.Values["txack"].(string); ok {
				var pl integration.TxAckEvent
				if err := proto.Unmarshal([]byte(b), &pl); err != nil {
					log.Fatal(err)
				}

				fmt.Println("=== TX ACK ===")
				fmt.Println(protojson.Format(&pl))
				fmt.Println("==============")
			}

			if b, ok := msg.Values["log"].(string); ok {
				var pl integration.LogEvent
				if err := proto.Unmarshal([]byte(b), &pl); err != nil {
					log.Fatal(err)
				}

				fmt.Println("=== LOG ===")
				fmt.Println(protojson.Format(&pl))
				fmt.Println("===========")
			}

			if b, ok := msg.Values["status"].(string); ok {
				var pl integration.StatusEvent
				if err := proto.Unmarshal([]byte(b), &pl); err != nil {
					log.Fatal(err)
				}

				fmt.Println("=== STATUS ===")
				fmt.Println(protojson.Format(&pl))
				fmt.Println("==============")
			}

			if b, ok := msg.Values["location"].(string); ok {
				var pl integration.LocationEvent
				if err := proto.Unmarshal([]byte(b), &pl); err != nil {
					log.Fatal(err)
				}

				fmt.Println("=== LOCATION ===")
				fmt.Println(protojson.Format(&pl))
				fmt.Println("================")
			}

			if b, ok := msg.Values["integration"].(string); ok {
				var pl integration.IntegrationEvent
				if err := proto.Unmarshal([]byte(b), &pl); err != nil {
					log.Fatal(err)
				}

				fmt.Println("=== INTEGRATION ===")
				fmt.Println(protojson.Format(&pl))
				fmt.Println("===================")
			}
		}
	}
}
