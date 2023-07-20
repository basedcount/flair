package server

import (
	"github.com/charmbracelet/log"
	"github.com/gofiber/fiber/v2"
)

func Serve() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		return c.SendString("Hello, World!")
	})

	log.Fatal(app.Listen(":3000"))
}
