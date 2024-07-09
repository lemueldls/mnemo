<script setup lang="ts">
const open = defineModel();

const namespace = "preview";


interface Package {
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

const allPackages = await $fetch<Package[]>(`https://packages.typst.org/${namespace}/index.json`);
allPackages
  .sort((a, b) => b.version.localeCompare(a.version))

const packages: { [name: string]: Package[] } = {};
for (const pkg of allPackages) {
  packages[pkg.name] ||= [];
  packages[pkg.name]!.push(pkg);

}

watchEffect(() => {
  console.log({ packages });
});
</script>

<template>

  <md-dialog :open="open" @closed="open = false">
    <span slot="headline">Packages</span>


    <form slot="content" method="dialog">
      <m3-elevated-card v-for="[pkg]  in packages" :key="pkg.name">
        <div class="flex flex-col gap-2">
          <h1 class="text-m3-on-surface-variant m3-title-large">
            {{ pkg.name }}
          </h1>

          <div class="flex flex-col gap-2">
            <span class="text-m3-on-surface-variant m3-body-large">
              {{ pkg.description }}
            </span>

            <div class="flex flex-col gap-2">
              <span class="text-m3-on-surface-variant m3-body-large">
                Version: {{ pkg.version }}
              </span>
            </div>
          </div>
        </div>
      </m3-elevated-card>
    </form>
  </md-dialog>
</template>


<style>
form {
  @apply grid gap-4;

  grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
}
</style>
