export const useNotifications = createSharedComposable(() => {
  const notifications = reactive(new Map<string, Notification>());
  let counter = 0;

  const createNotification = (message: string, options: NotificationOptions = {}): string => {
    const {
      type = "info",
      actions,
      dismissible = !!actions?.length || type !== "info",
      duration = dismissible ? null : 5000,
    } = options;

    const id = "n" + ++counter;
    const notification: Notification = {
      id,
      message,
      type,
      dismissible,
      duration,
      actions,
    };
    notifications.set(id, notification);

    if (duration !== null) {
      setTimeout(() => {
        notifications.delete(id);
      }, duration);
    }

    return id;
  };

  return {
    notifications,
    createNotification,
    dismiss(id: string) {
      notifications.delete(id);
    },
    dismissAll() {
      notifications.clear();
    },
  };
});

export interface Notification {
  id: string;
  message: string;
  type: "info" | "success" | "error" | "warning";
  dismissible: boolean;
  duration: number | null;
  actions?: NotificationAction[];
}

export interface NotificationOptions {
  type?: "info" | "success" | "error" | "warning";
  dismissible?: boolean;
  duration?: number | null;
  actions?: NotificationAction[];
}

export type NotificationVariant = "primary" | "secondary" | "destructive";

export interface NotificationAction {
  label: string;
  onClick: () => void | Promise<void>;
  variant?: NotificationVariant;
}
