# Mnemo

> Local-first note-taking app leveraging the Typst ecosystem.

## Inspirations

- [Obsidian](https://obsidian.md)
- [Noodle](https://github.com/noodle-run/noodle)
- [Research by UNMS](https://un.ms/research)

## Getting Started

### Prerequisites

- Node.js 22+
- pnpm 10+
- Rust (stable toolchain)

### Quickstart

```sh
pnpm install
pnpm dev
```

### Environment Variables

Create `.env` in the repository root with the following:

```dotenv
# Required only for production Tauri builds.
NUXT_PUBLIC_API_BASE_URL=http://localhost:3000

# Optional in development. Recommended in production.
NUXT_SESSION_PASSWORD=change-me-to-a-long-random-secret

# Optional: GitHub OAuth
NUXT_OAUTH_GITHUB_CLIENT_ID=
NUXT_OAUTH_GITHUB_CLIENT_SECRET=
NUXT_OAUTH_GITHUB_REDIRECT_URL=

# Optional: Polar integration (incomplete)
NUXT_POLAR_ACCESS_TOKEN=
NUXT_POLAR_WEBHOOK_SECRET=
```

## Self-Hosting

This project targets Cloudflare Workers with NuxtHub.

### Deploy to Cloudflare Workers

Build and deploy with Wrangler:

```sh
pnpm build
pnpm wrangler deploy
```

See [NuxtHub deployment docs](https://hub.nuxt.com/docs/getting-started/deploy) for more information.

## Roadmap

See [TODO.md](TODO.md) for the expanded roadmap.

## License

This project is licensed under [AGPL-3.0](https://choosealicense.com/licenses/agpl-3.0/). See the [LICENSE](LICENSE) file for details.
