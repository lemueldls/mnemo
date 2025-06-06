<script setup lang="ts">
definePageMeta({ layout: "empty" });

const session = useUserSession();

watchEffect(() => {
  console.log({
    ready: session.ready.value,
    loggedIn: session.loggedIn.value,
    user: session.user.value,
    session: { ...session.session.value },
  });
});

// window.open("mnemo://confirm")
// await navigateTo("/");
</script>
