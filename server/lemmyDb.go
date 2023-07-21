package server

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"

	"github.com/charmbracelet/log"
)

type SqlConf struct {
	Uri      string `json:"uri,omitempty"`
	Host     string `json:"host,omitempty"`
	Port     int    `json:"port,omitempty"`
	User     string `json:"user,omitempty"`
	Password string `json:"password,omitempty"`
	Database string `json:"database,omitempty"`
	PoolSize int    `json:"pool_size,omitempty"`
}

func (s SqlConf) String() string {
	return fmt.Sprintf("host=%s port=%d user=%s "+
		"password=%s dbname=%s sslmode=disable",
		s.Host, s.Port, s.User, s.Password, s.Database)
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
		return "", ErrNoDatabaseConf
	}

	dbConf := &SqlConf{}
	if err := json.Unmarshal(config, dbConf); err != nil {
		return "", errors.Join(err, ErrNoDatabaseConf)

	}

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
