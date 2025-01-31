export function usePageRouteQuery(name: string, defaultValue: string = "") {
  const route = useRoute();
  const { path: originalPath } = route;

  const routeQuery = ref(defaultValue);

  watchImmediate(
    () => route.query,
    () => {
      if (route.path === originalPath)
        routeQuery.value =
          [route.query?.[name]]?.flat?.()?.[0]?.toString() || defaultValue;
    },
  );

  return routeQuery;
}
