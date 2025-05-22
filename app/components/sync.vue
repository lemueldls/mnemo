<script setup lang="ts">
import { signInSocial } from "@daveyplate/better-auth-tauri";

const auth = useAuth();

async function login() {
  const { data, error } = await signInSocial({
    authClient: auth.client,
    provider: "github",
  });

  console.log({ data, error });
}
</script>

<template>
  <h3 class="m3-display-small">Sync</h3>

  <div class="text-m3-error p-4">TODO</div>

  <md-filled-button @click="login"> Continue with GitHub </md-filled-button>
</template>
