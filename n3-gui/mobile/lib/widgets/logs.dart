import 'package:flutter/material.dart';
import 'package:n3_mobile/models/base.dart';

mixin LogState<T extends DBTable> implements State {
  T get entity;

  @override
  Widget build(BuildContext context) {
    final data = this.entity.toJson().entries.toList();

    return ListView.builder(
      itemCount: data.length,
      itemBuilder: (context, index) => _entryToWidget(data[index]),
    );
  }

  Widget _entryToWidget(MapEntry<String, dynamic> entry) {
    if (entry.value is Map<String, dynamic>) {
      return _entryToWidgetNested(entry);
    }

    return _entryToWidgetSimple(entry);
  }

  Widget _entryToWidgetSimple(MapEntry<String, dynamic> entry) {
    return ListTile(
      title: Text(
        entry.key,
        overflow: TextOverflow.ellipsis,
      ),
      subtitle: Text(
        (entry.value != null ? entry.value.toString() : 'null'),
        overflow: TextOverflow.ellipsis,
      ),
    );
  }

  Widget _entryToWidgetNested(MapEntry<String, dynamic> entry) {
    final items = Map<String, dynamic>.from(entry.value).entries.toList();

    return ExpansionTile(
      initiallyExpanded: true,
      title: Text(entry.key),
      children: items.map(_entryToWidget).toList(),
    );
  }
}
