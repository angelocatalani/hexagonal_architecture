# Pokedex

[![Actions Status](https://github.com/angelocatalani/pok/actions/workflows/main.yml/badge.svg)](https://github.com/angelocatalani/pok/actions)
[![Actions Status](https://github.com/angelocatalani/pok/actions/workflows/audit.yml/badge.svg)](https://github.com/angelocatalani/pok/actions)

# Table of Contents

* [Usage](#usage)
* [Production API](#production-api)
* [Resources](#resources)

## Usage

First, we need to clone the repo:

```shell
git clone git@github.com:angelocatalani/newsletter.git
```

Secondly, we can run the server inside a local container, with `docker compose`

```shell
docker compose up
```

The CI builds and pushes on each successful commit the docker image.

We can run it locally in a new container with:

```shell
docker run -p 8080:8080 challengepokedex1/pokedex
```

We can also run the server locally with:

```shell
cargo run --bin pokedex
```

and the tests with:

```shell
cargo test
```

We can hit the `pokedex` rotes with:

 ```shell
curl -vv -X GET localhost:8080/health_check
```

 ```shell
curl -vv -X GET localhost:8080/pokemon/mewtwo
```

## Production API

TODO

## Resources

- [Assignment](https://docs.google.com/document/d/1P5i5AdnnJ7jTpxBJ6vrNGz-yGIT3zl68a94YZKuQovg/edit#)







