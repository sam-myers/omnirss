package main

import (
	"context"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/ilyakaznacheev/cleanenv"
	"github.com/sirupsen/logrus"
	"github.com/zmb3/spotify/v2"
	spotifyauth "github.com/zmb3/spotify/v2/auth"
)

var log *logrus.Logger
var spotifyClient *spotify.Client

func main() {
	// Config
	err := cleanenv.ReadEnv(&config)

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

	err = config.RequiredVarsSet()
	if err != nil {
		log.WithError(err).Fatal("Required config vars not set")
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
