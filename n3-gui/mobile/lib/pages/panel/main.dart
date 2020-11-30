import 'package:flutter/material.dart';
import 'package:n3_mobile/pages/panel/base.dart';
import 'package:n3_mobile/pages/panel/jobs.dart';
import 'package:n3_mobile/pages/panel/machines.dart';
import 'package:n3_mobile/pages/panel/summnary.dart';

class PanelMainPage extends StatefulWidget {
  @override
  State createState() => _State();
}

class _State extends State {
  final List<PanelItem> _items = [
    PanelSummary(),
    PanelJobs(),
    PanelMachines(),
  ];

  int _currentIndex = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Panel'),
      ),
      body: _items[_currentIndex],
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: _currentIndex,
        items: _items
            .map(
              (e) => BottomNavigationBarItem(
                label: e.label,
                icon: Icon(e.icon),
              ),
            )
            .toList(),
        onTap: (i) => setState(() => _currentIndex = i),
      ),
    );
  }
}
