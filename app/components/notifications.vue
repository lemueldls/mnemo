<script setup lang="ts">
import { parseBackticks } from "~/lib/editor/highlight";

const { notifications, dismiss } = useNotifications();

const variantToButtonComponentMap: { [V in NotificationVariant]: string } = {
  primary: "md-filled-button",
  secondary: "md-filled-tonal-button",
  destructive: "md-text-button",
};

const notificationList = computed(() => Array.from(notifications.values()));
const loadingActions = reactive(new Set<string>());

async function handleAction(
  notificationId: string,
  onClick: () => void | Promise<void>,
) {
  const actionKey = `${notificationId}`;
  loadingActions.add(actionKey);

  try {
    await onClick();
  } finally {
    loadingActions.delete(actionKey);
  }

  dismiss(notificationId);
}

function withCodeBlocks(message: string) {
  const span = document.createElement("span");
  parseBackticks(message, span);

  return span.innerHTML;
}
</script>

<template>
  <div v-if="notifications.size > 0" class="notifications-container">
    <md-elevation />

    <transition-group name="notification">
      <div
        v-for="notification in notificationList"
        :key="notification.id"
        :class="['notification-content', `notification-${notification.type}`]"
        role="alert"
        :aria-live="notification.type === 'error' ? 'assertive' : 'polite'"
      >
        <div class="notification-body">
          <span
            class="notification-message flex-1"
            v-html="withCodeBlocks(notification.message)"
          />

          <div
            v-if="notification.actions && notification.actions.length > 0"
            class="notification-actions"
          >
            <component
              :is="variantToButtonComponentMap[action.variant || 'primary']"
              v-for="(action, index) in notification.actions"
              :key="index"
              :disabled="loadingActions.has(notification.id)"
              @click="handleAction(notification.id, action.onClick)"
            >
              {{ action.label }}
            </component>
          </div>
        </div>

        <md-icon-button
          v-if="notification.dismissible"
          @click="dismiss(notification.id)"
          aria-label="Dismiss notification"
        >
          <md-icon>close</md-icon>
        </md-icon-button>
      </div>
    </transition-group>
  </div>
</template>

<style lang="scss" scoped>
@use "sass:map";
@use "@material/web/tokens";

$_md-sys-motion: tokens.md-sys-motion-values();

.notifications-container {
  @apply bg-surface-container-highest text-on-surface medium:bottom-1.5 animate-slide-in-right pointer-events-auto fixed right-1.5 flex flex-col gap-3 rounded-lg p-3;

  --md-elevation-level: 3;

  bottom: calc(4.75rem + env(safe-area-inset-bottom));

  width: 24rem;
  max-width: calc(100vw - 0.75rem);
  animation-timing-function: cubic-bezier(0.42, 1.67, 0.21, 0.9);
  animation-duration: 350ms;
}

.notification-content {
  @apply bg-surface-container-highest flex gap-1 rounded-lg;

  transition: all map.get($_md-sys-motion, duration-short3) ease-out;
}

.notification-body {
  @apply flex flex-1 flex-col gap-2;
}

.notification-message {
  @apply body-large m-0 break-words;

  word-break: break-word;
  overflow-wrap: break-word;

  :deep(code) {
    @apply bg-surface-container text-wrap rounded px-1 font-mono;
  }
}

.notification-actions {
  @apply flex gap-2;
}

/* Info state */
.notification-info {
  .notification-icon {
    @apply text-primary;
  }
}

/* Success state */
.notification-success {
  .notification-icon {
    @apply text-primary;
  }
}

/* Warning state */
.notification-warning {
  .notification-icon {
    @apply text-secondary;
  }
}

/* Error state */
.notification-error {
  .notification-icon {
    @apply text-error;
  }

  .notification-message {
    @apply text-error;
  }
}

.notification-enter-active {
  animation: slideIn map.get($_md-sys-motion, duration-short3)
    map.get($_md-sys-motion, easing-emphasized-decelerate);
}

.notification-leave-active {
  animation: slideOut map.get($_md-sys-motion, duration-short3)
    map.get($_md-sys-motion, easing-emphasized-accelerate);
}

.notification-move {
  transition: all map.get($_md-sys-motion, duration-short3)
    map.get($_md-sys-motion, easing-emphasized);
}
</style>
