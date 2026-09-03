# Security and Offline

## Offline requirement

Normal use must require no network connection.

The app should not depend on hosted services, remote APIs, telemetry, or CDN-loaded assets.

## Minimum permissions

Desired normal permissions:

- read the project root explicitly selected by the user
- no project write access
- no shell access
- no network access

Optional capabilities may be added later only when justified.

## Root confinement

The Rust scanner should canonicalize the selected root and intentionally remain within it.

## Symlinks

Symlinks should not be followed by default.

This prevents a project tree from implicitly expanding the scanner's filesystem access outside the selected root.

## Frontend isolation

The React/Cytoscape frontend should consume graph data rather than directly traversing the project filesystem.
