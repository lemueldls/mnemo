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
</script>

<template>
  <h3 class="m3-display-small">Sync</h3>

  <div class="text-m3-error p-4">TODO</div>

  <md-filled-tonal-button v-if="user" @click="logout">
    Logout
  </md-filled-tonal-button>
  <md-filled-button v-else disabled @click="login">
    Continue with GitHub
  </md-filled-button>
</template>
