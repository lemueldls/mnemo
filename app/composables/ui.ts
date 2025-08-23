export const useNewSpaceOpen = createSharedComposable(() => ref(false));
export const useEditingTask = createSharedComposable(() => ref<Task>());
