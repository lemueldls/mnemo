<script setup lang="ts">
import { signInSocial } from "@daveyplate/better-auth-tauri";

const auth = useAuth();
const { user } = auth;

async function login() {
  const { error } = await signInSocial({
    authClient: auth.client,
    provider: "github",
  });
  if (error) throw createError(error);
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
</script>

<template>
  <div class="flex flex-col gap-4">
    <h3 class="m3-display-small">Sync</h3>

    <md-outlined-card v-if="usage && quota" class="flex flex-col gap-2 p-4">
      <div class="m3-label-large flex justify-between">
        <strong>Local Quota</strong>

        <span>{{ formatBytes(usage) }} / {{ formatBytes(quota) }}</span>
      </div>

      <md-linear-progress :value="usage / quota" />
    </md-outlined-card>

    <span class="text-m3-error">TODO</span>

    <md-filled-tonal-button v-if="user" @click="logout">
      Logout
    </md-filled-tonal-button>
    <md-filled-button v-else disabled @click="login">
      Continue with GitHub
    </md-filled-button>
  </div>
</template>
