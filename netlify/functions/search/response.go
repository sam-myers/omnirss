package main

import (
	"bytes"
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
	var buff bytes.Buffer
	if err := htmlTemplate.Execute(&buff, r); err != nil {
		return responseInternalError(err)
	}
	return &events.APIGatewayProxyResponse{
		StatusCode: 200,
		Headers:    map[string]string{"Content-Type": "text/html"},
		Body:       buff.String(),
	}, nil
}

func responseValidationError() (*events.APIGatewayProxyResponse, error) {
	return &events.APIGatewayProxyResponse{
		StatusCode: 400,
		Headers:    map[string]string{"Content-Type": "text/json"},
		Body:       "",
	}, nil
}

func responseInternalError(err error) (*events.APIGatewayProxyResponse, error) {
	return &events.APIGatewayProxyResponse{
		StatusCode: 500,
		Headers:    map[string]string{"Content-Type": "text/json"},
		Body:       "",
	}, nil
}
