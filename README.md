# Flair

Flair is an augmentation for [Lemmy](https://join-lemmy.org) that adds user flair support to the backend. It can run both as a REST API microservice, as well as an importable Rust crate. 

## Features
- 🚩 Users can pick a user flair among those created by the mods
- 🔃 Users can change or remove their user flair whenever they want to
- ➕ Mods can create and remove user flairs in communities they moderate
- 👮‍♂️ Mods can change or remove other users' flairs

## API Reference
~~The full API documentation is available on our [documentation page](https://lemmy-flair.readme.io/) or in the `swagger.json` file.~~

At the present date the documentation is not yet available.

As detailed by the docs, some requests will require an `Authorization` header. This should be a Lemmy JWT owned by the user making the request. 

No token is required for read only operations such as seeing other people's flairs or seeing the list of a community's flairs.

## Deploy
Flair is designed to be ran adjacently to the Lemmy backend. In its current state, it is only possible to run the microservice from the same server where a Lemmy deployment is running.

Flair only includes a REST API supporting the microservice. If you are looking to add this feature on your Lemmy instance, you should also adopt a new UI. We recommend you check out [kaleidoscope](https://github.com/basedcount/kaleidoscope), a fork of the Lemmy UI built by us to be 100% integrated with Flair.
### Docker
It is recommended to deploy the microservice with Docker, by pulling the latest version of the [image](https://hub.docker.com/repository/docker/ornatot/flair/general) and adding it to the Lemmy docker-compose file.
### Bare metal (Cargo)
Alternatively, it is also possible to run the server with Cargo, using the following command:  
`cargo run -- serve`

If you choose this deployment method, make sure to set the `DOCKER` environment variable to `false`, as detailed in the next paragraph.

## Environment variables
Flair can be configured by modifying the following environment variables:
| `ENV_VAR`                      | type     | default          | description                                                                         |
| ------------------------------ | -------- | ---------------- | ----------------------------------------------------------------------------------- |
| `FLAIRS_PORT`                | `int` | `6969`   | Port where the service will be ran.               |
| `LEMMY_PORT` | `int` | `8536`   | Port where the associated Lemmy instance is running |
| `FLAIR_DB_URL` | `string` | `flairs.db`   | Path where the SQLite DB file will be saved                 |
| `DOCKER` | `bool` | `true`   | `true` if the service is running on Docker, `false` if the service is running on bare metal                 |


## Test
It is possible to test the application by running the `test.ts` script. A local Lemmy instance is required to test PUT and DELETE requests.

The script is developed for Bun but should also run with different Javascript runtimes.