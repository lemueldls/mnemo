<script setup lang="ts">
definePageMeta({ layout: "auth" });

const email = ref("");
const password = ref("");

const router = useRouter();

const { auth } = useSupabaseClient();

async function login() {
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
}

async function register() {
  await auth.signUp({ email: email.value, password: password.value });
}
</script>

<template>
  <m3-elevated-card class="gap-4 w-120">
    <h1 class="m3-title-large">Login</h1>

    <md-outlined-text-field
      :value="email"
      @input="email = $event.target.value" label="Email" type="email"
    />

    <md-outlined-text-field
      label="Password"
      type="password"
      :value="password"
      @input="password = $event.target.value"
    />

    <md-filled-button @click="login"> Login </md-filled-button>

    <md-filled-tonal-button @click="register">
      Register
    </md-filled-tonal-button>
  </m3-elevated-card>
</template>
