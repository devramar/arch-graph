# Event Synchronization

ARCH_NODE:EventSync

## Dependencies

ARCH_DEPENDENCY:DateKey

Used as the canonical day representation when constructing synchronization windows.

ARCH_DEPENDENCY:EventStore

Provides the local event state against which synchronization results are reconciled.

ARCH_DEPENDENCY:RemoteChanges

Represents a deliberately unresolved dependency for fixture coverage.
