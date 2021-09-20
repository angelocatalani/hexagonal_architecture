# Layered Pokedex

[![Actions Status](https://github.com/angelocatalani/pok/actions/workflows/main.yml/badge.svg)](https://github.com/angelocatalani/pok/actions)
[![Actions Status](https://github.com/angelocatalani/pok/actions/workflows/audit.yml/badge.svg)](https://github.com/angelocatalani/pok/actions)
[![Actions Status](https://github.com/angelocatalani/pok/actions/workflows/scheduled_build.yml/badge.svg)](https://github.com/angelocatalani/pok/actions)

`Layered Pokedex` is a 3 layer web application.

# Table of Contents

* [Usage](#usage)
* [Production API changes](#production-api-changes)
* [Resources](#resources)

## Usage

First, we need to clone the repo:

```shell
git clone git@github.com:angelocatalani/newsletter.git
```

and enter the main project directory:

```shell
cd pok
```

Secondly, we can run the server inside a local container, with [docker compose](https://docs.docker.com/get-docker/)

```shell
docker compose up
```

The CI builds and pushes on each successful commit the docker image from that commit.

We can run that image with:

```shell
docker run -p 8080:8080 challengepokedex1/pokedex
```

Finally, if we [install rustup](https://www.rust-lang.org/tools/install), we can run the server locally with:

```shell
cargo run --bin pokedex
```

and the tests with:

```shell
cargo test
```

We can hit the `pokedex` routes with:

 ```shell
curl -vv -X GET localhost:8080/health_check
```

 ```shell
curl -vv -X GET localhost:8080/pokemon/mewtwo
```

 ```shell
curl -vv -X GET localhost:8080/pokemon/translated/mewtwo
```

## Production API changes

### PokeApi GraphQL is in beta

We used the [PokeApi GraphQL](https://pokeapi.co/docs/graphql) to leverage the graph navigation to search a given
Pokemon and return only the relevant subset of all the possible fields. This is beneficial for our server since we
delegate the PokeApi to search and filter a given Pokemon.

However, the [PokeApi GraphQL](https://pokeapi.co/docs/graphql) is in beta and this means it is not stable enough for a
production environment: it could potentially make some braking changes or have bugs, that could break our server.

To improve the stability of our server, we could use instead the [rest endpoint](https://pokeapi.co/docs/v2) that is
stable.

### Funtranslations has a rate limit

We used the free version of the Funtranslations API that has a limit of 6 requests per hour.

In a production environment we could use the premium version with no limit plus a cache mechanism to avoid requesting
the same translation multiple times.

### Circuit Breaker

At the moment our server is directly using synchronous http calls to interact with the PokeAPI and Funtranslations
endpoints.

Our server implements a timeout and proper error handling to avoid waiting/crashing when the external services do not
behave correctly.

However, we could implement a [circuit breaker](https://martinfowler.com/bliki/CircuitBreaker.html)
to retry reasonably the failed request multiple times until returning an error or a valid response.

### Telemetry

Tracing logs should be stored in immutable database for analysis.

We should also collect other metrics to display such as:

- number of requests
- latency of each request
- cpu/memory usage

After that, we could set up warning rules to detect problems.

### Acceptance/Quality/Load tests

We should periodically run automated tests to check the correctness of the entire journey:

- the external APIs (PokeAPI, Funtranslations) work correctly
- our routes take no longer than `x` seconds to return a valid response

Finally, we could simulate stress conditions for our server with load tests.

### Rate limit

We could use a simple API token to implement a rate limit for our routes.

In this way, we could mitigate a DOS attack.

### Load balancer and autoscaler

We could use the load balancer to distribute requests across many servers.

We could use the autoscaler to spawn new servers when necessary (e.g., high cpu/memory/#requests)

### Configure CORS for Actix

At the moment our server does not define any CORS policy, and the browsers fall back to the SOP (same origin policy).

For to the front end of our application, it could be necessary to define a CORS policy.

## Resources

- [Assignment](https://docs.google.com/document/d/1P5i5AdnnJ7jTpxBJ6vrNGz-yGIT3zl68a94YZKuQovg/edit#)







