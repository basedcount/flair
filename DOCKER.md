Personal notes for publishing new versions of the bot with Docker:

# Build new container
`docker build -t ornatot/flair .`

# Run locally
`docker run -p 6969:6969 ornatot/flair`

# Publish image
`docker login`

`docker push ornatot/flair`