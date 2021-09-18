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

The violation of the SRP principle in the 3-layer architecture is due to the propagation of changes from the lower
layers to the upper layers.

To stop this change propagation we could use the DIP.

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

### Introduction

The hexagonal architecture organizes the code to make the business rules independent
of any kind of integration point (db, frameworks, UI, ...).

Independence means the business rules do not have external dependencies, but all
the other components of the application depends on the business rules.

The business rules are the `application core`.

The other components are called `adaptors` and interact with the `application core`
using the `ports`. 

The `application core` and the `ports` represents the hexagon.

The ports can be:

- incoming port (e.g., Use-case, Query): implemented by `services` inside the hexagon
- outgoing port (e.g., LoadAccountPort): implemented outside the hexagon

The adapters can be:

- incoming adapter (e.g., Controller): calls the incoming port
- outgoing adapter (e.g., Repository): implements the outgoing port

### Incoming ports (use cases) and services
https://stackoverflow.com/questions/62818105/interface-for-use-cases-application-services

In short, you usually don't need interfaces on use cases (incoming ports)
because your primary (incoming) adapters (client of your hexagon) depend upon the hexagon by nature.

What is more important is to make sure that your Application services only access interfaces
rather than concrete implementations that are dependent on infrastructure.
That means application services should only depend on interfaces to use repositories
or components that access other infrastructure.

https://blog.allegro.tech/2020/05/hexagonal-architecture-by-example.html

It is often assumed that each port needs to be an interface,
though it does not make much sense for inbound ports.

Interfaces, in general, allow you to decouple implementation from the component that uses it,
following the Dependency Inversion Principle.
They are essential to decouple the domain ArticleService from ExternalServiceClientAuthorRepository
hidden behind the AuthorRepository port.

Hiding ArticleService behind an interface (especially a meaningless IArticleService)
would most likely be seen as over-engineering and would give you nothing in return.

### Package Organization

Organize packages by bounded context (`feature/`):
- `adapter/[in|out]/...` 
- `domain/...`
- `application/[services|out_ports]/...` or `application/[services|ports]/[in|out]/...`

This structure reduces the:
- architecture-code gap
- aka screaming architecture

It is important to note:
- no dependencies from `application` to the `adapter`
- may be dependencies from the `application` and the `domain`
- may be dependencies from the `adapter` and the `domain`

### Domain model

The domain model can be rich or anemic.

If it is rich it implements the business rules operations and validations.
Otherwise, it is a data structure, and the business validations are delegated to the use case/service.

Domain models keep a state that can be modified by the business rules operations.
This because the domain models are the inputs to the outgoing ports that can for example
store the updated domain model to the db.

I prefer having a rich domain model so that the use case/service only need to orchestrate calls
to the domain models and outgoing ports.

### Service/Use case

As said before, I prefer having the service without the incoming port.
This means service and use-case are the same thing.

The use case:
- takes the valid input (validation can be delegated to the input constructor)
- validates business rules
- updates the domain model state
- returns the output to the caller adapter

The input is called: `...Command` and the constructor verifies its syntactical validity.
To avoid coupling between services, it is better to have a dedicated input for each service.

Validating business rules is the semantically validity of the use case.
It can happen at the domain model or in the use case.
Since I choose to have a rich domain model it happens in the domain model

The output should be dedicated for each service, since it could create coupling between
the calling adapter and the service.
In general, it is better to return as little as possible data.

Read-only use case should be somehow distinguished from services with side effect.
This plays well with the CQRS.
This is easy to be done with interfaces as incoming ports.
In our case, we can use an input called: `...Query`

## Resources

- [Get Your Hands Dirty on Clean Architecture](https://reflectoring.io/book/)
- [Buckpal App](https://github.com/thombergs/buckpal)
- [Reevaluating the Layered Architecture](https://javadevguy.wordpress.com/2019/01/06/reevaluating-the-layered-architecture/)