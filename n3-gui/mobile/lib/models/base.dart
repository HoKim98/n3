import 'dart:convert';

abstract class DBTable<K> {
  const DBTable();

  K get id;

  Map<String, dynamic> toJson();

  String toJsonEncoded() => json.encode(toJson());

  @override
  String toString() {
    return toJson().toString();
  }
}
