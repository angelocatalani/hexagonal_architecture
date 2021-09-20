# Hexagonal Architecture

Notes and proof of concepts about the Hexagonal Architecture in Rust.

# Table of contents

* [3-Layer architecture](#3-Layer-architecture)
* [Hexagonal architecture](#Hexagonal-architecture)
* [Resources](#resources)

## 3 Layer architecture

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

The input parameter is called: `...Command` (full mapping strategy) and the constructor verifies its syntactical validity.
To avoid coupling between services, it is better to have a dedicated input for each service.

Validating business rules is the semantically validity of the use case.
It can happen at the domain model or in the use case.
Since I choose to have a rich domain model if possible it happens in the domain model.

The output should be dedicated for each service, since it could create coupling between
the other adapters.
In general, it is better to return as little as possible data.

Read-only services should be somehow distinguished from services with side effect.
This plays well with the CQRS.
This is easy to be done with interfaces as incoming ports.
In our case, we can use an input called: `...Query`

Input and output of each port must be next to the port (inside `application/` or `domain`)

### Web Adapters

The web adapter is an incoming adapter.

It listens for http connections on a given route and performs:
- authorization checks
- map its input model to the input model of the service to call -> validate the conversion between the 2 layers
- call the service
- map the service's output to a http response

Adapter should contain the least possible routes (preferably only one).
In this way each adapter:
- has its own input/out model that is not shared between the other adapters
- contains a small number of services
- easier to unit-test

### Persistence Adapter

The persistence adapter is an outgoing adapter: it implements one or more outgoing ports.

The input and output of the outgoing port must be an hexagon domain entity because the hexagon calls the outgoing ports,
and it cannot depend on external adapters.

The outgoing ports should contain only one method (or very cohesive methods) and we should avoid having the repository
interface with many methods for the database interactions of all the services:

- _Depending on something that carries baggage you do not need, can cause you troubles you didn't expect 
- (Martin C. Robert)_
- the code is more difficult to understand and mock since some service will not use all the methods

However, the concrete outgoing adapter implementation, can implement more outgoing interfaces at once.

Ideally, we should have one adapter for each aggregate.
In this way we can separate the persistence needs of different bounded context.
In fact different bounded context must interact with each other through incoming ports
(cannot directly use outgoing ports of other bounded context).

To make the input parameter of the persistence adapter usable by the db utilities we could directly annotate
the domain model. However, this creates a dependency in the wrong direction that should be forbidden.
The correct way is to implement a proper mapping between the hexagon domain model and the adapter, but this results 
into more code.

Finally, services may need to call multiple adapter's operations transactional:
this can potentially result into a dependency in the wrong direction since the service will probably need to know
the db details to perform something transactional.

### Testing

There are 3 level of testing:
- unit test
- integration test
- system test

They have respectively a decreasing code coverage since the fewer components are mocked the more expensive they become.

Domain entity must be tested with unit test.

Services must be tested with unit test and mocks: test how the service interacts with the ports.
It is important to test only the significant interactions since the tests must verify the behaviour not the implementation.
Otherwise, the tests could fail every time we change the implementation but keep the same behaviour.

Outgoing adapter must be tested with integration tests spinning the required containers (databases,...).

The relevant path a user can take must be tested with system test.
System test mut contain the fewer possible mocks.
System test must be as much agnostic as possible from the web framework used.

### Mapping between boundaries

We have 3 layers:
- controller
- application domain + services
- outgoing adapter

Each of them could have its own I/O domain model, or they can share the same application domain model.

In the first case the SRP is always honoured, but we have boilerplate code to map between layers.
In the second case we have less code but the SRP is not honoured: in fact the application domain entities
will contain annotations related to serialization/deserialization/db pre-post processing/http parsing.

There are 3 different strategies that can be mixed with each other:
- no-mapping: use the same application domain entities for all layers
- two-way mapping: a dedicated domain model for each layer
- full mapping: a dedicated domain model for each layer + each service has its own input model

The no-mapping is fine until the In-Out adapter have special requirements (aside from annotations).
For example if the application domain needs to implement a complex deserialization algorithm,
to be used by the controller, it is better to delegate this logic to a custom input model for the controller.

The no-mapping and the two-way mapping have the problem that the domain model is used to communicate between layers.
This means the domain model can change for controller/out-adapter's reasons.

This is why the full mapping has a specific input for each service and outgoing adapter.

When using the full mapping strategy, we could perform input validation at the level of the input service.

However, in my opinion the application domain entity should enforce the required validations.
In this way we can fully leverage the Rust type system.
For example if I want to be sure an input string is a valid email, I would use an application domain entity,
`Email` to enforce that constraint, rather than trusting the service to perform its validation.

Finally, I think the best approach is:
- use if possible the no-mapping.
  - annotation violates the SRP, but they are fine.
- in case the application domain entity has something more complex, add specific domain input to the required layers:
  controller or outgoing adapter -> two-way mapping (this should happen rarely)
- if the service is a command, it is useful to add a specific input model when 
  there is not a clear mapping between the service input and the domain model

The domain model must be in charge of enforcing their syntactical validity at any given state.

### Configuration

The configuration components creates the application, instantiating the concrete classes and connecting with each other.

This component must also have access to configuration files.

I think a good approach is to implement the `TryFrom` for the `ConfigurationSettings` that build the application.

### Final Considerations

Package organization:
- `configuration/...`
- `feature/`
  - `adapter/[in|out]/...`
  - `domain/...`
  - `application/[services|outgoing_ports]/...`

Services are incoming ports without the use case interface: I decided not to have the use case interface because
the incoming adapter has a natural dependency on the hexagon.

Application domain entity must be self validating (syntactical validation).
I prefer having a rich domain model so that the use case/service only need to orchestrate calls
to the domain models and outgoing ports.

The outgoing ports should contain only one method (or very cohesive methods) and we should avoid having the repository
interface with many methods for the database interactions of all the services.
Ideally, we should have one adapter for each aggregate.

The input and output of each port must be next to the port (inside `application/` or `domain`).

Domain entity must be tested with unit test.

Services must be tested with unit test and mocks: test only the meaningful interactions.

Outgoing adapter must be tested with integration tests spinning the required containers

The relevant path a user can take must be tested with system test: agnostic as possible from the web framework used.

Use the no mapping strategy if possible.
If the service implements a non-trivial command it is ok to use the full mapping strategy for the controller.
If the domain entity has complexities due to external requirements adopt the 2 way mapping.


## Resources

- [Get Your Hands Dirty on Clean Architecture](https://reflectoring.io/book/)
- [Buckpal App](https://github.com/thombergs/buckpal)
- [Reevaluating the Layered Architecture](https://javadevguy.wordpress.com/2019/01/06/reevaluating-the-layered-architecture/)
- [SO question about use case](https://stackoverflow.com/questions/62818105/interface-for-use-cases-application-services)
- [Hexagonal architecture blog](https://blog.allegro.tech/2020/05/hexagonal-architecture-by-example.html)