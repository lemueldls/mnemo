import { getLocalTimeZone } from "@internationalized/date";

export const useTimeZone = createSharedComposable(() => getLocalTimeZone());
