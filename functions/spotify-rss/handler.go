package main

import (
	"bytes"
	"context"
	"github.com/aws/aws-lambda-go/events"
	"github.com/gorilla/feeds"
	"github.com/zmb3/spotify/v2"
	"strconv"
	"strings"
	"time"
)

func handler(ctx context.Context, request events.APIGatewayProxyRequest) (*events.APIGatewayProxyResponse, error) {
	log.WithField("request", request).Info("Received request")

	id := request.QueryStringParameters["id"]
	if id == "" {
		log.Warn("Query is empty. Returning validation error")
		return &events.APIGatewayProxyResponse{
			StatusCode: 404,
			Headers:    map[string]string{"Content-Type": "text/plain"},
			Body:       "No ID provided",
		}, nil
	}

	log.WithField("id", id).Debug("Fetching show from Spotify")

	show, err := spotifyClient.GetShow(ctx, spotify.ID(id), spotify.Market("US"))
	if err != nil {
		log.WithError(err).Warn("Failed to get show")
		return &events.APIGatewayProxyResponse{
			StatusCode: 500,
			Headers:    map[string]string{"Content-Type": "text/plain"},
			Body:       "Failed to get show",
		}, nil
	}

	feed := &feeds.Feed{
		Title:       show.Name,
		Link:        &feeds.Link{Href: show.ExternalURLs["spotify"]},
		Description: show.Description,
		Items:       []*feeds.Item{},
	}

	for _, episode := range show.Episodes.Episodes {
		dateParts := strings.Split(episode.ReleaseDate, "-")
		year, _ := strconv.Atoi(dateParts[0])
		month, _ := strconv.Atoi(dateParts[1])
		day, _ := strconv.Atoi(dateParts[2])
		created := time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.UTC)

		feed.Items = append(feed.Items, &feeds.Item{
			Title:       episode.Name,
			Link:        &feeds.Link{Href: episode.ExternalURLs["spotify"]},
			Description: episode.Description,
			Content:     episode.Description,
			Created:     created,
		})
	}

	var buff bytes.Buffer
	err = feed.WriteRss(&buff)
	if err != nil {
		log.WithError(err).Warn("Failed to write RSS")
		return &events.APIGatewayProxyResponse{
			StatusCode: 500,
			Headers:    map[string]string{"Content-Type": "text/plain"},
			Body:       "Failed to write RSS",
		}, err
	}

	return &events.APIGatewayProxyResponse{
		StatusCode: 500,
		Headers:    map[string]string{"Content-Type": "text/plain"},
		Body:       buff.String(),
	}, nil
}
