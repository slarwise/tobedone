# To Be Done

A TODO app with a twist!

- Sign up
- Get your list
- Random items are added within a configurable interval
- Mark items as done and they disappear
- Each completed item generates 33 credits
- Credits can be used to complete items that you actually haven't completed in
  real life
- If you have more than 100 credits you get a star next to your username
- Generate a QR code to show your credit rating
- All items are string
- An item can be an SQL injection

## Architecture

- Item generator
- Frontend
- Database that stores the items for each user
- Some kind of cron job that adds items

## Observability

- Grafana: Display metrics, tracing, logging
- Metrics: Prometheus
- Tracing: Something that uses opentelemetry
- Logging: Loki

## Operations

- Kubernetes cluster with the item generator, frontend and cronjob
- Persistent database

## CI

- Run tests

## CD

- Upload new docker images to ghcr on pushes to main
- ArgoCD to roll out new deployments when new images are available

## Documentation

- OpenAPI for the microservices
- Database schema
- Local development instructions

## Local development

### Items

`cargo run` to run the app. Build the image and start a container by doing:

```sh
docker build -t items ./items
docker run --rm items
```

View the openapi docs with `make docs`.
