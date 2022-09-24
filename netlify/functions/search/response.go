package main

import (
	"encoding/json"
	"github.com/aws/aws-lambda-go/events"
)

type Response struct {
	Query   string         `json:"query"`
	Results []SearchResult `json:"results"`
}

type SearchResult struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	ImageUrl    string `json:"image_url"`
	RssUrl      string `json:"rss_url"`
}

func (r *Response) ToHttp() (*events.APIGatewayProxyResponse, error) {
	bytes, err := json.Marshal(r)
	if err != nil {
		return responseInternalError(err)
	}

	return makeHttpResponse(200, string(bytes), nil)
}

func responseValidationError() (*events.APIGatewayProxyResponse, error) {
	return makeHttpResponse(400, "{}", nil)
}

func responseInternalError(err error) (*events.APIGatewayProxyResponse, error) {
	return makeHttpResponse(500, "{}", err)
}

func makeHttpResponse(code int, body string, err error) (*events.APIGatewayProxyResponse, error) {
	return &events.APIGatewayProxyResponse{
		StatusCode: code,
		Headers:    map[string]string{"Content-Type": "text/json"},
		Body:       body,
	}, err
}
