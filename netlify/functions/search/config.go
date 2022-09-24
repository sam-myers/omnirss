package main

import (
	"github.com/pkg/errors"
	spotifyauth "github.com/zmb3/spotify/v2/auth"
	"golang.org/x/oauth2/clientcredentials"
)

type Config struct {
	// Server
	BaseUrl string `env:"BASE_URL"`

	// Airtable
	SpotifyId     string `yaml:"spotify_id"     env:"SPOTIFY_ID"`
	SpotifySecret string `yaml:"spotify_secret" env:"SPOTIFY_SECRET"`

	// Logging
	Debug bool `yaml:"debug" env:"DEBUG" env-default:"false"`
}

func (c *Config) ClientCredentials() *clientcredentials.Config {
	return &clientcredentials.Config{
		ClientID:     c.SpotifyId,
		ClientSecret: c.SpotifySecret,
		TokenURL:     spotifyauth.TokenURL,
	}
}

var config Config

func (c *Config) RequiredVarsSet() error {
	if c.SpotifyId == "" {
		return errors.New("SPOTIFY_ID is required")
	}

	if c.SpotifySecret == "" {
		return errors.New("SPOTIFY_SECRET is required")
	}

	if c.BaseUrl == "" {
		return errors.New("BASE_URL is required")
	}

	return nil
}
