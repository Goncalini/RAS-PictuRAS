This summary provides a comprehensive discussion on requirements in software engineering, organized into four main topics: the definition of requirements, functional requirements, non-functional requirements, and user versus system requirements.

## Definition of Requirement

A requirement is a condition or capacity needed by a user to solve a problem or achieve a goal. It highlights the dual perspective of requirements:

- **User Needs:** What user expects the system to do;
- **System Capacities:** What the system must be capable of.

The IEEE 610.12-1990 standard defines a requirement as:

- A condition or capacity to solve a problem;
- A condition or capacity to satisfy a contract, standard, or specification;
- A documented representation of the above.

The classification divides requirements into **functional** and **non-functional** categories. Requirement can also be "candidate requirements" identified during elicitation but not yet approved for implementation.

## Functional Requirements

Functional requirements describe specific functionalities the system must perform to meet user needs. Key aspects include:

- **Independence from technology:** Functional requirements should not depend on specific design or implementation choices;
- **Completeness and coherence:** The set of functional requirements must fully cover client needs without contradictions, which is particularly challenging for complex systems.

They can also be distinguished between:

- **Explicit Requirements:** Clearly stated by stakeholders;
- **Implicit Requirements:** Assumed on domain knowledge but not explicitly requested by users.

## Non-Functional Requirements

Non-functional requirements impose constraints on how the system operates rather than what it does. They include aspects like speed, reliability, and aesthetic consideration. These requirements do not alter core functionalities but influence the system's usability and performance. Key points include:

- **Emergent Properties:** System-wide characteristics such as reliability or maintainability that cannot be attributed to individual components.
- **Conflicting Requirements:** Optimizing one non-functional requirement (e.g., performance) often impacts others (e.g., maintainability).

Non-functional requirements are further categorized based on different frameworks:

- **1. Sommerville's Framework:**
	- **Product Requirements:** Characterize aspects of the behavior of the system itself (e.g., performance, reliability, usability);
	- **Organizational Requirements:** Come from strategies and procedures established in the context of the manufacturing process of the system or the client organisation (e.g., process standards);
	- **External Requirements:** Have origin in external factors to the system and the development process (e.g., legal or ethical constraints).

- **2. Robertson's Framework:**
	- Types include appearance, usability, performance, operational needs, maintenance and support, security, cultural/political factors, and legal requirements.

## User and System Requirements

User and system requirements are distinguished based on their focus:

- **User Requirements:** Represent user needs in the problem domain, typically expressed in natural language for stakeholder comprehension;
- **System Requirements:** Detailed technical specifications oriented towards the solution domain, aiding engineers in designing the system.

Effective requirement analysis requires avoiding premature design decisions and ensuring the problem is well-characterized. Poorly formulated problems often lead to inadequate solutions.