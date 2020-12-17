import 'package:flutter/material.dart';
import 'package:n3_mobile/pages/panel/base.dart';

class PanelSummary extends StatefulWidget implements PanelItem {
  String get label => 'summary';
  IconData get icon => Icons.wifi;

  @override
  State createState() => _State();
}

class _State extends State {
  @override
  Widget build(BuildContext context) {
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
        ],
      ),
    );
  }
}
