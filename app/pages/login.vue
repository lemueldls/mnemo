<script setup lang="ts">
definePageMeta({ layout: "auth" });

const email = ref("");
const password = ref("");

const loginLoading = ref(false);
const registerLoading = ref(false);
const loading = computed(() => loginLoading.value || registerLoading.value);

const { auth } = useSupabaseClient();
const router = useRouter();

async function login() {
  loginLoading.value = true;

  const { data, error } = await auth.signInWithPassword({
    email: email.value,
    password: password.value,
  });

  if (error) {
    console.error(error);
  }

  if (data) {
    console.log({ data });
    router.push("/");
  }

  loginLoading.value = false;
}

async function register() {
  registerLoading.value = true;
  await auth.signUp({ email: email.value, password: password.value });
  registerLoading.value = false;
}
</script>

<template>
  <m3-elevated-card class="gap-4 w-120">
    <h1 class="m3-title-large">Login</h1>

    <md-outlined-text-field
      :value="email"
      @input="email = $event.target.value"
      label="Email"
      type="email"
      :disabled="loading"
    />

    <md-outlined-text-field
      label="Password"
      type="password"
      :value="password"
      @input="password = $event.target.value"
      :disabled="loading"
    />

    <md-filled-button @click="login" :disabled="loading">
      <template v-if="loginLoading">
        <md-progress-circular indeterminate />
      </template>
      <template v-else> Login </template>
    </md-filled-button>

    <md-filled-tonal-button @click="register" :disabled="loading">
      <template v-if="registerLoading">
        <md-progress-circular indeterminate />
      </template>
      <template v-else> Register </template>
    </md-filled-tonal-button>
  </m3-elevated-card>
</template>
