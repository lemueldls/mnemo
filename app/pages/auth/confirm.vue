<script lang="ts" setup>
definePageMeta({ layout: "empty" });

const session = useRouteQuery("session").value as string;
const redirect = useRouteQuery("redirect").value as string;
const platform = useRouteQuery("platform").value as string;

if (session) {
  const auth = useAuth();
  await auth.fetchSession();
}

const activeSession = useApiSession().value!;

if (platform === "true") {
  window.location.href = `mnemo:///auth/confirm?session=${encodeURIComponent(activeSession)}&redirect=${encodeURIComponent(redirect)}`;
} else {
  const redirectUrl = new URL(redirect);

  if (redirectUrl.origin === useRequestURL().origin)
    await navigateTo(redirectUrl.pathname);
  else {
    redirectUrl.searchParams.set("session", activeSession);

    window.location.href = new URL(
      `/auth/confirm?session=${encodeURIComponent(activeSession)}&redirect=${encodeURIComponent(redirect)}`,
      redirectUrl,
    ).href;
  }
}
</script>

<template>
  <div />
</template>
