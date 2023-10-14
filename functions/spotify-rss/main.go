package main

import (
	"context"
	"github.com/aws/aws-lambda-go/lambda"
	omnirssconfig "github.com/sam-myers/omnirss/packages/config"
	"github.com/sirupsen/logrus"
	"github.com/zmb3/spotify/v2"
	spotifyauth "github.com/zmb3/spotify/v2/auth"
	"github.com/getsentry/sentry-go"
)

var config *omnirssconfig.Config
var log *logrus.Logger
var spotifyClient *spotify.Client

func main() {
	var err error

	// Config
	config, err = omnirssconfig.NewConfigFromEnv()
	if err != nil {
		log.WithError(err).Fatal("Failed to read config")
		sentry.CaptureException(err)
	}

	// Init Sentry
	err = sentry.Init(sentry.ClientOptions{
		Dsn: config.SentryDsn,
	})
	if err != nil {
		log.WithError(err).Fatal("sentry.Init: %s", err)
		sentry.CaptureException(err)
	}
	defer sentry.Flush(time.Second * 5)

	// Init logging
	log = logrus.New()
	log.Formatter = &logrus.TextFormatter{}
	if config.Debug {
		log.SetLevel(logrus.DebugLevel)
	} else {
		log.SetLevel(logrus.InfoLevel)
	}

	// Init Spotify
	ctx := context.Background()
	token, err := config.ClientCredentials().Token(ctx)
	if err != nil {
		newErr := fmt.Errorf("Failed to get Spotify token: %w", err)
		log.WithError(newErr).Fatal()
		sentry.CaptureException(newErr)
	}
	httpClient := spotifyauth.New().Client(ctx, token)
	spotifyClient = spotify.New(httpClient)

	lambda.Start(handler)
}
