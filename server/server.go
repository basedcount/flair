package server

import (
	"database/sql"

	"github.com/charmbracelet/log"
	// "github.com/gofiber/fiber/v2"
)

func Serve(port string) {
	connStr, err := findDatabaseConfig()
	if err != nil {
		log.Fatal(err)
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
