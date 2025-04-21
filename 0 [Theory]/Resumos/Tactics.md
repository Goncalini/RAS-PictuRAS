
Tactics are architectural design decisions aimed at achieving specific quality attributes, such as availability, performance, or modifiability. These are key tools for architects to meet quality requirements and align system behavior with business goals.

- **Purpose:** Tactics link quality attribute requirements with architectural choices, providing a structured way to control how the system responds to stimuli;
- **Examples:** Introducing redundancy is a tactic for improving availability.

### Types of Tactics:

- Availability tactics;
- Modifiability tactics;
- Performance tactics;
- Security tactics;
- Testability tactics;
- Usability tactics.

## Availability Tactics

Availability measures the percentage of time a system is operational and correctly functioning. It is influenced by factors such as Mean Time Between Failures (MTBF) and Mean Time to Repair (MTTP).

### Key Strategies

**Fault Detection:**
- **Ping/Echo:** Components ping each other to ensure they are operational;
- **Heartbeat:** A component periodically sends a heartbeat signal to indicate it is functioning;
- **Exceptions:** Faults are detected through exceptions raised when predefined error conditions occur.

- **Fault Recovery:**
	- **Voting:** Redundant processors calculate outputs, and a majority decision resolves conflicts;
	- **Active Redundancy:** All redundant components operate in parallel; the first response is used;
	- **Passive Redundancy:** Standby components take over after ensuring their state is up-to-date.

- **Fault Prevention:**
	- **Removal from Service:** Temporarily removes a component for maintenance (e.g., rebooting to prevent memory leaks);
	- **Transaction:** Bundles steps into atomic units that can be rolled back to avoid data corruption;
	- **Process Monitor:** Identifies and restarts faulty processes.

Availability tactics aim to prevent faults from becoming failures, minimize downtime, and ensure quick recovery when issues arise.

## Performance Tactics

Performance refers to a system's ability to stimuli within a specific time or process a certain number of events per time unit. It impacts user experience, business revenue, and operational efficiency.

### Key Strategies

Performance tactics address two main aspects: **resource consumption** and **blocked time.**

- **Resource Demand:**
	- **Increase Computational Efficiency:** Optimize algorithms to reduce processing time;
	- **Reduce Computational Output:** Remove intermediaries or unnecessary processing steps;
	- **Manage Event Rate:** Decrease the number of events by reducing sampling frequencies or dropping low-priority requests.

- **Resource Management:**
	- **Introduce Concurrency:** Process multiple events or streams in parallel using multi threading;
	- **Maintain Multiple Copies:** Use replicas to reduce contention (e.g., client-server systems);
	- **Caching:** Store frequently accessed data in fast repositories to minimize latency;
	- **Increase Available Resources:** Add faster processors, more memory, or higher network bandwidth (a trade off with cost).

Performance tactics manage the time between stimulus and response, focusing on reducing latency and ensuring efficient resource utilization.
	