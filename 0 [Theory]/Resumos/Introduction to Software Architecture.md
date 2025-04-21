
- **Principles of Software Design:**
	- Abstraction, decomposition, modularization;
	- Coupling and cohesion;
		- **Coupling:** Coupling refers to the extent to which one module depends on another. It describes the level of interconnection between software components.
		- **Cohesion:** Cohesion refers to how closely related and focused the responsibilities of a module are. It measures the "single-mindedness" of a module.
	- Separation of interface and implementation;
	- Sufficiency, completeness, primitiveness;
	- Separation of concerns.
- **Definitions:**
	- **Software Design:** Turning specifications into operational software;
	- **Software Architecture:** A set of structures (static and dynamic) representing software components, their relationships, and governing principles.
- **Key Points:**
	- **Static Structures:** Design-time elements and their arrangement;
	- **Dynamic Structures:** Run-time elements and their interactions;
	- **Quality Attributes:** Non-functional properties like security and performance;
	- Functional requirements alone do not determine architecture;
	- Architecture reflects stakeholder priorities, technical constraints, and team expertise.
- **Architecture Risks:**
	- Architecture becomes crucial when:
		- The solution space is narrow;
		- Risk of failure is high;
		- Difficult quality attributes are needed;
		- A new domain is being explored.
- **Design Levels:**
	- **Architecture Design:** High-level modules and their interactions;
	- **Detailed Design:** Lower-level implementation specifics (everything else).
- **Minimizing Dependencies:**
	- Components should be loosely coupled to localize changes;
	- Communication mechanisms between components need careful design.

## Good Architectures

There are many examples of good architectures, like:

- **3-Tier Architecture:** Localizes changes and supports high transactional loads;
- **Cooperating Processes:** Fault isolation, suitable for operating systems;
- **Peer-to-Peer Systems:** Scalability, as seen in Skype's VOIP network.

### Quality Attributes

Architecture enables or constrains qualities such as:

- **Scalability:** A system built for 100 users may require redesign for 100,000 users;
- **Performance, security and reliability.**

Quality attribute evolution may demand drastic architectural changes.

### Orthogonality of Architecture and Functionality

- Architecture is separate but complementary to system functionality;
- A poor architecture choice complicates achieving both functionality and quality attributes.

### Characteristics of Good Architecture

- **Decision-Making:**
	- Developed by a small team with clear leadership;
	- Driven by a prioritized list of quality attributes.
- **Documentation:**
	- Captures decisions and rationale;
	- Uses stakeholder-specific views.
- **Evaluation:** Tested for its ability to support critical quality attributes.
- **Implementation:** Supports incremental development.

### Presumptive Architectures

- Common, domain.specific architectural patterns;
- Example: N-tier architecture for IT systems.