# Bevy Concepts

## Application

The Application is the main entry point of Bevy. It manages the application loop, schedules systems, and handles resources and events. It exists only during setup, and is not available once the application loop started.

## Schedule

A Schedule is a collection of Systems that are executed in a specific order. It is used to organize and control the flow of the application. The order can be specified by the user, or automatically inferred by Bevy.

## Plugin

A Plugin is a modular piece of functionality that can be added to a Bevy app. It encapsulates systems, resources, and configuration. It exists only at build time.

## World

The World is a data structure that stores all entities and their components. It provides methods to query and manipulate entities and components.

## Query

A Query is used to access entities and their components in a World. It allows systems to filter and iterate over entities with specific component sets.

## Commands

Commands are used to schedule changes to the World, such as adding or removing entities and components. They are executed later during the same frame, after the system that generated them ended.

## Resource

A Resource is a globally accessible data structure that is not tied to any specific entity. It is used to store shared data and state.

## Event and Observer

Events can be triggered by a system. They can be targeted at a specific entity.

Bevy will automatically trigger events for some of the World changes, like Component modifications or Entity creation.

An Observer is a System that reacts to an Event, and doesn't need to be added to a Schedule. It is executed at the earliest possible time by Bevy, and for each Event individually. It is used to implement reactive behavior.

## Message

Messages can be sent between systems to communicate data or trigger actions. They are buffered.

## Relations

Relations are a way to link entities together. The most common is the parent / children relation, which propagates things like position and visibility.
