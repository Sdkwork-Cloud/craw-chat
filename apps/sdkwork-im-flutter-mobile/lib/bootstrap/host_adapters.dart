class ImHostAdapters {
  const ImHostAdapters();
}

ImHostAdapters? _activeHostAdapters;

ImHostAdapters registerHostAdapters() {
  _activeHostAdapters ??= const ImHostAdapters();
  return _activeHostAdapters!;
}

ImHostAdapters getHostAdapters() => registerHostAdapters();
