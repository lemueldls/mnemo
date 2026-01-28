export function useScrollWidth(element: MaybeRefOrGetter<Element | null | undefined>) {
  const scrollWidth = ref(0);

  let observer: ResizeObserver;
  let updateScheduled = false;

  const elementRef = toRef(element);

  const updateWidth = () => {
    const element = elementRef.value;
    if (element) scrollWidth.value = element.scrollWidth;
    updateScheduled = false;
  };

  const scheduleUpdate = () => {
    if (!updateScheduled) {
      updateScheduled = true;
      requestAnimationFrame(updateWidth);
    }
  };

  whenever(
    elementRef,
    (element) => {
      observer = new ResizeObserver(scheduleUpdate);
      if (element) {
        for (const child of element.children) observer.observe(child);

        updateWidth();
      }

      // Use a separate MutationObserver to detect added/removed nodes
      const mutationObserver = new MutationObserver(scheduleUpdate);
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
    },
    { once: true, immediate: true },
  );

  watch(elementRef, (element) => {
    if (element && observer) {
      observer.disconnect();
      observer = new ResizeObserver(scheduleUpdate);

      for (const child of element.children) observer.observe(child);

      updateWidth();
    }
  });

  return scrollWidth;
}

export function useScrollHeight(element: MaybeRefOrGetter<Element | null | undefined>) {
  const scrollHeight = ref(0);

  let observer: ResizeObserver;
  let updateScheduled = false;

  const elementRef = toRef(element);

  const updateHeight = () => {
    const element = elementRef.value;

    if (element) scrollHeight.value = element.scrollHeight;
    updateScheduled = false;
  };

  const scheduleUpdate = () => {
    if (!updateScheduled) {
      updateScheduled = true;
      requestAnimationFrame(updateHeight);
    }
  };

  whenever(
    elementRef,
    (element) => {
      observer = new ResizeObserver(scheduleUpdate);
      if (element) {
        for (const child of element.children) observer.observe(child);

        updateHeight();
      }

      // Use a separate MutationObserver to detect added/removed nodes
      const mutationObserver = new MutationObserver(scheduleUpdate);
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
    },
    { once: true, immediate: true },
  );

  watch(elementRef, (element) => {
    if (element && observer) {
      observer.disconnect();
      observer = new ResizeObserver(scheduleUpdate);

      for (const child of element.children) observer.observe(child);

      updateHeight();
    }
  });

  return scrollHeight;
}
