import 'package:flutter/material.dart';
import 'package:n3_mobile/models/work.dart';
import 'package:n3_mobile/pages/panel/base.dart';

class WorkDetailSummary extends StatefulWidget implements PanelItem {
  String get label => 'summary';
  IconData get icon => Icons.wifi;

  final ValueNotifier<Work> work;

  WorkDetailSummary(this.work);

  @override
  State createState() => _State(work);
}

class _State extends State {
  final ValueNotifier<Work> work;

  _State(this.work);

  @override
  void initState() {
    super.initState();
    this.work.addListener(() => setState(() {}));
  }

  @override
  Widget build(BuildContext context) {
    final work = this.work.value;

    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          Icon(
            Icons.check,
            color: Colors.green,
            size: 120,
          ),
          Text(
            'No problems.',
            style: Theme.of(context).textTheme.headline4,
          ),
          Text(
            '${work.id}',
            style: Theme.of(context).textTheme.headline4,
          ),
        ],
      ),
    );
  }
}
