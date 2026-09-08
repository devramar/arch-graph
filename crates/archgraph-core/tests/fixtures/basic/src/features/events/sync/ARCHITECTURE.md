# Event Synchronization

ARCH_NODE:EventSync

EventSync coordinates reconciliation between locally persisted event state and remotely collected changes.

## References

ARCH_REFERENCE:DateKey

EventSync uses DateKey as the canonical day representation when constructing synchronization windows.

ARCH_REFERENCE:EventStore

EventSync reads local event state from EventStore and reconciles remote results against it.

ARCH_REFERENCE:RemoteChanges

EventSync delegates remote change collection to RemoteChanges before reconciliation.

ARCH_SUBREFERENCE:Password Management

EventSync keeps this password-handling concern local to its own architecture boundary.
