package server

import "errors"

var (
	ErrSetupDatabase = errors.New("issue setting up database")
	ErrMissingCredentials = errors.New("missing database credential")
	ErrNoDatabaseConf = errors.New("unable to locate database credentials")
)
