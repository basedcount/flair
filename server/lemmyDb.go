package server

import (
	"errors"
	"fmt"
	"github.com/hjson/hjson-go/v4"
	"os"
	"path/filepath"

	"github.com/charmbracelet/log"
)

type SqlConf struct {
	Database struct {
		Uri      string `json:"uri,omitempty"`
		Host     string `json:"host,omitempty"`
		Port     int    `json:"port,omitempty"`
		User     string `json:"user,omitempty"`
		Password string `json:"password,omitempty"`
		Database string `json:"database,omitempty"`
		PoolSize int    `json:"pool_size,omitempty"`
	} `json:"database,omitempty"`
}

func (s SqlConf) String() string {
	return fmt.Sprintf("host=%s port=%d user=%s "+
		"password=%s dbname=%s sslmode=disable",
		s.Database.Host, s.Database.Port, s.Database.User, s.Database.Password, s.Database.Database)
}

// findDatabaseConfig finds an appropriate connection to the Lemmy Database.
//   - Docs: https://join-lemmy.org/docs/administration/configuration.html
func findDatabaseConfig() (string, error) {
	cwd, err := os.Getwd()
	if err != nil {
		log.Warn("Cannot access current directory. Looking for $LEMMY_DATABASE_URL")
		return getLemmyDbEnv()

	}

	var defaultConfig = filepath.Join(cwd, "config", "config.hjson")
	configLocation := os.Getenv("LEMMY_CONFIG_LOCATION")
	if len(configLocation) > 0 {
		defaultConfig = configLocation
	}

	config, err := os.ReadFile(defaultConfig)
	if err != nil {
		log.Error(err)
		log.Info(
			"Lemmy's default config path is Lemmy's home directory + 'config/config.hjson'.\n" +
				"Try setting the LEMMY_CONFIG_LOCATION environment variable.")

		return "", ErrNoDatabaseConf
	}

	dbConf := &SqlConf{}
	if err := hjson.Unmarshal(config, dbConf); err != nil {
		log.Warn("Failed to parse config.hjson")
		return "", errors.Join(err, ErrNoDatabaseConf)

	}

	// log.Print(dbConf.String())
	return dbConf.String(), nil
}

func getLemmyDbEnv() (string, error) {
	lemmyDbUrl := os.Getenv("LEMMY_DATABASE_URL")
	if len(lemmyDbUrl) != 0 {
		return lemmyDbUrl, nil
	}

	log.Info("Databse Config", "not found", "LEMMY_DATABASE_URL")
	return "", ErrNoDatabaseConf
}
