package server

import (
	"database/sql"
	"fmt"
	"os"

	"github.com/charmbracelet/log"
	// "github.com/gofiber/fiber/v2"
)

const VERSION = "v0.0.1 (3)"

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
		log.Fatal("Unable to authenticate to Lemmy's database. Make sure to check your credientials.")
	}
	log.Info("Authenticated with Lemmy's database.")
	defer db.Close()

	err = db.Ping()
	if err != nil {
		log.Fatal("Unable to connect with Lemmy's database.")
	}

	// app := fiber.New()

	// app.Get("/user", func(c *fiber.Ctx) error {
	// 	return c.SendString("Hello, World!")
	// })

	// log.Fatal(app.Listen(port))
}
