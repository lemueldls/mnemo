export interface Task {
  id: string;
  spaceId: string;
  pinned: boolean;
  createdAt: number;
}

export const useTasks = createSharedComposable(async () => {
  const tasks = await useStorageMap<{ [id: string]: Task }>("tasks.json", {});

  return extendRef(tasks, {
    sorted: computed(() =>
      // Sort tasks by pinned status and creation date
      Object.values(tasks.value).sort((a, b) => {
        // Pinned tasks first
        if (a.pinned && !b.pinned) return -1;
        if (!a.pinned && b.pinned) return 1;

        // Then by creation date (oldest first)
        return a.createdAt - b.createdAt;
      }),
    ),
  });
});
