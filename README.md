# Portal

- 🔥 Astro
- 🎨 Tailwind CSS with aspect ratio and typography plugin
- 🎉 TypeScript
- ✏️ ESLint compatible with .astro files
- 🛠 Prettier compatible with .astro files
- 🦊 Husky
- 🚫 lint-staged
- 🚨 Commitlint
- 🔼 Vercel
- 🚢 Docker


# Start DEV environment

Starts a DEV build of the frontend and spins up a Postgres DB with provided seed data

1. docker compose -f docker-compose-dev.yml build
2. docker compose -f docker-compose-dev.yml up

# Start PROD environment

Fires up an ExpressJS server inside the Docker container and serves the frontend through it with SSR enabled.
Also spins up a Postgres DB (debatable if this should always be the case in PROD)

1. docker compose -f docker-compose-prod.yml build
2. docker compose -f docker-compose-prod.yml up

