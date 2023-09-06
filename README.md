# Flairs

## Setup
1. Find your DB's IP Address

```sh
docker ps
```

Find the name of your Lemmy instance. It should be something like:
`yourlemmydomainnospaces_proxy_1`. It'll likely be the first name that appears
on the `docker ps'` list.

Then, run the following command:

```sh
sudo docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' yourlemmydomainnospaces_proxy_1
```
