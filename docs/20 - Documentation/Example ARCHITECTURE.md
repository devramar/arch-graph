# Event Synchronization

ARCH_NODE:EventSync

## Description

Coordinates synchronization of remote event-day changes with locally stored event data.

---

## Purpose

Keep local event state current while allowing the rest of the application to work against stable local representations.

---

## Intended Usage

Consumers invoke the synchronization API rather than performing network reconciliation directly.

---

## Architecture

The system compares changed remote days against locally known event data and reconciles required updates through the event store.

---

## Dependencies

ARCH_DEPENDENCY:DateKey

Used as the canonical representation of calendar days when constructing synchronization windows and matching changed remote days.

ARCH_DEPENDENCY:EventStore

Provides the local event state against which synchronization results are reconciled.

---

## Invariants

- A synchronization operation uses canonical DateKeys for day identity.
- Callers should not need to understand persistence details.
- Network payload presence does not itself imply that event data is resident in memory.

---

## Relevant Files

`EventSync.ts`
: Main synchronization coordinator.
