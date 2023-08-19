package server

import "errors"

var (
	ErrMissingCredentials = errors.New("missing database credential")
	ErrNoDatabaseConf = errors.New("unable to locate database credentials")
)
