package server

import (
	"database/sql"
	_ "embed"
	"errors"
	"fmt"
	// "net/url"
	"os"

	"github.com/charmbracelet/log"
	"github.com/gofiber/fiber/v2"
	_ "github.com/lib/pq"
)

const VERSION = "flairs v0.0.4"

//go:embed sql/flair.sql
var flairSQL string

func Serve(port string) {
	args := os.Args[1:]
	if len(args) > 0 {
		switch args[0] {
		case "--version", "-v", "version":
			fmt.Println(VERSION)
			os.Exit(0)
		}
	}

	var connStr = os.Getenv("LEMMY_DATABASE_URL")
	if len(connStr) == 0 {
		var err error
		connStr, err = findDatabaseConfig()
		if err != nil {
			log.Fatal(err)
		}
	}
	db, err := sql.Open("postgres", connStr)
	if err != nil {
		log.Fatalf("Unable to authenticate to Lemmy's database: %q. Make sure to check your credientials", err)
	}
	log.Info("Authenticated with Lemmy's database.")
	defer db.Close()

	err = db.Ping()
	if err != nil {
		log.Fatalf("Unable to connect with Lemmy's database. %q", err)
	}

	if _, err := db.Exec(flairSQL); err != nil {
		log.Fatalf("Error: %q", errors.Join(ErrSetupDatabase, err))
	}
	log.Info("Updated database schema")
	log.Info("Starting Flairs server")

	// Actual server
	srv := NewServer(db)
	log.Fatal(srv.Start(port))

}

type Server struct {
	db *sql.DB
}

func NewServer(db *sql.DB) Server {
	return Server{db}
}

func (s *Server) Start(port string) error {
	app := fiber.New()

	app.Get("/api/user", func(c *fiber.Ctx) error {
		// name := url.QueryEscape(c.Query("name"))
		//TODO setup URL params for fetching user flairs
		//TODO add existing flairs to DB tables
		return nil
	})

	return app.Listen(port)
}
