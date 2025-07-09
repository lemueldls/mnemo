<script setup lang="ts">
import { signInSocial } from "@daveyplate/better-auth-tauri";
import { isTauri } from "@tauri-apps/api/core";

const auth = useAuth();
const { user } = auth;

async function login() {
  if (isTauri()) {
    const { error } = await signInSocial({
      authClient: auth.client,
      provider: "github",
    });
    if (error) throw createError(error);
  } else {
    const { data, error } = await auth.signIn.social({
      provider: "github",
    });
    if (error) throw createError(error);
  }
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

// const doc = await useCrdt();
// const undoManager = await useCrdtUndoManager();

// const changes = ref(doc.getAllChanges());

// doc.subscribe(() => {
//   changes.value = doc.getAllChanges();
// });
</script>

<template>
  <div class="flex h-full flex-col gap-4">
    <md-outlined-card v-if="usage && quota" class="flex flex-col gap-2 p-4">
      <div class="label-large flex justify-between">
        <strong>{{ $t("components.sync.local-quota") }}</strong>

        <span>{{ formatBytes(usage) }} / {{ formatBytes(quota) }}</span>
      </div>

      <md-linear-progress :value="usage / quota" />
    </md-outlined-card>

    <md-filled-tonal-button v-if="user" @click="logout">
      {{ $t("components.sync.logout") }}
    </md-filled-tonal-button>
    <md-filled-button v-else @click="login">
      {{ $t("components.sync.continue-with-github") }}
    </md-filled-button>

    <!-- <md-filled-card class="flex flex-1 flex-col gap-3 p-3">
      <div v-for="([peer, peerChanges], i) of changes.entries()" :key="i">
        <md-elevated-card
          v-for="(change, i) of peerChanges"
          :key="i"
          class="flex flex-col gap-3 p-3"
        >
          <span v-if="change.peer" class="flex items-center gap-2">
            <mx-icon name="p2p" />
            <strong>Peer:</strong> {{ change.peer }}
          </span>

          <span v-if="change.counter" class="flex items-center gap-2">
            <mx-icon name="scoreboard" />
            <strong>Counter:</strong> {{ change.counter }}
          </span>

          <span v-if="change.lamport" class="flex items-center gap-2">
            <mx-icon name="circles_ext" />
            <strong>Lamport:</strong> {{ change.lamport }}
          </span>

          <span v-if="change.timestamp" class="flex items-center gap-2">
            <mx-icon name="save_clock" />
            <strong>Timestamp:</strong> {{ $d(change.timestamp) }}
          </span>

          <span v-if="change.message" class="flex items-center gap-2">
            <mx-icon name="short_text" />
            <strong>Message:</strong> {{ change.message }}
          </span>
        </md-elevated-card>
      </div>
    </md-filled-card> -->
  </div>
</template>
