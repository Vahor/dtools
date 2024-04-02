interface ResourceManager<T> {
  load: () => Promise<void>;
  get: () => T;
}

