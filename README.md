# Mnemo

> Local-first note-taking app leveraging the Typst ecosystem.

## Inspirations

- [Obsidian](https://obsidian.md)
- [Noodle](https://github.com/noodle-run/noodle)
- [Research by UNMS](https://un.ms/research)

## Roadmap

See [TODO.md](TODO.md) for the expanded roadmap.

## Development

### Prerequisites

- Node.js 24+
- pnpm 11+
- Rust 1.85.0+

### Quickstart

```sh
pnpm install
pnpm dev
```

### Environment Variables

Create a `.env` in the repository root with the following:

```dotenv
# Required only for production Tauri builds.
NUXT_PUBLIC_API_BASE_URL=http://localhost:3000

# Optional in development. Recommended in production.
NUXT_SESSION_PASSWORD=change-me-to-a-long-random-secret

# Optional: GitHub OAuth
NUXT_OAUTH_GITHUB_CLIENT_ID=
NUXT_OAUTH_GITHUB_CLIENT_SECRET=
NUXT_OAUTH_GITHUB_REDIRECT_URL=
```

<!-- ```dotenv
# Optional: Polar integration (incomplete)
NUXT_POLAR_ACCESS_TOKEN=
NUXT_POLAR_WEBHOOK_SECRET=
``` -->

## Self-Hosting

This project targets Cloudflare Workers with NuxtHub.

### Deploy to Cloudflare Workers

Build and deploy with Wrangler:

```sh
pnpm build
pnpm wrangler deploy
```

See the [NuxtHub deployment docs](https://hub.nuxt.com/docs/getting-started/deploy) for more information.

## Installation

You can download the latest version of the application from the [GitHub Release page](https://github.com/lemueldls/mnemo/releases/latest).

### Windows (winget)

Run the following command:

```sh
winget install mnemo
```

### Arch Linux (AUR)

Install either the prebuilt package or build-from-source package from AUR:

```sh
# Prebuilt binary package
paru -S mnemo-bin

# Or build from source
paru -S mnemo
```

### Void Linux

Add the repository and install the package:

```sh
# Add the repository
echo "repository=https://github.com/lemueldls/mnemo/releases/latest/download" | sudo tee /etc/xbps.d/mnemo-repo.conf

# Install the package
sudo xbps-install -S mnemo
```

## License

This project is licensed under [AGPL-3.0](https://choosealicense.com/licenses/agpl-3.0/). See the [LICENSE](LICENSE) file for details.
