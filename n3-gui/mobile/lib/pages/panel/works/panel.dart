import 'package:flutter/material.dart';
import 'package:flutter_slidable/flutter_slidable.dart';
import 'package:intl/intl.dart';
import 'package:n3_mobile/models/work.dart';
import 'package:n3_mobile/pages/panel/base.dart';

class PanelWorks extends StatefulWidget implements PanelItem {
  String get label => 'works';
  IconData get icon => Icons.rotate_right_outlined;

  @override
  State createState() => _State();
}

class _State extends State {
  static List<Work> works = [];

  @override
  void initState() {
    super.initState();
    this._update();
  }

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: _State.works.length,
      itemBuilder: (context, index) {
        final work = _State.works[index];

        final command = work.command.commandToString().toUpperCase();
        final exec = work.exec;

        final isError = work.status.errorMsg != null;
        final isRunning = !isError && work.status.isRunning;

        var dateEnd = work.status.dateEnd != null
            ? DateFormat("yyyy-MM-dd HH:mm:ss").format((work.status.dateEnd))
            : 'Unknown';

        final icon = isError
            ? CircleAvatar(
                backgroundColor: Colors.red,
                foregroundColor: Colors.white,
                child: Icon(Icons.priority_high),
              )
            : isRunning
                ? CircleAvatar(
                    backgroundColor: Colors.orange,
                    foregroundColor: Colors.white,
                    child: Icon(Icons.model_training),
                  )
                : CircleAvatar(
                    backgroundColor: Colors.green,
                    foregroundColor: Colors.white,
                    child: Icon(Icons.check),
                  );

        return Slidable(
          key: ValueKey(work.id),
          actionPane: SlidableDrawerActionPane(),
          secondaryActions: [
            IconSlideAction(
              caption: 'More',
              color: Colors.grey.shade200,
              icon: Icons.more_horiz,
              onTap: () => _onWorkMore(work),
            ),
            IconSlideAction(
              caption: 'Delete',
              color: Colors.red,
              icon: Icons.delete,
              onTap: () => _onWorkDelete(work),
            ),
          ],
          child: ListTile(
            leading: icon,
            title: Text(
              '[$command] $exec',
              overflow: TextOverflow.ellipsis,
            ),
            subtitle: Text(
              isError ? work.status.errorMsg : 'ETA $dateEnd',
              overflow: TextOverflow.ellipsis,
            ),
            onTap: () => _onWorkMore(work),
          ),
        );
      },
    );
  }

  void _onWorkMore(final Work work) {
    Navigator.of(context).pushNamed(
      '/panel/work',
      arguments: work,
    );
  }

  void _onWorkDelete(final Work work) {
    print(work.toJson());
  }

  Future<void> _update() async {
    while (true) {
      await Future.delayed(const Duration(seconds: 1));
      if (!mounted) break;

      final works = await Work.getList(context);
      if (works == null || !mounted) break;
      setState(() => _State.works = works);
    }
  }
}
