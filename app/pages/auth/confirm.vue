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
else {
  const redirectUrl = new URL(redirect);
  1;

  if (redirectUrl.origin === window.location.origin)
    await navigateTo(redirectUrl.pathname);
  else await navigateTo(redirect, { external: true });
}
</script>

<template>
  <div />
</template>
