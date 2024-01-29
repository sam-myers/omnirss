module github.com/sam-myers/omnirss/netlify/functions/spotify-rss

go 1.19

require (
	github.com/aws/aws-lambda-go v1.46.0
	github.com/sam-myers/omnirss/packages/config v0.0.0
	github.com/sirupsen/logrus v1.9.3
	github.com/zmb3/spotify/v2 v2.3.0
)

require (
	github.com/BurntSushi/toml v1.1.0 // indirect
	github.com/golang/protobuf v1.5.2 // indirect
	github.com/gorilla/feeds v1.1.1
	github.com/ilyakaznacheev/cleanenv v1.4.0 // indirect
	github.com/joho/godotenv v1.4.0 // indirect
	golang.org/x/net v0.7.0 // indirect
	golang.org/x/oauth2 v0.1.0 // indirect
	golang.org/x/sys v0.5.0 // indirect
	google.golang.org/appengine v1.6.7 // indirect
	google.golang.org/protobuf v1.28.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
	olympos.io/encoding/edn v0.0.0-20201019073823-d3554ca0b0a3 // indirect
)

replace github.com/sam-myers/omnirss/packages/config v0.0.0 => ./../packages/config
