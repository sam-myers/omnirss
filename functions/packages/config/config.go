package config

import (
	"fmt"
	"github.com/getsentry/sentry-go"
	"github.com/ilyakaznacheev/cleanenv"
	spotifyauth "github.com/zmb3/spotify/v2/auth"
	"golang.org/x/oauth2/clientcredentials"
)

type Config struct {
	// Server
	Url string `env:"URL"`

	// Spotify
	SpotifyId     string `yaml:"spotify_id"     env:"SPOTIFY_ID"`
	SpotifySecret string `yaml:"spotify_secret" env:"SPOTIFY_SECRET"`

	// Sentry
	SentryDsn string `env:"SENTRY_DSN"`

	// Logging
	Debug bool `yaml:"debug" env:"DEBUG" env-default:"false"`
}

func NewConfigFromEnv() (*Config, error) {
	config := &Config{}
	err := cleanenv.ReadEnv(config)
	if err != nil {
		return nil, err
	}

	if err := sentry.Init(sentry.ClientOptions{
		Dsn: config.SentryDsn,
	}); err != nil {
		return nil, fmt.Errorf("sentry.Init: %s", err)
	}

	if err := config.requiredVarsSet(); err != nil {
		return nil, err
	}

	return config, nil
}

func (c *Config) ClientCredentials() *clientcredentials.Config {
	return &clientcredentials.Config{
		ClientID:     c.SpotifyId,
		ClientSecret: c.SpotifySecret,
		TokenURL:     spotifyauth.TokenURL,
	}
}

func (c *Config) requiredVarsSet() error {
	if c.SpotifyId == "" {
		return fmt.Errorf("SPOTIFY_ID is required")
	}

	if c.SpotifySecret == "" {
		return fmt.Errorf("SPOTIFY_SECRET is required")
	}

	if c.Url == "" {
		return fmt.Errorf("URL is required")
	}

	if c.SentryDsn == "" {
		return fmt.Errorf("SENTRY_DSN is required")
	}

	return nil
}
		return fmt.Errorf("SPOTIFY_ID is required")
	}

	if c.SpotifySecret == "" {
		return fmt.Errorf("SPOTIFY_SECRET is required")
	}

	if c.Url == "" {
		return fmt.Errorf("URL is required")
	}

	return nil
}
