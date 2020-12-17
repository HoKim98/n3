import 'package:flutter/material.dart';
import 'package:n3_mobile/models/work.dart';
import 'package:n3_mobile/pages/panel/base.dart';
import 'package:n3_mobile/pages/panel/works/detail.dart';
import 'package:n3_mobile/widgets/logs.dart';

class WorkDetailLog extends StatefulWidget implements PanelItem {
  String get label => 'logs';
  IconData get icon => Icons.list;

  final ValueNotifier<Work> work;

  WorkDetailLog(this.work);

  @override
  State createState() => _State(work);
}

class _State extends WorkDetailState with LogState<Work> {
  Work get entity => work.value;

  _State(ValueNotifier<Work> work) : super(work);
}
