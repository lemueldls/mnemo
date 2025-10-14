<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";
import { UseOnline } from "@vueuse/components";

const auth = useAuth();
const { user } = auth;

async function login() {
  const { platform } = useRuntimeConfig().public;

  const { error, data } = await auth.signIn.social({
    provider: "github",
    callbackURL: `/api/auth/callback?redirect=${encodeURIComponent(window.location.href)}&platform=${platform}`,
    disableRedirect: !!platform,
  });
  if (error) throw createError(error);

  if (platform) await openUrl(data.url!);
}

async function logout() {
  const { error } = await auth.signOut();
  if (error) throw createError(error);
}

const quota = ref<number>();
const usage = ref<number>();

onMounted(async () => {
  if (navigator.storage) {
    const estimate = await navigator.storage.estimate();
    quota.value = estimate.quota;
    usage.value = estimate.usage;
  }
});

const units = ["B", "KB", "MB", "GB", "TB", "PB"];
function formatBytes(bytes: number) {
  let i = 0;
  while (bytes >= 1024 && i < units.length - 1) {
    bytes /= 1024;
    i++;
  }

  return `${bytes.toFixed(2)} ${units[i]}`;
}

const keys = await getStorageKeys();
</script>

<template>
  <div class="flex h-full flex-col gap-3 overflow-hidden">
    <md-outlined-card class="flex flex-1 flex-col overflow-hidden p-3">
      <div class="flex-1 overflow-y-auto overflow-x-hidden">
        <storage-root :keys />
      </div>

      <template v-if="usage && quota">
        <md-divider class="my-2" />

        <div class="flex flex-col gap-2 p-1">
          <div class="label-large flex justify-between">
            <strong>{{ $t("components.sync.local-quota") }}</strong>

            <span>{{ formatBytes(usage) }} / {{ formatBytes(quota) }}</span>
          </div>

          <md-linear-progress :value="usage / quota" />
        </div>
      </template>
    </md-outlined-card>

    <md-outlined-card class="flex flex-col gap-3 p-3">
      <div class="flex justify-between">
        <h4 class="title-large">Sync</h4>

        <div
          class="bg-surface-container flex items-center gap-2 rounded-full px-2"
        >
          <UseOnline v-slot="{ isOnline }">
            <template v-if="isOnline">
              <div class="bg-lime size-2 rounded-full" />
              <span>Online</span>
            </template>
            <template v-else>
              <div class="bg-error size-2 rounded-full" />
              <span>Offline</span>
            </template>
          </UseOnline>
        </div>
      </div>

      <md-filled-tonal-button v-if="user" @click="logout">
        {{ $t("components.sync.logout") }}
      </md-filled-tonal-button>
      <md-filled-button v-else @click="login">
        {{ $t("components.sync.continue-with-github") }}
      </md-filled-button>
    </md-outlined-card>
  </div>
</template>
