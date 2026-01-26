<script lang="ts" setup>
import { match } from "ts-pattern";

definePageMeta({ layout: "blank" });

const token = usePageRouteQuery("token").value as string;
const redirect = usePageRouteQuery("redirect").value as string;
const platform = usePageRouteQuery("platform").value as string;

if (token) {
  useApiToken().value = encodeURIComponent(token);

  const auth = useAuth();
  await auth.fetchSession();
}

const bearerToken = useApiToken().value!;

const { idle } = useIdle();

match(platform)
  .with("windows", "darwin", "linux", "android", "ios", () => {
    window.location.href = `mnemo://auth/confirm?token=${bearerToken}&redirect=${encodeURIComponent(redirect)}`;

    whenever(idle, () => window.close(), { immediate: true });
  })
  // .with("android", "ios", () => {
  //   window.open(
  //     `/auth/confirm?token=${bearerToken}&redirect=${encodeURIComponent(redirect)}`,
  //     "_self",
  //   );
  // })
  .otherwise(async () => {
    const redirectUrl = new URL(redirect);

    if (redirectUrl.origin === useRequestURL().origin)
      await navigateTo(redirectUrl.href.replace(redirectUrl.origin, ""));
    else {
      redirectUrl.searchParams.set("token", bearerToken);

      window.location.href = new URL(
        `/auth/confirm?token=${bearerToken}&redirect=${encodeURIComponent(redirect)}`,
        redirectUrl,
      ).href;
    }
  });
</script>

<template>
  <div />
</template>
