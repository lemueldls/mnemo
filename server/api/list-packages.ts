import { parseTarGzip } from "nanotar";

export interface Package {
  name: string;
  version: string;
  entrypoint: string;
  authors: string[];
  license: string;
  description: string;
  repository: string;
  keywords: string[];
  compiler: string;
  exclude: string[];
  updatedAt: number;
}

export default defineCachedEventHandler(
  async (event) => {
    const { namespace } = await getQuery(event);

    const allPackages = await $fetch<Package[]>(
      `https://packages.typst.org/${namespace}/index.json`
    );
    allPackages.sort((a, b) => b.version.localeCompare(a.version));

    const packages: { [name: string]: Package[] } = {};
    for (const pkg of allPackages) {
      packages[pkg.name] ||= [];
      packages[pkg.name]!.push(pkg);
    }

    return packages;
  },
  { maxAge: 60 * 60 * 24 * 7, staleMaxAge: -1, getKey: (event) => event.path }
);
