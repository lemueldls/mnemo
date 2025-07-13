<script lang="ts" setup>
definePageMeta({ layout: "empty" });

const token = useRouteQuery("token").value as string;
const redirect = useRouteQuery("redirect").value as string;
const platform = useRouteQuery("platform").value as string;

useApiToken().value = token as string;

const auth = useAuth();
await auth.fetchSession();

if (platform === "true")
  await navigateTo(
    `mnemo:///auth/confirm?token=${encodeURIComponent(token)}&redirect=${encodeURIComponent(redirect)}`,
    { external: true },
  );
else await navigateTo(new URL(redirect).pathname);
</script>

<template>
  <div />
</template>
