package server

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"reflect"

	"github.com/hjson/hjson-go/v4"

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

	var configPath = filepath.Join(cwd, "config", "config.hjson")
	configLocation := os.Getenv("LEMMY_CONFIG_LOCATION")
	if len(configLocation) > 0 {
		configPath = configLocation
	}

	config, err := os.ReadFile(configPath)
	if err != nil {
		log.Error(err)
		log.Info(
			"Lemmy's default config path is Lemmy's home directory + 'config/config.hjson'.\n" +
				"Try setting the LEMMY_CONFIG_LOCATION environment variable.")

		return "", ErrNoDatabaseConf
	}

	dbConf := SqlConf{}
	configFileName := filepath.Base(configPath)

	switch filepath.Ext(configPath) {
	case "hjson":
		if err := hjson.Unmarshal(config, &dbConf); err != nil {
			log.Warnf("Failed to parse %s", configFileName)
			return "", errors.Join(err, ErrNoDatabaseConf)

		}
	case "json":
		if err := json.Unmarshal(config, &dbConf); err != nil {
			log.Warnf("Failed to parse %s", configFileName)
			return "", errors.Join(err, ErrNoDatabaseConf)
		}
	}

	if err = checkForCredentials(dbConf); err != nil {
		return "", err
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

func checkForCredentials(config SqlConf) error {
	value := reflect.ValueOf(config)
	valType := value.Type()

	var errorList []error

	for i := 0; i < value.NumField(); i++ {
		fieldValue := value.Field(i)
		fieldType := valType.Field(i)

		if fieldType.Name == "Host" || fieldType.Name == "Port" || fieldType.Name == "Database" || fieldType.Name == "User" {
			if fieldValue.IsZero() {
				errorList = append(errorList, fmt.Errorf("%q: %s", ErrMissingCredentials, fieldType.Name))
			}
		}

	}

	return errors.Join(errorList...)
}
