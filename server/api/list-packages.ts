export interface Package {
  name: string;
  version: string;
  entrypoint: string;
  authors: string[];
  license: string;
  description: string;
  repository: string;
  keywords: string[];
  categories: string[];
  compiler: string;
  exclude: string[];
  updatedAt: number;
}

const notPackage = new Set([
  "book",
  "report",
  "paper",
  "thesis",
  "poster",
  "flyer",
  "presentation",
  "cv",
  "office",
]);

export default defineCachedEventHandler(
  async (event) => {
    const { namespace } = getQuery(event);

    let allPackages = await $fetch<Package[]>(
      `https://packages.typst.org/${namespace}/index.json`,
    );
    allPackages = allPackages.filter(
      (pkg) =>
        !pkg.categories?.length ||
        notPackage.isDisjointFrom(new Set(pkg.categories)),
    );
    allPackages.sort((a, b) => b.version.localeCompare(a.version));

    const packages: { [name: string]: Package[] } = {};
    for (const pkg of allPackages) {
      packages[pkg.name] ||= [];
      packages[pkg.name]!.push(pkg);
    }

    return packages;
  },
  { maxAge: 60 * 60 * 24 * 7, staleMaxAge: -1, getKey: (event) => event.path },
);
