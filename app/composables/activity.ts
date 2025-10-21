export interface Activity {
  date: [number, number, number];
  kind: ActivityKind;
}

export type ActivityKind = "task-create" | "task-delete";

export const useActivity = createSharedComposable(() =>
  useStorageMap("activity.json"),
);

// export function markActivity(kind: ActivityKind) {}

// export const useNotesVistited = createSharedComposable(() =>
//   useStorageMap("activity.json"),
// );
