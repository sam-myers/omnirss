package main

import (
	"context"
	"fmt"
	"github.com/aws/aws-lambda-go/events"
	"github.com/zmb3/spotify/v2"
)

func handler(ctx context.Context, request events.APIGatewayProxyRequest) (*events.APIGatewayProxyResponse, error) {
	log.WithField("request", request).Debug("Received request")

	query := request.QueryStringParameters["query"]
	if query == "" {
		log.Warn("Query is empty. Returning validation error")
		return responseValidationError()
	}

	log.WithField("query", query).Info("Searching for podcasts")

	searchResult, err := spotifyClient.Search(ctx, query, spotify.SearchTypeShow, spotify.Market("US"))
	if err != nil {
		log.WithError(err).Error("Spotify API failed to search for podcasts")
		return responseInternalError(err)
	}

	log.WithField("shows", searchResult.Shows.Shows).Debug("Search result")

	response := Response{
		Query:   query,
		Results: make([]SearchResult, 0),
	}

	for _, show := range searchResult.Shows.Shows {
		imageUrl := ""
		for _, image := range show.Images {
			if image.Height == 300 {
				imageUrl = image.URL
				break
			}
		}

		response.Results = append(response.Results, SearchResult{
			Name:        show.Name,
			Description: show.Description,
			ImageUrl:    imageUrl,
			RssUrl:      fmt.Sprintf("%s/spotify/id/%s", config.BaseUrl, show.ID),
		})
	}

	log.WithField("response", response).Debug("Search results")

	return response.ToHttp()
}
