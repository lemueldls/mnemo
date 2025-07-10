<script lang="ts" setup>
definePageMeta({ layout: "empty" });

const token = useRouteQuery("token");

const cookie = useCookie(
  import.meta.dev ? "mnemo.session_token" : "__Secure-mnemo.session_token",
  import.meta.dev
    ? { sameSite: "lax", secure: false, httpOnly: false }
    : { sameSite: "none", secure: true, httpOnly: false },
);

cookie.value = token.value as string;

const auth = useAuth();
await auth.fetchSession();

await navigateTo("/");
</script>

<template>
  <div />
</template>
