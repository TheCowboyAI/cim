# Policies
Policies are Behaviors applied to specific States.

States are defined as a collection of Domain Events that have occured as a collection.

When these Collections reach a collective value, they can trigger reaction called a Policy.

These are also Aggregates in some cases, where transactions are involved.

Think of these as a way to determine if the CIM is in a Current State that I want to react to.

For example:
Customer has requested overnight delivery.
We get a notification of a weather Event that will occur between the Warehouse and the Customer potentially affecting delivery times. You want to alert the customer of this event and a potential delay that is out of your immediate control. You may trigger discounts or re-routing delivery to avoid the weather event.

This is a lot to think about, but with the ability to compose potential activities such as this, Policies help deliver the expected results.