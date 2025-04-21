
Design Patterns are proven solutions to common problems in software design. They encapsulate reusable experience rather than specific code and are valuable for addressing recurring challenges in a structured way.

https://refactoring.guru/design-patterns

### Key Ideas

- **Origin:** Patterns trace their roots to Christopher Alexander's work in architecture and were later adopted in software development;
- **Definition:** A textual description of a generic solution to a recurring problem in a given context;
- **Purpose:** Patterns are tools for **experience reuse**, providing predesigned solutions that can be adapted to specific needs.

### Types of Patterns

- **System Patterns** (e.g., architectural styles);
- **Design Patterns;**
- **Code Patterns.**

### Categories

- **Creational:** Focus on object creation (e.g., Singleton, Factory);
- **Structural:** Define object relationships (e.g., Strategy);
- **Behavioral:** Manage communication and responsibilities (e.g., Observer);

## Strategy Pattern

The **strategy pattern** is a **structural design pattern** that defines a family of algorithms, encapsulates each one, and makes them interchangeable.

### Problem

When inheritance is used to share behavior, maintaining and updating code becomes difficult due to:

- Code duplication in subclasses;
- Inconsistent behavior across inherited methods.

### Solution

- **Separate varying aspects** of the application from those that remain constant;
- Use **composition over inheritance:** A class delegates specific behavior to separate strategy classes;
- **Program to an interface, not an implementation:** This allows swapping different behaviors without modifying existing code.

### Example

A duck simulation program initially used inheritance to implement behaviors like _quack()_ and _fly()_. However, some ducks (e.g., rubber ducks) do not fly or quack in the same way. The strategy pattern separates these behaviors into independent classes (e.g., _FlyWithWings_ or _Squeak_) that can be composed with any duck object.

## Observer Pattern

The **observer pattern** is a **behavioral design pattern** that defines a **one-to-many dependency** between objects so that when one object changes state, all its dependent are automatically notified and updated.

### Problem

In systems like weather monitoring apps, adding new display types requires modifying the core weather data class, leading to high coupling and poor scalability.

### Solution

- Use the observer pattern to decouple the subject (e.g., weather data) from its observers (e.g.,  displays);
- The subject maintains a list of observers and notifies them whenever its state changes;
- Observers implement a common interface to update their state.

### Benefits

- Supports scalability and flexibility by allowing new observers to be added without changing the subject;
- Reduces coupling between components.

## Decorator Pattern

The **decorator pattern** is a **structural design pattern** that allows behavior to be added to individual objects dynamically, providing a flexible alternative to subclassing for extending functionality.

### Problem

When trying to extend functionality (e.g., adding condiments like mil or mocha to coffee), inheritance leads to:

- **Explosion of subclasses:** each combination of condiments and coffee types requires a new class;
- **Rigid structure:** Adding new features or beverages requires modifying existing classes, violating the **open-closed principle.**

### Solution

- Use the decorator pattern to compose objects with additional responsibilities at runtime;
- Decorators wrap the original object and add behavior dynamically.

### Example

A coffee shop application can use the decorator pattern to handle beverages and condiments:
- A base _Baverage_ class represents drinks;
- Condiments like _Milk_ or _Mocha_ are decorators that add cost and descriptions dynamically to _Beverage_ objects.

### Benefits

- Adheres to the **open-closed principle:** Classes are open for extension but closed for modification;
- Support flexible combinations of behaviors without exploding the class hierarchy.