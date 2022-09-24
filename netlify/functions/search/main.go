package main

import (
	"context"
	"github.com/aws/aws-lambda-go/lambda"
	omnirssconfig "github.com/sam-myers/omnirss/packages/config"
	"github.com/sirupsen/logrus"
	"github.com/zmb3/spotify/v2"
	spotifyauth "github.com/zmb3/spotify/v2/auth"
	"html/template"
)

var config *omnirssconfig.Config
var log *logrus.Logger
var spotifyClient *spotify.Client
var htmlTemplate *template.Template

func main() {
	var err error

	// Config
	config, err = omnirssconfig.NewConfigFromEnv()

	// Init logging
	log = logrus.New()
	log.Formatter = &logrus.TextFormatter{}
	if config.Debug {
		log.SetLevel(logrus.DebugLevel)
	} else {
		log.SetLevel(logrus.InfoLevel)
	}

	if err != nil {
		log.WithError(err).Fatal("Failed to read config")
	}

	// Load Template
	htmlTemplate, err = template.ParseFiles("template.html")
	if err != nil {
		log.WithError(err).Fatal("Failed to load template")
	}

	// Init Spotify
	ctx := context.Background()
	token, err := config.ClientCredentials().Token(ctx)
	if err != nil {
		log.WithError(err).Fatal("Failed to get Spotify token")
	}
	httpClient := spotifyauth.New().Client(ctx, token)
	spotifyClient = spotify.New(httpClient)

	lambda.Start(handler)
}
