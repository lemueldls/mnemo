import type { MapOldSources, MapSources } from "@vueuse/shared";

import type {
  WatchCallback,
  WatchOptions,
  WatchSource,
  WatchStopHandle,
} from "vue";

export function watchImmediateAsync<T extends Readonly<WatchSource<unknown>[]>>(
  source: [...T],
  cb: WatchCallback<MapSources<T>, MapOldSources<T, true>>,
  options?: Omit<WatchOptions<true>, "immediate">,
): Promise<WatchStopHandle>;
export function watchImmediateAsync<T>(
  source: WatchSource<T>,
  cb: WatchCallback<T, T | undefined>,
  options?: Omit<WatchOptions<true>, "immediate">,
): Promise<WatchStopHandle>;
export function watchImmediateAsync<T extends object>(
  source: T,
  cb: WatchCallback<T, T | undefined>,
  options?: Omit<WatchOptions<true>, "immediate">,
): Promise<WatchStopHandle>;

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export async function watchImmediateAsync<T = any>(
  source: T,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  cb: any,
  options?: Omit<WatchOptions<true>, "immediate">,
): Promise<WatchStopHandle> {
  return new Promise<WatchStopHandle>((resolve) => {
    let stop: WatchStopHandle;
    // eslint-disable-next-line prefer-const
    stop = watchImmediate(
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      source as any,
      (newValue, oldValue, onCleanup) => {
        cb(newValue, oldValue, onCleanup);
        resolve(stop);
      },
      { ...options },
    );
  });
}
