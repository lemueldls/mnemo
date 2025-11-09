<script lang="ts" setup>
import type { Package } from "~~/server/api/list-packages.get";

const props = defineProps<{
  spaceId: string;
  namespace: string;
  versionedPackage: Package[];
}>();

const selectedIndex = ref(0);
const pkg = computed(() => props.versionedPackage[selectedIndex.value]!);

const installedPackages = await useInstalledPackages(() => props.spaceId);
const installedPackageByName = computed(() => {
  const { name } = pkg.value;

  return installedPackages.value.filter((pkgItem) => name === pkgItem.name);
});
const installedPackage = computed(() => {
  const { version } = pkg.value;

  return installedPackageByName.value.find(
    (pkgItem) => version === pkgItem.version,
  );
});

const loading = ref(false);

async function installPackage(pkg: Package) {
  loading.value = true;

  const pkgSpec = {
    namespace: props.namespace,
    name: pkg.name,
    version: pkg.version,
  };

  await installTypstPackage(pkgSpec);
  installedPackages.push(pkgSpec);

  loading.value = false;
}

function uninstallPackage(pkg: Package) {
  const pkgs = installedPackages.value;
  for (let i = pkgs.length - 1; i >= 0; i--) {
    const pkgItem = pkgs[i]!;

    if (
      pkg.name === pkgItem.name &&
      pkg.version === pkgItem.version
      // && props.namespace === pkgItem.namespace
    ) {
      installedPackages.delete(i, 1);
    }
  }
}
</script>

<template>
  <md-elevated-card class="flex flex-col gap-4 p-4">
    <div class="medium:flex-row flex flex-col justify-between gap-2">
      <h1 class="text-on-surface-variant title-large font-semibold">
        {{ pkg.name }}
      </h1>

      <md-outlined-select
        :label="$t('components.package-card.version')"
        :value="selectedIndex"
        @input="selectedIndex = $event.target.value"
      >
        <md-select-option
          v-for="({ version }, i) in versionedPackage"
          :key="i"
          :value="i"
          :selected="i === selectedIndex"
        >
          <span slot="headline">
            v{{ version }}

            <template
              v-if="
                installedPackageByName.find(
                  (pkgItem) => version === pkgItem.version,
                )
              "
            >
              {{ $t("components.package-card.installed") }}
            </template>
          </span>
        </md-select-option>
      </md-outlined-select>
    </div>

    <div class="flex flex-1 flex-col gap-2">
      <!-- <div class="flex gap-2">
        <div
          v-for="(category, i) in pkg.categories"
          :key="i"
          class="bg-surface-container text-on-surface-variant rounded-xl px-2 py-1"
        >
          {{ category }}
        </div>
      </div> -->

      <span
        class="text-on-surface-variant body-large line-clamp-2 h-[2lh]"
        :title="pkg.description"
      >
        {{ pkg.description }}
      </span>
    </div>

    <div class="flex gap-2">
      <md-filled-tonal-button
        class="flex-[2]"
        :title="pkg.repository"
        @click.prevent="openExternalUrl(pkg.repository)"
      >
        {{ $t("components.package-card.repository") }}
      </md-filled-tonal-button>

      <md-outlined-button
        v-if="installedPackage"
        class="flex-[3]"
        @click.prevent="uninstallPackage(pkg)"
      >
        {{ $t("components.package-card.uninstall") }}
      </md-outlined-button>
      <md-filled-button
        v-else
        :disabled="loading"
        class="flex-[3]"
        @click.prevent="installPackage(pkg)"
      >
        {{
          loading
            ? $t("components.package-card.installing")
            : $t("components.package-card.install")
        }}
      </md-filled-button>
    </div>
  </md-elevated-card>
</template>
