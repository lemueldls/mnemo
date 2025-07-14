<script lang="ts" setup>
definePageMeta({ layout: "empty" });

const token = useRouteQuery("token").value as string;
const redirect = useRouteQuery("redirect").value as string;
const platform = useRouteQuery("platform").value as string;

if (token) {
  useApiToken().value = token;

  const auth = useAuth();
  await auth.fetchSession();
}

const activeToken = useApiToken().value!;

if (platform === "true") {
  window.location.href = `mnemo:///auth/confirm?token=${encodeURIComponent(activeToken)}&redirect=${encodeURIComponent(redirect)}`;
} else {
  const redirectUrl = new URL(redirect);

  if (redirectUrl.origin === useRequestURL().origin)
    await navigateTo(redirectUrl.pathname);
  else {
    redirectUrl.searchParams.set("token", activeToken);

    window.location.href = new URL(
      `/auth/confirm?token=${encodeURIComponent(activeToken)}&redirect=${encodeURIComponent(redirect)}`,
      redirectUrl,
    ).href;
  }
}
</script>

<template>
  <div />
</template>
