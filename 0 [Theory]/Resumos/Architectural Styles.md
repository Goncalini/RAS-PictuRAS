An **architectural style** is analogous to an architectural style in a building construction (e.g., Gothic, Modernist). It provides a structured approach to system design, comprising;

- A set of components types (e.g., processes, procedures) performing specific functions;
- A topological arrangement of components indicating runtime relationships;
- Semantic constraints that define how components interact;
- Connectors (e.g., data streams, sockets) for communication.

Styles act as reusable constraints, guiding system design while promoting consistency, maintainability, and improved developer communication.

### Platonic vs Embodied Styles

- **Platonic Styles** are idealized and appear in theoretical descriptions but are rarely found in real-world systems;
- **Embodied Styles** are their practical implementations which may deviate from the ideal due to real-world constraints. For example, while the platonic client-server style assumes servers are unaware of clients, embodied versions may include servers pushing updates to clients.

## Architectural Styles vs Design Patterns

Architectural styles operate at a larger scale, defining the overall structure of a system, while design patterns address localized design problems.

- A system typically adheres to a **single dominant architectural style** (e.g., client-server);
- Multiple design patterns (e.g., REST, MVC) can coexist within a single system.

## Common Architectural Styles

### Layered Style

- Organizes a system into stacked **layers**, each serving as a virtual machine for the layer above it;
- Layers typically communicate only with the layer directly beneath them, forming a directed acyclic graph (DAG);
- **Quality attributes:** Promotes modifiability, portability, and reusability;
- **Challenges:** In practice, layers may "skip" or violate constraints (e.g., callbacks to higher layers).

### Big Ball of Mud

- Characterized by a lack of structure, promiscuous data sharing, and expedient but crude fixes;
- Results in poor maintainability and extensibility, often referred to as "spaghetti code";
- While sub optimal, this style is sometimes a pragmatic approach in fast-paced or resource-limited environments.

### Pipe-and-Filter Style

- Processes data in stages, where **filters** perform computations and **pipes** transport data between them.

**Constraints:**
- Filters are independent and do not share state;
- Data flows in one direction, often without loops.

**Benefits:** Enables incremental processing and concurrency.
**Example:** A Unix shell pipeline like `cat | grep | cut`.

### Batch-Sequential Style

- Similar to pipe-and-filter but with **coarser granularity;**
- Each stage completes its processing before passing data to the next stage, often via intermediate files;
- Lacks concurrency and interaction between stages, making it slower but easier to manage.

### Model-Centered Style

- Features a central **model** (data score) with independent views and controllers interacting with it;
- Related to patterns like MVC (Model-View-Controller);
- **Benefits:** Enhances modifiability and extensibility, simplifies state management, and supports concurrency;
- Suitable for systems with uncertain future configurations.

### Publish-Subscribe Style

- Comments publish and subscribe to events without knowing each other;
- An **event bus connector** delivers events from publishers to subscribers;
- **Benefits:** Decouples producers and consumers of events, improving maintainability and scalability.

### Client-Server Style & N-Tier Architecture

- **Client-Server:** Clients request services from servers synchronously while servers remain unaware of clients unless contacted;
- **N-Tier Architecture:** Extends client-server into multiple tiers,  typically including:
	- **User Interface Tier** (handles user interactions);
	- **Business Logic Tier** (processes rules and logic);
	- **Persistence Tier** (handles data storage).
- Requests flow in one direction, and each tier has exclusive responsibilities.

### Peer-to-Peer Style

- All nodes act as peers, capable of both requesting and providing services;
- Lacks hierarchical relationships, creating a network of egalitarian nodes;
- **Advantages:** Decentralization and fault tolerance.

### Map-Reduce Style

- Designed for processing large datasets by distributing computations across multiple nodes;
- Increases scalability and resilience to node failures;
- Commonly used in search engines and social networking platforms.