<script setup lang="ts">
defineProps<{ path: StoragePath; name: string | number }>();

const expanded = ref(false);
</script>

<template>
  <div class="relative cursor-pointer" @click="expanded = !expanded">
    <md-ripple />

    <div class="flex items-center">
      <md-icon v-if="path.kind === 'directory'">
        {{ expanded ? "arrow_drop_down" : "arrow_right" }}
      </md-icon>
      <md-icon v-else>
        <!--  -->
      </md-icon>

      <span class="flex-1 truncate">{{ name }}</span>

      <md-icon-button>
        <md-icon>more_horiz</md-icon>
      </md-icon-button>
    </div>
  </div>

  <div v-if="expanded && path.kind === 'directory'" class="pl-1">
    <storage-root :keys="path.children" />
  </div>
</template>
