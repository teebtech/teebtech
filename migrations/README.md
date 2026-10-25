# Database Migrations

This directory contains PostgreSQL schema migration scripts managed by [SQLx](https://github.com/launchbadge/sqlx).

## Prerequisites

Install `sqlx-cli`:

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

Ensure `.env` contains a valid `DATABASE_URL`:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/teebtech_dev
```

## Commands

### Create a new migration
```bash
sqlx migrate add -r <migration_name>
```

### Apply pending migrations
```bash
sqlx migrate run
```

### Revert latest migration
```bash
sqlx migrate revert
```
