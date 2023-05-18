T? mapValueOfType<T>(Map<String, dynamic> json, String key) {
  final value = json[key];
  if (value is T) {
    return value;
  }
  return null;
}
