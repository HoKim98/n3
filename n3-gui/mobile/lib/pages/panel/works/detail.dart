import 'package:flutter/material.dart';
import 'package:n3_mobile/models/work.dart';
import 'package:n3_mobile/pages/panel/base.dart';
import 'package:n3_mobile/pages/panel/works/log.dart';
import 'package:n3_mobile/pages/panel/works/summary.dart';

class WorkDetailPage extends StatefulWidget {
  final Work work;

  WorkDetailPage(this.work);

  @override
  State createState() => _State(ValueNotifier(work));
}

class _State extends State {
  final List<PanelItem Function(ValueNotifier<Work>)> _itemsRaw = [
    (work) => WorkDetailSummary(work),
    (work) => WorkDetailLog(work),
  ];
  List<PanelItem> _items;

  ValueNotifier<Work> work;

  int _currentIndex = 0;

  _State(this.work);

  @override
  void initState() {
    super.initState();
    this._items = _itemsRaw.map((e) => e(work)).toList();
    _update();
  }

  @override
  void dispose() {
    super.dispose();
    this.work.dispose();
    this.work = null;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Work'),
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

  void _update() async {
    while (true) {
      await Future.delayed(const Duration(seconds: 1));
      if (!mounted || this.work == null) break;

      final work = await Work.get(context, this.work.value.id);
      if (!mounted || this.work == null) break;
      this.work.value = work;
    }
  }
}

abstract class WorkDetailState extends State {
  final ValueNotifier<Work> work;

  WorkDetailState(this.work);

  @override
  void initState() {
    super.initState();
    this.work.addListener(_updateWork);
  }

  void _updateWork() {
    if (!mounted || this.work == null) return;
    setState(() {});
  }
}
