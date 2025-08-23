export interface Task {
  id: string;
  spaceId: string;
  pinned: boolean;
  createdAt: number;
}

export const useTasks = createSharedComposable(() =>
  useStorageMap<{ [id: string]: Task }>("tasks", {}),
);
