export const useNewSpaceOpen = createSharedComposable(() => ref(false));

export const useNewTaskOpen = createSharedComposable(() => ref(false));
export const useEditingTask = createSharedComposable(() => ref<Task>());
