import 'package:shared_preferences/shared_preferences.dart';

enum Pref<T> {
  currentUser<int>('currentUser'),
  currentUserName<String>('currentUserName');

  const Pref(this._key);

  final String _key;
}

extension GenericGet<T> on Pref<T> {
  Future<T?> value() async {
    final prefs = await SharedPreferences.getInstance();

    return prefs.get(_key) as T?;
  }

  Future<void> set(T value) async {
    final prefs = await SharedPreferences.getInstance();

    switch (T) {
      case int:
        await prefs.setInt(_key, value as int);
        break;
      case String:
        await prefs.setString(_key, value as String);
        break;
      case bool:
        await prefs.setBool(_key, value as bool);
        break;
      case double:
        await prefs.setDouble(_key, value as double);
        break;
      case List<String>:
        await prefs.setStringList(_key, value as List<String>);
        break;
      default:
        throw Exception('Unsupported type');
    }
  }
}
