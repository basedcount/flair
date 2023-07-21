package server

import "errors"

var (
	ErrNoDatabaseConf = errors.New("unable to locate database credentials")
)
