# Hexagonal Architecture

Notes and proof of concepts about the Hexagonal Architecture in Rust.

# Table of contents

* [3-Layer architecture](#3-Layer-architecture)
* [Hexagonal architecture](#Hexagonal-architecture)
* [Resources](#resources)

## 3 Layer architecture

### Layered Pokedex

`Layered Pokedex` is a 3 layer web application.

Initially the `translated` endpoint does not support caching.

The code is clean and fully tested. So far the layered approach has been the best approach.

To implement a caching mechanism with Redis we cannot inject the Redis client, inside the `Translated` service because:

- the resulting code would be difficult to unit-test (how to mock the Redis client).
- the `Translated` service would have an additional reason to change

So, the best approach is to create a cache service that wraps the Redis client.

This means, the `translated` endpoint is in charge of checking and updating the cache.

For Redis connections we are using multiplexing instead of a connection pool, basically for simplicity (avoid adding
the `bb8` dependency)
(https://redis.com/blog/multiplexing-explained/)

### Drawbacks

#### The SRP is violated

If the persistence layer changes, the domain layer also changes. Same thing for the presentation layer.

#### Promotes database-driven design instead of domain-driven

To develop a feature we need to start with the persistence layer.

However, it is the domain that contains the behaviour of the features.

And the behaviour should be independent of the persistence aspect.

#### The design does not reflect the natural structure of the business requirements.

The division into presentation, domain and persistence is a technical choice which breaks up cohesive functional units,
into at least 3 distinct pieces for the corresponding layers.

This means a given feature is implemented in 3 different layers instead of a single cohesive unit.

#### Domain logic is difficult to test

Domain logic depends on the persistence layer without any form of dependency injection.

This means it is impossible to unit test it, since we need an up and running database.

#### ORM can mix domain and persistence logic

When using ORM in a layered application, we could end up using for the domain code, the ORM classes or adding ORM
annotations to our domain classes.

This makes the domain, and the persistence layer even more tightly coupled.

## Hexagonal architecture

The violation of the SRP principle in the 3-layer architecture is due to the propagation of changes from the lower
layers to the upper layers.

To stop this change propagation we can use the DIP.

The behaviour of our business requirement: the core code must not depend on external dependencies associated with any
kind of integration points.

The core code interacts with the external dependencies using interfaces. In this way, the core code does not have any
outgoing dependencies. And all dependencies are towards the core code.

In terms of hexagonal architecture we have:

- the hexagon: core code + ports + services
- adapters

The ports can be:

- incoming port (e.g., Use-case, Query): implemented by services (query, use case, ...)
- outgoing port (e.g., LoadAccountPort)

The adapters can be:

- incoming adapter (e.g., Controller): calls the incoming port, and the concrete implementation is inside the hexagon
- outgoing adapter (e.g., Repository): implements the outgoing port, and are called by the hexagon

Incoming ports are the only way to interact with the hexagon. Incoming ports are use cases implemented by a service in
the hexagon. Incoming ports are used by incoming adapters.

Outgoing ports are the only way for the hexagon to interact with the integration points. Outgoing ports are implemented
by outgoing adapters Outgoing ports are used by the hexagon (e.g., the service) to interact with the integration points.

Incoming ports are not implemented by the incoming adapater: they are implemented by the service and used by the
incoming adapter.

Outgoing ports are implemented by the outgoing adapter

## Resources

- [Get Your Hands Dirty on Clean Architecture](https://reflectoring.io/book/)
- [Buckpal App](https://github.com/thombergs/buckpal)
- [Reevaluating the Layered Architecture](https://javadevguy.wordpress.com/2019/01/06/reevaluating-the-layered-architecture/)