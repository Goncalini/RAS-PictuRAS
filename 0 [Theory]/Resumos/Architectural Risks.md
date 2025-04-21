
The concept of failure is central to the design process, and it is by thinking in terms of obviating failure that successful designs are achieved.

## Risk-Driven Approach

- **Core Idea:**
	- Success in design stems from mitigating potential failures;
	- Developers must anticipate and address possible failures early;
	- The focus is on balancing design effort with actual risks rather than over-designing or neglecting architecture
- **Guiding Questions:**
	- How much architecture work is needed?
	- Which techniques should be applied to minimize risks effectively?
- **Three-Step Process:**
	- **Identify and Prioritize Risks:** Understand potential areas of failure and their impact;
	- **Apply Techniques:** Use architectural strategies tailored to the identified risks;
	- **Evaluate Reduction:** Assess if risks have been adequately mitigated before proceeding.
- **Philosophy:**
	- Avoid wasting effort on low-impact issues;
	- Address high-priority risks with deliberate, minimal techniques.

## Risks

_Risk = Probability of Failure x Impact of Failure_

Risks may not always materialize but should still be accounted for.

- **Types of Risks:**
	- **Project Management Risks:** Examples include losing key personnel or misunderstanding customer needs;
	- **Software Engineering Risks:** Examples include scalability issues or unanticipated consequences of changes.
- **Prototypical Risks:**
	- Common patterns of risks observed in specific domains:
		- **IT Projects:** Poor problem definition or scattered domain knowledge;
		- **Systems:** Performance, security, and reliability concerns;
		- **Web Applications:** Scalability and security.
- **Prioritization:**
	- Risks are categorized by:
		- Business stakeholders' priorities;
		- Perceived difficulty by the development team.
- **Mitigation Techniques:**
	- Tailored to specific risks:
		- Software Engineering: Design patterns, prototyping, security analysis;
		- Other Fields: Stress calculations, thermal analysis.
- **Effort Proportionality:**
	- Design effort should match the risk's severity and likelihood;
	- Not all parts of the system need equal attention; focus on areas with high risks.

## Styles of Design

Three main architectural design styles are outlined, all of which can align with the risk-driven approach:

- **Evolutionary Design (No Design Up Front - NDUF):**
	- Design evolves during implementation;
	- Popular in Agile methodologies:
		- Refactoring for cleaner designs;
		- Test-Driven Development ensures stability;
		- Continuous integration promotes collaboration.
	- Risks: May result in uncoordinated or incomplete architectures.
- **Planned Design (Big Design Up Front - BDUF):**
	- Detailed architecture is completed before implementation;
	- Analogous to building bridges, where construction starts after design is finalized;
	- Appropriate when:
		- Multiple teams share the architecture;
		- Feedback is incorporated post-design via prototyping.
	- Criticized for inefficiency but suitable for complex projects.
- **Minimal Planned Design (Enough Design Up Front - EDUF):**
	- A hybrid approach:
		- Initial design tackles the highest risks;
		- Flexibility is maintained for iterative, evolutionary improvements.
	- Encourages balancing structure with adaptability.