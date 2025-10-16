export function useScrollHeight(
  element: MaybeRefOrGetter<Element | undefined>,
) {
  const scrollHeight = ref(0);

  let observer: ResizeObserver;

  const elementRef = toRef(element);

  const updateHeight = () => {
    const element = elementRef.value;
    if (element) scrollHeight.value = element.scrollHeight;
  };

  tryOnMounted(() => {
    const element = elementRef.value;

    observer = new ResizeObserver(updateHeight);
    if (element) {
      for (const child of element.children) {
        observer.observe(child);
      }

      updateHeight();
    }

    // Use a separate MutationObserver to detect added/removed nodes
    const mutationObserver = new MutationObserver(updateHeight);
    if (element) {
      mutationObserver.observe(element, {
        childList: true,
        // subtree: true,
      });
    }

    tryOnUnmounted(() => {
      observer.disconnect();
      mutationObserver.disconnect();
    });
  });

  watch(elementRef, (element) => {
    if (element && observer) {
      observer.disconnect();
      observer = new ResizeObserver(updateHeight);

      for (const child of element.children) {
        observer.observe(child);
      }

      updateHeight();
    }
  });

  return scrollHeight;
}
