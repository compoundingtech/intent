# Spec

## Status

Draft.

## Worker Model

The worker writes one event per completed review and stores the latest cursor in
the project state file.

This uses file-backed cursors because the team rejected a database after
deciding that a database would be overkill for local-only review runs.
