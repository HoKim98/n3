import 'package:flutter/material.dart';
import 'package:n3_mobile/pages/panel/base.dart';

class PanelJobs extends StatefulWidget implements PanelItem {
  String get label => 'jobs';
  IconData get icon => Icons.rotate_right_outlined;

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
            'No problem.',
            style: Theme.of(context).textTheme.headline4,
          ),
        ],
      ),
    );
  }
}
