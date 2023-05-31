# Fire up database

```
docker compose up -d
```

# Initialize database

```
sqlx database create
sqlx migrate run
```

# Reset database

```
sqlx database drop
sqlx migrate run
```

# Calculate ratings for games played
docker exec -it portal-server-1 ./ratings
