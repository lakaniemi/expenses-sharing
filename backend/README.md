# Backend

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Diesel cli](https://diesel.rs/guides/getting-started)

## Development

Run `docker-compose up` to start necessary necessary services.

Set up `.env` file:

```
DATABASE_URL=postgres://postgres:postgres@localhost/expense_sharing
```

## Migrations

```bash
# Create new migration
diesel migration generate <migration_name>

# Run all pending migrations
diesel migration run

# Re-run the last migration to test that up/down actually works
diesel migration redo
```

PGAdmin4 client is running at http://localhost:8888/browser/
