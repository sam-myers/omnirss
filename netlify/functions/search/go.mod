module github.com/sam-myers/omnirss/functions/search

go 1.19

require (
	github.com/aws/aws-lambda-go v1.34.1
	github.com/sam-myers/omnirss/packages/config v0.0.0
	github.com/sirupsen/logrus v1.9.0
	github.com/zmb3/spotify/v2 v2.3.0
)

replace github.com/sam-myers/omnirss/packages/config v0.0.0 => ../../../packages/config

require (
	github.com/BurntSushi/toml v1.1.0 // indirect
	github.com/golang/protobuf v1.5.2 // indirect
	github.com/ilyakaznacheev/cleanenv v1.3.0 // indirect
	github.com/joho/godotenv v1.4.0 // indirect
	golang.org/x/net v0.0.0-20220624214902-1bab6f366d9e // indirect
	golang.org/x/oauth2 v0.0.0-20220909003341-f21342109be1 // indirect
	golang.org/x/sys v0.0.0-20220715151400-c0bba94af5f8 // indirect
	google.golang.org/appengine v1.6.7 // indirect
	google.golang.org/protobuf v1.28.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
	olympos.io/encoding/edn v0.0.0-20201019073823-d3554ca0b0a3 // indirect
)
