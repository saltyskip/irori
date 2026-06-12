# Irori Marketing Site

Landing page and API documentation for Irori.

## Quick Start

```bash
npm install
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to see the landing page.

API documentation is available at [http://localhost:3000/api/docs](http://localhost:3000/api/docs) (requires server running at localhost:3000 on a different port, or CORS proxy).

## Features

- **Landing page** — Beautiful introduction to Irori with features and philosophy
- **Scalar API docs** — Interactive OpenAPI documentation (requires running server)
- **Next.js 14** — Modern React framework with App Router
- **Tailwind CSS** — Utility-first styling
- **TypeScript** — Type-safe development

## Development

```bash
# Development server
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Lint code
npm run lint
```

## API Documentation

The API docs route fetches OpenAPI spec from your running Irori server. Make sure the server is running at `http://localhost:3000` (or update the URL in `src/app/api/docs/route.ts`).

### Local Development with Server

Terminal 1: Start the Rust server
```bash
cd server
cargo run
```

Terminal 2: Start the marketing site
```bash
cd marketing
npm run dev
```

Then visit:
- Landing page: http://localhost:3000
- API docs: http://localhost:3000/api/docs (after server is running)

## Build & Deploy

```bash
npm run build
npm start
```

For Docker deployment, create a Dockerfile that:
1. Builds with `npm run build`
2. Serves with `npm start`
3. Exposes port 3000
